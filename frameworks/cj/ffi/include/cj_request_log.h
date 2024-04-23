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
#ifndef OH_CJ_REQUEST_LOG_H
#define OH_CJ_REQUEST_LOG_H

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

#ifdef LOG_DOMAIN
#undef LOG_DOMAIN
#endif
#ifdef LOG_TAG
#undef LOG_TAG
#endif

#define LOG_TAG "CJ-Request"
#define LOG_DOMAIN 0xD001C50

#define MAKE_FILE_NAME (__builtin_strrchr(__FILE__, '/') ? __builtin_strrchr(__FILE__, '/') + 1 : __FILE__)

#define REQUEST_HILOGF(fmt, ...)                                                                        \
    HILOG_FATAL(LOG_CORE, "[%{public}s %{public}s %{public}d] " fmt,                                    \
    MAKE_FILE_NAME, __FUNCTION__, __LINE__, ##__VA_ARGS__)

#define REQUEST_HILOGE(fmt, ...)                                                                        \
    HILOG_ERROR(LOG_CORE, "[%{public}s %{public}s %{public}d] " fmt,                                    \
    MAKE_FILE_NAME, __FUNCTION__, __LINE__, ##__VA_ARGS__)

#define REQUEST_HILOGW(fmt, ...)                                                                        \
    HILOG_WARN(LOG_CORE, "[%{public}s %{public}s %{public}d] " fmt,                                     \
    MAKE_FILE_NAME, __FUNCTION__, __LINE__, ##__VA_ARGS__)

#define REQUEST_HILOGI(fmt, ...)                                                                        \
    HILOG_INFO(LOG_CORE, "[%{public}s %{public}s %{public}d] " fmt,                                     \
    MAKE_FILE_NAME, __FUNCTION__, __LINE__, ##__VA_ARGS__)

#define REQUEST_HILOGD(fmt, ...)                                                                        \
    HILOG_DEBUG(LOG_CORE, "[%{public}s %{public}s %{public}d] " fmt,                                    \
    MAKE_FILE_NAME, __FUNCTION__, __LINE__, ##__VA_ARGS__)

#endif /* OH_CJ_REQUEST_LOG_H */