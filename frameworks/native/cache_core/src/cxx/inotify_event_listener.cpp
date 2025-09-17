/*
 * Copyright (C) 2025 Huawei Device Co., Ltd.
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#include "inotify_event_listener.h"

#include <sys/epoll.h>
#include <sys/inotify.h>
#include <sys/types.h>
#include <unistd.h>

#include <cerrno>
#include <climits>
#include <csignal>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <filesystem>

#include "cxx.h"
#include "ffrt.h"
#include "log.h"
#include "wrapper.rs.h"

namespace OHOS::Request {
DirectoryMonitor::DirectoryMonitor(const std::string &directory, rust::Box<DirRebuilder> callback)
{
    directory_ = fs::path(directory);
    callback_ = callback.into_raw();
}

DirectoryMonitor::~DirectoryMonitor()
{
    Stop();
    Cleanup();
    rust::Box<DirRebuilder>::from_raw(callback_);
}

void DirectoryMonitor::Start()
{
    if (running_) {
        return;
    }
    if (SetupInotify() == -1) {
        Cleanup();
        return;
    }
    if (SetupEpoll() == -1) {
        Cleanup();
        return;
    }
    running_ = true;
    Run();
    Cleanup();
}

void DirectoryMonitor::Stop()
{
    if (!running_) {
        return;
    }
    running_ = false;
}

int DirectoryMonitor::SetupInotify()
{
    int ret = -1;
    // create inotify instance.
    ret = inotify_init1(IN_NONBLOCK | IN_CLOEXEC);
    if (ret == -1) {
        REQUEST_HILOGE("inotify_init1 fail, err : %{public}s", strerror(errno));
        return ret;
    }
    inotify_fd_ = ret;
    // add directory event watcher.
    ret = inotify_add_watch(inotify_fd_, directory_.c_str(), IN_DELETE_SELF | IN_MOVE_SELF);
    if (ret == -1) {
        REQUEST_HILOGE("inotify_add_watch fail, err : %{public}s", strerror(errno));
    }
    return ret;
}

int DirectoryMonitor::SetupEpoll()
{
    int ret = -1;
    // create epoll instance.
    ret = epoll_create1(0);
    if (ret == -1) {
        REQUEST_HILOGE("create epoll instance fail, code : %{public}s", strerror(errno));
        return ret;
    }
    epoll_fd_ = ret;
    ret = AddToEpoll(inotify_fd_, EPOLLIN);
    if (ret == -1) {
        REQUEST_HILOGE("add inotify fd to epoll fail, code : %{public}s", strerror(errno));
    }
    return ret;
}

int DirectoryMonitor::AddToEpoll(int fd, uint32_t events)
{
    epoll_event ev{};
    ev.events = events;
    ev.data.fd = fd;
    // Register a new file descriptor (such as socket, pipe, file, etc.) to the epoll instance.
    return epoll_ctl(epoll_fd_, EPOLL_CTL_ADD, fd, &ev);
}

int DirectoryMonitor::Run()
{
    constexpr int MAX_EVENT = 10;
    epoll_event events[MAX_EVENT];
    while (running_) {
        // wait for epoll events.
        int num_events = epoll_wait(epoll_fd_, events, MAX_EVENT, -1);
        if (num_events == -1) {
            // Upon receiving an interrupt signal, it does not return but continues to execute.
            if (errno == EINTR) {
                continue;
            }
            REQUEST_HILOGE("epoll_wait fail, errno : %{public}s", strerror(errno));
            running_ = false;
            return -1;
        }
        for (int i = 0; i < num_events; ++i) {
            if (events[i].data.fd == inotify_fd_) {
                HandleInotify();
            }
        }
    }
    return 0;
}

void DirectoryMonitor::HandleInotify()
{
    constexpr size_t EVENT_SIZE = sizeof(inotify_event);
    constexpr size_t BUF_LEN = 1024 * (EVENT_SIZE + NAME_MAX + 1);

    char buffer[BUF_LEN];
    ssize_t len = read(inotify_fd_, buffer, BUF_LEN);
    if (len == -1) {
        // Ignore this two errno in non-blocking mode.
        if (errno == EAGAIN || errno == EWOULDBLOCK) {
            return;
        }
        REQUEST_HILOGE("read inotify_fd_ fail, err : %{public}s", strerror(errno));
        running_ = false;
        return;
    }

    for (char *ptr = buffer; ptr < buffer + len;) {
        auto *event = reinterpret_cast<inotify_event *>(ptr);
        ptr += EVENT_SIZE + event->len;

        if (event->mask & (IN_DELETE_SELF | IN_MOVE_SELF)) {
            if (callback_ == nullptr) {
                running_ = false;
                return;
            }
            callback_->remove_store_dir();
            running_ = false;
        }
    }
}

void DirectoryMonitor::Cleanup()
{
    if (watch_descriptor_ != -1) {
        inotify_rm_watch(inotify_fd_, watch_descriptor_);
    }
    if (inotify_fd_ != -1) {
        close(inotify_fd_);
    }
    if (epoll_fd_ != -1) {
        close(epoll_fd_);
    }
    inotify_fd_ = -1;
    epoll_fd_ = -1;
    watch_descriptor_ = -1;
}

} // namespace OHOS::Request