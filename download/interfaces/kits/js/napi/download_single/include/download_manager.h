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
    static sptr<DownloadManager> GetInstance();
    DownloadTask *EnqueueTask(const DownloadConfig &config, ExceptionError &err);
    bool Pause(uint32_t taskId);
    bool Query(uint32_t taskId, DownloadInfo &info);
    bool QueryMimeType(uint32_t taskId, std::string &mimeType);
    bool Remove(uint32_t taskId);
    bool Resume(uint32_t taskId);

    bool On(uint32_t taskId, const std::string &type, const sptr<DownloadNotifyInterface> &listener);
    bool Off(uint32_t taskId, const std::string &type);

    bool CheckPermission();
    
    void OnRemoteSaDied(const wptr<IRemoteObject> &object);
    bool LoadDownloadServer();
    void LoadServerSuccess();
    void LoadServerFail();
private:
    sptr<DownloadServiceInterface> GetDownloadServiceProxy();

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
