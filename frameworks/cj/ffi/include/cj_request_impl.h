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

#include <map>
#include <string>

#include "cj_request_ffi.h"
#include "constant.h"
#include "napi_base_context.h"
#include "request_common.h"

namespace OHOS::CJSystemapi::Request {

using OHOS::Request::Action;
using OHOS::Request::Config;
using OHOS::Request::ExceptionError;
using OHOS::Request::ExceptionErrorCode;
using OHOS::Request::FileSpec;
using OHOS::Request::Filter;
using OHOS::Request::FormItem;
using OHOS::Request::Mode;
using OHOS::Request::State;
using OHOS::Request::TaskInfo;

class CJRequestImpl {
public:
    CJRequestImpl() = default;
    ~CJRequestImpl() = default;

    static RetReqData CreateTask(OHOS::AbilityRuntime::Context *context, CConfig *ffiConfig);
    static RetTask GetTask(OHOS::AbilityRuntime::Context *context, std::string taskId,
                           RequestNativeOptionCString &cToken);
    static void FreeTask(std::string taskId);
    static RetError RemoveTask(std::string taskId);
    static RetTaskInfo ShowTask(std::string taskId);
    static RetTaskInfo TouchTask(std::string taskId, const char *token);
    static RetTaskArr SearchTask(CFilter &filter);
    static ExceptionError Convert2Filter(CFilter &filter, Filter &out);
    static RetError ProgressOn(char *event, std::string taskId, void *callback);
    static RetError ProgressOff(char *event, std::string taskId, void *callback);
    static RetError TaskStart(std::string taskId);
    static RetError TaskPause(std::string taskId);
    static RetError TaskResume(std::string taskId);
    static RetError TaskStop(std::string taskId);

    static RetError Convert2RetErr(ExceptionErrorCode code);
    static RetError Convert2RetErr(ExceptionError &err);
    static std::map<std::string, std::string> ConvertCArr2Map(const CHashStrArr *cheaders);
    static void Convert2Config(CConfig *config, Config &out);
    static CTaskInfo Convert2CTaskInfo(TaskInfo &task);
    static RequestCArrString Convert2CStringArray(std::vector<std::string> &tids);

    static Filter Convert2Filter(CFilter &filter);
    static void Convert2CConfig(Config &config, CConfig &out);
    static CConfigDataTypeUion Convert2RequestData(Action action, std::string &data, const std::vector<FileSpec> &files,
                                                   const std::vector<FormItem> &forms);
    static std::string ParseBundle(RequestNativeOptionCString &bundle);
    static int64_t ParseBefore(RequestNativeOptionInt64 &before);
    static int64_t ParseAfter(RequestNativeOptionInt64 &after, int64_t before);
    static State ParseState(RequestNativeOptionUInt32 &state);
    static Action ParseAction(RequestNativeOptionUInt32 &action);
    static Mode ParseMode(RequestNativeOptionUInt32 &mode);

    static ExceptionError ParseToken(RequestNativeOptionCString &cToken, std::string &out);

private:
    static RetError TaskExec(std::string execType, std::string taskId);
};

} // namespace OHOS::CJSystemapi::Request

#endif // OHOS_REQUEST_CJ_REQUEST_IMPL_H