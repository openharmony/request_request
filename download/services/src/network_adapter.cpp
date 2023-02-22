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
#include <functional>
#include <memory>
#include <new>
#include <set>
#include <singleton.h>
#include <type_traits>
#include "i_net_conn_callback.h"
#include "net_specifier.h"
#include "net_conn_client.h"
#include "network_state.h"
#include "net_conn_constants.h"
#include "telephony_errors.h"
#include "core_service_client.h"
#include "log.h"

using namespace OHOS::NetManagerStandard;
using namespace OHOS::Telephony;
namespace OHOS::Request::Download {
constexpr int32_t INVALID_SLOT_ID = -1;

NetworkAdapter &NetworkAdapter::GetInstance()
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
        return false;
    }
    sptr<NetConnCallbackObserver> observer = new(std::nothrow) NetConnCallbackObserver(*this);
    if (observer == nullptr) {
        DOWNLOAD_HILOGE("new operator error.observer is nullptr");
        return false;
    }
    int nRet = DelayedSingleton<NetConnClient>::GetInstance()->RegisterNetConnCallback(specifier, observer, 0);
    if (nRet == NETMANAGER_SUCCESS) {
        callback_ = callback;
        return true;
    }
    DOWNLOAD_HILOGE("Failed to register the callback retcode= %{public}d", nRet);
    return false;
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
    if (netAllCap->netCaps_.count(NetCap::NET_CAPABILITY_INTERNET)) {
        netAdapter_.isOnline_ = true;
        UpdateRoaming();
        if (netAllCap->bearerTypes_.count(NetBearType::BEARER_CELLULAR)) {
            DOWNLOAD_HILOGI("Bearer Cellular");
            netAdapter_.networkInfo_.networkType_ = NETWORK_MOBILE;
            netAdapter_.networkInfo_.isMetered_ = true;
        } else if (netAllCap->bearerTypes_.count(NetBearType::BEARER_WIFI)) {
            DOWNLOAD_HILOGI("Bearer Wifi");
            netAdapter_.networkInfo_.networkType_ = NETWORK_WIFI;
            netAdapter_.networkInfo_.isMetered_ = false;
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
    netAdapter_.networkInfo_.networkType_ = NETWORK_INVALID;
    netAdapter_.networkInfo_.isMetered_ = false;
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

void NetworkAdapter::NetConnCallbackObserver::UpdateRoaming()
{
    int32_t slotId = INVALID_SLOT_ID;
    DelayedRefSingleton<CoreServiceClient>::GetInstance().GetPrimarySlotId(slotId);
    if (slotId <= INVALID_SLOT_ID) {
        DOWNLOAD_HILOGE("GetDefaultCellularDataSlotId InValidData");
        return;
    }
    sptr<NetworkState> networkClient = nullptr;
    DelayedRefSingleton<CoreServiceClient>::GetInstance().GetNetworkState(slotId, networkClient);
    if (networkClient == nullptr) {
        DOWNLOAD_HILOGE("networkState is nullptr");
        return;
    }
    DOWNLOAD_HILOGI("Roaming = %{public}d", networkClient->IsRoaming());
    netAdapter_.networkInfo_.isRoaming_ = networkClient->IsRoaming();
}

NetworkInfo NetworkAdapter::GetNetworkInfo()
{
    return networkInfo_;
}
} // namespace OHOS::Request::Download
