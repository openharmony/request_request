/*
 * Copyright (c) 2022 Huawei Device Co., Ltd.
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
 
#include "notify_callback.h"
#include "upload_task.h"

using namespace OHOS::Request::UploadNapi;

namespace OHOS::Request::Upload {
NotifyCallback::NotifyCallback(napi_env env, napi_value callback)
    : env_(env)
{
    napi_create_reference(env, callback, 1, &callback_);
    napi_get_uv_event_loop(env, &loop_);
}

NotifyCallback::~NotifyCallback()
{
    napi_delete_reference(env_, callback_);
}

napi_ref NotifyCallback::GetCallback()
{
    return callback_;
}

napi_env NotifyCallback::GetEnv()
{
    return env_;
}

std::vector<TaskState> NotifyCallback::GetTaskStates()
{
    std::lock_guard<std::mutex> lock(mutex_);
    return taskStates_;
}

void UvOnNotify(uv_work_t *work, int status)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Notify. uv_queue_work start");
    std::shared_ptr<NotifyWorker> notifyWorker(reinterpret_cast<NotifyWorker *>(work->data),
        [work](NotifyWorker *data) {
            delete data;
            delete work;
    });
    if (notifyWorker == nullptr || notifyWorker->observer == nullptr) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "notifyWorker->observer == nullptr");
        return;
    }
    napi_handle_scope scope = nullptr;
    napi_open_handle_scope(notifyWorker->observer->GetEnv(), &scope);
    napi_value callback = nullptr;
    napi_value global = nullptr;
    napi_value args[1] = {nullptr};
    napi_value result;
    napi_status callStatus = napi_generic_failure;
    args[0] = UploadNapi::JSUtil::Convert2JSValue(notifyWorker->observer->GetEnv(),
                                                  notifyWorker->observer->GetTaskStates());
    napi_get_reference_value(notifyWorker->observer->GetEnv(),
                             notifyWorker->observer->GetCallback(), &callback);
    napi_get_global(notifyWorker->observer->GetEnv(), &global);
    callStatus = napi_call_function(notifyWorker->observer->GetEnv(), global, callback, 1, args, &result);
    if (callStatus != napi_ok) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI,
            "Notify callback failed callStatus:%{public}d", callStatus);
    }
    napi_close_handle_scope(notifyWorker->observer->GetEnv(), scope);
}

void NotifyCallback::Notify(const std::vector<TaskState> &taskStates)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "NotifyCallback::Notify in");
    NotifyWorker *notifyWorker = new (std::nothrow)NotifyWorker();
    if (notifyWorker == nullptr) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Failed to create NotifyWorker");
        return;
    }
    
    {
        std::lock_guard<std::mutex> lock(mutex_);
        notifyWorker->observer = shared_from_this();
        taskStates_ = taskStates;
    }

    uv_work_t *work = new (std::nothrow)uv_work_t();
    if (work == nullptr) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Failed to create uv work");
        delete notifyWorker;
        return;
    }
    work->data = notifyWorker;
    int ret = uv_queue_work(loop_, work, [](uv_work_t *work) {}, UvOnNotify);
    if (ret != 0) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "Notify. uv_queue_work Failed");
        delete notifyWorker;
        delete work;
    }
}
} // end of OHOS::Request::Upload