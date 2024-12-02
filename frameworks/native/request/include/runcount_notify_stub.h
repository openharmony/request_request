/*
 * Copyright (c) 2024 Huawei Device Co., Ltd.
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

#ifndef RUNCOUNT_NOTIFY_STUB_H
#define RUNCOUNT_NOTIFY_STUB_H

#include <unistd.h>

#include <cstdint>
#include <fstream>
#include <memory>

#include "iremote_stub.h"
#include "js_common.h"
#include "notify_interface.h"
#include "visibility.h"

namespace OHOS::Request {
class RunCountNotifyStub : public IRemoteStub<NotifyInterface> {
public:
    static sptr<RunCountNotifyStub> GetInstance();
    explicit RunCountNotifyStub() = default;
    ~RunCountNotifyStub() override = default;
    REQUEST_API int32_t OnRemoteRequest(
        uint32_t code, MessageParcel &data, MessageParcel &reply, MessageOption &option) override;
    virtual void CallBack(const Notify &notify) override;
    virtual void Done(const TaskInfo &taskInfo) override;

private:
    void OnCallBack(MessageParcel &data);
};
} // namespace OHOS::Request
#endif // RUNCOUNT_NOTIFY_STUB_H