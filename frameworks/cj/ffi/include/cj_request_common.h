/*
 * Copyright (c) 2024 Huawei Device Co., Ltd.
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef OH_CJ_REQUEST_UTILS_H
#define OH_CJ_REQUEST_UTILS_H

#include <string>
#include <vector>

#include "cj_request_ffi.h"
#include "constant.h"
#include "request_common.h"

namespace OHOS::CJSystemapi::Request {
using OHOS::Request::Action;
using OHOS::Request::ExceptionError;
using OHOS::Request::FileSpec;
using OHOS::Request::FormItem;
using OHOS::Request::Progress;
using OHOS::Request::Reason;
using OHOS::Request::Response;

void ReadBytesFromFile(const std::string &filePath, std::vector<uint8_t> &fileData);
char *MallocCString(const std::string &origin);
bool IsPathValid(const std::string &filePath);
std::string SHA256(const char *str, size_t len);
ExceptionError ConvertError(int32_t errorCode);
void RemoveFile(const std::string &filePath);

CProgress Convert2CProgress(const Progress &in);
CResponse Convert2CResponse(const std::shared_ptr<Response> &in);
std::string GetSaveas(const std::vector<FileSpec> &files, Action action);
uint32_t Convert2Broken(Reason code);
std::string Convert2ReasonMsg(Reason code);
CHashStrArr Convert2CHashStrArr(const std::map<std::string, std::string> &extras);
CFormItemArr Convert2CFormItemArr(const std::vector<FileSpec> &files, const std::vector<FormItem> &forms);
bool CheckApiVersionAfter19();
} // namespace OHOS::CJSystemapi::Request

#endif
