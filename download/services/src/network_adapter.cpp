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

#include "network_adapter.h"
#include "net_specifier.h"
#include "net_conn_client.h"
#include "net_conn_constants.h"
#include "telephony_errors.h"
#include "core_service_client.h"
#include "constant.h"
#include "log.h"

using namespace OHOS::NetManagerStandard;
using namespace OHOS::Telephony;
using namespace OHOS::Request::Download;
namespace OHOS {
namespace MiscServices {
const int32_t ERROR = -1;
NetworkAdapter& NetworkAdapter::GetInstance()
{
    static NetworkAdapter adapter;
    return adapter;
}

bool NetworkAdapter::RegOnNetworkChange(RegCallBack&& callback)
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

bool NetworkAdapter::IsOnline()
{
    return isOnline_;
}

int32_t NetworkAdapter::NetConnCallbackObserver::NetAvailable(sptr<NetHandle> &netHandle)
{
    return 0;
}

int32_t NetworkAdapter::NetConnCallbackObserver::NetCapabilitiesChange(sptr <NetHandle> &netHandle,
                                                                       const sptr <NetAllCapabilities> &netAllCap)
{
    DOWNLOAD_HILOGD("Observe net capabilities change. start");
    if (netAllCap->netCaps_.count(NetCap::NET_CAPABILITY_VALIDATED)) {
        netAdapter_.isOnline_ = true;
        GetRoaming();
        if (netAllCap->bearerTypes_.count(NetBearType::BEARER_CELLULAR)) {
            DOWNLOAD_HILOGD("BearerCellular :[%{public}d]", NetBearType::BEARER_CELLULAR);
            netAdapter_.networkType_ = static_cast<uint32_t>(NETWORK_MOBILE);
        }
        if (netAllCap->bearerTypes_.count(NetBearType::BEARER_WIFI)) {
            DOWNLOAD_HILOGD("BearerWifi :[%{public}d]", NetBearType::BEARER_WIFI);
            netAdapter_.networkType_ = static_cast<uint32_t>(NETWORK_WIFI);
            netAdapter_.isMetered_ = false;
        } else {
            netAdapter_.isMetered_ = true;
        }
        if (netAdapter_.callback_ != nullptr) {
            netAdapter_.callback_();
            DOWNLOAD_HILOGD("NetCapabilitiesChange callback");
        }
    } else {
        netAdapter_.isOnline_ = false;
    }
    DOWNLOAD_HILOGD("Observe net capabilities change. end");
    return 0;
}

int32_t NetworkAdapter::NetConnCallbackObserver::NetConnectionPropertiesChange(sptr<NetHandle> &netHandle,
                                                                               const sptr<NetLinkInfo> &info)
{
    return 0;
}

int32_t NetworkAdapter::NetConnCallbackObserver::NetLost(sptr<NetHandle> &netHandle)
{
    DOWNLOAD_HILOGD("Observe bearer cellular lost");
    netAdapter_.networkType_ = 0;
    netAdapter_.isMetered_ = false;
    if (netAdapter_.callback_ != nullptr) {
        netAdapter_.callback_();
        DOWNLOAD_HILOGD("NetCapabilitiesChange callback");
    }
    return 0;
}

int32_t NetworkAdapter::NetConnCallbackObserver::NetUnavailable()
{
    return 0;
}

int32_t NetworkAdapter::NetConnCallbackObserver::NetBlockStatusChange(sptr<NetHandle> &netHandle, bool blocked)
{
    return 0;
}

void NetworkAdapter::NetConnCallbackObserver::GetRoaming()
{
    auto slotId = DelayedRefSingleton<CoreServiceClient>::GetInstance().GetPrimarySlotId();
    if (slotId == TELEPHONY_ERR_IPC_CONNECT_STUB_FAIL || slotId == ERROR) {
        DOWNLOAD_HILOGE("GetDefaultCellularDataSlotId InValidData");
        return;
    }
    auto networkState = DelayedRefSingleton<CoreServiceClient>::GetInstance().GetNetworkState(slotId);
    if (networkState == nullptr) {
        DOWNLOAD_HILOGE("networkState is nullptr");
        return;
    }
    DOWNLOAD_HILOGE("Roaming = %{public}d",networkState->IsRoaming());
    netAdapter_.isRoaming_ = networkState->IsRoaming();
}

uint32_t NetworkAdapter::GetNetworkType()
{
    return networkType_;
}

bool NetworkAdapter::GetMetered()
{
    return isMetered_;
}

bool NetworkAdapter::GetRoaming()
{
    return isRoaming_;
}
} // namespace MiscServices
} // namespace OHOS
