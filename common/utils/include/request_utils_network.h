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

#ifndef REQUEST_UTILS_OBSERVER_NETWORK_H
#define REQUEST_UTILS_OBSERVER_NETWORK_H

#include <memory>
#include <vector>

#include "cxx.h"
#include "net_all_capabilities.h"
#include "net_conn_callback_stub.h"
#include "net_handle.h"
#include "net_link_info.h"

namespace OHOS {
namespace Request {
using namespace OHOS::NetManagerStandard;

struct NetObserverWrapper;
struct NetInfo;

class NetObserver : public NetConnCallbackStub {
public:
    NetObserver(rust::Box<NetObserverWrapper>);
    ~NetObserver() = default;

    int32_t NetAvailable(sptr<NetHandle> &netHandle) override;

    int32_t NetCapabilitiesChange(sptr<NetHandle> &netHandle, const sptr<NetAllCapabilities> &netAllCap) override;
    int32_t NetLost(sptr<NetHandle> &netHandle) override;

private:
    rust::Box<NetObserverWrapper> inner_;
};

class NetUnregistration {
public:
    NetUnregistration(sptr<NetObserver> observer);
    ~NetUnregistration() = default;

    int32_t unregister() const;

private:
    sptr<NetObserver> observer_;
};

std::unique_ptr<NetUnregistration> RegisterNetObserver(rust::Box<NetObserverWrapper> wrapper, int32_t &error);

} // namespace Request
} // namespace OHOS
#endif