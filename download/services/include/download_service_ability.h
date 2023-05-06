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

#ifndef DOWNLOAD_SERVICE_ABILITY_H
#define DOWNLOAD_SERVICE_ABILITY_H

#include <mutex>
#include <stdint.h>
#include <iosfwd>
#include <functional>
#include <mutex>
#include <map>
#include <string>
#include <vector>
#include <memory>
#include "nocopyable.h"
#include "refbase.h"
#include "event_handler.h"
#include "system_ability.h"
#include "download_info.h"
#include "download_config.h"
#include "download_notify_interface.h"
#include "download_service_stub.h"


namespace OHOS::Request::Download {
enum class ServiceRunningState { STATE_NOT_START, STATE_RUNNING };
class IKeyguardStateCallback;

class DownloadServiceAbility : public SystemAbility, public DownloadServiceStub {
    DECLARE_SYSTEM_ABILITY(DownloadServiceAbility);

public:
    DISALLOW_COPY_AND_MOVE(DownloadServiceAbility);
    DownloadServiceAbility(int32_t systemAbilityId, bool runOnCreate);
    DownloadServiceAbility();
    ~DownloadServiceAbility();
    static sptr<DownloadServiceAbility> GetInstance();

    int32_t Request(const DownloadConfig &config, ExceptionError &err) override;
    bool Pause(uint32_t taskId) override;
    bool Query(uint32_t taskId, DownloadInfo &info) override;
    bool QueryMimeType(uint32_t taskId, std::string &mimeType) override;
    bool Remove(uint32_t taskId) override;
    bool Resume(uint32_t taskId) override;

    bool On(uint32_t taskId, const std::string &type, const sptr<DownloadNotifyInterface> &listener) override;
    bool Off(uint32_t taskId, const std::string &type) override;

    bool CheckPermission() override;

    int Dump(int fd, const std::vector<std::u16string> &args) override;

    static void NotifyHandler(const std::string& type, uint32_t taskId, int64_t argv1, int64_t argv2, bool isNotify);

protected:
    void OnDump() override;
    void OnStart() override;
    void OnStop() override;

private:
    int32_t Init();
    void InitServiceHandler();
    void ManualStart();
    void AddUnregisteredNotify(uint32_t taskId, const std::string &type);
    bool DoUnregisteredNotify(uint32_t taskId, const std::string &type);
private:
    ServiceRunningState state_;
    static std::mutex instanceLock_;
    static sptr<DownloadServiceAbility> instance_;
    static std::shared_ptr<AppExecFwk::EventHandler> serviceHandler_;
    std::map<std::string, sptr<DownloadNotifyInterface>> registeredListeners_;
    std::vector<sptr<DownloadNotifyInterface>> unlockVecListeners_;
    std::mutex listenerMapMutex_;
    std::mutex lock_;
    const int32_t startTime_ = 1900;
    const int32_t extraMonth_ = 1;
    std::mutex unregisteredNotifyMutex_;
    std::map<std::string, uint32_t> unregisteredNotify_;
};
} // namespace OHOS::Request::Download
#endif // DOWNLOAD_SYSTEM_ABILITY_H
