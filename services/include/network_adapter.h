/*
 * Copyright (c) 2023 Huawei Device Co., Ltd.
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

#ifndef REQUEST_NETWORK_ADAPTER_H
#define REQUEST_NETWORK_ADAPTER_H

#include <stdint.h>

#include <functional>
#include <mutex>

#include "c_enumration.h"
#include "net_all_capabilities.h"
#include "net_conn_callback_stub.h"
#include "net_handle.h"
#include "net_link_info.h"
#include "refbase.h"

struct NetworkInfo {
    NetworkInner networkType = NetworkInner::NET_LOST;
    bool isMetered = false;
    bool isRoaming = false;
};

namespace OHOS::Request {
class NetworkAdapter {
public:
    using RegCallBack = std::function<void()>;

    bool RegOnNetworkChange(RegCallBack &&callback);
    bool IsOnline();
    static NetworkAdapter &GetInstance();
    friend class NetConnCallbackObserver;
    NetworkInfo *GetNetworkInfo();
    void UpdateNetworkInfo();
    void UpdateNetworkInfoInner(const NetManagerStandard::NetAllCapabilities &capabilities);

public:
    class NetConnCallbackObserver : public NetManagerStandard::NetConnCallbackStub {
    public:
        explicit NetConnCallbackObserver(NetworkAdapter &netAdapter) : netAdapter_(netAdapter)
        {
        }
        ~NetConnCallbackObserver() override = default;
        int32_t NetAvailable(sptr<NetManagerStandard::NetHandle> &netHandle) override;

        int32_t NetCapabilitiesChange(sptr<NetManagerStandard::NetHandle> &netHandle,
            const sptr<NetManagerStandard::NetAllCapabilities> &netAllCap) override;

        int32_t NetConnectionPropertiesChange(sptr<NetManagerStandard::NetHandle> &netHandle,
            const sptr<NetManagerStandard::NetLinkInfo> &info) override;

        int32_t NetLost(sptr<NetManagerStandard::NetHandle> &netHandle) override;

        int32_t NetUnavailable() override;

        int32_t NetBlockStatusChange(sptr<NetManagerStandard::NetHandle> &netHandle, bool blocked) override;

    private:
        NetworkAdapter &netAdapter_;
    };

private:
    bool GetNetAllCapabilities(NetManagerStandard::NetAllCapabilities &capabilities);
    void UpdateRoaming();
    RegCallBack callback_ = nullptr;
    bool isOnline_ = false;
    std::mutex mutex_;
    std::mutex roamingMutex_;
    NetworkInfo networkInfo_;
};
} // namespace OHOS::Request

#ifdef __cplusplus
extern "C" {
#endif

typedef void (*NetworkCallback)(void);
bool IsOnline();
void RegisterNetworkCallback(NetworkCallback fun);
NetworkInfo *GetNetworkInfo(void);
void UpdateNetworkInfo(void);

#ifdef __cplusplus
}
#endif

#endif
