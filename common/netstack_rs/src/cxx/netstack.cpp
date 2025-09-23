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

#include "dns_config_client.h"
#include "http_client_request.h"
#include "net_conn_client.h"
#include "net_handle.h"

#ifdef __cplusplus
extern "C" {
#endif
int32_t NetSysGetResolvConf(uint16_t netId, struct ResolvConfig *config);
#ifdef __cplusplus
}
#endif

namespace OHOS::Request {
using namespace OHOS::NetStack::HttpClient;
using namespace OHOS::NetManagerStandard;

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

rust::vec<rust::string> GetHeaders(HttpClientResponse &response)
{
    rust::vec<rust::string> ret;

    if (response.GetHeaders().empty()) {
        response.ParseHeaders();
    }
    std::map<std::string, std::string> headers = response.GetHeaders();
    for (auto header : headers) {
        ret.emplace_back(rust::string::lossy(header.first));
        ret.emplace_back(rust::string::lossy(header.second));
    }
    return ret;
};

rust::vec<rust::string> GetResolvConf()
{
    rust::vec<rust::string> dns;
    NetHandle handle;
    auto code = NetConnClient::GetInstance().GetDefaultNet(handle);
    if (code != 0) {
        return dns;
    }
    int32_t netId = handle.GetNetId();
    if (netId < 0 || netId > UINT16_MAX) {
        return dns;
    }
    ResolvConfig config = {};
    int ret = NetSysGetResolvConf(netId, &config);
    if (ret != 0) {
        return dns;
    }

    for (size_t i = 0; i < MAX_SERVER_NUM; i++) {
        if (config.nameservers[i][0] == '\0') {
            continue;
        }
        size_t len = strnlen(config.nameservers[i], MAX_SERVER_LENGTH + 1);
        dns.push_back(rust::string::lossy(config.nameservers[i], len));
    }
    return dns;
}

rust::string GetHttpAddress(const HttpClientResponse &response)
{
    auto statistics = response.GetHttpStatistics();
    return rust::string::lossy(statistics.serverIpAddress.address_);
}

} // namespace OHOS::Request