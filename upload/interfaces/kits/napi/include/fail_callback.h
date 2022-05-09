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

#ifndef FAIL_CALLBACK
#define FAIL_CALLBACK

#include <uv.h>
#include "i_fail_callback.h"
#include "js_util.h"
#include "napi/native_common.h"
#include "napi/native_api.h"
#include "napi/native_node_api.h"

namespace OHOS::Request::Upload {
class FailCallback : public IFailCallback {
public:
    FailCallback(napi_env env, napi_value callback);
    virtual ~FailCallback();
    void Fail(const unsigned int error) override;
private:
    struct FailWorker {
        const FailCallback *callback = nullptr;
        const unsigned int error;
        FailWorker(const FailCallback * const & callbackIn, const unsigned int &errorIn)
            : callback(callbackIn),
              error(errorIn) {}
    };
    void CheckQueueWorkRet(int ret, FailWorker *failWorker, uv_work_t *work);
    napi_status status_ = napi_ok;
    napi_ref callback_ = nullptr;
    napi_env env_;
    uv_loop_s *loop_ = nullptr;
};
} // end of OHOS::Request::Upload
#endif