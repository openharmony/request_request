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

#ifndef OHOS_REQUEST_CJ_RESPONSE_LISTENER_H
#define OHOS_REQUEST_CJ_RESPONSE_LISTENER_H

#include <list>
#include <mutex>
#include <string>

#include "cj_listener_list.h"
#include "i_response_listener.h"
#include "request_common.h"

namespace OHOS::CJSystemapi::Request {
using OHOS::Request::Response;

class CJResponseListener : public OHOS::Request::IResponseListener,
                           public ListenerList,
                           public std::enable_shared_from_this<CJResponseListener> {
public:
    explicit CJResponseListener(const std::string &taskId) : ListenerList(taskId, SubscribeType::RESPONSE) {}

    void AddListener(std::function<void(CResponse)> cb, CFunc cbId);
    void RemoveListener(CFunc cbId = nullptr);
    void OnResponseReceive(const std::shared_ptr<Response> &response) override;

private:
};

} // namespace OHOS::CJSystemapi::Request

#endif // OHOS_REQUEST_JS_RESPONSE_LISTENER_H