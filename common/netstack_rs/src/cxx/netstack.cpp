/*
 * Copyright (C) 2025 Huawei Device Co., Ltd.
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

#include "netstack.h"

#include <cstring>
#include <memory>

#include "http_client_request.h"

namespace OHOS::Request {
static const std::string SSL_TYPE_TLS = "TLS";
static const std::string SSL_TYPE_TLCP = "TLCP";

void SetRequestSslType(HttpClientRequest &request, const std::string &sslType)
{
    if (sslType == SSL_TYPE_TLS) {
        request.SetSslType(SslType::TLS);
    } else if (sslType == SSL_TYPE_TLCP) {
        request.SetSslType(SslType::TLCP);
    }
    return;
}

} // namespace OHOS::Request