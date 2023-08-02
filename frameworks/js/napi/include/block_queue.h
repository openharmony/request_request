/*
 * Copyright (c) 2023 Huawei Device Co., Ltd.
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

#ifndef OHOS_BLOCK_QUEUE_H
#define OHOS_BLOCK_QUEUE_H
#include <condition_variable>
#include <mutex>
#include <queue>

namespace OHOS {
template<typename T> class BlockQueue {
public:
    explicit BlockQueue(uint32_t timeout) : timeout_(timeout)
    {
    }

    ~BlockQueue() = default;

    void Pop()
    {
        std::lock_guard<std::mutex> lock(queuesMutex_);
        queues_.pop();
        cv_.notify_all();
    }

    void Push(const T &data)
    {
        std::lock_guard<std::mutex> lock(queuesMutex_);
        queues_.push(data);
    }

    void Wait(const T &data)
    {
        if (!IsReady(data)) {
            std::unique_lock<std::mutex> lock(cvMutex_);
            cv_.wait_for(lock, std::chrono::milliseconds(timeout_), [&data, this]() { return IsReady(data); });
        }
    }

    bool IsReady(const T &data)
    {
        std::lock_guard<std::mutex> lock(queuesMutex_);
        return data == queues_.front();
    }

private:
    const uint32_t timeout_;
    std::mutex queuesMutex_;
    std::queue<T> queues_;
    std::mutex cvMutex_;
    std::condition_variable cv_;
};
} // namespace OHOS
#endif // OHOS_BLOCK_QUEUE_H
