/*
 * Copyright (c) 2025 Huawei Device Co., Ltd.
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

#include "request_common_utils.h"

namespace OHOS::Request {
Faults CommonUtils::GetFaultByReason(Reason code)
{
    static const std::unordered_map<Reason, Faults> reasonFaultMap = {
        { REASON_OK, Faults::OTHERS },
        { TASK_SURVIVAL_ONE_MONTH, Faults::OTHERS },
        { WAITTING_NETWORK_ONE_DAY, Faults::OTHERS },
        { STOPPED_NEW_FRONT_TASK, Faults::OTHERS },
        { RUNNING_TASK_MEET_LIMITS, Faults::OTHERS },
        { USER_OPERATION, Faults::OTHERS },
        { APP_BACKGROUND_OR_TERMINATE, Faults::OTHERS },
        { NETWORK_OFFLINE, Faults::DISCONNECTED },
        { UNSUPPORTED_NETWORK_TYPE, Faults::OTHERS },
        { BUILD_CLIENT_FAILED, Faults::PARAM },
        { BUILD_REQUEST_FAILED, Faults::PARAM },
        { GET_FILESIZE_FAILED, Faults::FSIO },
        { CONTINUOUS_TASK_TIMEOUT, Faults::TIMEOUT },
        { CONNECT_ERROR, Faults::TCP },
        { REQUEST_ERROR, Faults::PROTOCOL },
        { UPLOAD_FILE_ERROR, Faults::OTHERS },
        { REDIRECT_ERROR, Faults::REDIRECT },
        { PROTOCOL_ERROR, Faults::PROTOCOL },
        { IO_ERROR, Faults::FSIO },
        { UNSUPPORT_RANGE_REQUEST, Faults::PROTOCOL },
        { OTHERS_ERROR, Faults::OTHERS },
        { ACCOUNT_STOPPED, Faults::OTHERS },
        { NETWORK_CHANGED, Faults::OTHERS },
        { DNS, Faults::DNS },
        { TCP, Faults::TCP },
        { SSL, Faults::SSL },
        { INSUFFICIENT_SPACE, Faults::OTHERS },
        { NETWORK_APP, Faults::DISCONNECTED },
        { NETWORK_ACCOUNT, Faults::DISCONNECTED },
        { APP_ACCOUNT, Faults::OTHERS },
        { NETWORK_APP_ACCOUNT, Faults::DISCONNECTED },
        { LOW_SPEED, Faults::LOW_SPEED },
    };
    static const std::unordered_set<Faults> downgradeFaults = { Faults::PARAM, Faults::DNS, Faults::TCP, Faults::SSL,
        Faults::REDIRECT };
    constexpr int32_t detailVersion = 12;
    auto iter = reasonFaultMap.find(code);
    if (iter == reasonFaultMap.end()) {
        return Faults::OTHERS;
    }
    Faults fault = iter->second;
    int32_t sdkVersion = GetSdkApiVersion();
    REQUEST_HILOGD("GetSdkApiVersion %{public}d", sdkVersion);

    if (sdkVersion < detailVersion && downgradeFaults.count(fault)) {
        return Faults::OTHERS;
    }
    return fault;
}

std::string CommonUtils::GetMsgByReason(Reason code)
{
    static const std::unordered_map<Reason, std::string> reasonMsg = {
        { REASON_OK, REASON_OK_INFO },
        { TASK_SURVIVAL_ONE_MONTH, TASK_SURVIVAL_ONE_MONTH_INFO },
        { WAITTING_NETWORK_ONE_DAY, WAITTING_NETWORK_ONE_DAY_INFO },
        { STOPPED_NEW_FRONT_TASK, STOPPED_NEW_FRONT_TASK_INFO },
        { RUNNING_TASK_MEET_LIMITS, RUNNING_TASK_MEET_LIMITS_INFO },
        { USER_OPERATION, USER_OPERATION_INFO },
        { APP_BACKGROUND_OR_TERMINATE, APP_BACKGROUND_OR_TERMINATE_INFO },
        { NETWORK_OFFLINE, NETWORK_OFFLINE_INFO },
        { UNSUPPORTED_NETWORK_TYPE, UNSUPPORTED_NETWORK_TYPE_INFO },
        { BUILD_CLIENT_FAILED, BUILD_CLIENT_FAILED_INFO },
        { BUILD_REQUEST_FAILED, BUILD_REQUEST_FAILED_INFO },
        { GET_FILESIZE_FAILED, GET_FILESIZE_FAILED_INFO },
        { CONTINUOUS_TASK_TIMEOUT, CONTINUOUS_TASK_TIMEOUT_INFO },
        { CONNECT_ERROR, CONNECT_ERROR_INFO },
        { REQUEST_ERROR, REQUEST_ERROR_INFO },
        { UPLOAD_FILE_ERROR, UPLOAD_FILE_ERROR_INFO },
        { REDIRECT_ERROR, REDIRECT_ERROR_INFO },
        { PROTOCOL_ERROR, PROTOCOL_ERROR_INFO },
        { IO_ERROR, IO_ERROR_INFO },
        { UNSUPPORT_RANGE_REQUEST, UNSUPPORT_RANGE_REQUEST_INFO },
        { OTHERS_ERROR, OTHERS_ERROR_INFO },
        { ACCOUNT_STOPPED, ACCOUNT_STOPPED_INFO },
        { NETWORK_CHANGED, NETWORK_CHANGED_INFO },
        { DNS, DNS_INFO },
        { TCP, TCP_INFO },
        { SSL, SSL_INFO },
        { INSUFFICIENT_SPACE, INSUFFICIENT_SPACE_INFO },
        { NETWORK_APP, NETWORK_APP_INFO },
        { NETWORK_ACCOUNT, NETWORK_ACCOUNT_INFO },
        { APP_ACCOUNT, APP_ACCOUNT_INFO },
        { NETWORK_APP_ACCOUNT, NETWORK_ACCOUNT_APP_INFO },
        { LOW_SPEED, LOW_SPEED_INFO },
    };
    auto iter = reasonMsg.find(code);
    if (iter == reasonMsg.end()) {
        return "unknown";
    }
    return iter->second;
}

} // namespace OHOS::Request
