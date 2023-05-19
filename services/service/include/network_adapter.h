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

#ifndef REQUEST_NETWORK_ADAPTER_H
#define REQUEST_NETWORK_ADAPTER_H

#include <stdint.h>
#include <mutex>
#include <functional>
#include "net_handle.h"
#include "net_link_info.h"
#include "net_all_capabilities.h"
#include "net_conn_callback_stub.h"
#include "refbase.h"
#include "constant.h"
namespace OHOS::Request::Download {
struct NetworkInfo {
    NetworkType networkType_ = NETWORK_INVALID;
    bool isMetered_ = false;
    bool isRoaming_ = false;
};
class NetworkAdapter {
public:
    using RegCallBack = std::function<void()>;

    bool RegOnNetworkChange(RegCallBack&& callback);
    bool IsOnline();
    static NetworkAdapter& GetInstance();
    friend class NetConnCallbackObserver;
    NetworkInfo GetNetworkInfo();
private:
    class NetConnCallbackObserver :  public NetManagerStandard::NetConnCallbackStub {
    public:
        explicit NetConnCallbackObserver(NetworkAdapter &netAdapter) : netAdapter_(netAdapter) {}
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
        void UpdateRoaming();
    private:
        NetworkAdapter& netAdapter_;
    };

    RegCallBack callback_ = nullptr;
    bool isOnline_ = false;
    std::mutex mutex_;
    NetworkInfo networkInfo_;
};
}   // namespace OHOS::Request::Download
#endif /* REQUEST_NETWORK_ADAPTER_H */
