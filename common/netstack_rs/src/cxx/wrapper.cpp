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

#include "wrapper.h"

#include <memory>

#include "http_client_error.h"
#include "wrapper.rs.h"
namespace OHOS::Request {
using namespace OHOS::NetStack::HttpClient;

void OnCallback(const std::shared_ptr<HttpClientTask> &task, rust::Box<CallbackWrapper> callback)
{
    CallbackWrapper *raw_ptr = callback.into_raw();
    auto shared = std::shared_ptr<CallbackWrapper>(
        raw_ptr, [](CallbackWrapper *ptr) { rust::Box<CallbackWrapper>::from_raw(ptr); });
    task->OnSuccess([shared](const HttpClientRequest &request, const HttpClientResponse &response) {
        shared->on_success(request, response);
    });
    task->OnFail([shared](const HttpClientRequest &request, const HttpClientResponse &response,
                     const HttpClientError &error) { shared->on_fail(request, response, error); });
    task->OnCancel([shared](const HttpClientRequest &request, const HttpClientResponse &response) {
        shared->on_cancel(request, response);
    });
    auto weak = task->weak_from_this();
    task->OnDataReceive([shared, weak](const HttpClientRequest &, const uint8_t *data, size_t size) {
        auto httpTask = weak.lock();
        if (httpTask != nullptr) {
            shared->on_data_receive(httpTask, data, size);
        }
    });
    task->OnProgress([shared](const HttpClientRequest &, u_long dlTotal, u_long dlNow, u_long ulTotal, u_long ulNow) {
        shared->on_progress(dlTotal, dlNow, ulTotal, ulNow);
    });
};

rust::vec<rust::string> GetHeaders(HttpClientResponse &response)
{
    rust::vec<rust::string> ret;

    if (response.GetHeaders().empty()) {
        response.ParseHeaders();
    }
    std::map<std::string, std::string> headers = response.GetHeaders();
    for (auto header : headers) {
        ret.emplace_back(header.first);
        ret.emplace_back(header.second);
    }
    return ret;
};

} // namespace OHOS::Request