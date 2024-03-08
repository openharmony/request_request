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

#ifndef REQUEST_RUN_COUNT_OBSERVER_H
#define REQUEST_RUN_COUNT_OBSERVER_H
#include <stdint.h>

#include <functional>

namespace OHOS::Request {
class RunCountObserver {
public:
    ~RunCountObserver();
    using RegCallBack = std::function<void(int32_t uid, int32_t state, int32_t pid)>;
    static RunCountObserver &GetInstance();
    bool RegisterRunCountChanged(RegCallBack &&callback);

public:
    RunCountObserver();
    RegCallBack callback_ = nullptr;
};
} // namespace OHOS::Request

#ifdef __cplusplus
extern "C" {
#endif

typedef void (*RunCountCallback)(int32_t);
void RegisterRunCountCallback(RunCountCallback fun);

#ifdef __cplusplus
}
#endif

#endif // REQUEST_RUN_COUNT_OBSERVER_H