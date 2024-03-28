/*
 * Copyright (C) 2024 Huawei Device Co., Ltd.
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

#ifndef OHOS_REQUEST_RUNNING_TASK_COUNT_H
#define OHOS_REQUEST_RUNNING_TASK_COUNT_H

#include <memory>
#include <mutex>
#include <vector>

#include "../../interfaces/inner_kits/running_count/include/running_task_count.h"
#include "iremote_proxy.h"
#include "js_common.h"
#include "log.h"
#include "notify_interface.h"
#include "peer_holder.h"
#include "request_service_interface.h"

namespace OHOS::Request {
class FwkRunningTaskCountManager;
class FwkIRunningTaskObserver {
public:
    std::shared_ptr<IRunningTaskObserver> GetInnerObserver();
    void UpdateRunningTaskCount();

public:
    ~FwkIRunningTaskObserver() = default;
    FwkIRunningTaskObserver(std::shared_ptr<IRunningTaskObserver> ob);

private:
    std::shared_ptr<IRunningTaskObserver> pInnerOb_;
};

class FwkRunningTaskCountManager {
public:
    static std::unique_ptr<FwkRunningTaskCountManager> &GetInstance();
    int32_t GetCount();
    void SetCount(int runCount);
    void AttachObserver(std::shared_ptr<IRunningTaskObserver> ob);
    void DetachObserver(std::shared_ptr<IRunningTaskObserver> ob);
    void NotifyAllObservers();
    bool HasObserver();
    bool SaIsOnline();
    void SetSaStatus(bool isOnline);

    ~FwkRunningTaskCountManager() = default;
    FwkRunningTaskCountManager(const FwkRunningTaskCountManager &) = delete;
    FwkRunningTaskCountManager(FwkRunningTaskCountManager &&) = delete;
    FwkRunningTaskCountManager &operator=(const FwkRunningTaskCountManager &) = delete;

private:
    FwkRunningTaskCountManager() = default;
    bool saIsOnline_;
    int count_ = 0;
    std::mutex observersLock_;
    std::mutex countLock_;
    std::vector<std::shared_ptr<FwkIRunningTaskObserver>> observers_;
};

} // namespace OHOS::Request

#endif // OHOS_REQUEST_RUNNING_TASK_COUNT_H