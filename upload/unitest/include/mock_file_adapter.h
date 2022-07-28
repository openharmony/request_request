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

#ifndef OHOS_REQUEST_UPLOAD_MOCK_FILEADAPTER_TEST_H
#define OHOS_REQUEST_UPLOAD_MOCK_FILEADAPTER_TEST_H

#include "gtest/gtest.h"
#include "gmock/gmock.h"
#include "i_file_adapter.h"

namespace OHOS::Request::Upload {
class MockFileAdapter : public IFileAdapter {
public:
    MockFileAdapter() = default;
    virtual ~MockFileAdapter() = default;

    MOCK_METHOD2(DataAbilityOpenFile, int32_t(std::string &,
                                               std::shared_ptr<OHOS::AbilityRuntime::Context> &));
    MOCK_METHOD1(InternalGetFilePath,
                 std::string(std::shared_ptr<OHOS::AbilityRuntime::Context> &));
};
}  // namespace OHOS::Request::Upload
#endif  // MOCK_FILEADAPTER_TEST_H