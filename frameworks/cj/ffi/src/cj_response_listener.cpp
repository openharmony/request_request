/*
 * Copyright (C) 2024 Huawei Device Co., Ltd.
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

#include "cj_response_listener.h"
#include "log.h"
#include "request_manager.h"

namespace OHOS::CJSystemapi::Request {

using OHOS::Request::RequestManager;

void CJResponseListener::AddListener(std::function<void(CResponse)> cb, CFunc cbId)
{
    this->AddListenerInner(cb, cbId);
    if (this->validCbNum == 1 && this->type_ != SubscribeType::REMOVE) {
        RequestManager::GetInstance()->AddListener(this->taskId_, this->type_, shared_from_this());
    }
}

void CJResponseListener::RemoveListener(CFunc cbId)
{
    this->RemoveListenerInner(cbId);
    if (this->validCbNum == 0 && this->type_ != SubscribeType::REMOVE) {
        RequestManager::GetInstance()->RemoveListener(this->taskId_, this->type_, shared_from_this());
    }
}

void CJResponseListener::OnResponseReceive(const std::shared_ptr<Response> &response)
{
    REQUEST_HILOGI("CJOnRespRecv tid %{public}s", response->taskId.c_str());
    this->OnMessageReceive(response);
}

} // namespace OHOS::CJSystemapi::Request