/*
 * Copyright (C) 2022 Huawei Device Co., Ltd.
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

#ifndef DOWNLOAD_MANAGER_H
#define DOWNLOAD_MANAGER_H

#include <map>
#include <mutex>
#include <condition_variable>

#include "data_ability_helper.h"
#include "iremote_object.h"
#include "refbase.h"
#include "visibility.h"

#include "download_notify_stub.h"
#include "download_service_interface.h"

#include "download_config.h"
#include "download_info.h"
#include "download_task.h"

namespace OHOS::Request::Download {
class DownloadSaDeathRecipient : public IRemoteObject::DeathRecipient {
public:
    explicit DownloadSaDeathRecipient();
    ~DownloadSaDeathRecipient() = default;
    void OnRemoteDied(const wptr<IRemoteObject> &object) override;
};

class DownloadManager : public RefBase {
public:
    DownloadManager();
    ~DownloadManager();
    DOWNLOAD_API static sptr<DownloadManager> GetInstance();
    DOWNLOAD_API DownloadTask *EnqueueTask(const DownloadConfig &config, ExceptionError &err);
    DOWNLOAD_API bool Pause(uint32_t taskId);
    DOWNLOAD_API bool Query(uint32_t taskId, DownloadInfo &info);
    DOWNLOAD_API bool QueryMimeType(uint32_t taskId, std::string &mimeType);
    DOWNLOAD_API bool Remove(uint32_t taskId);
    DOWNLOAD_API bool Resume(uint32_t taskId);

    DOWNLOAD_API bool On(uint32_t taskId, const std::string &type, const sptr<DownloadNotifyInterface> &listener);
    DOWNLOAD_API bool Off(uint32_t taskId, const std::string &type);

    DOWNLOAD_API bool CheckPermission();
    
    void OnRemoteSaDied(const wptr<IRemoteObject> &object);
    DOWNLOAD_API bool LoadDownloadServer();
    void LoadServerSuccess();
    void LoadServerFail();
private:
    sptr<DownloadServiceInterface> GetDownloadServiceProxy();
    int32_t Retry(int32_t &errorCode,const DownloadConfig &config);
    void DealErrorCode(int32_t errorCode, ExceptionError &err);

private:
    static std::mutex instanceLock_;
    static sptr<DownloadManager> instance_;
    std::mutex downloadMutex_;
    std::mutex conditionMutex_;

    sptr<DownloadServiceInterface> downloadServiceProxy_;
    sptr<DownloadSaDeathRecipient> deathRecipient_;
    std::condition_variable downloadSyncCon_;
    bool ready_ = false;
    static constexpr int LOAD_SA_TIMEOUT_MS = 15000;
};
} // namespace OHOS::Request::Download
#endif // DOWNLOAD_MANAGER_H
