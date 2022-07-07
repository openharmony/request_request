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

#include "download_thread.h"

#include "download_service_manager.h"

namespace OHOS::Request::Download {
DownloadThread::DownloadThread(DownloadServiceManager* mgr)
    : isRunning_(false), thread_(Run, this), mgr_(mgr)
{
}

DownloadThread::~DownloadThread()
{
    thread_.join();
}

void DownloadThread::Start()
{
}
void DownloadThread::Stop()
{
    isRunning_ = false;
}

void DownloadThread::Run(DownloadThread *this_)
{
    if (this_ == nullptr) {
        return;
    }
    this_->isRunning_ = true;
    while (this_->isRunning_) {
        if (this_->mgr_ != nullptr) {
            if (!this_->mgr_->ProcessTask()) {
                std::this_thread::sleep_for(std::chrono::seconds(this_->mgr_->GetInterval()));
                std::this_thread::yield();
            }
        }
    }
}
} // namespace OHOS::Request::Download