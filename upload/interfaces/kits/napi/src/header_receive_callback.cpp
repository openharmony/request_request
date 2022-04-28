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

#include "header_receive_callback.h"
#include "upload_task.h"

namespace OHOS::Request::Upload {
HeaderReceiveCallback::HeaderReceiveCallback(napi_env env, napi_value callback)
    : env_(env)
{
    napi_create_reference(env, callback, 1, &callback_);
    napi_get_uv_event_loop(env, &loop_);
}

HeaderReceiveCallback::~HeaderReceiveCallback()
{
    napi_delete_reference(env_, callback_);
    status_ = napi_generic_failure;
}

void HeaderReceiveCallback::CheckQueueWorkRet(int ret, HeaderReceiveWorker *headerReceiveWorker, uv_work_t *work)
{
    if (ret != 0) {
        if (headerReceiveWorker != nullptr) {
            delete headerReceiveWorker;
            headerReceiveWorker = nullptr;
        }
        if (work != nullptr) {
            delete work;
            work = nullptr;
        }
    }
}

void HeaderReceiveCallback::HeaderReceive(const std::string &header)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "HeaderReceive. header : %{public}s", header.c_str());
    HeaderReceiveWorker *headerReceiveWorker = new HeaderReceiveWorker(this, header);
    uv_work_t *work = new uv_work_t;
    work->data = headerReceiveWorker;
    int ret = uv_queue_work(loop_, work,
        [](uv_work_t *work) {},
        [](uv_work_t *work, int status) {
            HeaderReceiveWorker *headerReceiveWorkerInner = reinterpret_cast<HeaderReceiveWorker *>(work->data);
            napi_value jsHeader = nullptr;
            napi_value callback = nullptr;
            napi_value args[1];
            napi_value global = nullptr;
            napi_value result;
            napi_status callStatus = napi_generic_failure;
            napi_env tmpEnv = headerReceiveWorkerInner->callback->env_;
            if (headerReceiveWorkerInner->callback->env_ == tmpEnv &&
                headerReceiveWorkerInner->callback->status_ == napi_ok) {
                jsHeader = UploadNapi::JSUtil::Convert2JSString(headerReceiveWorkerInner->callback->env_,
                    headerReceiveWorkerInner->header);
                args[0] = { jsHeader };
            } else {
                goto EXIT_CODE;
            }
            if (headerReceiveWorkerInner->callback->env_ == tmpEnv &&
                headerReceiveWorkerInner->callback->callback_ != nullptr &&
                headerReceiveWorkerInner->callback->status_ == napi_ok) {
                napi_get_reference_value(headerReceiveWorkerInner->callback->env_,
                    headerReceiveWorkerInner->callback->callback_, &callback);
                napi_get_global(headerReceiveWorkerInner->callback->env_, &global);
                callStatus =
                    napi_call_function(headerReceiveWorkerInner->callback->env_, global, callback, 1, args, &result);
            } else {
                goto EXIT_CODE;
            }
            if (callStatus != napi_ok) {
                UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI,
                    "HeaderReceive callback failed callStatus:%{public}d callback:%{public}p", callStatus, callback);
            }
EXIT_CODE :
            delete headerReceiveWorkerInner;
            headerReceiveWorkerInner = nullptr;
            delete work;
            work = nullptr;
        });
    CheckQueueWorkRet(ret, headerReceiveWorker, work);
}
} // end of OHOS::Request::Upload