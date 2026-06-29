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
/**
 * @brief Observer interface for changes in the number of running tasks.
 *
 * External modules implement this interface and register it via
 * SubscribeRunningTaskCount. The callback is invoked when the number of
 * running tasks in the request service changes.
 */
class IRunningTaskObserver {
public:
    virtual ~IRunningTaskObserver() = default;
    /**
     * @brief Callback for running task count updates.
     * @param count Total number of tasks currently in the running state.
     */
    virtual void OnRunningTaskCountUpdate(int count) = 0;
};

/**
 * @brief Subscribe to running task count change events.
 * @param ob Observer instance, must not be null.
 * @return 0 on success, other values are error codes.
 */
REQUEST_API int32_t SubscribeRunningTaskCount(std::shared_ptr<IRunningTaskObserver> ob);
/**
 * @brief Unsubscribe from running task count change events.
 * @param ob Previously registered observer instance; passing an unregistered instance is a no-op.
 */
REQUEST_API void UnsubscribeRunningTaskCount(std::shared_ptr<IRunningTaskObserver> ob);

} // namespace OHOS::Request

#endif // REQUEST_RUNNING_TASK_COUNT_H