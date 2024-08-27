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

#ifndef MODULE_INIT_H
#define MODULE_INIT_H
#define USE_OPENSSL

#include <curl/curl.h>
#include <pthread.h>

#include <cstddef>

#include "log.h"
#ifdef USE_OPENSSL
#include <openssl/crypto.h>
#endif

namespace OHOS::Request {
class ModuleInit {
public:
    ModuleInit() noexcept;
    virtual ~ModuleInit();
#ifdef USE_OPENSSL
private:
    static unsigned long ThreadIdCallback(void);
    static void LockCallback(int mode, int type, char *file, int line);
    static void InitLocks(void);
    static void KillLocks(void);
#endif
};

} // namespace OHOS::Request

#endif // MODULE_INIT_H