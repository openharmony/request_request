/*
 * Copyright (c) 2022 Huawei Device Co., Ltd.
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

#ifndef MOCK_OBFILE_TEST_H
#define MOCK_OBFILE_TEST_H

#include "gtest/gtest.h"
#include "upload_task.h"
#include "gmock/gmock.h"

namespace OHOS::Request::Upload {
class MockObfile : public ObtainFile {
public:
    MockObfile() = default;
    virtual ~MockObfile() = default;

    MOCK_METHOD4(GetFile, uint32_t(FILE **, std::string&, uint32_t&,
        std::shared_ptr<OHOS::AbilityRuntime::Context> &));
    MOCK_METHOD4(GetDataAbilityFile, uint32_t(FILE **, std::string&, uint32_t&,
        std::shared_ptr<OHOS::AbilityRuntime::Context> &));
    MOCK_METHOD4(GetInternalFile, uint32_t(FILE **, std::string&, uint32_t&,
        std::shared_ptr<OHOS::AbilityRuntime::Context> &));
};
}  // namespace OHOS::Request::Upload
#endif  // MOCK_OBFILE_TEST_H