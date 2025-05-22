/*
 * Copyright (c) 2023 Huawei Device Co., Ltd.
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

#ifndef REQUEST_COMMON_UTILS_H
#define REQUEST_COMMON_UTILS_H

#include <cstdint>
#include <unordered_map>
#include <unordered_set>
#include <vector>

#include "log.h"
#include "parameter.h"
#include "request_common.h"
#include "visibility.h"

namespace OHOS::Request {
class CommonUtils {
private:
    static constexpr const char *REASON_OK_INFO = "Task successful";
    static constexpr const char *TASK_SURVIVAL_ONE_MONTH_INFO = "The task has not been completed for a month yet";
    static constexpr const char *WAITTING_NETWORK_ONE_DAY_INFO = "The task waiting for network recovery has not been "
                                                                 "completed for a day yet";
    static constexpr const char *STOPPED_NEW_FRONT_TASK_INFO = "Stopped by a new front task";
    static constexpr const char *RUNNING_TASK_MEET_LIMITS_INFO = "Too many task in running state";
    static constexpr const char *USER_OPERATION_INFO = "User operation";
    static constexpr const char *APP_BACKGROUND_OR_TERMINATE_INFO = "The app is background or terminate";
    static constexpr const char *NETWORK_OFFLINE_INFO = "NetWork is offline";
    static constexpr const char *UNSUPPORTED_NETWORK_TYPE_INFO = "NetWork type not meet the task config";
    static constexpr const char *BUILD_CLIENT_FAILED_INFO = "Build client error";
    static constexpr const char *BUILD_REQUEST_FAILED_INFO = "Build request error";
    static constexpr const char *GET_FILESIZE_FAILED_INFO = "Failed because cannot get the file size from the server "
                                                            "and "
                                                            "the precise is setted true by user";
    static constexpr const char *CONTINUOUS_TASK_TIMEOUT_INFO = "Continuous processing task time out";
    static constexpr const char *CONNECT_ERROR_INFO = "Connect error";
    static constexpr const char *REQUEST_ERROR_INFO = "Request error";
    static constexpr const char *UPLOAD_FILE_ERROR_INFO = "There are some files upload failed";
    static constexpr const char *REDIRECT_ERROR_INFO = "Redirect error";
    static constexpr const char *PROTOCOL_ERROR_INFO = "Http protocol error";
    static constexpr const char *IO_ERROR_INFO = "Io Error";
    static constexpr const char *UNSUPPORT_RANGE_REQUEST_INFO = "Range request not supported";
    static constexpr const char *OTHERS_ERROR_INFO = "Some other error occured";
    static constexpr const char *ACCOUNT_STOPPED_INFO = "Account stopped";
    static constexpr const char *NETWORK_CHANGED_INFO = "Network changed";
    static constexpr const char *DNS_INFO = "DNS error";
    static constexpr const char *TCP_INFO = "TCP error";
    static constexpr const char *SSL_INFO = "TSL/SSL error";
    static constexpr const char *INSUFFICIENT_SPACE_INFO = "Insufficient space error";
    static constexpr const char *NETWORK_APP_INFO = "NetWork is offline and the app is background or terminate";
    static constexpr const char *NETWORK_ACCOUNT_INFO = "NetWork is offline and the account is stopped";
    static constexpr const char *APP_ACCOUNT_INFO = "The account is stopped and the app is background or terminate";
    static constexpr const char *NETWORK_ACCOUNT_APP_INFO = "NetWork is offline and the account is stopped and the "
                                                            "app is"
                                                            "background or terminate";
    static constexpr const char *LOW_SPEED_INFO = "Below low speed limit";

public:
    REQUEST_API static Faults GetFaultByReason(Reason code);
    REQUEST_API static std::string GetMsgByReason(Reason code);
};
} // namespace OHOS::Request
#endif // COMMON_UTILS_H
