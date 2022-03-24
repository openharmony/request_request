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
#ifndef UPLOAD_TIMER_INFO_
#define UPLOAD_TIMER_INFO_

#include "itimer_info.h"

namespace OHOS::Request::Upload {
using TimerOutFunc = std::function<void()>;
class UploadTimerInfo : public MiscServices::ITimerInfo {
public:
    UploadTimerInfo() {}
    virtual ~UploadTimerInfo() {}

    void OnTrigger() override;
    void SetType(const int &type) override;
    void SetRepeat(bool repeat) override;
    void SetInterval(const uint64_t &interval) override;
    void SetWantAgent(std::shared_ptr<OHOS::AbilityRuntime::WantAgent::WantAgent> wantAgent) override;
    void SetCallbackInfo(TimerOutFunc &&callBack);

private:
    TimerOutFunc callBack_ = nullptr;
};
} // namespace OHOS::Request::Upload
#endif