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

#include "upload_timer_info.h"

namespace OHOS::Request::Upload {
void UploadTimerInfo::OnTrigger()
{
    if (callBack_ != nullptr) {
        callBack_();
    }
}

void UploadTimerInfo::SetCallbackInfo(TimerOutFunc &&callBack)
{
    callBack_ = callBack;
}

void UploadTimerInfo::SetType(const int &_type)
{
    type = _type;
}

void UploadTimerInfo::SetRepeat(bool _repeat)
{
    repeat = _repeat;
}
void UploadTimerInfo::SetInterval(const uint64_t &_interval)
{
    interval = _interval;
}
void UploadTimerInfo::SetWantAgent(std::shared_ptr<OHOS::AbilityRuntime::WantAgent::WantAgent> _wantAgent)
{
    wantAgent = _wantAgent;
}
} // namespace OHOS::Request::Upload