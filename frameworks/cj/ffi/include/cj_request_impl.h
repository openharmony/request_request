/*
 * Copyright (c) 2024 Huawei Device Co., Ltd.
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

#ifndef OHOS_REQUEST_CJ_REQUEST_IMPL_H
#define OHOS_REQUEST_CJ_REQUEST_IMPL_H

#include <string>
#include <map>
#include "napi_base_context.h"
#include "cj_request_ffi.h"
#include "constant.h"
#include "js_common.h"

namespace OHOS::CJSystemapi::Request {

using OHOS::Request::ExceptionError;
using OHOS::Request::ExceptionErrorCode;
using OHOS::Request::Config;

class CJRequestImpl {
public:
    CJRequestImpl() = default;
    ~CJRequestImpl() = default;

    static RetReqData CreateTask(OHOS::AbilityRuntime::Context* context, CConfig *ffiConfig);
    static RetError RemoveTask(std::string taskId);
    static void FreeTask(std::string taskId);
    static RetError ProgressOn(char *event, std::string taskId, void (*callback)(CProgress progress));
    static RetError ProgressOff(char *event, std::string taskId, void *callback);
    static RetError TaskStart(std::string taskId);
    static RetError TaskPause(std::string taskId);
    static RetError TaskResume(std::string taskId);
    static RetError TaskStop(std::string taskId);

    static RetError Convert2RetErr(ExceptionErrorCode code);
    static RetError Convert2RetErr(ExceptionError &err);
    static std::map<std::string, std::string> ConvertCArr2Map(const CHashStrArr *cheaders);
    static void Convert2Config(CConfig *config, Config &out);
private:
    static RetError TaskExec(std::string execType, std::string taskId);
};

} // namespace OHOS::CJSystemapi::Request

#endif // OHOS_REQUEST_CJ_REQUEST_IMPL_H