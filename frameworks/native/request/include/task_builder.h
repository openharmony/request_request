/*
 * Copyright (C) 2025 Huawei Device Co., Ltd.
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

#ifndef RUNCOUNT_TASK_BUILDER_H
#define RUNCOUNT_TASK_BUILDER_H

#include "request_common.h"

namespace OHOS::Request {

class TaskBuilder {
public:
    TaskBuilder &setAction(Action action);
    TaskBuilder &setUrl(const std::string &url);
    TaskBuilder &setTitle(const std::string &title);
    TaskBuilder &setDescription(const std::string &description);
    TaskBuilder &setMode(Mode mode);
    TaskBuilder &setOverwrite(bool overwrite);
    TaskBuilder &setMethod(const std::string &method);
    TaskBuilder &setHeaders(const std::map<std::string, std::string> &headers);
    TaskBuilder &setData(const std::string &data);
    TaskBuilder &setData(const std::vector<FormItem> &data);
    TaskBuilder &setData(const std::vector<FileSpec> &data);
    TaskBuilder &setSaveAs(const std::string &saveas);
    TaskBuilder &setNetwork(Network network);
    TaskBuilder &setMetered(bool metered);
    TaskBuilder &setRoaming(bool roaming);
    TaskBuilder &setRetry(bool retry);
    TaskBuilder &setRedirect(bool redirect);
    TaskBuilder &setProxy(const std::string &proxy);
    TaskBuilder &setIndex(uint32_t index);
    TaskBuilder &setBegins(int begins);
    TaskBuilder &setEnds(int ends);
    TaskBuilder &setGauge(bool gauge);
    TaskBuilder &setPrecise(bool precise);
    TaskBuilder &setToken(const std::string &token);
    TaskBuilder &setPriority(uint32_t priority);
    TaskBuilder &setExtras(const std::map<std::string, std::string> &extras);

    // Sets the timeout configuration for the task.
    // connectionTimeout: Maximum time in seconds to wait for connection establishment.
    //                    Default is 60 seconds if set to 0.
    // totalTimeout: Maximum total time in seconds for the entire task.
    //               Default is 7 days (with notification) or 10 minutes (without notification) if set to 0.
    TaskBuilder &setTimeout(const Timeout &timeout);

    // Sets the minimum speed requirement for the task.
    // If the transfer speed falls below the threshold for the specified duration,
    // the task will fail with a LOW_SPEED error.
    // speed: Minimum speed threshold in bytes per second. 0 means no speed limit.
    // duration: Duration in seconds to monitor the speed. 0 means no duration check.
    TaskBuilder &setMinSpeed(const MinSpeed &minSpeed);

public:
    std::pair<Config, ExceptionErrorCode> build();

private:
    Config config{
        .roaming = true,
    };
    bool checkAction();
    bool checkUrl();
    bool checkData();
    bool checkIndex();
    bool checkProxy();
    bool checkToken();
    bool checkDescription();
    bool checkSaveas();
    bool checkBundle();
    bool checkTitle();
    void checkCertsPath();
    void checkCertificatePins();
    void checkMethod();
    void checkOtherConfig();
};
} // namespace OHOS::Request
#endif // RUNCOUNT_TASK_BUILDER_H