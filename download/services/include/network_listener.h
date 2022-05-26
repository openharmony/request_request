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

#ifndef REQUEST_NETWORK_LISTENER_H
#define REQUEST_NETWORK_LISTENER_H

#include "net_all_capabilities.h"
#include "net_conn_callback_stub.h"
#include "singleton.h"

namespace OHOS {
namespace MiscServices {
using RegCallBack = std::function<void()>;
class NetworkListener
{
    DECLARE_DELAYED_SINGLETON(NetworkListener)
public:
    bool RegOnNetworkChange(RegCallBack&& callback);
    bool IsOnline();
    void SetNetworkStatus(bool isOnline);

    class NetConnCallbackObserver :  public NetManagerStandard::NetConnCallbackStub {
    public:
        explicit NetConnCallbackObserver(NetworkListener &NetListener) : NetListener_(NetListener) {}
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
        NetworkListener& NetListener_;
    };
private:
    RegCallBack callback_ = nullptr;
    static bool isOnline_;
    static std::mutex mutex_;
};
}   // namespace MiscServices
}   // namespace OHOS
#endif /* REQUEST_NETWORK_LISTENER_H */
