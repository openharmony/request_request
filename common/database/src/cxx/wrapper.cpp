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

#include "wrapper.h"

#include <cstdint>

#include "base/request/request/common/include/log.h"
#include "cxx.h"
#include "wrapper.rs.h"
namespace OHOS::Request {

OpenCallback::OpenCallback(rust::Box<OpenCallbackWrapper> &&inner) : inner_(std::move(inner))
{
}
int OpenCallback::OnCreate(RdbStore &store)
{
    return inner_->on_create(store);
}
int OpenCallback::OnUpgrade(RdbStore &store, int oldVersion, int newVersion)
{
    return inner_->on_upgrade(store, oldVersion, newVersion);
}
int OpenCallback::OnDowngrade(RdbStore &store, int oldVersion, int newVersion)
{
    return inner_->on_downgrade(store, oldVersion, newVersion);
}
int OpenCallback::OnOpen(RdbStore &store)
{
    return inner_->on_open(store);
}
int OpenCallback::onCorruption(std::string databaseFile)
{
    return inner_->on_corrupt(databaseFile);
}

int GetString(RowEntity &rowEntity, int index, rust::string &value)
{
    std::string val;
    int ret = rowEntity.Get(index).GetString(val);
    value = val;
    return ret;
}

int GetBlob(RowEntity &rowEntity, int index, rust::vec<uint8_t> &value)
{
    std::vector<uint8_t> val;
    int ret = rowEntity.Get(index).GetBlob(val);
    for (auto &v : val) {
        value.push_back(v);
    }
    return ret;
}

std::shared_ptr<RdbStore> GetRdbStore(
    const RdbStoreConfig &config, int version, rust::Box<OpenCallbackWrapper> openCallbackWrapper, int &errCode)
{
    OpenCallback callback(std::move(openCallbackWrapper));
    return RdbHelper::GetRdbStore(config, version, callback, errCode);
}

} // namespace OHOS::Request