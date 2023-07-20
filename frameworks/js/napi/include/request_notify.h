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

#include <atomic>
#include <string>
#include <mutex>
#include "uv.h"
#include "constant.h"
#include "visibility.h"
#include "js_common.h"
#include "notify_stub.h"
#include "uv_queue.h"
#include "napi/native_api.h"
#include "noncopyable.h"


namespace OHOS::Request {
struct CallbackData {
    napi_env env;
    napi_ref ref;
    std::mutex mutex;
    Notify notify;
    bool valid;
    ~CallbackData()
    {
        if (valid == true) {
            UvQueue::DeleteRef(env, ref);
        }
    }
};

struct NotifyDataPtr {
    std::shared_ptr<CallbackData> dataPtr;
};

class RequestNotify : public NotifyStub {
public:
    REQUEST_API explicit RequestNotify(napi_env env, napi_value callback);
    REQUEST_API explicit RequestNotify() = default;
    virtual ~RequestNotify();
    void CallBack(const Notify &notify) override;
    void Done(const TaskInfo &taskInfo) override;
    void GetDataPtrParam(const std::shared_ptr<CallbackData> &dataPtr, const Notify &notify);
    void DeleteCallbackRef();
    static void GetCallBackData(NotifyDataPtr *notifyDataPtr);
    static void ConvertCallBackData(const std::shared_ptr<CallbackData> &dataPtr, uint32_t &paramNumber,
        napi_value *value);
    napi_env env_;
    napi_ref ref_;
private:
    std::shared_ptr<CallbackData> data_;
    std::atomic<bool> valid_;
};
} // namespace OHOS::Request

#endif // REQUEST_NOTIFY_H