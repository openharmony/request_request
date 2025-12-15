/*
 * Copyright (c) 2024 Huawei Device Co., Ltd.
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


import { BusinessError } from '@ohos.base';

export default namespace cacheDownload {
    loadLibrary("cache_download_ani")

    export interface CacheDownloadOptions {
        headers?: Record<string, string>;
        sslType?: SslType;
        caPath?: string;
        cacheStrategy?: CacheStrategy;
    }

    export enum SslType {
        TLS = 'TLS',
        TLCP = 'TLCP'
    }

    export enum CacheStrategy {
        FORCE = 0,
        LAZY = 1
    }

    export native function download(url: string, options: CacheDownloadOptions): void;

    export native function cancel(url: string): void;

    export native function setMemoryCacheSize(bytes: long): void;

    export native function setFileCacheSize(bytes: long): void;
}
