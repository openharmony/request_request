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

#ifndef NOTIFY_CALLBACK_H
#define NOTIFY_CALLBACK_H

#include <uv.h>
#include <map>
#include <vector>
#include "upload_common.h"
#include "js_util.h"
#include "i_notify_callback.h"
#include "napi/native_common.h"
#include "napi/native_api.h"
#include "napi/native_node_api.h"

namespace OHOS::Request::Upload {
class NotifyCallback : public INotifyCallback {
public:
    NotifyCallback(napi_env env, napi_value callback);
    virtual ~NotifyCallback();
    void Notify(const std::vector<TaskState> &taskStates) override;
    napi_ref GetCallback() override;
private:
    struct NotifyWorker {
        const NotifyCallback *callback = nullptr;
        const std::vector<TaskState> taskStates;
        NotifyWorker(const NotifyCallback *callbackIn, const std::vector<TaskState> &taskStatesIn)
            : callback(callbackIn), taskStates(taskStatesIn) {}
    };

    napi_ref callback_ = nullptr;
    napi_env env_;
    uv_loop_s *loop_ = nullptr;
};
} // end of OHOS::Request::Upload
#endif