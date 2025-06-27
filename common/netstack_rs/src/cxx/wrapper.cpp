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

#include <cstring>
#include <memory>

#include "dns_config_client.h"
#include "http_client_error.h"
#include "http_client_response.h"
#include "log.h"
#include "net_conn_client.h"
#include "net_handle.h"
#include "wrapper.rs.h"

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
        REQUEST_HILOGE("Cache Download GetDefaultNet failed, code : %{public}d", code);
        return dns;
    }
    int32_t netId = handle.GetNetId();
    if (netId < 0 || netId > UINT16_MAX) {
        REQUEST_HILOGE("Cache Download GetNetId Illegal, id : %{public}d", netId);
        return dns;
    }
    ResolvConfig config = {};
    int ret = NetSysGetResolvConf(netId, &config);
    if (ret != 0) {
        REQUEST_HILOGE("Cache Download NetSysGetResolvConf failed, ret : %{public}d", ret);
        return dns;
    }

    for (size_t i = 0; i < sizeof(config.nameservers) / sizeof(config.nameservers[0]); i++) {
        if (config.nameservers[i][0] == '\0') {
            continue;
        }
        std::string server(config.nameservers[i], strnlen(config.nameservers[i], sizeof(config.nameservers[0])));
        dns.emplace_back(std::move(server));
    }
    return dns;
}

void GetPerformanceInfo(const HttpClientResponse &response, RustPerformanceInfo &performance)
{
    OHOS::NetStack::HttpClient::PerformanceInfo cpp_perf = response.GetPerformanceTiming();
    performance.set_dns_timing(cpp_perf.dnsTiming);
    performance.set_connect_timing(cpp_perf.connectTiming);
    performance.set_tls_timing(cpp_perf.tlsTiming);
    performance.set_first_send_timing(cpp_perf.firstSendTiming);
    performance.set_first_receive_timing(cpp_perf.firstReceiveTiming);
    performance.set_total_timing(cpp_perf.totalTiming);
    performance.set_redirect_timing(cpp_perf.redirectTiming);
}

} // namespace OHOS::Request