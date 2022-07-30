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

#include "task_info_dumper_factory.h"
#include "dump_task_info.h"
#include "i_dumper.h"

namespace OHOS::Request::Download {
std::shared_ptr<IDumper> TaskInfoDumperFactory::CreateDumper()
{
    return std::make_shared<DumpTaskInfo>();
}
}
