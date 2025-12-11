/*
 * Copyright (C) 2024 Huawei Device Co., Ltd.
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

#include "request_utils_wrapper.h"

#include <memory>
#include <sstream>

#include "ani.h"
#include "ani_base_context.h"
#include "openssl/sha.h"
#include "data_ability_helper.h"
#include "network_security_config.h"

namespace OHOS::Request {

rust::string GetCacheDir()
{
    auto context = Context::GetApplicationContext();
    if (context == nullptr) {
        return "";
    } else {
        return context->GetCacheDir();
    }
}

rust::string GetBaseDir()
{
    auto context = Context::GetApplicationContext();
    if (context == nullptr) {
        return "";
    } else {
        return context->GetBaseDir();
    }
}

rust::string SHA256(rust::str input)
{
    unsigned char hash[SHA256_DIGEST_LENGTH];
    SHA256_CTX sha256;
    SHA256_Init(&sha256);
    SHA256_Update(&sha256, input.data(), input.length());
    SHA256_Final(hash, &sha256);
    std::stringstream ss;
    for (int i = 0; i < SHA256_DIGEST_LENGTH; i++) {
        // 2 means setting hte width of the output.
        ss << std::hex << std::setw(2) << std::setfill('0') << static_cast<int>(hash[i]);
    }
    return ss.str();
}

bool IsStageContext(AniEnv *env, AniObject *obj)
{
    ani_boolean stageMode;
    AbilityRuntime::IsStageContext(reinterpret_cast<ani_env *>(env), *reinterpret_cast<ani_object *>(obj), stageMode);
    return stageMode == 1;
}

std::shared_ptr<AbilityRuntime::Context> GetStageModeContext(AniEnv **env, AniObject *obj)
{
    return AbilityRuntime::GetStageModeContext(reinterpret_cast<ani_env *>(*env), *reinterpret_cast<ani_object *>(obj));
}

bool IsCleartextPermitted(std::string const &hostname)
{
    bool cleartextPermitted = true;
    OHOS::NetManagerStandard::NetworkSecurityConfig::GetInstance().IsCleartextPermitted(hostname, cleartextPermitted);
    return cleartextPermitted;
}

rust::vec<rust::string> GetTrustAnchorsForHostName(std::string const &hostname)
{
    std::vector<std::string> trustAnchors;
    OHOS::NetManagerStandard::NetworkSecurityConfig::GetInstance().GetTrustAnchorsForHostName(hostname, trustAnchors);
    rust::vec<rust::string> ret;
    for (auto &anchor : trustAnchors) {
        ret.push_back(anchor);
    }
    return ret;
}

rust::string GetCertificatePinsForHostName(std::string const &hostname)
{
    if (OHOS::NetManagerStandard::NetworkSecurityConfig::GetInstance().IsPinOpenMode(hostname)) {
        return "";
    }
    std::string certificatePins;
    OHOS::NetManagerStandard::NetworkSecurityConfig::GetInstance().GetPinSetForHostName(hostname, certificatePins);
    return certificatePins;
}
} // namespace OHOS::Request