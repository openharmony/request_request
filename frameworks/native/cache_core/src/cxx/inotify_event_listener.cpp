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
/**
 * @class DirectoryMonitor
 * @brief Monitors a directory for changes using inotify and epoll
 */
DirectoryMonitor::DirectoryMonitor(const std::string &directory, rust::Box<DirRebuilder> callback)
{
    // Convert the directory string to filesystem path
    directory_ = fs::path(directory);
    // Take ownership of the Rust callback by converting it to a raw pointer
    callback_ = callback.into_raw();
}

DirectoryMonitor::~DirectoryMonitor()
{
    // Ensure monitoring is stopped
    Stop();
    // Clean up system resources
    Cleanup();
    // Reconstruct the Rust Box to properly deallocate the callback
    rust::Box<DirRebuilder>::from_raw(callback_);
}

/**
 * @brief Starts the directory monitoring
 *
 * Initializes inotify and epoll, then enters the monitoring loop
 */
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

/**
 * @brief Stops the directory monitoring
 *
 * Signals the monitoring loop to exit
 */
void DirectoryMonitor::Stop()
{
    if (!running_) {
        return;
    }
    running_ = false;
}

/**
 * @brief Sets up inotify for directory monitoring
 * @return int File descriptor on success, -1 on failure
 */
int DirectoryMonitor::SetupInotify()
{
    int ret = -1;
    // Create inotify instance with non-blocking and close-on-exec flags
    ret = inotify_init1(IN_NONBLOCK | IN_CLOEXEC);
    if (ret == -1) {
        REQUEST_HILOGE("inotify_init1 fail, err : %{public}s", strerror(errno));
        return ret;
    }
    inotify_fd_ = ret;
    // Add watch for directory deletion/move events
    ret = inotify_add_watch(inotify_fd_, directory_.c_str(), IN_DELETE_SELF | IN_MOVE_SELF);
    if (ret == -1) {
        REQUEST_HILOGE("inotify_add_watch fail, err : %{public}s", strerror(errno));
    }
    watch_descriptor_ = ret;
    return ret;
}

/**
 * @brief Sets up epoll for monitoring inotify events
 * @return int File descriptor on success, -1 on failure
 */
int DirectoryMonitor::SetupEpoll()
{
    int ret = -1;
    // Create epoll instance
    ret = epoll_create1(0);
    if (ret == -1) {
        REQUEST_HILOGE("create epoll instance fail, code : %{public}s", strerror(errno));
        return ret;
    }
    epoll_fd_ = ret;
    // Add inotify fd to epoll for read events
    ret = AddToEpoll(inotify_fd_, EPOLLIN);
    if (ret == -1) {
        REQUEST_HILOGE("add inotify fd to epoll fail, code : %{public}s", strerror(errno));
    }
    return ret;
}

/**
 * @brief Adds a file descriptor to epoll monitoring
 * @param fd File descriptor to monitor
 * @param events Events to monitor for (EPOLLIN, etc.)
 * @return int 0 on success, -1 on failure
 */
int DirectoryMonitor::AddToEpoll(int fd, uint32_t events)
{
    epoll_event ev{};
    ev.events = events;
    ev.data.fd = fd;
    // Register the file descriptor with epoll
    return epoll_ctl(epoll_fd_, EPOLL_CTL_ADD, fd, &ev);
}

/**
 * @brief Main monitoring loop
 * @return int 0 on normal exit, -1 on error
 */
int DirectoryMonitor::Run()
{
    constexpr int MAX_EVENT = 10;
    epoll_event events[MAX_EVENT];
    while (running_) {
        // Wait for events with no timeout (blocks until events occur)
        int num_events = epoll_wait(epoll_fd_, events, MAX_EVENT, -1);
        if (num_events == -1) {
            // Handle interrupt signal
            if (errno == EINTR) {
                continue;
            }
            REQUEST_HILOGE("epoll_wait fail, errno : %{public}s", strerror(errno));
            running_ = false;
            return -1;
        }
        // Process all received events
        for (int i = 0; i < num_events; ++i) {
            if (events[i].data.fd == inotify_fd_) {
                HandleInotify();
            }
        }
    }
    return 0;
}

/**
 * @brief Handles inotify events
 */
void DirectoryMonitor::HandleInotify()
{
    constexpr size_t EVENT_SIZE = sizeof(inotify_event);
    constexpr size_t BUF_LEN = 1024 * (EVENT_SIZE + NAME_MAX + 1);

    // Read events from inotify
    char buffer[BUF_LEN];
    ssize_t len = read(inotify_fd_, buffer, BUF_LEN);
    if (len == -1) {
        // Handle non-blocking mode errors
        if (errno == EAGAIN || errno == EWOULDBLOCK) {
            return;
        }
        REQUEST_HILOGE("read inotify_fd_ fail, err : %{public}s", strerror(errno));
        running_ = false;
        return;
    }

    // Process each event in the buffer
    for (char *ptr = buffer; ptr < buffer + len;) {
        auto *event = reinterpret_cast<inotify_event *>(ptr);
        ptr += EVENT_SIZE + event->len;

        // Check for directory deletion/move events
        if (event->mask & (IN_DELETE_SELF | IN_MOVE_SELF)) {
            if (callback_ == nullptr) {
                running_ = false;
                return;
            }
            // Notify Rust callback about directory removal
            callback_->remove_store_dir();
            running_ = false;
        }
    }
}

/**
 * @brief Cleans up system resources
 *
 * Removes inotify watches and closes file descriptors
 */
void DirectoryMonitor::Cleanup()
{
    // Remove inotify watch if it was created
    if (watch_descriptor_ != -1) {
        inotify_rm_watch(inotify_fd_, watch_descriptor_);
    }
    // Close inotify file descriptor if it was opened
    if (inotify_fd_ != -1) {
        close(inotify_fd_);
    }
    // Close epoll file descriptor if it was opened
    if (epoll_fd_ != -1) {
        close(epoll_fd_);
    }
    // Reset all descriptors to invalid state
    inotify_fd_ = -1;
    epoll_fd_ = -1;
    watch_descriptor_ = -1;
}

} // namespace OHOS::Request