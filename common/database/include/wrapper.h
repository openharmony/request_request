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
#ifndef REQUEST_DATABASE_WRAPPER_H

#define REQUEST_DATABASE_WRAPPER_H

#include <cstdint>
#include <iostream>
#include <memory>
#include <vector>

#include "cxx.h"
#include "rdb_helper.h"
#include "rdb_open_callback.h"
#include "rdb_store.h"
#include "result_set.h"
#include "value_object.h"
namespace OHOS::Request {
using namespace OHOS::NativeRdb;

struct OpenCallbackWrapper;

inline void BindI32(const int32_t value, std::vector<ValueObject> &args)
{
    args.push_back(ValueObject(value));
}

inline void BindI64(const int64_t value, std::vector<ValueObject> &args)
{
    args.push_back(ValueObject(value));
}

inline void BindDouble(const double value, std::vector<ValueObject> &args)
{
    args.push_back(ValueObject(value));
}

inline void BindBool(const bool value, std::vector<ValueObject> &args)
{
    args.push_back(ValueObject(value));
}

inline void BindString(const rust::str value, std::vector<ValueObject> &args)
{
    args.push_back(ValueObject(std::string(value)));
}

inline void BindBlob(const rust::slice<const uint8_t> value, std::vector<ValueObject> &args)
{
    auto blob = std::vector<uint8_t>(value.begin(), value.end());
    args.push_back(ValueObject(blob));
}

inline void BindNull(std::vector<ValueObject> &args)
{
    args.push_back(ValueObject(std::monostate()));
}

inline int GetI32(RowEntity &rowEntity, int index, int32_t &value)
{
    return rowEntity.Get(index).GetInt(value);
}

inline int GetI64(RowEntity &rowEntity, int index, int64_t &value)
{
    return rowEntity.Get(index).GetLong(value);
}

inline int GetDouble(RowEntity &rowEntity, int index, double &value)
{
    return rowEntity.Get(index).GetDouble(value);
}

inline int GetBool(RowEntity &rowEntity, int index, bool &value)
{
    return rowEntity.Get(index).GetBool(value);
}

inline bool IsNull(RowEntity &rowEntity, int index)
{
    return rowEntity.Get(index).GetType() == ValueObject::TypeId::TYPE_NULL;
}

int GetString(RowEntity &rowEntity, int index, rust::string &value);

int GetBlob(RowEntity &rowEntity, int index, rust::vec<uint8_t> &value);

inline std::unique_ptr<std::vector<ValueObject>> NewVector()
{
    return std::make_unique<std::vector<ValueObject>>();
}

inline std::unique_ptr<RdbStoreConfig> NewConfig(rust::str path)
{
    return std::make_unique<RdbStoreConfig>(std::string(path));
}

inline std::unique_ptr<RowEntity> NewRowEntity()
{
    return std::make_unique<RowEntity>();
}

inline int32_t Execute(RdbStore &store, const rust::str sql, const std::unique_ptr<std::vector<ValueObject>> args)
{
    return store.Execute(std::string(sql), *args).first;
}

inline std::shared_ptr<ResultSet> Query(
    RdbStore &store, const rust::str sql, const std::unique_ptr<std::vector<ValueObject>> args)
{
    return store.QueryByStep(std::string(sql), *args);
}

std::shared_ptr<RdbStore> GetRdbStore(
    const RdbStoreConfig &config, int version, rust::Box<OpenCallbackWrapper> callback, int &errCode);

class OpenCallback : public RdbOpenCallback {
public:
    OpenCallback(rust::Box<OpenCallbackWrapper> &&inner);
    int OnCreate(RdbStore &store) override;
    int OnUpgrade(RdbStore &store, int oldVersion, int newVersion) override;
    int OnDowngrade(RdbStore &store, int currentVersion, int targetVersion) override;
    int OnOpen(RdbStore &store) override;
    int onCorruption(std::string databaseFile) override;

private:
    rust::Box<OpenCallbackWrapper> inner_;
};

} // namespace OHOS::Request

#endif // REQUEST_DATABASE_WRAPPER_H