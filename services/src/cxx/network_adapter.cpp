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

#include <singleton.h>

#include <functional>
#include <memory>
#include <new>
#include <set>
#include <string>
#include <type_traits>

#include "net_conn_client.h"

#ifdef REQUEST_TELEPHONY_CORE_SERVICE
#include "cellular_data_client.h"
#include "core_service_client.h"
#endif
#include "i_net_conn_callback.h"
#include "log.h"
#include "net_conn_client.h"
#include "net_conn_constants.h"
#include "net_specifier.h"
#ifdef REQUEST_TELEPHONY_CORE_SERVICE
#include "iservice_registry.h"
#include "network_state.h"
#include "system_ability_definition.h"
#include "telephony_errors.h"
#endif
using namespace OHOS::NetManagerStandard;
namespace OHOS::Request {
NetworkAdapter &NetworkAdapter::GetInstance()
{
    static NetworkAdapter adapter;
    return adapter;
}

bool NetworkAdapter::RegOnNetworkChange(RegCallBack &&callback)
{
    callback_ = callback;
    NetSpecifier netSpecifier;
    NetAllCapabilities netAllCapabilities;
    netAllCapabilities.netCaps_.insert(NetCap::NET_CAPABILITY_INTERNET);
    netSpecifier.netCapabilities_ = netAllCapabilities;
    sptr<NetSpecifier> specifier = new (std::nothrow) NetSpecifier(netSpecifier);
    if (specifier == nullptr) {
        REQUEST_HILOGE("new operator error.specifier is nullptr");
        return false;
    }
    sptr<NetConnCallbackObserver> observer = new (std::nothrow) NetConnCallbackObserver(*this);
    if (observer == nullptr) {
        REQUEST_HILOGE("new operator error.observer is nullptr");
        return false;
    }
    int nRet = NetConnClient::GetInstance().RegisterNetConnCallback(specifier, observer, 0);
    if (nRet == NETMANAGER_SUCCESS) {
        REQUEST_HILOGD("RegisterNetConnCallback successfully registered");
        return true;
    }
    REQUEST_HILOGE("Failed to register the callback retcode= %{public}d", nRet);
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

bool NetworkAdapter::GetNetAllCapabilities(NetManagerStandard::NetAllCapabilities &capabilities)
{
    NetHandle handle;
    int32_t ret = NetConnClient::GetInstance().GetDefaultNet(handle);
    if (ret != NETMANAGER_SUCCESS) {
        REQUEST_HILOGE("get default net failed");
        return false;
    }
    ret = NetConnClient::GetInstance().GetNetCapabilities(handle, capabilities);
    if (ret != NETMANAGER_SUCCESS) {
        REQUEST_HILOGE("get net capabilities failed with reason: %{public}d", ret);
        return false;
    }
    return true;
}

void NetworkAdapter::UpdateNetworkInfo()
{
    NetAllCapabilities capabilities;
    if (!GetNetAllCapabilities(capabilities)) {
        isOnline_ = false;
        return;
    }
    UpdateNetworkInfoInner(capabilities);
}

void NetworkAdapter::UpdateNetworkInfoInner(const NetManagerStandard::NetAllCapabilities &capabilities)
{
    if (capabilities.netCaps_.find(NET_CAPABILITY_INTERNET) != capabilities.netCaps_.end()) {
        isOnline_ = true;
        networkInfo_.networkType = NetworkInner::NET_LOST;
        if (capabilities.bearerTypes_.find(NetBearType::BEARER_CELLULAR) != capabilities.bearerTypes_.end()) {
            REQUEST_HILOGD("Bearer Cellular");
            networkInfo_.networkType = NetworkInner::CELLULAR;
            networkInfo_.isMetered = true;
        }
        if (capabilities.bearerTypes_.find(NetBearType::BEARER_WIFI) != capabilities.bearerTypes_.end()) {
            REQUEST_HILOGD("Bearer Wifi");
            if (networkInfo_.networkType == NetworkInner::CELLULAR) {
                networkInfo_.networkType = NetworkInner::ANY;
            } else {
                networkInfo_.networkType = NetworkInner::WIFI;
            }
            networkInfo_.isMetered = false;
        }
        UpdateRoaming();
    } else {
        isOnline_ = false;
    }
}

int32_t NetworkAdapter::NetConnCallbackObserver::NetCapabilitiesChange(
    sptr<NetHandle> &netHandle, const sptr<NetAllCapabilities> &netAllCap)
{
    REQUEST_HILOGD("Observe net capabilities change. start");
    netAdapter_.UpdateNetworkInfoInner(*netAllCap);
    if (netAdapter_.callback_ != nullptr) {
        netAdapter_.callback_();
        REQUEST_HILOGD("NetCapabilitiesChange callback");
    }
    return 0;
}

int32_t NetworkAdapter::NetConnCallbackObserver::NetConnectionPropertiesChange(
    sptr<NetHandle> &netHandle, const sptr<NetLinkInfo> &info)
{
    return 0;
}

int32_t NetworkAdapter::NetConnCallbackObserver::NetLost(sptr<NetHandle> &netHandle)
{
    REQUEST_HILOGE("Observe bearer cellular lost");
    netAdapter_.networkInfo_.networkType = NetworkInner::NET_LOST;
    netAdapter_.networkInfo_.isMetered = false;
    netAdapter_.isOnline_ = false;
    if (netAdapter_.callback_ != nullptr) {
        netAdapter_.callback_();
        REQUEST_HILOGD("NetCapabilitiesChange callback");
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

void NetworkAdapter::UpdateRoaming()
{
#ifdef REQUEST_TELEPHONY_CORE_SERVICE
    REQUEST_HILOGD("upload roaming");

    // Check telephony SA.
    {
        std::lock_guard<std::mutex> lock(roamingMutex_);

        auto sm = SystemAbilityManagerClient::GetInstance().GetSystemAbilityManager();
        if (sm == nullptr) {
            networkInfo_.isRoaming = false;
            REQUEST_HILOGE("GetSystemAbilityManager return null");
            return;
        }
        auto systemAbility = sm->CheckSystemAbility(TELEPHONY_CORE_SERVICE_SYS_ABILITY_ID);
        if (systemAbility == nullptr) {
            networkInfo_.isRoaming = false;
            REQUEST_HILOGE("Telephony SA not found");
            return;
        }
    }

    constexpr int32_t INVALID_SLOT_ID = -1;
    int32_t maxSlotNum = DelayedRefSingleton<OHOS::Telephony::CoreServiceClient>::GetInstance().GetMaxSimCount();
    bool isSim = false;
    for (int32_t i = 0; i < maxSlotNum; ++i) {
        if (DelayedRefSingleton<OHOS::Telephony::CoreServiceClient>::GetInstance().IsSimActive(i)) {
            isSim = true;
            break;
        }
    }
    if (!isSim) {
        REQUEST_HILOGD("no sim");
        return;
    }

    int32_t slotId =
        DelayedRefSingleton<OHOS::Telephony::CellularDataClient>::GetInstance().GetDefaultCellularDataSlotId();
    if (slotId <= INVALID_SLOT_ID) {
        REQUEST_HILOGE("GetDefaultCellularDataSlotId InValidData");
        return;
    }
    sptr<OHOS::Telephony::NetworkState> networkClient = nullptr;
    DelayedRefSingleton<OHOS::Telephony::CoreServiceClient>::GetInstance().GetNetworkState(slotId, networkClient);
    if (networkClient == nullptr) {
        REQUEST_HILOGE("networkState is nullptr");
        return;
    }
    REQUEST_HILOGI("Roaming = %{public}d", networkClient->IsRoaming());
    networkInfo_.isRoaming = networkClient->IsRoaming();
#endif
}

NetworkInfo *NetworkAdapter::GetNetworkInfo()
{
    return &networkInfo_;
}
} // namespace OHOS::Request

using namespace OHOS::Request;
bool IsOnline()
{
    NetworkAdapter::GetInstance().UpdateNetworkInfo();
    return NetworkAdapter::GetInstance().IsOnline();
}

void RegisterNetworkCallback(NetworkCallback fun)
{
    NetworkAdapter::GetInstance().RegOnNetworkChange(fun);
    REQUEST_HILOGI("running RegisterNetworkCallback end");
}

NetworkInfo *GetNetworkInfo(void)
{
    return NetworkAdapter::GetInstance().GetNetworkInfo();
}

void UpdateNetworkInfo(void)
{
    NetworkAdapter::GetInstance().UpdateNetworkInfo();
}
