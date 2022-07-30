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

#include "dump_service_impl.h"

#include <cstdio>
#include <string>
#include <vector>
#include <utility>
#include <map>
#include "dumper_factory.h"
#include "i_dumper.h"
#include "task_info_dumper_factory.h"

namespace OHOS::Request::Download {
DumpServiceImpl::DumpServiceImpl()
{
    InitDumperFactoryMap();
}

DumpServiceImpl::~DumpServiceImpl()
{
    dumperFactoryMap_.clear();
}

void DumpServiceImpl::InitDumperFactoryMap()
{
    dumperFactoryMap_.insert(std::make_pair(DumperType::TASK_INFO_DUMPER, std::make_shared<TaskInfoDumperFactory>()));
}

DumpServiceImpl &DumpServiceImpl::GetInstance()
{
    static DumpServiceImpl instance;
    return instance;
}

DumperType DumpServiceImpl::GetDumperType(const std::string &cmd) const
{
    if (cmd == "-h") {
        return HELP_DUMPER;
    } else if (cmd == "-t") {
        return TASK_INFO_DUMPER;
    } else {
        return DUMPER_NUM;
    }
}

int DumpServiceImpl::Dump(int fd, const std::vector<std::string> &args)
{
    if (args.empty()) {
        DumpHelp(fd);
        return 0;
    }

    DumperType dumperType = GetDumperType(args[0]);
    if (dumperType == HELP_DUMPER) {
        DumpHelp(fd);
        return 0;
    }

    DumperFactoryMap::const_iterator it = dumperFactoryMap_.find(dumperType);
    if (it == dumperFactoryMap_.end()) {
        dprintf(fd, "invalid arg\n");
        return 0;
    }

    auto dumper = it->second->CreateDumper();
    if (dumper != nullptr) {
        dumper->Dump(fd, {args.begin() + 1, args.end()});
    }

    return 0;
}

void DumpServiceImpl::DumpHelp(int fd) const
{
    constexpr const char *DEFAULT_HELPER =
        "usage:\n"
        "  -h                    help text for the tool\n"
        "  -t [taskid]           with no taskid: display all task summary info; taskid: display one task detail info\n";
    dprintf(fd, "%s\n", DEFAULT_HELPER);
}
}