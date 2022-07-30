/*
 * Copyright (C) 2021-2022 Huawei Device Co., Ltd.
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

#ifndef DOWNLOAD_CONFIG_H
#define DOWNLOAD_CONFIG_H

#include <map>
#include <stdint.h>
#include <functional>
#include <iosfwd>
#include <string>

namespace OHOS::Request::Download {
class DownloadConfig final {
public:
    DownloadConfig();

    void SetUrl(const std::string &url);

    void SetHeader(const std::string &key, const std::string &val);

    void SetMetered(bool enableMetered);

    void SetRoaming(bool enableRoaming);

    void SetDescription(const std::string &description);

    void SetNetworkType(uint32_t type);

    void SetFilePath(const std::string &filePath);

    void SetTitle(const std::string &title);

    void SetFD(int32_t fd);

    void SetFDError(int32_t fdError);

    [[nodiscard]] const std::string &GetUrl() const;

    [[nodiscard]] const std::map<std::string, std::string> &GetHeader() const;

    [[nodiscard]] bool GetMetered() const;

    [[nodiscard]] bool GetRoaming() const;

    [[nodiscard]] const std::string &GetDescription() const;

    [[nodiscard]] uint32_t GetNetworkType() const;

    [[nodiscard]] const std::string &GetFilePath() const;

    [[nodiscard]] const std::string &GetTitle() const;

    int32_t GetFD() const;

    int32_t GetFDError() const;

    void Dump(bool isFull = true) const;

private:
    std::string url_;

    std::map<std::string, std::string> header_;

    bool enableMetered_;

    bool enableRoaming_;

    std::string description_;

    uint32_t networkType_;

    std::string filePath_;

    std::string title_;

    int32_t fd_;

    int32_t fdError_;
};
} // namespace OHOS::Request::Download

#endif /* DOWNLOAD_CONFIG_H */
