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
#include <string>
#include <type_traits>

#include "core_service_client.h"
#include "i_net_conn_callback.h"
#include "log.h"
#include "net_conn_client.h"
#include "net_conn_constants.h"
#include "net_specifier.h"
#include "network_state.h"
#include "telephony_errors.h"

using namespace OHOS::NetManagerStandard;
using namespace OHOS::Telephony;
namespace OHOS::Request {
constexpr int32_t INVALID_SLOT_ID = -1;

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
    int nRet = DelayedSingleton<NetConnClient>::GetInstance()->RegisterNetConnCallback(specifier, observer, 0);
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

int32_t NetworkAdapter::NetConnCallbackObserver::NetCapabilitiesChange(sptr<NetHandle> &netHandle,
    const sptr<NetAllCapabilities> &netAllCap)
{
    REQUEST_HILOGD("Observe net capabilities change. start");
    if (netAllCap->netCaps_.count(NetCap::NET_CAPABILITY_INTERNET)) {
        netAdapter_.isOnline_ = true;
        UpdateRoaming();
        if (netAllCap->bearerTypes_.count(NetBearType::BEARER_CELLULAR)) {
            REQUEST_HILOGI("Bearer Cellular");
            netAdapter_.networkInfo_.networkType = Network::CELLULAR;
            netAdapter_.networkInfo_.isMetered = true;
        } else if (netAllCap->bearerTypes_.count(NetBearType::BEARER_WIFI)) {
            REQUEST_HILOGI("Bearer Wifi");
            netAdapter_.networkInfo_.networkType = Network::WIFI;
            netAdapter_.networkInfo_.isMetered = false;
        }
        if (netAdapter_.callback_ != nullptr) {
            netAdapter_.callback_();
            REQUEST_HILOGD("NetCapabilitiesChange callback");
        }
    } else {
        netAdapter_.isOnline_ = false;
    }
    REQUEST_HILOGD("Observe net capabilities change. end");
    return 0;
}

int32_t NetworkAdapter::NetConnCallbackObserver::NetConnectionPropertiesChange(sptr<NetHandle> &netHandle,
    const sptr<NetLinkInfo> &info)
{
    return 0;
}

int32_t NetworkAdapter::NetConnCallbackObserver::NetLost(sptr<NetHandle> &netHandle)
{
    REQUEST_HILOGD("Observe bearer cellular lost");
    netAdapter_.networkInfo_.networkType = Network::ANY;
    netAdapter_.networkInfo_.isMetered = false;
    netAdapter_.isOnline_ = false;
    if (netAdapter_.callback_ != nullptr) {
        netAdapter_.callback_();
        REQUEST_HILOGI("NetCapabilitiesChange callback");
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
        REQUEST_HILOGE("GetDefaultCellularDataSlotId InValidData");
        return;
    }
    sptr<NetworkState> networkClient = nullptr;
    DelayedRefSingleton<CoreServiceClient>::GetInstance().GetNetworkState(slotId, networkClient);
    if (networkClient == nullptr) {
        REQUEST_HILOGE("networkState is nullptr");
        return;
    }
    REQUEST_HILOGI("Roaming = %{public}d", networkClient->IsRoaming());
    netAdapter_.networkInfo_.isRoaming = networkClient->IsRoaming();
}

NetworkInfo *NetworkAdapter::GetNetworkInfo()
{
    return &networkInfo_;
}
} // namespace OHOS::Request

using namespace OHOS::Request;
bool IsOnline()
{
    bool ret = NetworkAdapter::GetInstance().IsOnline();
    REQUEST_HILOGI("IsOnline result is %{public}d", ret);
    return ret;
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
