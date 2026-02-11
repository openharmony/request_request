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


import { BusinessError, Callback } from '@ohos.base';

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

    export enum ErrorCode {
        OTHERS = 0xFF,
        DNS = 0x00,
        TCP = 0x10,
        SSL = 0x20,
        HTTP = 0x30,
    }

    export interface ResourceInfo {
        readonly size: long;
    }

    export class ResourceInfoInner implements ResourceInfo {
        readonly size: long;
    }

    export interface NetworkInfo {
        readonly dnsServers: string[];
        readonly ip?: string;
    }

    export class NetworkInfoInner implements NetworkInfo {
        readonly dnsServers: string[];
        readonly ip?: string;
    }

    export interface PerformanceInfo {
        readonly dnsTime: double;
        readonly connectTime: double;
        readonly tlsTime: double;
        readonly firstSendTime: double;
        readonly firstReceiveTime: double;
        readonly totalTime: double;
        readonly redirectTime: double;
    }

    export class PerformanceInfoInner implements PerformanceInfo {
        readonly dnsTime: double;
        readonly connectTime: double;
        readonly tlsTime: double;
        readonly firstSendTime: double;
        readonly firstReceiveTime: double;
        readonly totalTime: double;
        readonly redirectTime: double;
    }

    export interface DownloadInfo {
        readonly resource: ResourceInfo;
        readonly network: NetworkInfo;
        readonly performance: PerformanceInfo;
    }

    export class DownloadInfoInner implements DownloadInfo {
        readonly resource: ResourceInfo;
        readonly network: NetworkInfo;
        readonly performance: PerformanceInfo;
    }

    export interface DownloadError {
        readonly errorCode: ErrorCode;
        readonly message: string;
    }

    export class DownloadErrorInner implements DownloadError {
        readonly errorCode: ErrorCode;
        readonly message: string;
    }

    export native function getDownloadInfo(url: string): DownloadInfo | undefined;

    export native function setDownloadInfoListSize(size: long): void;

    export native function download(url: string, options: CacheDownloadOptions): void;

    export native function cancel(url: string): void;

    export native function setMemoryCacheSize(bytes: long): void;

    export native function setFileCacheSize(bytes: long): void;

    export native function clearMemoryCache(): void;

    export native function clearFileCache(): void;

    export native function onDownloadSuccess(url: string, callback: Callback<void>): void;

    export native function onDownloadError(url: string, callback: Callback<DownloadError>): void;

    export native function offDownloadSuccess(url: string, callback?: Callback<void>): void;

    export native function offDownloadError(url: string, callback?: Callback<DownloadError>): void;
}
