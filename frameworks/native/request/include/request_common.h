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

#ifndef REQUEST_COMMON_H
#define REQUEST_COMMON_H

#include <cstdint>
#include <map>
#include <optional>
#include <string>
#include <vector>

#include "constant.h"

namespace OHOS::Request {

enum class Action : uint32_t {
    DOWNLOAD = 0,
    UPLOAD,
    ANY,
};

enum class Mode : uint32_t {
    BACKGROUND = 0,
    FOREGROUND,
    ANY,
};

enum class Visibility : uint32_t {
    NONE = 0b00,
    COMPLETION = 0b01,
    PROGRESS = 0b10,
    ANY = 0b11,
};

enum class Network : uint32_t {
    ANY = 0,
    WIFI,
    CELLULAR,
};

enum class Version : uint32_t {
    API8 = 0,
    API9,
    API10,
};

enum Reason : uint32_t {
    REASON_OK = 0,
    TASK_SURVIVAL_ONE_MONTH,
    WAITTING_NETWORK_ONE_DAY,
    STOPPED_NEW_FRONT_TASK,
    RUNNING_TASK_MEET_LIMITS,
    USER_OPERATION,
    APP_BACKGROUND_OR_TERMINATE,
    NETWORK_OFFLINE,
    UNSUPPORTED_NETWORK_TYPE,
    BUILD_CLIENT_FAILED,
    BUILD_REQUEST_FAILED,
    GET_FILESIZE_FAILED,
    CONTINUOUS_TASK_TIMEOUT,
    CONNECT_ERROR,
    REQUEST_ERROR,
    UPLOAD_FILE_ERROR,
    REDIRECT_ERROR,
    PROTOCOL_ERROR,
    IO_ERROR,
    UNSUPPORT_RANGE_REQUEST,
    OTHERS_ERROR,
    ACCOUNT_STOPPED,
    NETWORK_CHANGED,
    DNS,
    TCP,
    SSL,
    INSUFFICIENT_SPACE,
    NETWORK_APP,
    NETWORK_ACCOUNT,
    APP_ACCOUNT,
    NETWORK_APP_ACCOUNT,
    LOW_SPEED,
};

enum WaitingReason : uint32_t {
    TaskQueueFull = 0x00,
    NetworkNotMatch = 0x01,
    AppBackground = 0x02,
    UserInactivated = 0x03,
};

enum class SubscribeType : uint32_t {
    COMPLETED = 0,
    FAILED,
    HEADER_RECEIVE,
    PAUSE,
    PROGRESS,
    REMOVE,
    RESUME,
    RESPONSE,
    FAULT_OCCUR,
    WAIT,
    BUTT,
};

struct UploadResponse {
    int32_t code;
    std::string data;
    std::string headers;
};

struct FormItem {
    std::string name;
    std::string value;
};

struct FileSpec {
    std::string name;
    std::string uri;
    std::string filename;
    std::string type;
    int32_t fd = -1;
    bool isUserFile = false;
    bool hasContentType = false;
};

struct Notification {
    std::optional<std::string> title = std::nullopt;
    std::optional<std::string> text = std::nullopt;
    std::optional<std::string> wantAgent = std::nullopt;
    bool disable = false;
    uint32_t visibility = 0b01;
};

struct MinSpeed {
    int64_t speed = 0;
    int64_t duration = 0;
};

struct Timeout {
    uint64_t connectionTimeout = 0;
    uint64_t totalTimeout = 0;
};

struct Config {
    Action action;
    std::string url;
    std::vector<std::string> certsPath;
    Version version;
    std::string bundleName;
    uint32_t bundleType = 0;
    Mode mode = Mode::BACKGROUND;
    Network network = Network::ANY;
    uint32_t index = 0;
    int64_t begins = 0;
    int64_t ends = -1;
    uint32_t priority = 0;
    bool overwrite = false;
    bool metered = false;
    bool roaming = false;
    bool retry = true;
    bool redirect = true;
    bool gauge = false;
    bool precise = false;
    bool background = false;
    bool withErrCode = true;
    bool firstInit = true;
    bool multipart = false;
    std::string title;
    std::string saveas;
    std::string method;
    std::string token = "null";
    std::string description;
    std::string data;
    std::string proxy;
    std::string certificatePins;
    std::map<std::string, std::string> headers;
    std::vector<FormItem> forms;
    std::vector<FileSpec> files;
    std::vector<std::string> bodyFileNames;
    std::map<std::string, std::string> extras;
    Notification notification;
    MinSpeed minSpeed;
    Timeout timeout;
};

enum class State : uint32_t {
    INITIALIZED = 0x00,
    WAITING = 0x10,
    RUNNING = 0x20,
    RETRYING = 0x21,
    PAUSED = 0x30,
    STOPPED = 0x31,
    COMPLETED = 0x40,
    FAILED = 0x41,
    REMOVED = 0x50,
    DEFAULT = 0x60,
    ANY = 0x61,
};

struct Progress {
    State state;
    uint32_t index;
    uint64_t processed;
    uint64_t totalProcessed;
    std::vector<int64_t> sizes;
    std::map<std::string, std::string> extras;
    std::vector<uint8_t> bodyBytes;
};

enum class Faults : uint32_t {
    OTHERS = 0xFF,
    DISCONNECTED = 0x00,
    TIMEOUT = 0x10,
    PROTOCOL = 0x20,
    PARAM = 0x30,
    FSIO = 0x40,
    DNS = 0X50,
    TCP = 0X60,
    SSL = 0X70,
    REDIRECT = 0x80,
    LOW_SPEED = 0x90,
};

struct TaskState {
    std::string path;
    uint32_t responseCode{ REASON_OK };
    std::string message;
};

struct NotifyData {
    SubscribeType type;
    uint32_t taskId;
    Progress progress;
    Action action;
    Version version;
    Mode mode;
    std::vector<TaskState> taskStates;
};

enum class EventType : uint32_t {
    DATA_CALLBACK = 0,
    HEADER_CALLBACK,
    TASK_STATE_CALLBACK,
    PROGRESS_CALLBACK,
    BUTT,
};

struct Notify {
    EventType type = EventType::BUTT;
    std::vector<int64_t> data;
    std::vector<TaskState> taskStates;
    Progress progress;
};

struct TaskInfo {
    Version version;
    std::string uid;
    std::string bundle;
    std::string url;
    std::string data;
    std::vector<FileSpec> files;
    std::vector<FormItem> forms;
    std::string tid;
    std::string title;
    std::string description;
    Action action;
    Mode mode;
    std::string mimeType;
    Progress progress;
    bool gauge;
    uint64_t ctime;
    uint64_t mtime;
    bool retry;
    uint32_t tries;
    Faults faults;
    Reason code;
    std::string reason;
    bool withSystem = false;
    uint32_t priority;
    uint64_t taskTime = 0;
    std::map<std::string, std::string> extras;
    std::vector<TaskState> taskStates;
};

struct TaskInfoRet {
    ExceptionErrorCode code;
    TaskInfo info;
};

struct TaskIdAndToken {
    std::string tid;
    std::string token;
};

struct SpeedConfig {
    std::string tid;
    int64_t maxSpeed;
};

struct Filter {
    std::string bundle;
    int64_t before;
    int64_t after;
    State state = State::ANY;
    Action action = Action::ANY;
    Mode mode = Mode::ANY;
};

enum DownloadErrorCode {
    ERROR_CANNOT_RESUME,
    ERROR_DEVICE_NOT_FOUND,
    ERROR_FILE_ALREADY_EXISTS,
    ERROR_FILE_ERROR,
    ERROR_HTTP_DATA_ERROR,
    ERROR_INSUFFICIENT_SPACE,
    ERROR_TOO_MANY_REDIRECTS,
    ERROR_UNHANDLED_HTTP_CODE,
    ERROR_UNKNOWN,
    ERROR_OFFLINE,
    ERROR_UNSUPPORTED_NETWORK_TYPE,
};

enum DownloadStatus {
    SESSION_SUCCESS,
    SESSION_RUNNING,
    SESSION_PENDING,
    SESSION_PAUSED,
    SESSION_FAILED,
    SESSION_UNKNOWN,
};

enum RemoveTaskChecker {
    DoNothing,
    ClearFile,
    ClearFileAndRemoveTask,
};

struct DownloadInfo {
    uint32_t downloadId;
    DownloadErrorCode failedReason;
    std::string fileName;
    std::string filePath;
    PausedReason pausedReason;
    DownloadStatus status;
    std::string url;
    std::string downloadTitle;
    int64_t downloadTotalBytes;
    std::string description;
    int64_t downloadedBytes;
};

struct Response {
    std::string taskId;
    std::string version;
    int32_t statusCode;
    std::string reason;
    std::map<std::string, std::vector<std::string>> headers;
};

struct TaskRet {
    ExceptionErrorCode code;
    std::string tid;
};

static uint64_t REQUEST_FDSAN_TAG = 0xC01C50;

static uint32_t VISIBILITY_COMPLETION = 0b00000001;
static uint32_t VISIBILITY_PROGRESS = 0b00000010;

} // namespace OHOS::Request
#endif //REQUEST_COMMON_H