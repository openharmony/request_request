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

#include "preload_common.h"

#include <cstdint>
#include <limits>
#include <mutex>
#include <string>

#include "base/request/request/common/include/constant.h"
#include "js_native_api.h"
#include "js_native_api_types.h"
#include "napi/native_common.h"

static const std::string SSL_TYPE_TLS = "TLS";
static const std::string SSL_TYPE_TLCP = "TLCP";

constexpr int64_t MIN_RETRY_COUNT = 0;
constexpr int64_t MAX_RETRY_COUNT = 10;

constexpr int64_t MIN_NETWORK_CHECK_TIMEOUT = 0;
constexpr int64_t MAX_NETWORK_CHECK_TIMEOUT = 20;
constexpr int64_t MIN_HTTP_TOTAL_TIMEOUT = 1;
// httpTotalTimeout upper limit: u32::MAX / 1000 (since API unit is seconds, netstack unit is ms)
constexpr int64_t MAX_HTTP_TOTAL_TIMEOUT = static_cast<int64_t>(std::numeric_limits<uint32_t>::max() / 1000);

// Sentinel value for "not set by user" - use -1 to indicate use global default
constexpr int32_t SENTINEL_NOT_SET = -1;

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
    // undefined/null 视为缺失，使用默认值
    if (IsValueMissingOrSkipped(env, napiSslType)) {
        options->sslType = SslType::DEFAULT;
        return;
    }
    // 类型必须是 string
    if (GetValueType(env, napiSslType) != napi_string) {
        options->sslType = SslType::DEFAULT;
        return;
    }
    std::string sslType = GetStringValueWithDefault(env, napiSslType);
    if (sslType == SSL_TYPE_TLS) {
        options->sslType = SslType::TLS;
    } else if (sslType == SSL_TYPE_TLCP) {
        options->sslType = SslType::TLCP;
    } else {
        options->sslType = SslType::TLS;
    }
}

void GetCacheStrategy(napi_env env, napi_value arg, bool &isUpdate)
{
    napi_value napiCacheStrategy = GetNamedProperty(env, arg, "cacheStrategy");
    // undefined/null 视为缺失，使用默认值
    if (IsValueMissingOrSkipped(env, napiCacheStrategy)) {
        isUpdate = true;
        return;
    }
    // 类型必须是 number
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

bool SetOptionsRetry(napi_env env, napi_value arg, std::unique_ptr<PreloadOptions> &options)
{
    options->retry.maxRetryCount = SENTINEL_NOT_SET;

    napi_value retry = GetNamedProperty(env, arg, "retry");
    if (IsValueMissingOrSkipped(env, retry)) {
        return true;
    }
    if (GetValueType(env, retry) != napi_valuetype::napi_object) {
        return false;
    }

    napi_value maxRetryCount = GetNamedProperty(env, retry, "maxRetryCount");
    if (IsValueMissingOrSkipped(env, maxRetryCount)) {
        return true;
    }
    if (GetValueType(env, maxRetryCount) != napi_number) {
        return false;
    }

    int64_t value = GetValueNum(env, maxRetryCount);
    if (value < MIN_RETRY_COUNT || value > MAX_RETRY_COUNT) {
        return false;
    }
    options->retry.maxRetryCount = static_cast<int32_t>(value);
    return true;
}

bool SetOptionsTimeout(napi_env env, napi_value arg, std::unique_ptr<PreloadOptions> &options)
{
    options->timeout.networkCheckTimeout = SENTINEL_NOT_SET;
    options->timeout.httpTotalTimeout = SENTINEL_NOT_SET;

    napi_value timeout = GetNamedProperty(env, arg, "timeout");
    if (IsValueMissingOrSkipped(env, timeout)) {
        return true;
    }
    if (GetValueType(env, timeout) != napi_valuetype::napi_object) {
        return false;
    }

    napi_value networkCheckTimeout = GetNamedProperty(env, timeout, "networkCheckTimeout");
    if (!IsValueMissingOrSkipped(env, networkCheckTimeout)) {
        if (GetValueType(env, networkCheckTimeout) != napi_number) {
            return false;
        }
        int64_t value = GetValueNum(env, networkCheckTimeout);
        if (value < MIN_NETWORK_CHECK_TIMEOUT || value > MAX_NETWORK_CHECK_TIMEOUT) {
            return false;
        }
        options->timeout.networkCheckTimeout = static_cast<int32_t>(value);
    }

    napi_value httpTotalTimeout = GetNamedProperty(env, timeout, "httpTotalTimeout");
    if (!IsValueMissingOrSkipped(env, httpTotalTimeout)) {
        if (GetValueType(env, httpTotalTimeout) != napi_number) {
            return false;
        }
        int64_t value = GetValueNum(env, httpTotalTimeout);
        if (value < MIN_HTTP_TOTAL_TIMEOUT || value > MAX_HTTP_TOTAL_TIMEOUT) {
            return false;
        }
        options->timeout.httpTotalTimeout = static_cast<int32_t>(value);
    }
    return true;
}
} // namespace OHOS::Request