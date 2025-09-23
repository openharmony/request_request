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

#ifndef REQUEST_INOTIFY_EVENT_LISTENT_H
#define REQUEST_INOTIFY_EVENT_LISTENT_H

#include <atomic>
#include <cstdint>
#include <filesystem>
#include <functional>
#include <string>

namespace fs = std::filesystem;

namespace rust {
inline namespace cxxbridge1 {
template<typename T> class Box;
} // namespace cxxbridge1
} // namespace rust

namespace OHOS::Request {

struct DirRebuilder;

class DirectoryMonitor {
public:
    DirectoryMonitor(const std::string &directory, rust::Box<DirRebuilder> callback);
    DirectoryMonitor(DirectoryMonitor &&other) = delete;
    DirectoryMonitor &operator=(DirectoryMonitor &&other) = delete;

    DirectoryMonitor(const DirectoryMonitor &) = delete;
    DirectoryMonitor &operator=(const DirectoryMonitor &) = delete;
    ~DirectoryMonitor();
    void Start();
    void Stop();

private:
    int Run();
    int SetupInotify();
    int SetupEpoll();
    int AddToEpoll(int fd, uint32_t events);
    void HandleInotify();
    void Cleanup();

    fs::path directory_;
    DirRebuilder *callback_;

    int inotify_fd_ = -1;
    int epoll_fd_ = -1;
    int watch_descriptor_ = -1;
    std::atomic<bool> running_{ false };
};
} // namespace OHOS::Request

#endif