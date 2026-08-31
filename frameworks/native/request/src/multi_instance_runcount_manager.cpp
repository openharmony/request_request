/*
 * Copyright (C) 2026 Huawei Device Co., Ltd.
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

#ifdef SUPPORT_MULTI_INSTANCE

#include "multi_instance_runcount_manager.h"

#include "iservice_registry.h"
#include "log.h"
#include "request_common.h"
#include "request_manager_impl.h"
#include "request_service_interface.h"
#include "runcount_notify_stub.h"
#include "system_ability_definition.h"

namespace OHOS::Request {

MultiInstanceRunCountManager &MultiInstanceRunCountManager::GetInstance()
{
    static MultiInstanceRunCountManager instance;
    return instance;
}

int32_t MultiInstanceRunCountManager::RestoreSubRunCount()
{
    // Guard against re-entrancy: SubRunCount immediately pushes the count back,
    // re-entering SetCountByPid.
    bool expected = false;
    if (!rebuilding_.compare_exchange_strong(expected, true)) {
        REQUEST_HILOGI("RestoreSubRunCount: skip, rebuild in progress");
        return E_OK;
    }

    auto sam = SystemAbilityManagerClient::GetInstance().GetSystemAbilityManager();
    if (sam == nullptr) {
        REQUEST_HILOGE("RestoreSubRunCount: get SAM failed");
        rebuilding_.store(false);
        return E_OTHER;
    }

    std::vector<sptr<IRemoteObject>> saList;
    if (sam->GetExtensionRunningSaList(DOWNLOAD_SERVER_EXTENSION, saList) != E_OK) {
        REQUEST_HILOGE("RestoreSubRunCount: GetExtensionRunningSaList failed");
        rebuilding_.store(false);
        return E_OTHER;
    }
    REQUEST_HILOGI("RestoreSubRunCount begin, running download_server instances: %{public}zu", saList.size());

    int32_t result = RebuildFromSaList(saList);
    rebuilding_.store(false);
    return result;
}

int32_t MultiInstanceRunCountManager::RebuildFromSaList(const std::vector<sptr<IRemoteObject>> &saList)
{
    // Rebuild the live set: each SubRunCount pushes its count back immediately,
    // so re-subscribing repopulates pidCountMap_ (dead pids fall out).
    {
        std::lock_guard<std::mutex> lock(lock_);
        pidCountMap_.clear();
    }

    int32_t result = E_OK;
    auto listener = RunCountNotifyStub::GetInstance();
    for (const auto &obj : saList) {
        auto proxy = iface_cast<RequestServiceInterface>(obj);
        if (proxy == nullptr) {
            continue;
        }
        int ret = proxy->SubRunCount(listener);
        if (ret != E_OK) {
            REQUEST_HILOGE("RestoreSubRunCount: SubRunCount failed, ret: %{public}d", ret);
            result = E_OTHER;
        }
    }
    return result;
}

void MultiInstanceRunCountManager::SetCountByPid(int32_t pid, int count)
{
    std::lock_guard<std::mutex> lock(lock_);
    if (count == 0) {
        // No running tasks: drop the pid so the map only holds live counts.
        pidCountMap_.erase(pid);
    } else {
        pidCountMap_[pid] = count;
    }
}

int MultiInstanceRunCountManager::GetTotalCount()
{
    std::lock_guard<std::mutex> lock(lock_);
    int total = 0;
    for (const auto &[pid, cnt] : pidCountMap_) {
        total += cnt;
    }
    REQUEST_HILOGD("GetTotalCount: %{public}d, instances: %{public}zu", total, pidCountMap_.size());
    return total;
}

} // namespace OHOS::Request

#endif // SUPPORT_MULTI_INSTANCE
