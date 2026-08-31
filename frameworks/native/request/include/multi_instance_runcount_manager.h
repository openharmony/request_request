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

#ifndef OHOS_REQUEST_MULTI_INSTANCE_RUNCOUNT_MANAGER_H
#define OHOS_REQUEST_MULTI_INSTANCE_RUNCOUNT_MANAGER_H

#ifdef SUPPORT_MULTI_INSTANCE

#include <atomic>
#include <mutex>
#include <unordered_map>
#include <vector>

#include "iremote_object.h"
#include "refbase.h"

namespace OHOS::Request {

/// Extension tag used to group all download_server SA instances for
/// GetExtensionRunningSaList queries.
constexpr const char *DOWNLOAD_SERVER_EXTENSION = "download_server";

/// Encapsulates multi-instance run-count aggregation.
///
/// In multi-instance mode, each OS user has a dedicated download_server SA
/// process. This class aggregates per-pid counts pushed by each instance via
/// RunCountNotifyStub. RestoreSubRunCount is called when the observer set
/// transitions empty -> non-empty (first subscriber) and re-subscribes every
/// running instance, so new instances are counted and dead ones drop out of
/// the map. Callers (FwkRunningTaskCountManager, RunCountNotifyStub) delegate
/// here under #ifdef SUPPORT_MULTI_INSTANCE.
class MultiInstanceRunCountManager {
public:
    static MultiInstanceRunCountManager &GetInstance();

    /// Re-subscribes every running instance via the extension API. Re-entrant-safe:
    /// the immediate count push-back from SubRunCount must not rebuild recursively.
    /// Returns E_OK on success (or when another rebuild is already in progress),
    /// an error code if SAM/extension enumeration or every instance subscription
    /// failed — matching the single-instance SubscribeRunningTaskCount contract.
    int32_t RestoreSubRunCount();

    /// Records the count reported by a specific SA instance (keyed by pid).
    void SetCountByPid(int32_t pid, int count);

    /// Returns the aggregated total across all instances.
    int GetTotalCount();

private:
    MultiInstanceRunCountManager() = default;
    // Rebuilds pidCountMap_ from an explicit SA list (clears then re-subscribes
    // each). Split out so the rebuild loop is unit-testable without a live SAM.
    int32_t RebuildFromSaList(const std::vector<sptr<IRemoteObject>> &saList);
    std::mutex lock_;
    std::unordered_map<int32_t, int> pidCountMap_;
    std::atomic<bool> rebuilding_{false};
};

} // namespace OHOS::Request

#endif // SUPPORT_MULTI_INSTANCE
#endif // OHOS_REQUEST_MULTI_INSTANCE_RUNCOUNT_MANAGER_H
