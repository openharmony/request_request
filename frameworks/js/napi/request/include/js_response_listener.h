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

#ifndef OHOS_REQUEST_JS_RESPONSE_LISTENER_H
#define OHOS_REQUEST_JS_RESPONSE_LISTENER_H

#include "i_response_listener.h"
#include "listener_list.h"

namespace OHOS::Request {
class JSResponseListener
    : public IResponseListener
    , public ListenerList
    , public std::enable_shared_from_this<JSResponseListener> {
public:
    JSResponseListener(napi_env env, const std::string &taskId) : ListenerList(env, taskId, SubscribeType::RESPONSE)
    {
    }
    napi_status AddListener(napi_value cb);
    napi_status RemoveListener(napi_value cb = nullptr);
    void OnResponseReceive(const std::shared_ptr<Response> &response) override;

private:
    std::mutex responseMutex_;
    std::shared_ptr<Response> response_;
};

} // namespace OHOS::Request

#endif // OHOS_REQUEST_JS_RESPONSE_LISTENER_H