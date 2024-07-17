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

#include "account.h"

#include <memory>

#include "cxx.h"
#include "log.h"
#include "os_account_info.h"
#include "os_account_manager.h"
#include "os_account_subscribe_info.h"
#include "os_account_subscriber.h"

namespace OHOS::Request {
SubscriberWrapper::SubscriberWrapper(OS_ACCOUNT_SUBSCRIBE_TYPE type, rust::box<TaskManagerTx> task_manager,
    rust::fn<void(const int &id, const TaskManagerTx &task_manager)> on_accounts_changed,
    rust::fn<void(const int &newId, const int &oldId, const TaskManagerTx &task_manager)> on_accounts_switch)
    : OsAccountSubscriber(OsAccountSubscribeInfo(type, ""))
{
    task_manager_ = task_manager.into_raw();
    on_accounts_changed_ = on_accounts_changed;
    on_accounts_switch_ = on_accounts_switch;
}

SubscriberWrapper::~SubscriberWrapper()
{
    rust::box<TaskManagerTx>::from_raw(task_manager_);
}

void SubscriberWrapper::OnAccountsChanged(const int &id)
{
    REQUEST_HILOGI("Account Change to %{public}d", id);
    on_accounts_changed_(id, *task_manager_);
}

void SubscriberWrapper::OnAccountsSwitch(const int &newId, const int &oldId)
{
    REQUEST_HILOGI("AccountsSwitch newAccount=%{public}d, oldAccount=%{public}d", newId, oldId);
    on_accounts_switch_(newId, oldId, *task_manager_);
}

int RegistryAccountSubscriber(OS_ACCOUNT_SUBSCRIBE_TYPE type, rust::box<TaskManagerTx> task_manager,
    rust::fn<void(const int &id, const TaskManagerTx &task_manager)> on_accounts_changed,
    rust::fn<void(const int &newId, const int &oldId, const TaskManagerTx &task_manager)> on_accounts_switch)
{
    auto const Wrapper = std::static_pointer_cast<OsAccountSubscriber>(
        std::make_shared<SubscriberWrapper>(type, std::move(task_manager), on_accounts_changed, on_accounts_switch));
    return OsAccountManager::SubscribeOsAccount(Wrapper);
}

int GetForegroundOsAccountLocalId(int &id)
{
    return OsAccountManager::GetForegroundOsAccountLocalId(id);
}

rust::String GetOhosAccountUid()
{
    AccountSA::OhosAccountInfo accountInfo;
    ErrCode errCode = AccountSA::OhosAccountKits::GetInstance().GetOhosAccountInfo(accountInfo);
    if (errCode != ERR_OK) {
        REQUEST_HILOGE("GetOhosAccountInfo err: %{public}d, %{public}s", errCode, accountInfo.uid_.c_str());
        return rust::String("ohosAnonymousUid");
    }
    REQUEST_HILOGD("GetOhosAccountInfo ok: %{public}s", accountInfo.uid_.c_str());
    return rust::String(accountInfo.uid_);
}
} // namespace OHOS::Request