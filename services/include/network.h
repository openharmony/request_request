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

#ifndef REQUEST_NETWORK_H
#define REQUEST_NETWORK_H

#include <memory>

#include "cxx.h"
#include "net_all_capabilities.h"
#include "net_conn_callback_stub.h"
#include "net_handle.h"
#include "net_link_info.h"

namespace OHOS::Request {
using namespace OHOS::NetManagerStandard;
struct NetworkInner;
struct NetworkTaskManagerTx;
class RequestNetCallbackStub : public NetConnCallbackStub {
public:
    RequestNetCallbackStub(rust::box<NetworkInner> network, rust::box<NetworkTaskManagerTx> task_manager,
        rust::fn<void(const NetworkTaskManagerTx &task_manager)> notifyTaskManagerOnline,
        rust::fn<void(const NetworkTaskManagerTx &task_manager)> notifyTaskManagerOffline);
    ~RequestNetCallbackStub();

    int32_t NetAvailable(sptr<NetHandle> &netHandle) override;
    int32_t NetLost(sptr<NetHandle> &netHandle) override;
    int32_t NetUnavailable() override;
    int32_t NetCapabilitiesChange(sptr<NetHandle> &netHandle, const sptr<NetAllCapabilities> &netAllCap) override;

private:
#ifdef REQUEST_DEVICE_WATCH
    void SetNet();
#endif
    void HandleNetCap(const sptr<NetAllCapabilities> &netAllCap);
    bool IsRoaming();
    NetworkInner *networkNotifier_;
    NetworkTaskManagerTx *task_manager_;
    rust::fn<void(const NetworkTaskManagerTx &task_manager)> notifyTaskManagerOnline_;
    rust::fn<void(const NetworkTaskManagerTx &task_manager)> notifyTaskManagerOffline_;
#ifdef REQUEST_TELEPHONY_CORE_SERVICE
    std::mutex roamingMutex_;
#endif
};

class NetworkRegistry {
public:
    NetworkRegistry(sptr<RequestNetCallbackStub> callback);
    ~NetworkRegistry();

private:
    sptr<RequestNetCallbackStub> callback_;
};

std::unique_ptr<NetworkRegistry> RegisterNetworkChange(rust::box<NetworkInner> notifier,
    rust::box<NetworkTaskManagerTx> task_manager,
    rust::fn<void(const NetworkTaskManagerTx &task_manager)> notifyTaskManagerOnline,
    rust::fn<void(const NetworkTaskManagerTx &task_manager)> notifyTaskManagerOffline);

} // namespace OHOS::Request
#endif