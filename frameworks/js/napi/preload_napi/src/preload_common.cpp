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

#include <cstdint>
#include <mutex>
#include <string>

#include "base/request/request/common/include/constant.h"
#include "js_native_api.h"
#include "js_native_api_types.h"
#include "napi/native_common.h"
#include "napi_utils.h"

static const std::string SSL_TYPE_TLS = "TLS";
static const std::string SSL_TYPE_TLCP = "TLCP";

namespace OHOS::Request {

inline napi_status SetPerformanceField(napi_env env, napi_value performance, double field_value, const char *js_name)
{
    napi_value value;
    napi_status status = napi_create_double(env, field_value, &value);
    if (status != napi_ok) {
        return status;
    }

    return napi_set_named_property(env, performance, js_name, value);
}

void SetOptionsHeaders(napi_env env, napi_value arg, std::unique_ptr<PreloadOptions> &options)
{
    napi_value headers = nullptr;
    if (napi_get_named_property(env, arg, "headers", &headers) == napi_ok
        && GetValueType(env, headers) == napi_valuetype::napi_object) {
        auto names = GetPropertyNames(env, headers);
        for (auto name : names) {
            auto value = GetPropertyValue(env, headers, name);
            options->headers.emplace_back(std::make_pair(name, value));
        }
    }
}

void SetOptionsSslType(napi_env env, napi_value arg, std::unique_ptr<PreloadOptions> &options)
{
    napi_value napiSslType = GetNamedProperty(env, arg, "sslType");
    if (napiSslType != nullptr) {
        std::string sslType = GetStringValueWithDefault(env, napiSslType);
        if (sslType == SSL_TYPE_TLS) {
            options->sslType = SslType::TLS;
        } else if (sslType == SSL_TYPE_TLCP) {
            options->sslType = SslType::TLCP;
        } else {
            options->sslType = SslType::TLS;
        }
    } else {
        options->sslType = SslType::DEFAULT;
    }
}

void GetCacheStrategy(napi_env env, napi_value arg, bool &isUpdate)
{
    napi_value napiCacheStrategy = GetNamedProperty(env, arg, "cacheStrategy");
    if (napiCacheStrategy != nullptr) {
        if (GetValueType(env, napiCacheStrategy) != napi_number) {
            isUpdate = true;
            return;
        }
        int64_t numCacheStrategy = GetValueNum(env, napiCacheStrategy);
        if (numCacheStrategy == static_cast<int64_t>(CacheStrategy::LAZY)) {
            isUpdate = false;
        } else {
            isUpdate = true;
        }
    } else {
        isUpdate = true;
    }
}

bool BuildInfoResource(napi_env env, const CppDownloadInfo &result, napi_value &jsInfo)
{
    napi_status status;
    napi_value resource;
    status = napi_create_object(env, &resource);
    if (status != napi_ok) {
        return false;
    }

    napi_value sizeValue;
    status = napi_create_int64(env, result.resource_size(), &sizeValue);
    if (status != napi_ok) {
        return false;
    }

    status = napi_set_named_property(env, resource, "size", sizeValue);
    if (status != napi_ok) {
        return false;
    }

    status = napi_set_named_property(env, jsInfo, "resource", resource);
    if (status != napi_ok) {
        return false;
    }

    return true;
}

bool BuildInfoNetwork(napi_env env, const CppDownloadInfo &result, napi_value &jsInfo)
{
    napi_status status;
    napi_value network;
    status = napi_create_object(env, &network);
    if (status != napi_ok) {
        return false;
    }
    if (!result.server_addr().empty()) {
        napi_value ipValue;
        status = napi_create_string_utf8(env, result.server_addr().c_str(), NAPI_AUTO_LENGTH, &ipValue);
        if (status != napi_ok) {
            return false;
        }
        status = napi_set_named_property(env, network, "ip", ipValue);
        if (status != napi_ok) {
            return false;
        }
    }
    std::vector<std::string> dnsServers = result.dns_servers();
    napi_value dnsArray;
    status = napi_create_array_with_length(env, dnsServers.size(), &dnsArray);
    if (status != napi_ok) {
        return false;
    }
    for (size_t i = 0; i < dnsServers.size(); i++) {
        const std::string &server = dnsServers[i];
        napi_value dnsItem;
        status = napi_create_string_utf8(env, server.c_str(), NAPI_AUTO_LENGTH, &dnsItem);
        if (status != napi_ok) {
            return false;
        }

        status = napi_set_element(env, dnsArray, i, dnsItem);
        if (status != napi_ok) {
            return false;
        }
    }
    status = napi_set_named_property(env, network, "dnsServers", dnsArray);
    if (status != napi_ok) {
        return false;
    }
    status = napi_set_named_property(env, jsInfo, "network", network);
    if (status != napi_ok) {
        return false;
    }
    return true;
}

bool BuildInfoPerformance(napi_env env, const CppDownloadInfo &result, napi_value &jsInfo)
{
    napi_status status;
    napi_value performance;
    status = napi_create_object(env, &performance);
    if (status != napi_ok) {
        return false;
    }

    if ((status = SetPerformanceField(env, performance, result.dns_time(), "dnsTime")) != napi_ok) {
        return false;
    }
    if ((status = SetPerformanceField(env, performance, result.connect_time(), "connectTime")) != napi_ok) {
        return false;
    }
    if ((status = SetPerformanceField(env, performance, result.tls_time(), "tlsTime")) != napi_ok) {
        return false;
    }
    if ((status = SetPerformanceField(env, performance, result.first_send_time(), "firstSendTime")) != napi_ok) {
        return false;
    }
    if ((status = SetPerformanceField(env, performance, result.first_recv_time(), "firstReceiveTime")) != napi_ok) {
        return false;
    }
    if ((status = SetPerformanceField(env, performance, result.total_time(), "totalTime")) != napi_ok) {
        return false;
    }
    if ((status = SetPerformanceField(env, performance, result.redirect_time(), "redirectTime")) != napi_ok) {
        return false;
    }

    status = napi_set_named_property(env, jsInfo, "performance", performance);
    if (status != napi_ok) {
        return false;
    }

    return true;
}
} // namespace OHOS::Request