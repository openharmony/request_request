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
#include "get_proxy.h"

#include <want.h>

#include <mutex>

#include "common_event_data.h"
#include "common_event_manager.h"
#include "common_event_notify.h"
#include "common_event_publish_info.h"
#include "log.h"
#include "net_conn_client.h"

std::mutex g_proxyMutex;
using namespace OHOS::EventFwk;
static constexpr const char *DEFAULT_HTTP_PROXY_HOST = "NONE";
static constexpr const char *DEFAULT_HTTP_PROXY_EXCLUSION_LIST = "NONE";

SysNetProxyManager &SysNetProxyManager::GetInstance()
{
    static SysNetProxyManager proxyManager;
    return proxyManager;
}

CStringWrapper SysNetProxyManager::GetHost()
{
    REQUEST_HILOGD("SysNetProxyManager::GetHost() is %{public}s", host_.c_str());
    return WrapperCString(host_);
}
CStringWrapper SysNetProxyManager::GetPort()
{
    REQUEST_HILOGD("SysNetProxyManager::GetPort() is %{public}s", port_.c_str());
    return WrapperCString(port_);
}
CStringWrapper SysNetProxyManager::GetExclusionList()
{
    REQUEST_HILOGD("SysNetProxyManager::GetExclusionList() is %{public}s", exclusionList_.c_str());
    return WrapperCString(exclusionList_);
}

void SysNetProxyManager::SubscriberEvent()
{
    REQUEST_HILOGD("SubscriberEvent start.");
    if (subscriber_) {
        REQUEST_HILOGE("Common Event is already subscribered.");
        return;
    }
    g_proxyMutex.lock();
    InitProxy(host_, port_, exclusionList_);
    g_proxyMutex.unlock();
    OHOS::EventFwk::MatchingSkills matchingSkills;
    matchingSkills.AddEvent(OHOS::EventFwk::CommonEventSupport::COMMON_EVENT_HTTP_PROXY_CHANGE);
    OHOS::EventFwk::CommonEventSubscribeInfo subscribeInfo(matchingSkills);
    subscriber_ = std::make_shared<SysNetProxySubscriber>(subscribeInfo);

    bool subscribeResult = OHOS::EventFwk::CommonEventManager::SubscribeCommonEvent(subscriber_);
    if (subscribeResult == false) {
        REQUEST_HILOGE("Start sysproxy listen, subscribe common event failed");
        return;
    }
}

void SysNetProxyManager::InitProxy(std::string &host, std::string &port, std::string &exclusion)
{
    OHOS::NetManagerStandard::HttpProxy httpProxy;
    int32_t ret = OHOS::NetManagerStandard::NetConnClient::GetInstance().GetDefaultHttpProxy(httpProxy);
    if (ret != OHOS::NetManagerStandard::NET_CONN_SUCCESS) {
        REQUEST_HILOGE("Netproxy config change, get default http proxy from OH network failed");
        return;
    }
    std::string host_res = httpProxy.GetHost();
    host = host_res;
    if (host == DEFAULT_HTTP_PROXY_HOST) {
        host = std::string();
    }
    std::string httpProxyExclusions;
    for (const auto &s : httpProxy.GetExclusionList()) {
        httpProxyExclusions.append(s + ",");
    }
    if (!httpProxyExclusions.empty()) {
        httpProxyExclusions.pop_back();
    }

    exclusion = httpProxyExclusions;
    if (exclusion == DEFAULT_HTTP_PROXY_EXCLUSION_LIST) {
        exclusion = std::string();
    }
    std::string port_res = std::to_string(httpProxy.GetPort());
    port = port_res;
}

void SysNetProxySubscriber::OnReceiveEvent(const OHOS::EventFwk::CommonEventData &data)
{
    const std::string action = data.GetWant().GetAction();
    REQUEST_HILOGD("Receive system proxy change action: %{public}s", action.c_str());
    if (action != OHOS::EventFwk::CommonEventSupport::COMMON_EVENT_HTTP_PROXY_CHANGE) {
        REQUEST_HILOGE("Receive system proxy change, action error, action is %{public}s", action.c_str());
        return;
    }
    std::string host;
    std::string port;
    std::string exclusionList;
    const std::string proxyContent = data.GetWant().GetStringParam("HttpProxy");
    SysNetProxyManager::GetInstance().GetHttpProxy(proxyContent, host, port, exclusionList);
    g_proxyMutex.lock();
    SysNetProxyManager::GetInstance().SetHttpProxy(host, port, exclusionList);
    g_proxyMutex.unlock();
}

void SysNetProxyManager::GetHttpProxy(
    const std::string proxyContent, std::string &host, std::string &port, std::string &exclusionList)
{
    typedef std::string::const_iterator iter_t;
    iter_t proxyContentEnd = proxyContent.end();
    iter_t hostStart = proxyContent.cbegin();
    iter_t hostEnd = std::find(hostStart, proxyContentEnd, '\t');
    std::string hostContent = std::string(hostStart, hostEnd);
    hostEnd += 1;
    iter_t portStart = hostEnd;
    iter_t portEnd = std::find(portStart, proxyContentEnd, '\t');
    std::string portContent = std::string(portStart, portEnd);
    host = hostContent;
    port = portContent;
    if (portEnd != proxyContentEnd) {
        portEnd += 1;
        iter_t exclusionListStart = portEnd;
        std::string exclusionListContent = std::string(exclusionListStart, proxyContentEnd);
        exclusionList = exclusionListContent;
    }
}

void RegisterProxySubscriber()
{
    SysNetProxyManager::GetInstance().SubscriberEvent();
}

CStringWrapper GetHost()
{
    return SysNetProxyManager::GetInstance().GetHost();
}

CStringWrapper GetPort()
{
    return SysNetProxyManager::GetInstance().GetPort();
}

CStringWrapper GetExclusionList()
{
    return SysNetProxyManager::GetInstance().GetExclusionList();
}
