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

#include "cpp/task.h"
#include "cxx.h"
#include "wrapper.rs.h"

void FfrtSpawn(rust::Box<ClosureWrapper> closure)
{
    ffrt::submit([closure = closure.into_raw()]() mutable {
        closure->run();
        rust::Box<ClosureWrapper>::from_raw(closure);
    });
}

void FfrtSleep(uint64_t ms)
{
    ffrt::this_task::sleep_for(std::chrono::milliseconds(ms));
}