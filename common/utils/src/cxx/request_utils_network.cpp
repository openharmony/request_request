/*
 * Copyright (c) 2025 Huawei Device Co., Ltd.
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

#include "request_utils_network.h"

#include <cstdint>

#include "cxx.h"
#include "iservice_registry.h"
#include "net_conn_callback_stub.h"
#include "net_conn_client.h"
#include "net_specifier.h"
#include "observe/network/wrapper.rs.h"
#include "refbase.h"

namespace OHOS {
namespace Request {
using namespace OHOS::NetManagerStandard;

NetObserver::NetObserver(rust::Box<NetObserverWrapper> wrapper) : inner_(std::move(wrapper))
{
}

int32_t NetObserver::NetAvailable(sptr<NetHandle> &netHandle)
{
    inner_->net_available(netHandle->GetNetId());
    return 0;
}

int32_t NetObserver::NetLost(sptr<NetHandle> &netHandle)
{
    inner_->net_lost(netHandle->GetNetId());
    return 0;
}

int32_t NetObserver::NetCapabilitiesChange(sptr<NetHandle> &netHandle, const sptr<NetAllCapabilities> &netAllCap)
{
    rust::vec<NetCap> caps;
    for (auto cap : netAllCap->netCaps_) {
        caps.push_back(cap);
    }
    rust::vec<NetBearType> bearTypes;
    for (auto bearType : netAllCap->bearerTypes_) {
        bearTypes.push_back(bearType);
    }
    NetInfo info{
        .caps = caps,
        .bear_types = bearTypes,
    };
    inner_->net_capability_changed(netHandle->GetNetId(), info);
    return 0;
}

NetUnregistration::NetUnregistration(sptr<NetObserver> observer) : observer_(observer)
{
}

int32_t NetUnregistration::unregister() const
{
    return NetConnClient::GetInstance().UnregisterNetConnCallback(observer_);
}

std::unique_ptr<NetUnregistration> RegisterNetObserver(rust::Box<NetObserverWrapper> wrapper, int32_t &error)
{
    sptr<NetObserver> stub = sptr<NetObserver>::MakeSptr(std::move(wrapper));

    int ret = NetConnClient::GetInstance().RegisterNetConnCallback(stub);
    if (ret != 0) {
        error = ret;
        return nullptr;
    }
    return std::make_unique<NetUnregistration>(stub);
}
} // namespace Request
} // namespace OHOS