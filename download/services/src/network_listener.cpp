/*
 * Copyright (c) 2022 Huawei Device Co., Ltd.
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

#include "network_listener.h"
#include "net_specifier.h"
#include "net_conn_client.h"
#include "net_conn_constants.h"
#include "log.h"

using namespace OHOS::NetManagerStandard;
namespace OHOS {
namespace MiscServices {

bool NetworkListener::isOnline_ = false;
std::mutex NetworkListener::mutex_;

NetworkListener::NetworkListener() {
}

NetworkListener::~NetworkListener() {
}

bool NetworkListener::RegOnNetworkChange(RegCallBack&& callback)
{
    NetSpecifier netSpecifier;
    NetAllCapabilities netAllCapabilities;
    netAllCapabilities.netCaps_.insert(NetCap::NET_CAPABILITY_INTERNET);
    netSpecifier.netCapabilities_ = netAllCapabilities;
    sptr<NetSpecifier> specifier = new(std::nothrow) NetSpecifier(netSpecifier);
    if (specifier == nullptr) {
        DOWNLOAD_HILOGE("new operator error.specifier is nullptr");
        return NET_CONN_ERR_INPUT_NULL_PTR;
    }
    sptr<NetConnCallbackObserver> observer = new(std::nothrow) NetConnCallbackObserver(*this);
    if (observer == nullptr) {
        DOWNLOAD_HILOGE("new operator error.observer is nullptr");
        return NET_CONN_ERR_INPUT_NULL_PTR;
    }
    int nRet = DelayedSingleton<NetConnClient>::GetInstance()->RegisterNetConnCallback(specifier, observer, 0);
    if (nRet == NET_CONN_SUCCESS) {
       callback_ = callback;
    }
 
    DOWNLOAD_HILOGD("RegisterNetConnCallback retcode= %{public}d", nRet);
    return nRet;
}

bool NetworkListener::IsOnline()
{
    return isOnline_;
}

void NetworkListener::SetNetworkStatus(bool isOnline)
{
    std::lock_guard<std::mutex> autoLock(mutex_);
    isOnline_ = isOnline;
}

int32_t NetworkListener::NetConnCallbackObserver::NetAvailable(sptr<NetHandle> &netHandle)
{
    return 0;
}

int32_t NetworkListener::NetConnCallbackObserver::NetCapabilitiesChange(sptr<NetHandle> &netHandle,
                                                       const sptr<NetAllCapabilities> &netAllCap)
{
    DOWNLOAD_HILOGD("Observe net capabilities change. start");
    if (netAllCap->netCaps_.count(NetCap::NET_CAPABILITY_VALIDATED)) {
       NetListener_.SetNetworkStatus(true);
       if (NetListener_.callback_ != nullptr){
           NetListener_.callback_();
           DOWNLOAD_HILOGD("NetCapabilitiesChange callback");
       }
    } else {
        NetListener_.SetNetworkStatus(false);
    }
    DOWNLOAD_HILOGD("Observe net capabilities change. end");
    return 0;
}

int32_t NetworkListener::NetConnCallbackObserver::NetConnectionPropertiesChange(sptr<NetHandle> &netHandle,
                                                               const sptr<NetLinkInfo> &info)
{
    return 0;
}

int32_t NetworkListener::NetConnCallbackObserver::NetLost(sptr<NetHandle> &netHandle)
{
    return 0;
}

int32_t NetworkListener::NetConnCallbackObserver::NetUnavailable()
{
    return 0;
}

int32_t NetworkListener::NetConnCallbackObserver::NetBlockStatusChange(sptr<NetHandle> &netHandle, bool blocked)
{
    return 0;
}
} // namespace MiscServices
} // namespace OHOS
