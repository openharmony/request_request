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

#include "progress_callback.h"
#include "upload_task.h"
#include "upload_common.h"
using namespace OHOS::Request::UploadNapi;

namespace OHOS::Request::Upload {
ProgressCallback::ProgressCallback(napi_env env, napi_value callback)
    : env_(env)
{
    napi_create_reference(env, callback, 1, &callback_);
    napi_get_uv_event_loop(env, &loop_);
}

ProgressCallback::~ProgressCallback()
{
    if (callback_ != nullptr) {
        napi_delete_reference(env_, callback_);
    }
}

napi_ref ProgressCallback::GetCallback()
{
    return callback_;
}

napi_env ProgressCallback::GetEnv()
{
    return env_;
}

int64_t ProgressCallback::GetUploadedSize()
{
    return uploadedSize_;
}

int64_t ProgressCallback::GetTotalSize()
{
    return totalSize_;
}

void UvOnProgress(uv_work_t *work, int status)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Progress. uv_queue_work start");
    std::shared_ptr<ProgressWorker> progressWorker(reinterpret_cast<ProgressWorker *>(work->data),
        [work](ProgressWorker *data) {
            delete data;
            delete work;
    });
    
    if (progressWorker == nullptr || progressWorker->observer == nullptr) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "progressWorker->observer == nullptr");
        return;
    }
    napi_handle_scope scope = nullptr;
    napi_open_handle_scope(progressWorker->observer->GetEnv(), &scope);
    napi_value callback = nullptr;
    napi_value args[2];
    napi_value global = nullptr;
    napi_value result;
    napi_status calStatus = napi_generic_failure;
    napi_create_int64(progressWorker->observer->GetEnv(), progressWorker->observer->GetUploadedSize(), &args[0]);
    napi_create_int64(progressWorker->observer->GetEnv(), progressWorker->observer->GetTotalSize(), &args[1]);
    napi_get_reference_value(progressWorker->observer->GetEnv(), progressWorker->observer->GetCallback(), &callback);
    napi_get_global(progressWorker->observer->GetEnv(), &global);
    calStatus = napi_call_function(progressWorker->observer->GetEnv(), global, callback, TWO_ARG, args, &result);
    if (calStatus != napi_ok) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI,
            "Progress callback failed calStatus:%{public}d", calStatus);
    }
    napi_close_handle_scope(progressWorker->observer->GetEnv(), scope);
}


void ProgressCallback::Progress(const int64_t uploadedSize, const int64_t totalSize)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI,
        "Progress. uploadedSize : %lld, totalSize : %lld", (long long)uploadedSize, (long long)totalSize);
    ProgressWorker *progressWorker = new (std::nothrow)ProgressWorker();
    if (progressWorker == nullptr) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Failed to create progressWorker");
        return;
    }
    progressWorker->observer = shared_from_this();
    uploadedSize_ = uploadedSize;
    totalSize_ = totalSize;

    uv_work_t *work = new (std::nothrow)uv_work_t();
    if (work == nullptr) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Failed to create uv work");
        delete progressWorker;
        return;
    }
    work->data = progressWorker;
    int ret = uv_queue_work(loop_, work, [](uv_work_t *work) {}, UvOnProgress);
    if (ret != 0) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "Progress. uv_queue_work Failed");
        delete progressWorker;
        delete work;
    }
}
} // end of OHOS::Request::Upload