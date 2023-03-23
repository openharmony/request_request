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


#define USE_OPENSSL

#include <curl/curl.h>
#include <pthread.h>
#include <cstddef>
#include "upload_hilog_wrapper.h"
static pthread_mutex_t *g_lockArray = nullptr;

#ifdef USE_OPENSSL
#include <openssl/crypto.h>

using namespace OHOS::Request::Upload;
static void LockCallback(int mode, int type, char *file, int line)
{
    (void)file;
    (void)line;
    if (mode & CRYPTO_LOCK) {
        pthread_mutex_lock(&(g_lockArray[type]));
    } else {
        pthread_mutex_unlock(&(g_lockArray[type]));
    }
}

static unsigned long ThreadIdCallback(void)
{
    unsigned long ret = static_cast<unsigned long>(pthread_self());
    return ret;
}

using THREAD_ID_CALLBACK = unsigned long (*)(void);
using LOCK_CALLBACK = void (*)(int mode, int type, char *file, int line);
static void InitLocks(void)
{
    THREAD_ID_CALLBACK threadIdCallback;
    LOCK_CALLBACK lockCallback;
    threadIdCallback = ThreadIdCallback;
    lockCallback = LockCallback;
    g_lockArray = reinterpret_cast<pthread_mutex_t *>(OPENSSL_malloc(CRYPTO_num_locks() * sizeof(pthread_mutex_t)));
    if (g_lockArray == nullptr) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "failed to create openssl lock");
        return;
    }
    for (int i = 0; i < CRYPTO_num_locks(); i++) {
        pthread_mutex_init(&(g_lockArray[i]), nullptr);
    }
    CRYPTO_set_id_callback(threadIdCallback);
    CRYPTO_set_locking_callback(lockCallback);
}

static void KillLocks(void)
{
    int i;
    CRYPTO_set_locking_callback(NULL);
    for (i = 0; i < CRYPTO_num_locks(); i++) {
        pthread_mutex_destroy(&(g_lockArray[i]));
    }
    OPENSSL_free(g_lockArray);
}
#endif

class ModuleInit {
public:
    ModuleInit() noexcept;
    virtual ~ModuleInit();
};

ModuleInit::ModuleInit() noexcept
{
    curl_global_init(CURL_GLOBAL_ALL);
#if defined(USE_OPENSSL)
    InitLocks();
#endif
}

ModuleInit::~ModuleInit()
{
#if defined(USE_OPENSSL)
    KillLocks();
#endif
    curl_global_cleanup();
}

static ModuleInit mi;