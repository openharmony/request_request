/*
 * Copyright (C) 2022 Huawei Device Co., Ltd.
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

#ifndef DOWNLOAD_COMMON_H
#define DOWNLOAD_COMMON_H

namespace OHOS::Request::Download {
enum DownloadError {
    E_DOWNLOAD_OK,
    E_DOWNLOAD_SA_DIED,
    E_DOWNLOAD_READ_PARCEL_ERROR,
    E_DOWNLOAD_WRITE_PARCEL_ERROR,
    E_DOWNLOAD_PUBLISH_FAIL,
    E_DOWNLOAD_TRANSACT_ERROR,
    E_DOWNLOAD_DEAL_FAILED,
    E_DOWNLOAD_PARAMETERS_INVALID,
    E_DOWNLOAD_SET_RTC_FAILED,
    E_DOWNLOAD_NOT_FOUND,
    E_DOWNLOAD_NO_PERMISSION,
};
} // namespace OHOS::Request::Download

#endif // DOWNLOAD_COMMON_H