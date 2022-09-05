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
#include <mutex>
#include "async_call.h"
#include "download_notify_stub.h"
#include "download_task.h"
#include "napi/native_api.h"
#include "noncopyable.h"


namespace OHOS::Request::Download {
struct NotifyData {
    napi_env env;
    napi_ref ref;
    uint32_t paramNumber;
    std::mutex mutex_;
    std::vector<uint32_t> params;
    NotifyData(napi_env envIn, napi_ref refIn, uint32_t paramNumberIn, std::vector<uint32_t> &paramsIn)
        :env(envIn), ref(refIn), paramNumber(paramNumberIn) {}
};

class DownloadBaseNotify : public DownloadNotifyStub {
public:
    ACE_DISALLOW_COPY_AND_MOVE(DownloadBaseNotify);
    DOWNLOAD_API explicit DownloadBaseNotify(napi_env env, uint32_t paramNumber, napi_ref ref);
    virtual ~DownloadBaseNotify();
    void CallBack(const std::vector<uint32_t> &params);
	NotifyDataPtr *GetNotifyDataPtr();

private:
    NotifyDataPtr {
        std::shared_ptr<NotifyData> notifyData;
    }
    std::shared_ptr<NotifyData> notifyData_;
};
} // namespace OHOS::Request::Download

#endif // DOWNLOAD_BASE_NOTIFY_H