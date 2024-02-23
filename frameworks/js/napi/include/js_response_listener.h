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

#include <list>
#include <string>

#include "i_response_listener.h"
#include "napi/native_api.h"
#include "napi_utils.h"

namespace OHOS::Request {
class JSResponseListener
    : public IResponseListener
    , public std::enable_shared_from_this<JSResponseListener> {
public:
    JSResponseListener(napi_env env, const std::string &taskId) : env_(env), taskId_(taskId)
    {
    }
    napi_status AddListener(napi_value cb);
    napi_status RemoveListener(napi_value cb = nullptr);
    void OnResponseReceive(const std::shared_ptr<Response> &response) override;
    bool HasListener();

private:
    bool IsListenerAdded(napi_value cb);

private:
    const napi_env env_;
    const std::string taskId_;
    std::list<napi_ref> allCb_;
};

} // namespace OHOS::Request

#endif // OHOS_REQUEST_JS_RESPONSE_LISTENER_H