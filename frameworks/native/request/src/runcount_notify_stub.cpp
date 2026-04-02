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

#include "accesstoken_kit.h"
#include "base/request/request/interfaces/inner_kits/running_count/include/running_task_count.h"
#include "download_server_ipc_interface_code.h"
#include "ipc_skeleton.h"
#include "log.h"
#include "parcel_helper.h"
#include "request_running_task_count.h"
#include "string_ex.h"

namespace OHOS::Request {
constexpr int32_t EXPECT_UID = 3815;
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

    if (CheckCallingPermission(code)) {
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
    REQUEST_HILOGD("RunCount num %{public}d", runCount);

    FwkRunningTaskCountManager::GetInstance()->SetCount(runCount);
    FwkRunningTaskCountManager::GetInstance()->NotifyAllObservers();
}

bool RunCountNotifyStub::CheckCallingPermission(uint32_t code)
{
    auto callerToken = IPCSkeleton::GetCallingTokenID();
    auto tokenType = Security::AccessToken::AccessTokenKit::GetTokenTypeFlag(callerToken);
    auto callerUid = IPCSkeleton::GetCallingUid();
    bool res = true;
    if (code != static_cast<uint32_t>(RequestNotifyInterfaceCode::REQUEST_NOTIFY_RUNCOUNT)) {
        REQUEST_HILOGE("Access forbidden, the interface code is %{public}d", code);
        res = false;
    }
    if (callerUid != EXPECT_UID) {
        REQUEST_HILOGE("Access forbidden, the uid is %{public}d", callerUid);
        res = false;
    }
    if (tokenType != Security::AccessToken::ATokenTypeEnum::TOKEN_NATIVE) {
        REQUEST_HILOGE("Access forbidden, the token type is %{public}d", tokenType);
        res = false;
    }
    return res;
}

} // namespace OHOS::Request
