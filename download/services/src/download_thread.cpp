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
DownloadThread::DownloadThread()
    : isRunning_(false), thread_(Run, this)
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
    auto mgr  = DownloadServiceManager::GetInstance();
    while (this_->isRunning_) {
        if (mgr != nullptr) {
            if (!mgr->ProcessTask()) {
                std::this_thread::sleep_for(std::chrono::seconds(mgr->GetInterval()));
                std::this_thread::yield();
            }
        }
    }
}
} // namespace OHOS::Request::Download