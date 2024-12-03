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

#ifndef OHOS_REQUEST_REQUEST_H
#define OHOS_REQUEST_REQUEST_H

#include <map>
#include <memory>
#include <mutex>

#include "i_notify_data_listener.h"
#include "i_response_listener.h"

namespace OHOS::Request {

class Request {
public:
    Request(const std::string &taskId);
    const std::string &getId() const;
    void AddListener(const SubscribeType &type, const std::shared_ptr<IResponseListener> &listener);
    void RemoveListener(const SubscribeType &type, const std::shared_ptr<IResponseListener> &listener);
    void AddListener(const SubscribeType &type, const std::shared_ptr<INotifyDataListener> &listener);
    void RemoveListener(const SubscribeType &type, const std::shared_ptr<INotifyDataListener> &listener);
    bool HasListener();
    void OnResponseReceive(const std::shared_ptr<Response> &response);
    void OnNotifyDataReceive(const std::shared_ptr<NotifyData> &notifyData);
    bool NeedNotify(const std::shared_ptr<NotifyData> &notifyData);

private:
    const std::string taskId_;
    std::mutex listenerMutex_;
    std::shared_ptr<IResponseListener> responseListener_;
    std::map<SubscribeType, std::shared_ptr<INotifyDataListener>> notifyDataListenerMap_;
    std::map<SubscribeType, std::shared_ptr<NotifyData>> unusedNotifyData_;
    bool needRemove_ = true;
};

} // namespace OHOS::Request

#endif // OHOS_REQUEST_REQUEST_H