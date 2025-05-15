/*
 * Copyright (c) 2025 Huawei Device Co., Ltd.
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

#ifndef REQUEST_LISTENER_LIST_H
#define REQUEST_LISTENER_LIST_H
#include <ani.h>
#include <list>
#include <mutex>
#include <string>
#include "request_common.h"

namespace OHOS::Request {
class ListenerList {
public:
    ListenerList()
    {
    }

protected:
    void OnMessageReceive(ani_env* env, std::vector<ani_ref> &args);
    ani_status AddListenerInner(ani_ref cb);

protected:
    std::list<std::pair<bool, ani_ref>> allCb_;
    std::recursive_mutex allCbMutex_;
    std::atomic<uint32_t> validCbNum{ 0 };
};

} // namespace OHOS::Request

#endif // OHOS_REQUEST_LISTENER_LIST_H
