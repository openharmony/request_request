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
    status_ = napi_generic_failure;
}

void ProgressCallback::CheckQueueWorkRet(int ret, ProgressWorker *progressWorker, uv_work_t *work)
{
    if (ret != 0) {
        if (progressWorker != nullptr) {
            delete progressWorker;
            progressWorker = nullptr;
        }
        if (work != nullptr) {
            delete work;
            work = nullptr;
        }
    }
}

void ProgressCallback::Progress(const int64_t uploadedSize, const int64_t totalSize)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI,
        "Progress. uploadedSize : %lld, totalSize : %lld", uploadedSize, totalSize);
    ProgressWorker *progressWorker = new ProgressWorker(this, uploadedSize, totalSize);
    uv_work_t *work = new uv_work_t;
    work->data = progressWorker;
    int ret = uv_queue_work(loop_, work,
        [](uv_work_t *work) {},
        [](uv_work_t *work, int status) {
            ProgressWorker *progressWorkerInner = reinterpret_cast<ProgressWorker *>(work->data);
            napi_value jsUploaded = nullptr;
            napi_value jsTotal = nullptr;
            napi_value callback = nullptr;
            napi_value args[2];
            napi_value global = nullptr;
            napi_value result;
            napi_status calStatus = napi_generic_failure;
            napi_env tmpEnv = progressWorkerInner->callback->env_;
            if (progressWorkerInner->callback->env_ == tmpEnv && progressWorkerInner->callback->status_ == napi_ok) {
                napi_create_int64(progressWorkerInner->callback->env_, progressWorkerInner->uploadedSize, &jsUploaded);
                args[0] = jsUploaded;
                napi_create_int64(progressWorkerInner->callback->env_, progressWorkerInner->totalSize, &jsTotal);
                args[1] = jsTotal;
            } else {
                goto EXIT_CODE;
            }
            if (progressWorkerInner->callback->env_ == tmpEnv && progressWorkerInner->callback->callback_ != nullptr &&
                progressWorkerInner->callback->status_ == napi_ok) {
                napi_get_reference_value(progressWorkerInner->callback->env_,
                    progressWorkerInner->callback->callback_, &callback);
                napi_get_global(progressWorkerInner->callback->env_, &global);
                calStatus = napi_call_function(progressWorkerInner->callback->env_, global, callback, 2, args, &result);
            } else {
                goto EXIT_CODE;
            }
            if (calStatus != napi_ok) {
                UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI,
                    "Progress callback failed calStatus:%{public}d callback:%{public}p", calStatus, callback);
            }
EXIT_CODE :
            delete progressWorkerInner;
            progressWorkerInner = nullptr;
            delete work;
            work = nullptr;
        });
    CheckQueueWorkRet(ret, progressWorker, work);
}
} // end of OHOS::Request::Upload