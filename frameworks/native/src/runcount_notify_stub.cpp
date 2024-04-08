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

#include "runcount_notify_stub.h"

#include <cstdint>
#include <memory>
#include <thread>

#include "base/request/request/interfaces/inner_kits/running_count/include/running_task_count.h"
#include "download_server_ipc_interface_code.h"
#include "log.h"
#include "parcel_helper.h"
#include "request_running_task_count.h"
#include "string_ex.h"

namespace OHOS::Request {
void RunCountNotifyStub::CallBack(const Notify &notify)
{
}

void RunCountNotifyStub::Done(const TaskInfo &taskInfo)
{
}

sptr<RunCountNotifyStub> RunCountNotifyStub::GetInstance()
{
    static sptr<RunCountNotifyStub> instance(new RunCountNotifyStub());
    return instance;
}

int32_t RunCountNotifyStub::OnRemoteRequest(
    uint32_t code, MessageParcel &data, MessageParcel &reply, MessageOption &option)
{
    auto descriptorToken = data.ReadInterfaceToken();
    if (descriptorToken != GetDescriptor()) {
        REQUEST_HILOGE("Remote descriptor not the same as local descriptor.");
        return IPCObjectStub::OnRemoteRequest(code, data, reply, option);
    }

    if (code == static_cast<uint32_t>(RequestNotifyInterfaceCode::REQUEST_NOTIFY_RUNCOUNT)) {
        OnCallBack(data);
        return ERR_NONE;
    } else {
        REQUEST_HILOGE("Other interface code received, check needed.");
        return IPCObjectStub::OnRemoteRequest(code, data, reply, option);
    }
}

void RunCountNotifyStub::OnCallBack(MessageParcel &data)
{
    REQUEST_HILOGD("Receive callback");
    int runCount = data.ReadInt64();
    REQUEST_HILOGI("RunCount num %{public}d", runCount);

    FwkRunningTaskCountManager::GetInstance()->SetCount(runCount);
    FwkRunningTaskCountManager::GetInstance()->NotifyAllObservers();
}

} // namespace OHOS::Request
