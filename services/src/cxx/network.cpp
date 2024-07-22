/*
 * Copyright (c) 2024 Huawei Device Co., Ltd.
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

#include "network.h"

#include <cstdint>

#include "cxx.h"
#include "log.h"
#include "manage/network.rs.h"
#include "net_all_capabilities.h"
#include "net_conn_callback_stub.h"
#include "net_conn_client.h"
#include "net_specifier.h"
#include "refbase.h"

#ifdef REQUEST_TELEPHONY_CORE_SERVICE
#include "cellular_data_client.h"
#include "core_service_client.h"
#endif

#ifdef REQUEST_TELEPHONY_CORE_SERVICE
#include "iservice_registry.h"
#include "network_state.h"
#include "system_ability_definition.h"
#include "telephony_errors.h"
#endif

namespace OHOS::Request {
using namespace OHOS::NetManagerStandard;

RequestNetCallbackStub::RequestNetCallbackStub(
    rust::box<NetworkInner> network, rust::box<NetworkTaskManagerTx> task_manager,
    rust::fn<void(const NetworkTaskManagerTx &task_manager)> notifyTaskManagerOnline,
    rust::fn<void(const NetworkTaskManagerTx &task_manager)> notifyTaskManagerOffline

)
{
    networkNotifier_ = network.into_raw();
    task_manager_ = task_manager.into_raw();
    notifyTaskManagerOnline_ = notifyTaskManagerOnline;
    notifyTaskManagerOffline_ = notifyTaskManagerOffline;
}

RequestNetCallbackStub::~RequestNetCallbackStub()
{
    rust::Box<NetworkInner>::from_raw(networkNotifier_);
    rust::Box<NetworkTaskManagerTx>::from_raw(task_manager_);
}

void RequestNetCallbackStub::HandleNetCap(const sptr<NetAllCapabilities> &netAllCap)
{
    if (netAllCap->netCaps_.find(NetCap::NET_CAPABILITY_VALIDATED) == netAllCap->netCaps_.end()) {
        networkNotifier_->notify_offline();
        notifyTaskManagerOffline_(*task_manager_);
        return;
    }

    for (auto bearerType : netAllCap->bearerTypes_) {
        auto networkInfo = NetworkInfo();
        if (bearerType == NetManagerStandard::NetBearType::BEARER_WIFI) {
            networkInfo.network_type = NetworkType::Wifi;
            networkInfo.is_metered = false;
            networkInfo.is_roaming = false;

            if (networkNotifier_->notify_online(networkInfo)) {
                notifyTaskManagerOnline_(*task_manager_);
            }
            return;
        } else if (bearerType == NetManagerStandard::NetBearType::BEARER_CELLULAR) {
            networkInfo.network_type = NetworkType::Cellular;
            networkInfo.is_metered = true;
            networkInfo.is_roaming = this->IsRoaming();

            if (networkNotifier_->notify_online(networkInfo)) {
                notifyTaskManagerOnline_(*task_manager_);
            }
            return;
        };
    }
    if (networkNotifier_->notify_online(NetworkInfo{
            .network_type = NetworkType::Other,
            .is_metered = false,
            .is_roaming = false,
        })) {
        notifyTaskManagerOnline_(*task_manager_);
    }
    return;
}

int32_t RequestNetCallbackStub::NetAvailable(sptr<NetHandle> &netHandle)
{
    sptr<NetAllCapabilities> netAllCap = sptr<NetAllCapabilities>::MakeSptr();
    int32_t ret = NetConnClient::GetInstance().GetNetCapabilities(*netHandle, *netAllCap);
    if (ret != 0) {
        REQUEST_HILOGE("GetNetCapabilities failed, ret = %{public}d", ret);
        return ret;
    }
    this->HandleNetCap(netAllCap);
    return 0;
}

int32_t RequestNetCallbackStub::NetLost(sptr<NetHandle> &netHandle)
{
    networkNotifier_->notify_offline();
    notifyTaskManagerOffline_(*task_manager_);
    return 0;
}

int32_t RequestNetCallbackStub::NetUnavailable()
{
    networkNotifier_->notify_offline();
    notifyTaskManagerOffline_(*task_manager_);
    return 0;
}

int32_t RequestNetCallbackStub::NetCapabilitiesChange(
    sptr<NetHandle> &netHandle, const sptr<NetAllCapabilities> &netAllCap)
{
    REQUEST_HILOGI("NetCapabilitiesChange");
    this->HandleNetCap(netAllCap);
    return 0;
}

bool RequestNetCallbackStub::IsRoaming()
{
#ifdef REQUEST_TELEPHONY_CORE_SERVICE
    REQUEST_HILOGD("upload roaming");
    // Check telephony SA.
    {
        std::lock_guard<std::mutex> lock(roamingMutex_);

        auto sm = SystemAbilityManagerClient::GetInstance().GetSystemAbilityManager();
        if (sm == nullptr) {
            REQUEST_HILOGE("GetSystemAbilityManager return null");
            return false;
        }
        auto systemAbility = sm->CheckSystemAbility(TELEPHONY_CORE_SERVICE_SYS_ABILITY_ID);
        if (systemAbility == nullptr) {
            REQUEST_HILOGE("Telephony SA not found");
            return false;
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
        return false;
    }

    int32_t slotId =
        DelayedRefSingleton<OHOS::Telephony::CellularDataClient>::GetInstance().GetDefaultCellularDataSlotId();
    if (slotId <= INVALID_SLOT_ID) {
        REQUEST_HILOGE("GetDefaultCellularDataSlotId InValidData");
        return false;
    }
    sptr<OHOS::Telephony::NetworkState> networkClient = nullptr;
    DelayedRefSingleton<OHOS::Telephony::CoreServiceClient>::GetInstance().GetNetworkState(slotId, networkClient);
    if (networkClient == nullptr) {
        REQUEST_HILOGE("networkState is nullptr");
        return false;
    }
    REQUEST_HILOGI("Roaming = %{public}d", networkClient->IsRoaming());
    return networkClient->IsRoaming();
#else
    REQUEST_HILOGE("Telephony SA not found");
    return false;
#endif
}

std::unique_ptr<NetworkRegistry> RegisterNetworkChange(rust::box<NetworkInner> notifier,
    rust::box<NetworkTaskManagerTx> task_manager,
    rust::fn<void(const NetworkTaskManagerTx &task_manager)> notifyTaskManagerOnline,
    rust::fn<void(const NetworkTaskManagerTx &task_manager)> notifyTaskManagerOffline)
{
    REQUEST_HILOGI("RegisterNetworkChange");
    sptr<RequestNetCallbackStub> callbackStub = sptr<RequestNetCallbackStub>::MakeSptr(
        std::move(notifier), std::move(task_manager), notifyTaskManagerOnline, notifyTaskManagerOffline);
    if (callbackStub == nullptr) {
        REQUEST_HILOGE("callbackStub is nullptr");
        return nullptr;
    }
    int ret = NetConnClient::GetInstance().RegisterNetConnCallback(callbackStub);
    if (ret != 0) {
        REQUEST_HILOGE("RegisterNetConnCallback failed, ret = %{public}d", ret);
        return nullptr;
    }
    return std::make_unique<NetworkRegistry>(callbackStub);
}

NetworkRegistry::NetworkRegistry(sptr<RequestNetCallbackStub> callback)
{
    callback_ = callback;
}

NetworkRegistry::~NetworkRegistry()
{
    REQUEST_HILOGI("UnregisterNetworkChange");
    int32_t ret = NetConnClient::GetInstance().UnregisterNetConnCallback(callback_);
    if (ret != 0) {
        REQUEST_HILOGE("UnregisterNetConnCallback failed, ret = %{public}d", ret);
    }
}

} // namespace OHOS::Request