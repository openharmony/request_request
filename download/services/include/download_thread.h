/*
 * Copyright (c) 2022 Huawei Device Co., Ltd.
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

#ifndef DOWNLOAD_THREAD_H
#define DOWNLOAD_THREAD_H

#include <memory>
#include <thread>

namespace OHOS::Request::Download {
class DownloadServiceManager;

class DownloadThread final {
public:
    explicit DownloadThread();
    ~DownloadThread();

    void Start();
    void Stop();

private:
    static void Run(DownloadThread *this_);

private:
    bool isRunning_;
    std::thread thread_;
};
} // namespace OHOS::Request::Download
#endif // DOWNLOAD_THREAD_H
