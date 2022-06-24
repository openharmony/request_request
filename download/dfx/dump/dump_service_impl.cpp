/*
 * Copyright (C) 2022 Huawei Device Co., Ltd.
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


#include <cstdio>
#include <string>
#include <vector>
#include "i_dumper.h"
#include "task_info_dumper_factory.h"
#include "dump_service_impl.h"

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

DumperType DumpServiceImpl::GetDumperType(const std::string &agr)
{
    if (agr == "-h") {
        return HELP_DUMPER;
    } else if (agr == "-t") {
        return TASK_INFO_DUMPER;
    } else {
        return DUMPER_NUM;
    }
}

int DumpServiceImpl::Dump(int fd, const std::vector<std::string> &args)
{
    if (args.size() == 0) {
        DumpHelp(fd);
        return 0;
    }

    DumperType dumperType = GetDumperType(args[0]);
    if (dumperType == HELP_DUMPER) {
        DumpHelp(fd);
        return 0;
    }

    DumperFactoryMap::iterator it = dumperFactoryMap_.find(dumperType);
    if (it == dumperFactoryMap_.end()) {
        dprintf(fd, "invalid arg\n");
        return 0;
    }

    std::shared_ptr<DumperFactory> dumperFactory = it->second;
    std::shared_ptr<IDumper> dumper = dumperFactory->CreateDumper();
    if (dumper != nullptr) {
        std::vector<std::string> dumpAgr = args;
        dumpAgr.erase(dumpAgr.begin());
        dumper->Dump(fd, dumpAgr);
    }

    return 0;
}

void DumpServiceImpl::DumpHelp(int fd)
{
    const char* helper =
        "usage:\n"
        "  -h                    help text for the tool\n"
        "  -t [taskid]           with no taskid: display all task summary info; taskid: display one task detail info\n";
    dprintf(fd, "%s\n", helper);
}
}