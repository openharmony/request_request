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

#ifndef OHOS_REQUEST_LISTENER_LIST_H
#define OHOS_REQUEST_LISTENER_LIST_H

#include <list>
#include <mutex>
#include <string>

#include "napi/native_api.h"
#include "napi_utils.h"

namespace OHOS::Request {
class ListenerList {
public:
    ListenerList(napi_env env, const std::string &taskId, const SubscribeType &type)
        : env_(env), taskId_(taskId), type_(type)
    {
    }
    bool HasListener();

protected:
    bool IsListenerAdded(napi_value cb);
    void OnMessageReceive(napi_value *value, uint32_t paramNumber);
    napi_status AddListenerInner(napi_value cb);
    napi_status RemoveListenerInner(napi_value cb = nullptr);

protected:
    const napi_env env_;
    const std::string taskId_;
    const SubscribeType type_;
    std::list<std::pair<bool, napi_ref>> allCb_;
    std::recursive_mutex allCbMutex_;
    std::atomic<uint32_t> validCbNum{ 0 };
};

} // namespace OHOS::Request

#endif // OHOS_REQUEST_LISTENER_LIST_H
