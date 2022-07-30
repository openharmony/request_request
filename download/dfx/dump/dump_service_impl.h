/*
 * Copyright (C) 2021-2022 Huawei Device Co., Ltd.
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

#ifndef DUMP_SERVICE_IMPL_H
#define DUMP_SERVICE_IMPL_H

#include <map>
#include <memory>
#include <iosfwd>
#include "dumper_factory.h"

namespace OHOS::Request::Download {
enum DumperType : uint32_t {
    HELP_DUMPER = 0,
    TASK_INFO_DUMPER,
    DUMPER_NUM,
};

class DumpServiceImpl {
public:
    static DumpServiceImpl &GetInstance();
    int Dump(int fd, const std::vector<std::string> &args);
private:
    DumpServiceImpl();
    virtual ~DumpServiceImpl();
    DumpServiceImpl(DumpServiceImpl const &) = delete;
    void operator=(DumpServiceImpl const &) = delete;
    DumpServiceImpl(DumpServiceImpl &&) = delete;
    DumpServiceImpl &operator=(DumpServiceImpl &&) = delete;

    void InitDumperFactoryMap();
    void DumpHelp(int fd) const;
    DumperType GetDumperType(const std::string &cmd) const;
private:
    using DumperFactoryMap = std::map<DumperType, std::shared_ptr<DumperFactory>>;
    DumperFactoryMap dumperFactoryMap_;
};
}

#endif // DUMP_SERVICE_IMPL_H
