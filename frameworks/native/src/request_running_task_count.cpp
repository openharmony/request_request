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

#include "request_running_task_count.h"

#include <fcntl.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>

#include <cstdint>
#include <memory>

#include "download_server_ipc_interface_code.h"
#include "iremote_broker.h"
#include "parcel_helper.h"
#include "request_manager_impl.h"
#include "runcount_notify_stub.h"

namespace OHOS::Request {
using namespace OHOS::HiviewDFX;
// impl FwkIRunningTaskObserver
FwkIRunningTaskObserver::FwkIRunningTaskObserver(std::shared_ptr<IRunningTaskObserver> ob)
{
    pInnerOb_ = ob;
}

void FwkIRunningTaskObserver::UpdateRunningTaskCount()
{
    pInnerOb_->OnRunningTaskCountUpdate(FwkRunningTaskCountManager::GetInstance()->GetCount());
}

std::shared_ptr<IRunningTaskObserver> FwkIRunningTaskObserver::GetInnerObserver()
{
    return pInnerOb_;
}

// impl FwkRunningTaskCountManager
std::unique_ptr<FwkRunningTaskCountManager> &FwkRunningTaskCountManager::GetInstance()
{
    static std::unique_ptr<FwkRunningTaskCountManager> instance(new FwkRunningTaskCountManager());
    return instance;
}

int FwkRunningTaskCountManager::GetCount()
{
    return count_;
}

void FwkRunningTaskCountManager::SetCount(int runCount)
{
    std::lock_guard<std::mutex> lock(countLock_);
    count_ = runCount;
}

void FwkRunningTaskCountManager::AttachObserver(std::shared_ptr<IRunningTaskObserver> ob)
{
    auto pNewFwkOb = std::make_shared<FwkIRunningTaskObserver>(ob);
    std::lock_guard<std::mutex> lock(observersLock_);
    observers_.push_back(pNewFwkOb);
    REQUEST_HILOGD("Fwk runcount manager has push observer, now has %{public}d observers",
        static_cast<int32_t>(observers_.size()));
}

void FwkRunningTaskCountManager::DetachObserver(std::shared_ptr<IRunningTaskObserver> ob)
{
    int32_t eraseCnt = 0;
    std::lock_guard<std::mutex> lock(observersLock_);
    auto it = observers_.begin();
    while (it != observers_.end()) {
        if ((*it)->GetInnerObserver().get() == ob.get()) {
            // Just erase shared_ptr from vector, no need to delete.
            it = observers_.erase(it);
            eraseCnt++;
        } else {
            it++;
        }
    }

    if (!eraseCnt) {
        REQUEST_HILOGE("Detach observer failed, not found the unsubscribe ob in obervers");
        return;
    }
}

bool FwkRunningTaskCountManager::HasObserver()
{
    return !observers_.empty();
}

bool FwkRunningTaskCountManager::SaIsOnline()
{
    return saIsOnline_;
}

void FwkRunningTaskCountManager::SetSaStatus(bool isOnline)
{
    saIsOnline_ = isOnline;
}

void FwkRunningTaskCountManager::NotifyAllObservers()
{
    REQUEST_HILOGD("Notify runcount to %{public}d observers.", static_cast<int32_t>(observers_.size()));
    std::lock_guard<std::mutex> observer_lock(observersLock_);
    auto it = observers_.begin();
    while (it != observers_.end()) {
        (*it)->UpdateRunningTaskCount();
        it++;
    }
}

// impl Sub && UnSub
int32_t SubscribeRunningTaskCount(std::shared_ptr<IRunningTaskObserver> ob)
{
    if (FwkRunningTaskCountManager::GetInstance()->HasObserver()) {
        FwkRunningTaskCountManager::GetInstance()->AttachObserver(ob);
        ob->OnRunningTaskCountUpdate(FwkRunningTaskCountManager::GetInstance()->GetCount());
        return E_OK;
    }

    FwkRunningTaskCountManager::GetInstance()->AttachObserver(ob);
    auto listener = RunCountNotifyStub::GetInstance();
    RequestManagerImpl::GetInstance()->SubscribeSA();
    int32_t ret = RequestManagerImpl::GetInstance()->SubRunCount(listener);
    if (ret != E_OK) {
        // IPC is failed, but observer has attached.
        REQUEST_HILOGE("Subscribe running task count failed, ret: %{public}d.", ret);
        return ret;
    }
    if (!FwkRunningTaskCountManager::GetInstance()->SaIsOnline()) {
        ob->OnRunningTaskCountUpdate(0);
    }
    return E_OK;
}

void UnsubscribeRunningTaskCount(std::shared_ptr<IRunningTaskObserver> ob)
{
    FwkRunningTaskCountManager::GetInstance()->DetachObserver(ob);
    if (FwkRunningTaskCountManager::GetInstance()->HasObserver()) {
        REQUEST_HILOGD("Unsubscribe running task count success.");
        return;
    }

    int32_t ret = RequestManagerImpl::GetInstance()->UnsubRunCount();
    RequestManagerImpl::GetInstance()->UnsubscribeSA();
    if (ret != E_OK) {
        REQUEST_HILOGE("Unsubscribe running task count failed, ret: %{public}d.", ret);
    }
}

} // namespace OHOS::Request