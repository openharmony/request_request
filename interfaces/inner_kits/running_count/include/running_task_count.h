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
#ifndef REQUEST_RUNNING_TASK_COUNT_H
#define REQUEST_RUNNING_TASK_COUNT_H

#include <memory>

#include "visibility.h"
namespace OHOS::Request {
class IRunningTaskObserver {
public:
    virtual ~IRunningTaskObserver() = default;
    virtual void OnRunningTaskCountUpdate(int count) = 0;
};

REQUEST_API int32_t SubscribeRunningTaskCount(std::shared_ptr<IRunningTaskObserver> ob);
REQUEST_API void UnsubscribeRunningTaskCount(std::shared_ptr<IRunningTaskObserver> ob);

} // namespace OHOS::Request

#endif // REQUEST_RUNNING_TASK_COUNT_H