/*
 * Copyright (C) 2021-2022 Huawei Device Co., Ltd.
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

#ifndef DOWNLOAD_BASE_NOTIFY_H
#define DOWNLOAD_BASE_NOTIFY_H

#include <string>
#include "async_call.h"
#include "download_notify_stub.h"
#include "download_task.h"
#include "napi/native_api.h"
#include "noncopyable.h"


namespace OHOS::Request::Download {
struct NotifyData {
    napi_env env;
    napi_ref ref;
    std::string type;
    DownloadTask *task;
    uint32_t firstArgv;
    uint32_t secondArgv;
};

class DownloadBaseNotify : public DownloadNotifyStub {
public:
    ACE_DISALLOW_COPY_AND_MOVE(DownloadBaseNotify);
    explicit DownloadBaseNotify(napi_env env, const std::string &type, const DownloadTask *task, napi_ref ref);
    virtual ~DownloadBaseNotify();
    void OnCallBack(MessageParcel &data) override;

protected:
	NotifyData *GetNotifyData();

protected:
    napi_env env_;
    std::string type_;
    DownloadTask *task_;
    napi_ref ref_;
};
} // namespace OHOS::Request::Download

#endif // DOWNLOAD_BASE_NOTIFY_H