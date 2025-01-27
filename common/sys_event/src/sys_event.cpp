/*
* Copyright (C) 2025 Huawei Device Co., Ltd.
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

#include "sys_event.h"

#include <cstdint>
#include <string>
#include <unordered_map>

#include "hisysevent.h"
#include "log.h"

namespace OHOS {
namespace Request {

namespace {
//event params
const std::string PARAM_DFX_CODE = "CODE";
const std::string PARAM_BUNDLE_NAME = "BUNDLE_NAME";
const std::string PARAM_MODULE_NAME = "MODULE_NAME";
const std::string PARAM_EXTRA_INFO = "EXTRA_INFO";

} // namespace

void SysEventLog::SendSysEventLog(const std::string &eventName, const uint32_t dCode, const std::string bundleName,
    const std::string moduleName, const std::string extraInfo)
{
    auto iter = sysEventMap_.find(eventName);
    if (iter == sysEventMap_.end()) {
        return;
    }

    SysEventInfo info = { .dCode = dCode, .bundleName = bundleName, .moduleName = moduleName, .extraInfo = extraInfo };
    iter->second(info);
}

void SysEventLog::SendSysEventLog(const std::string &eventName, const uint32_t dCode, const std::string extraInfo)
{
    auto iter = sysEventMap_.find(eventName);
    if (iter == sysEventMap_.end()) {
        return;
    }

    SysEventInfo info = { .dCode = dCode, .bundleName = "", .moduleName = "", .extraInfo = extraInfo };
    iter->second(info);
}

void SysEventLog::SendSysEventLog(
    const std::string &eventName, const uint32_t dCode, const int32_t one, const int32_t two)
{
    auto iter = sysEventMap_.find(eventName);
    if (iter == sysEventMap_.end()) {
        return;
    }

    SysEventInfo info = { .dCode = dCode,
        .bundleName = "",
        .moduleName = "",
        .extraInfo = "expect" + std::to_string(one) + "=" + std::to_string(two) };
    iter->second(info);
}

std::unordered_map<std::string, void (*)(const SysEventInfo &info)> SysEventLog::sysEventMap_ = {
    { STATISTIC_EVENT, [](const SysEventInfo &info) { SendStatisticEvent(info); } },
    { FAULT_EVENT, [](const SysEventInfo &info) { SendFaultEvent(info); } },
};

template<typename... Types>
int32_t SysEventLog::HisysWrite(const std::string &eventName, HiviewDFX::HiSysEvent::EventType type, Types... keyValues)
{
    int32_t res = HiSysEventWrite(HiviewDFX::HiSysEvent::Domain::REQUEST, eventName,
        static_cast<HiviewDFX::HiSysEvent::EventType>(type), keyValues...);
    return res;
}

void SysEventLog::SendStatisticEvent(const SysEventInfo &info)
{
    HisysWrite(STATISTIC_EVENT, HiviewDFX::HiSysEvent::EventType::STATISTIC, PARAM_DFX_CODE, info.dCode,
        PARAM_BUNDLE_NAME, info.bundleName, PARAM_MODULE_NAME, info.moduleName, PARAM_EXTRA_INFO, info.extraInfo);
}

void SysEventLog::SendFaultEvent(const SysEventInfo &info)
{
    HisysWrite(FAULT_EVENT, HiviewDFX::HiSysEvent::EventType::FAULT, PARAM_DFX_CODE, info.dCode, PARAM_BUNDLE_NAME,
        info.bundleName, PARAM_MODULE_NAME, info.moduleName, PARAM_EXTRA_INFO, info.extraInfo);
}

} // namespace Request
} // namespace OHOS
