/*
 * Copyright (C) 2023 Huawei Device Co., Ltd.
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

#ifndef DOWNLOAD_SERVER_IPC_INTERFACE_CODE_H
#define DOWNLOAD_SERVER_IPC_INTERFACE_CODE_H

/* SAID: 3706*/
namespace OHOS {
namespace Request {
enum class RequestInterfaceCode {
    CMD_REQUEST = 0,
    CMD_PAUSE,
    CMD_QUERY,
    CMD_QUERYMIMETYPE,
    CMD_REMOVE,
    CMD_RESUME,
    CMD_START,
    CMD_STOP,
    CMD_SHOW,
    CMD_TOUCH,
    CMD_SEARCH,
    CMD_GETTASK,
    CMD_CLEAR,
    CMD_OPENCHANNEL,
    CMD_SUBSCRIBE,
    CMD_UNSUBSCRIBE,
    CMD_SUB_RUNCOUNT,
    CMD_UNSUB_RUNCOUNT,
};

enum class RequestNotifyInterfaceCode {
    REQUEST_NOTIFY = 0,
    REQUEST_DONE_NOTIFY,
    REQUEST_NOTIFY_RUNCOUNT,
};
} // namespace Request
} // namespace OHOS

#endif //DOWNLOAD_SERVER_IPC_INTERFACE_CODE_H