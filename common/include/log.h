/*
 * Copyright (C) 2023 Huawei Device Co., Ltd.
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

#ifndef REQUEST_LOG
#define REQUEST_LOG

#undef HILOG_RAWFORMAT
#ifndef CONFIG_REQUEST_TEST
#include "hilog/log.h"

#ifdef REQUEST_HILOGF
#undef REQUEST_HILOGF
#endif

#ifdef REQUEST_HILOGE
#undef REQUEST_HILOGE
#endif

#ifdef REQUEST_HILOGW
#undef REQUEST_HILOGW
#endif

#ifdef REQUEST_HILOGD
#undef REQUEST_HILOGD
#endif

#ifdef REQUEST_HILOGI
#undef REQUEST_HILOGI
#endif

#define REQUEST_LOG_TAG "Requestkit"
#define REQUEST_LOG_DOMAIN 0xD001C50
static constexpr OHOS::HiviewDFX::HiLogLabel REQUEST_LOG_LABEL = { LOG_CORE, REQUEST_LOG_DOMAIN, REQUEST_LOG_TAG };

#define MAKE_FILE_NAME (__builtin_strrchr(__FILE__, '/') ? __builtin_strrchr(__FILE__, '/') + 1 : __FILE__)

#define REQUEST_HILOGF(fmt, ...)                                                           \
    (void)HILOG_IMPL(LOG_CORE, LOG_FATAL, REQUEST_LOG_LABEL.domain, REQUEST_LOG_LABEL.tag, \
        "[%{public}s %{public}s %{public}d] " fmt, MAKE_FILE_NAME, __FUNCTION__, __LINE__, ##__VA_ARGS__)

#define REQUEST_HILOGE(fmt, ...)                                                           \
    (void)HILOG_IMPL(LOG_CORE, LOG_ERROR, REQUEST_LOG_LABEL.domain, REQUEST_LOG_LABEL.tag, \
        "[%{public}s %{public}s %{public}d] " fmt, MAKE_FILE_NAME, __FUNCTION__, __LINE__, ##__VA_ARGS__)

#define REQUEST_HILOGW(fmt, ...)                                                          \
    (void)HILOG_IMPL(LOG_CORE, LOG_WARN, REQUEST_LOG_LABEL.domain, REQUEST_LOG_LABEL.tag, \
        "[%{public}s %{public}s %{public}d] " fmt, MAKE_FILE_NAME, __FUNCTION__, __LINE__, ##__VA_ARGS__)

#define REQUEST_HILOGD(fmt, ...)                                                           \
    (void)HILOG_IMPL(LOG_CORE, LOG_DEBUG, REQUEST_LOG_LABEL.domain, REQUEST_LOG_LABEL.tag, \
        "[%{public}s %{public}s %{public}d] " fmt, MAKE_FILE_NAME, __FUNCTION__, __LINE__, ##__VA_ARGS__)

#define REQUEST_HILOGI(fmt, ...)                                                          \
    (void)HILOG_IMPL(LOG_CORE, LOG_INFO, REQUEST_LOG_LABEL.domain, REQUEST_LOG_LABEL.tag, \
        "[%{public}s %{public}s %{public}d] " fmt, MAKE_FILE_NAME, __FUNCTION__, __LINE__, ##__VA_ARGS__)

#else

#define REQUEST_HILOGF(fmt, ...)
#define REQUEST_HILOGE(fmt, ...)
#define REQUEST_HILOGW(fmt, ...)
#define REQUEST_HILOGD(fmt, ...)
#define REQUEST_HILOGI(fmt, ...)
#endif // CONFIG_REQUEST_LOG

#endif /* REQUEST_LOG */