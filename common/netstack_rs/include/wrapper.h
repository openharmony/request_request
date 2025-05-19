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

#ifndef REQUEST_PRE_DOWNLOAD_WRAPPER_H
#define REQUEST_PRE_DOWNLOAD_WRAPPER_H

#include <memory>

#include "cxx.h"
#include "http_client.h"
#include "http_client_request.h"
#include "http_client_response.h"
#include "http_client_task.h"
namespace OHOS::Request {
using namespace OHOS::NetStack::HttpClient;
struct CallbackWrapper;

void OnCallback(const std::shared_ptr<HttpClientTask> &task, rust::Box<CallbackWrapper> callback);

inline std::unique_ptr<HttpClientRequest> NewHttpClientRequest()
{
    return std::make_unique<HttpClientRequest>();
}

inline void SetBody(HttpClientRequest &request, const uint8_t *data, size_t size)
{
    request.SetBody(data, size);
}

inline std::shared_ptr<HttpClientTask> NewHttpClientTask(const HttpClientRequest &request)
{
    auto &session = NetStack::HttpClient::HttpSession::GetInstance();
    return session.CreateTask(request);
}

rust::vec<rust::string> GetHeaders(HttpClientResponse &response);

} // namespace OHOS::Request

#endif