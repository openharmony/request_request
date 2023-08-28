/*
 * Copyright (C) 2023 Huawei Device Co., Ltd.
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

#ifndef REQUEST_NOTIFY_H
#define REQUEST_NOTIFY_H

#include <string>
#include <chrono>
#include <mutex>
#include "uv.h"
#include "constant.h"
#include "visibility.h"
#include "js_common.h"
#include "notify_stub.h"
#include "uv_queue.h"
#include "napi/native_api.h"
#include "noncopyable.h"
#include "block_queue.h"


namespace OHOS::Request {
struct NotifyEventInfo {
    int64_t timestamp{};
    bool operator==(const NotifyEventInfo &info) const
    {
        return (timestamp == info.timestamp);
    }
};

class RequestNotify : public NotifyStub {
public:
    REQUEST_API explicit RequestNotify(napi_env env, napi_value callback);
    REQUEST_API explicit RequestNotify() = default;
    virtual ~RequestNotify();
    void CallBack(const Notify &notify) override;
    void Done(const TaskInfo &taskInfo) override;
    void SetNotify(const Notify &notify);
    void DeleteCallbackRef();
    void ConvertCallBackData(uint32_t &paramNumber, napi_value *value, uint32_t valueSize);
    void ExecCallBack();

    bool valid_;
    napi_env env_;
    std::mutex validMutex_;
    napi_ref ref_;
    Notify notify_;
    std::mutex notifyMutex_;
    NotifyEventInfo info_;
    static BlockQueue<NotifyEventInfo> notifyQueue_;
};

struct NotifyDataPtr {
    sptr<RequestNotify> callback = nullptr;
};
} // namespace OHOS::Request

#endif // REQUEST_NOTIFY_H