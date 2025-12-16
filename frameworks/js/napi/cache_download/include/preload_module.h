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

#ifndef REQUEST_PRE_DOWNLOAD_MODULE_H
#define REQUEST_PRE_DOWNLOAD_MODULE_H

#include <list>
#include <map>
#include "napi/native_api.h"
#include "napi/native_common.h"
#include "napi/native_node_api.h"

namespace OHOS::Request {
// Error code enumeration for cache download operations
enum class ErrorCode: uint32_t {
    // Other unspecified errors
    OTHERS = 0xFF,
    
    // DNS-related errors
    DNS = 0x00,
    
    // TCP connection errors
    TCP = 0x10,
    
    // SSL security errors
    SSL = 0x20,
    
    // HTTP protocol errors
    HTTP = 0x30,
};

// Struct representing a download error with error code and message
struct DownloadError {
    // Error code identifying the specific type of cache download failure
    const ErrorCode errorCode;
    
    // Descriptive message explaining the failure reason
    const std::string message;
    
    // Constructor initializing error code and message
    DownloadError(ErrorCode code, const std::string& msg) : errorCode(code), message(msg) {}
};

// Callback type enumeration
enum class CallbackType {
    SUCCESS,
    ERROR
};

class CallbackInfo {
    friend class CallbackManager;

public:
    CallbackInfo(napi_env env, const std::string &url)
        : env_(env), url_(url)
    {
    }
    
    napi_status RemoveCallback(napi_value cb);
    napi_status RegisterCallback(napi_value cb);
    void InvokeSuccessCallbacks(napi_value values[]);
    void InvokeErrorCallbacks(napi_value values[]);

protected:
    const napi_env env_;
    const std::string url_;
    std::list<std::pair<bool, napi_ref>> allCb_;
    std::recursive_mutex allCbMutex_;
    std::atomic<uint32_t> validCbNum{ 0 };
    bool isInvoked_{ false };
    size_t toDeleteCount_{ 0 };
    
    void CleanupCallbacks();
};

class Data;

class CallbackManager {
public:
    static CallbackManager &GetInstance();
    
    napi_status RegisterCallback(const std::string& url, CallbackType type, napi_env env, napi_value callback);
    
    napi_status RemoveCallback(const std::string& url, CallbackType type, napi_env env, napi_value callback);
    
    void InvokeSuccessCallbacks(const std::string& url,
        std::shared_ptr<Data> data, napi_env env, const std::string& taskId);
    
    void InvokeErrorCallbacks(const std::string& url,
        std::shared_ptr<DownloadError> error, napi_env env, const std::string& taskId);
  
private:
    CallbackManager() = default;
    
    CallbackManager(const CallbackManager&) = delete;
    CallbackManager& operator=(const CallbackManager&) = delete;
    
    std::recursive_mutex mgrMutex_;
    
    std::map<std::string, std::shared_ptr<CallbackInfo>> successCallbacks_;
    
    std::map<std::string, std::shared_ptr<CallbackInfo>> errorCallbacks_;
};

}

#endif