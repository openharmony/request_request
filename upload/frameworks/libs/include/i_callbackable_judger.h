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

#ifndef I_CALLBACKABLE_JUDGER
#define I_CALLBACKABLE_JUDGER

namespace OHOS::Request::Upload {
class IFailCallback;
class IProgressCallback;
class IHeaderReceiveCallback;
class ICallbackAbleJudger {
public:
    ICallbackAbleJudger() = default;
    virtual ~ICallbackAbleJudger()
    {}
    virtual bool JudgeFail(IFailCallback *target) = 0;
    virtual bool JudgeProgress(IProgressCallback *target) = 0;
    virtual bool JudgeHeaderReceive(IHeaderReceiveCallback *target) = 0;
};
} // end of OHOS::Request::Upload
#endif