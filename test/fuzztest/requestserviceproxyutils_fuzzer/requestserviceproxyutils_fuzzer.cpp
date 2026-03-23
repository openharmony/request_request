/*
 * Copyright (c) 2026 Huawei Device Co., Ltd.
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

#include "requestserviceproxyutils_fuzzer.h"
#include "../requestserviceproxy_fuzzer/requestserviceproxy_fuzzer_common.h"

using namespace OHOS::Request;

namespace OHOS {

bool Utf8UtilsFuzzTestGetNextByte(FuzzedDataProvider &provider)
{
    std::vector<uint8_t> num = convertToVectorUint8_t(provider);
    size_t size = num.size();
    if (size < SIZE_ONE) {
        return true;
    }
    Utf8Utils::RunUtf8Validation(num);
    if (size < SIZE_TWO) {
        return true;
    }
    num[0] = 0x81;
    Utf8Utils::RunUtf8Validation(num);
    if (size < SIZE_THREE) {
        return true;
    }
    num[0] = 0xC2;
    num[1] = 0xA9;
    Utf8Utils::RunUtf8Validation(num);
    if (size < SIZE_FOUR) {
        return true;
    }
    num[0] = 0xE2;
    num[1] = 0x82;
    num[SIZE_TWO] = 0xAC;
    Utf8Utils::RunUtf8Validation(num);
    if (size < SIZE_FIVE) {
        return true;
    }
    num[0] = 0xF0;
    num[1] = 0x9F;
    num[SIZE_TWO] = 0x98;
    num[SIZE_THREE] = 0x80;
    Utf8Utils::RunUtf8Validation(num);
    num[0] = 0x80;
    Utf8Utils::RunUtf8Validation(num);
    num[0] = 0xC0;
    num[1] = 0x80;
    Utf8Utils::RunUtf8Validation(num);
    return true;
}

void MarshalConfigBase(OHOS::MessageParcel &data)
{
    Config config;
    data.WriteUint32(static_cast<uint32_t>(config.action));
    data.WriteUint32(static_cast<uint32_t>(config.mode));
    data.WriteUint32(config.bundleType);
    data.WriteBool(config.overwrite);
    data.WriteUint32(static_cast<uint32_t>(config.network));
    config.metered = data.WriteBool(config.metered);
    data.WriteBool(config.roaming);
    data.WriteBool(config.retry);
    data.WriteBool(config.redirect);
    data.WriteUint32(config.index);
    data.WriteInt64(config.begins);
    data.WriteInt64(config.ends);
    data.WriteBool(config.gauge);
    data.WriteBool(config.precise);
    data.WriteUint32(config.priority);
    data.WriteBool(config.background);
    data.WriteBool(config.multipart);
    data.WriteString("bundleName");
    data.WriteString("url");
    data.WriteString("title");
    data.WriteString("description");
    data.WriteString("method");
}

bool ParcelHelperFuzzTestUnMarshalConfig(FuzzedDataProvider &provider)
{
    std::vector<std::string> string = convertToVectorString(provider);
    std::vector<uint8_t> num = convertToVectorUint8_t(provider);
    Config config;
    OHOS::MessageParcel data;
    MarshalConfigBase(data);
    data.WriteUint32(num[0]);
    data.WriteString(string[0]);
    ParcelHelper::UnMarshalConfig(data, config);
    ParcelHelper::UnMarshalConfigHeaders(data, config);
    ParcelHelper::UnMarshalConfigHeaders(data, config);
    ParcelHelper::UnMarshalConfigExtras(data, config);
    ParcelHelper::UnMarshalConfigFormItem(data, config);
    ParcelHelper::UnMarshalConfigFileSpec(data, config);
    ParcelHelper::UnMarshalConfigBodyFileName(data, config);
    return true;
}

} // namespace OHOS

extern "C" int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size)
{
    FuzzedDataProvider provider(data, size);
    OHOS::Utf8UtilsFuzzTestGetNextByte(provider);
    OHOS::ParcelHelperFuzzTestUnMarshalConfig(provider);
    return 0;
}
