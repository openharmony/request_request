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

#ifndef ACCOUNT_H
#define ACCOUNT_H
#include <memory>
#include <vector>

#include "cxx.h"
#include "errors.h"
#include "ohos_account_kits.h"
#include "os_account_info.h"
#include "os_account_manager.h"
#include "os_account_subscribe_info.h"
#include "os_account_subscriber.h"
#include "refbase.h"

namespace OHOS::Request {
using namespace OHOS::AccountSA;

struct TaskManagerTx;
class SubscriberWrapper : public OsAccountSubscriber {
public:
    explicit SubscriberWrapper(OS_ACCOUNT_SUBSCRIBE_TYPE type, rust::box<TaskManagerTx> task_manager,
        rust::fn<void(const int &id, const TaskManagerTx &task_manager)> on_accounts_changed,
        rust::fn<void(const int &newId, const int &oldId, const TaskManagerTx &task_manager)> on_accounts_switch);

    ~SubscriberWrapper();

    virtual void OnAccountsChanged(const int &id) override;
    virtual void OnAccountsSwitch(const int &newId, const int &oldId) override;

private:
    TaskManagerTx *task_manager_;
    rust::fn<void(const int &id, const TaskManagerTx &task_manager)> on_accounts_changed_;
    rust::fn<void(const int &newId, const int &oldId, const TaskManagerTx &task_manager)> on_accounts_switch_;
};

int RegistryAccountSubscriber(OS_ACCOUNT_SUBSCRIBE_TYPE type, rust::box<TaskManagerTx> task_manager,
    rust::fn<void(const int &id, const TaskManagerTx &task_manager)> on_accounts_changed,
    rust::fn<void(const int &newId, const int &oldId, const TaskManagerTx &task_manager)> on_accounts_switch);

inline ErrCode GetForegroundOsAccount(int &account)
{
    return OsAccountManager::GetForegroundOsAccountLocalId(account);
}

inline ErrCode GetBackgroundOsAccounts(rust::vec<int> &accounts)
{
    auto v = std::vector<int32_t>();
    auto ret = OsAccountManager::GetBackgroundOsAccountLocalIds(v);
    if (ret == 0) {
        for (auto &account : v) {
            accounts.push_back(account);
        };
    }
    return ret;
}

inline ErrCode GetOsAccountLocalIdFromUid(const int uid, int &id)
{
    return OsAccountManager::GetOsAccountLocalIdFromUid(uid, id);
}

rust::String GetOhosAccountUid();
} // namespace OHOS::Request
#endif // ACCOUNT_H