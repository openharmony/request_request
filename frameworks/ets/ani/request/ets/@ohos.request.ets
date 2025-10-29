/*
 * Copyright (c) 2022-2025 Huawei Device Co., Ltd.
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


import { AsyncCallback, Callback } from '@ohos.base';
import { BusinessError } from '@ohos.base';
import BaseContext from 'application.BaseContext';

export default namespace request {
  loadLibrary("request_ani")

  const EXCEPTION_PERMISSION: int;
  const EXCEPTION_PARAMCHECK: int;
  const EXCEPTION_UNSUPPORTED: int;
  const EXCEPTION_FILEIO: int;
  const EXCEPTION_FILEPATH: int;
  const EXCEPTION_SERVICE: int;
  const EXCEPTION_OTHERS: int;

  const NETWORK_MOBILE: int;

  const NETWORK_WIFI: int;

  const ERROR_CANNOT_RESUME: int;

  const ERROR_DEVICE_NOT_FOUND: int;

  const ERROR_FILE_ALREADY_EXISTS: int;

  const ERROR_FILE_ERROR: int;

  const ERROR_HTTP_DATA_ERROR: int;

  const ERROR_INSUFFICIENT_SPACE: int;

  const ERROR_TOO_MANY_REDIRECTS: int;

  const ERROR_UNHANDLED_HTTP_CODE: int;

  const ERROR_UNKNOWN: int;

  const ERROR_OFFLINE: int;

  const ERROR_UNSUPPORTED_NETWORK_TYPE: int;

  const PAUSED_QUEUED_FOR_WIFI: int;

  const PAUSED_WAITING_FOR_NETWORK: int;

  const PAUSED_WAITING_TO_RETRY: int;

  const PAUSED_BY_USER: int;

  const PAUSED_UNKNOWN: int;

  const SESSION_SUCCESSFUL: int;

  const SESSION_RUNNING: int;

  const SESSION_PENDING: int;

  const SESSION_PAUSED: int;

  const SESSION_FAILED: int;

  native function downloadFileSync(context: BaseContext, config: DownloadConfig): DownloadTask;

  export function downloadFile(context: BaseContext, config: DownloadConfig, callback: AsyncCallback<DownloadTask>): void {
    let p1 = taskpool.execute((): DownloadTask => {
        return downloadFileSync(context, config);
    })
    p1.then((content: NullishType) => {
        callback(null, content as DownloadTask);
    }, (err: Error): void => {
        callback(err as BusinessError, new DownloadTaskInner);
    });
  }

  export function downloadFile(context: BaseContext, config: DownloadConfig): Promise<DownloadTask> {
    return new Promise<DownloadTask>((resolve, reject) => {
      taskpool.execute((): DownloadTask => {
          return downloadFileSync(context, config);
      }).then((content: NullishType) => {
          resolve(content as DownloadTask);
      }, (err: Error): void => {
          reject(err as BusinessError);
      });
    });
  }

  native function uploadFileSync(context: BaseContext, config: UploadConfig): UploadTask;

  export function uploadFile(context: BaseContext, config: UploadConfig, callback: AsyncCallback<UploadTask>): void {
    let p1 = taskpool.execute((): UploadTask => {
        return uploadFileSync(context,config);
    })
    p1.then((content: NullishType) => {
        callback(null, content as UploadTask);
    }, (err: Error): void => {
        callback(err as BusinessError, new UploadTaskInner);
    });
  }

  export function uploadFile(context: BaseContext, config: UploadConfig): Promise<UploadTask> {
    return new Promise<UploadTask>((resolve, reject) => {
      taskpool.execute((): UploadTask => {
          return uploadFileSync(context, config);
      }).then((content: NullishType) => {
          resolve(content as UploadTask);
      }, (err: Error): void => {
          reject(err as BusinessError);
      });
    });
  }

  interface DownloadConfig {
    url: string;
    header?: Record<string, string>;
    enableMetered?: boolean;
    enableRoaming?: boolean;
    description?: string;
    networkType?: int;
    filePath?: string;
    title?: string;
    background?: boolean;
  }

  interface DownloadInfo {
    description: string;
    downloadedBytes: long;
    downloadId: long;
    failedReason: int;
    fileName: string;
    filePath: string;
    pausedReason: int;
    status: int;
    targetURI: string;
    downloadTitle: string;
    downloadTotalBytes: long;
  }

  class DownloadInfoInner implements DownloadInfo{
    description: string;
    downloadedBytes: long;
    downloadId: long;
    failedReason: int;
    fileName: string;
    filePath: string;
    pausedReason: int;
    status: int;
    targetURI: string;
    downloadTitle: string;
    downloadTotalBytes: long;
  }

  export type DownloadProgressCallback = (receivedSize: long, totalSize: long) => void;
  export type DownloadCompleteCallback = () => void;
  export type DownloadPauseCallback = () => void;
  export type DownloadRemoveCallback = () => void;
  export type DownloadFailCallback = (err: int) => void;

  interface DownloadTask {
    readonly taskId: string;

    onProgress(callback: DownloadProgressCallback): void;
    offProgress(callback?: DownloadProgressCallback): void;
    onComplete(callback: DownloadCompleteCallback): void;
    onPause(callback: DownloadPauseCallback): void;
    onRemove(callback: DownloadRemoveCallback): void;
    offComplete(callback?: DownloadCompleteCallback): void;
    offPause(callback?: DownloadPauseCallback): void;
    offRemove(callback?: DownloadRemoveCallback): void;
    onFail(callback: DownloadFailCallback): void;
    offFail(callback?: DownloadFailCallback): void;

    // onEvents(type: 'complete' | 'pause' | 'remove', callback: () => void): void;
    // onFails(type: 'fail', callback: (err: int) => void): void;
    // onProgresss(type: 'progress', callback: (receivedSize: long, totalSize: long) => void): void;
    // overload on { onEvents, onFails, onProgresss }

    // offEvents(type: 'complete' | 'pause' | 'remove', callback?: () => void): void;
    // offProgresss(type: 'progress', callback?: (receivedSize: long, totalSize: long) => void): void;
    // offFails(type: 'fail', callback?: (err: int) => void): void;
    // overload off { offEvents, offFails, offProgresss }

    delete(callback: AsyncCallback<boolean>): void;

    delete(): Promise<boolean>;

    suspend(callback: AsyncCallback<boolean>): void;

    suspend(): Promise<boolean>;

    restore(callback: AsyncCallback<boolean>): void;

    restore(): Promise<boolean>;

    getTaskInfo(callback: AsyncCallback<DownloadInfo>): void;

    getTaskInfo(): Promise<DownloadInfo>;

    getTaskMimeType(callback: AsyncCallback<string>): void;

    getTaskMimeType(): Promise<string>;
  }

  export class DownloadTaskInner implements DownloadTask {
    static {
      loadLibrary("request_ani")
    }

    readonly taskId: string;

    native onProgressInner(callback: (receivedSize: long, totalSize: long) => void): void;
    native onEvent(type: string, callback: () => void): void;
    native onFailInner(callback: (err: int) => void): void;

    native offProgressInner(callback?: (receivedSize: long, totalSize: long) => void): void;
    native offEvent(type: string, callback?: () => void): void;
    native offFailInner(callback?: (err: int) => void): void;
    native offEvents(type: string):void;

    onProgress(callback: DownloadProgressCallback): void {
      this.onProgressInner(callback);
    }

    offProgress(callback?: DownloadProgressCallback): void {
      if(callback == undefined) {
        this.offEvents('progress');
      } else {
        this.offProgressInner(callback);
      }
    }

    onComplete(callback: DownloadCompleteCallback): void {
      this.onEvent('complete', callback);
    }

    onPause(callback: DownloadPauseCallback): void {
      this.onEvent('pause', callback);
    }

    onRemove(callback: DownloadRemoveCallback): void {
      this.onEvent('remove', callback);
    }

    offComplete(callback?: DownloadCompleteCallback): void {
      if(callback == undefined) {
        this.offEvents('complete_download');
      } else {
        this.offEvent('complete', callback);
      }
    }

    offPause(callback?: DownloadPauseCallback): void {
      if(callback == undefined) {
        this.offEvents('pause');
      } else {
        this.offEvent('pause', callback);
      }
    }

    offRemove(callback?: DownloadRemoveCallback): void {
      if(callback == undefined) {
        this.offEvents('remove');
      } else {
        this.offEvent('remove', callback);
      }
    }

    onFail(callback: DownloadFailCallback): void {
      this.onFailInner(callback);
    }

    offFail(callback?: DownloadFailCallback): void {
      if(callback == undefined) {
        this.offEvents('fail_download');
      } else {
        this.offFailInner(callback);
      }
    }

    // onEvents(type: 'complete' | 'pause' | 'remove', callback: () => void): void {
    //   this.onEvent(type, callback);
    // }

    // onFails(type: 'fail', callback: (err: int) => void): void {
    //   this.onFail(callback);
    // }

    // onProgresss(type: 'progress', callback: (receivedSize: long, totalSize: long) => void): void {
    //   this.onProgress(callback);
    // }

    // offEvents(type: 'complete' | 'pause' | 'remove', callback?: () => void): void {
    //   this.offEvent(type, callback);
    // }

    // offProgresss(type: 'progress', callback?: (receivedSize: long, totalSize: long) => void): void {
    //   this.offProgress(callback);
    // }

    // offFails(type: 'fail', callback?: (err: int) => void): void {
    //   this.offFail(callback);
    // }

    native deleteSync(): boolean;

    delete(callback: AsyncCallback<boolean>): void {
        let p1 = taskpool.execute((): boolean => {
            return this.deleteSync();
        })
        p1.then((content: NullishType) => {
            callback(null, content as boolean);
        }, (err: Error): void => {
            callback(err as BusinessError, false);
        });
    }

    delete(): Promise<boolean> {
        return new Promise<boolean>((resolve, reject) => {
            taskpool.execute((): boolean => {
                return this.deleteSync();
            }).then((content: NullishType) => {
                resolve(content as boolean);
            }, (err: Error): void => {
                reject(err as BusinessError);
            });
        });
    }

    native suspendSync(): boolean;

    suspend(callback: AsyncCallback<boolean>): void {
        let p1 = taskpool.execute((): boolean => {
            return this.suspendSync();
        })
        p1.then((content: NullishType) => {
            callback(null, content as boolean);
        }, (err: Error): void => {
            callback(err as BusinessError, false);
        });
    }

    suspend(): Promise<boolean> {
        return new Promise<boolean>((resolve, reject) => {
            taskpool.execute((): boolean => {
                return this.suspendSync();
            }).then((content: NullishType) => {
                resolve(content as boolean);
            }, (err: Error): void => {
                reject(err as BusinessError);
            });
        });
    }

    native restoreSync(): boolean;

    restore(callback: AsyncCallback<boolean>): void {
        let p1 = taskpool.execute((): boolean => {
            return this.restoreSync();
        })
        p1.then((content: NullishType) => {
            callback(null, content as boolean);
        }, (err: Error): void => {
            callback(err as BusinessError, false);
        });
    }

    restore(): Promise<boolean> {
        return new Promise<boolean>((resolve, reject) => {
            taskpool.execute((): boolean => {
                return this.restoreSync();
            }).then((content: NullishType) => {
                resolve(content as boolean);
            }, (err: Error): void => {
                reject(err as BusinessError);
            });
        });
    }

    native getTaskInfoSync(): DownloadInfo;

    getTaskInfo(callback: AsyncCallback<DownloadInfo>): void {
        let p1 = taskpool.execute((): DownloadInfo => {
            return this.getTaskInfoSync();
        })
        p1.then((content: NullishType) => {
            callback(null, content as DownloadInfo);
        }, (err: Error): void => {
            callback(err as BusinessError, new DownloadInfoInner);
        });
    }

    getTaskInfo(): Promise<DownloadInfo> {
        return new Promise<DownloadInfo>((resolve, reject) => {
            taskpool.execute((): DownloadInfo => {
                return this.getTaskInfoSync();
            }).then((content: NullishType) => {
                resolve(content as DownloadInfo);
            }, (err: Error): void => {
                reject(err as BusinessError);
            });
        });
    }

    native getTaskMimeTypeSync(): string;

    getTaskMimeType(callback: AsyncCallback<string>): void {
        let p1 = taskpool.execute((): string => {
            return this.getTaskMimeTypeSync();
        })
        p1.then((content: NullishType) => {
            callback(null, content as string);
        }, (err: Error): void => {
            callback(err as BusinessError, "");
        });
    }

    getTaskMimeType(): Promise<string> {
        return new Promise<string>((resolve, reject) => {
            taskpool.execute((): string => {
                return this.getTaskMimeTypeSync();
            }).then((content: NullishType) => {
                resolve(content as string);
            }, (err: Error): void => {
                reject(err as BusinessError);
            });
        });
    }
  }

  interface File {
    filename: string;
    name: string;
    uri: string;
    type: string;
  }

  interface RequestData {
    name: string;
    value: string;
  }

  interface UploadConfig {
    url: string;
    header: Record<string, string>;
    method: string;
    index?: int;
    begins?: long;
    ends?: long;
    files: Array<File>;
    data: Array<RequestData>;
  }

  interface TaskState {
    path: string;
    responseCode: int;
    message: string;
  }

  class TaskStateInner implements TaskState {
    path: string;
    responseCode: int;
    message: string;
  }

  export type UploadProgressCallback = (uploadedSize: long, totalSize: long) => void;
  export type UploadHeaderReceiveCallback = (header: Record<string, string>) => void;

  interface UploadTask {
    readonly taskId: string;

    onProgress(callback: UploadProgressCallback): void;
    offProgress(callback?: UploadProgressCallback): void;
    onHeaderReceive(callback: UploadHeaderReceiveCallback): void;
    offHeaderReceive(callback?: UploadHeaderReceiveCallback): void;
    onComplete(callback: Callback<Array<TaskState>>): void;
    onFail(callback: Callback<Array<TaskState>>): void;
    offComplete(callback?: Callback<Array<TaskState>>): void;
    offFail(callback?: Callback<Array<TaskState>>): void;

    // onEvent(type: 'complete' | 'fail', callback: (states: Array<TaskState>) => void): void;
    // onProgress(type: 'progress', callback: (uploadedSize: long, totalSize: long) => void): void;
    // onHeaderReceive(type: 'header_receive', callback: (header: Record<string, string>) => void): void;
    // overload on { onEvent, onProgress, onHeaderReceive }

    // offEvent(type: 'complete' | 'fail', callback?: (states: Array<TaskState>) => void): void;
    // offProgress(type: 'progress', callback?: (uploadedSize: long, totalSize: long) => void): void;
    // offHeaderReceive(type: 'header_receive', callback?: (header: Record<string, string>) => void): void;
    // overload off { offEvent, offProgress, offHeaderReceive }

    delete(callback: AsyncCallback<boolean>): void;


    delete(): Promise<boolean>;
  }

  class UploadTaskInner implements UploadTask {
    readonly taskId: string;
    native deleteSync(): boolean;

    delete(callback: AsyncCallback<boolean>): void {
      let p1 = taskpool.execute((): boolean => {
          return this.deleteSync();
      })
      p1.then((content: NullishType) => {
          callback(null, content as boolean);
      }, (err: Error): void => {
          callback(err as BusinessError, false);
      });
    }

    delete(): Promise<boolean> {
      return new Promise<boolean>((resolve, reject) => {
          taskpool.execute((): boolean => {
              return this.deleteSync();
          }).then((content: NullishType) => {
              resolve(content as boolean);
          }, (err: Error): void => {
              reject(err as BusinessError);
          });
      });
    }

    onProgress(callback: UploadProgressCallback): void {
      this.onProgressInner(callback);
    }

    offProgress(callback?: UploadProgressCallback): void {
      if(callback == undefined) {
        this.offEvents('progress');
      } else {
        this.offProgressInner(callback);
      }
    }

    onHeaderReceive(callback: UploadHeaderReceiveCallback): void {
      this.onHeaderReceiveInner(callback);
    }

    offHeaderReceive(callback?: UploadHeaderReceiveCallback): void {
      if(callback == undefined) {
        this.offEvents('header_receive');
      } else {
        this.offHeaderReceiveInner(callback);
      }
    }

    onComplete(callback: Callback<Array<TaskState>>): void {
      this.onEventInner('complete', callback);
    }

    onFail(callback: Callback<Array<TaskState>>): void {
      this.onEventInner('fail', callback);
    }

    offComplete(callback?: Callback<Array<TaskState>>): void {
      if(callback == undefined) {
        this.offEvents('complete_upload');
      } else {
        this.offEventInner('complete', callback);
      }
    }

    offFail(callback?: Callback<Array<TaskState>>): void {
      if(callback == undefined) {
        this.offEvents('fail_upload');
      } else {
        this.offEventInner('fail', callback);
      }
    }

    // onEvent(type: 'complete' | 'fail', callback: (states: Array<TaskState>) => void): void {
    //   this.onEventInner(type, callback);
    // }

    // onProgress(type: 'progress', callback: (uploadedSize: long, totalSize: long) => void): void {
    //   this.onProgressInner(callback);
    // }

    // onHeaderReceive(type: 'header_receive', callback: (header: Record<string, string>) => void): void {
    //   this.onHeaderReceiveInner(callback);
    // }

    // offEvent(type: 'complete' | 'fail', callback?: (states: Array<TaskState>) => void): void {
    //   this.offEventInner(type, callback);
    // }

    // offProgress(type: 'progress', callback?: (uploadedSize: long, totalSize: long) => void): void {
    //   this.offProgressInner(callback);
    // }

    // offHeaderReceive(type: 'header_receive', callback?: (header: Record<string, string>) => void): void {
    //   this.offHeaderReceiveInner(callback);
    // }

    native onProgressInner(callback: (uploadedSize: long, totalSize: long) => void): void;
    native onEventInner(type: string, callback: Callback<Array<TaskState>>):void;
    native onHeaderReceiveInner(callback: (header: Record<string, string>) => void): void;
    native offProgressInner(callback?: (uploadedSize: long, totalSize: long) => void): void;
    native offEventInner(type: string, callback?: Callback<Array<TaskState>>):void;
    native offHeaderReceiveInner(callback?: (header: Record<string, string>) => void): void;
    native offEvents(type: string):void;
  }

  export namespace agent {
    loadLibrary("request_ani")

    enum Action {
      DOWNLOAD,
      UPLOAD
    }

    enum Mode {
      BACKGROUND,
      FOREGROUND
    }

    enum Network {
      ANY,
      WIFI,
      CELLULAR
    }

    enum BroadcastEvent {
      COMPLETE = 'ohos.request.event.COMPLETE'
    }

    interface FileSpec {
      path: string;
      contentType?: string;
      filename?: string;
      extras?: Record<string, string>;
    }

    class FileSpecInner implements FileSpec {
      path: string;
      contentType?: string;
      filename?: string;
      extras?: Record<string, string>;
    }

    interface FormItem {
      name: string;
      value: string | FileSpec | Array<FileSpec>;
    }

    class FormItemInner implements FormItem {
      name: string;
      value: string | FileSpec | Array<FileSpec>;
    }

    interface Notification {
      title?: string;
      text?: string;
    }

    interface Config {
      action: Action;
      url: string;
      title?: string;
      description?: string;

      mode?: Mode;
      overwrite?: boolean;
      method?: string;
      headers?: Record<string, string>;
      data?: string | Array<FormItem>;
      saveas?: string;
      network?: Network;
      metered?: boolean;
      roaming?: boolean;
      retry?: boolean;
      redirect?: boolean;
      proxy?: string;
      index?: int;
      begins?: long;
      ends?: long;
      gauge?: boolean;
      precise?: boolean;
      token?: string;
      priority?: int;
      extras?: Record<string, string>;
      multipart?: boolean;
      notification?: Notification;
    }

    class ConfigInner implements Config {
      action: Action;
      url: string;
      title?: string;
      description?: string;

      mode?: Mode;
      overwrite?: boolean;
      method?: string;
      headers?: Record<string, string>;
      data?: string | Array<FormItem>;
      saveas?: string;
      network?: Network;
      metered?: boolean;
      roaming?: boolean;
      retry?: boolean;
      redirect?: boolean;
      proxy?: string;
      index?: int;
      begins?: long;
      ends?: long;
      gauge?: boolean;
      precise?: boolean;
      token?: string;
      priority?: int;
      extras?: Record<string, string>;
      multipart?: boolean;
      notification?: Notification;
    }

    enum State {
      INITIALIZED = 0x00,
      WAITING = 0x10,
      RUNNING = 0x20,
      RETRYING = 0x21,
      PAUSED = 0x30,
      STOPPED = 0x31,
      COMPLETED = 0x40,
      FAILED = 0x41,
      REMOVED = 0x50
    }

    interface Progress {
      readonly state: State;
      readonly index: int;
      readonly processed: long;
      readonly sizes: Array<long>;
      readonly extras?: Record<string, string>;
    }

    class ProgressInner implements Progress {
      readonly state: State;
      readonly index: int;
      readonly processed: long;
      readonly sizes: Array<long>;
      readonly extras?: Record<string, string>;
    }

    enum Faults {
      OTHERS = 0xFF,
      DISCONNECTED = 0x00,
      TIMEOUT = 0x10,
      PROTOCOL = 0x20,
      PARAM = 0x30,
      FSIO = 0x40,
      DNS = 0x50,
      TCP = 0x60,
      SSL = 0x70,
      REDIRECT = 0x80
    }

    interface Filter {
      bundle?: string;
      before?: long;
      after?: long;
      state?: State;
      action?: Action;
      mode?: Mode;
    }

    interface TaskInfo {
      readonly uid?: string;
      readonly bundle?: string;
      readonly saveas?: string;
      readonly url?: string;
      readonly data?: string | Array<FormItem>;
      readonly tid: string;
      readonly title: string;
      readonly description: string;
      readonly action: Action;
      readonly mode: Mode;
      readonly priority: int;
      readonly mimeType: string;
      readonly progress: Progress;
      readonly gauge: boolean;
      readonly ctime: long;
      readonly mtime: long;
      readonly retry: boolean;
      readonly tries: int;
      readonly faults: Faults;
      readonly reason: string;
      readonly extras?: Record<string, string>;
    }

    class TaskInfoInner implements TaskInfo{
      readonly uid?: string;
      readonly bundle?: string;
      readonly saveas?: string;
      readonly url?: string;
      readonly data?: string | Array<FormItem>;
      readonly tid: string;
      readonly title: string;
      readonly description: string;
      readonly action: Action;
      readonly mode: Mode;
      readonly priority: int;
      readonly mimeType: string;
      readonly progress: Progress;
      readonly gauge: boolean;
      readonly ctime: long;
      readonly mtime: long;
      readonly retry: boolean;
      readonly tries: int;
      readonly faults: Faults;
      readonly reason: string;
      readonly extras?: Record<string, string>;
    }

    interface HttpResponse {
      readonly version: string;
      readonly statusCode: int;
      readonly reason: string;
      readonly headers: Map<string, Array<string>>;
    }

    class HttpResponseInner implements HttpResponse {
      readonly version: string;
      readonly statusCode: int;
      readonly reason: string;
      readonly headers: Map<string, Array<string>>;
    }

    interface Task {
      readonly tid: string;
      config: Config;

      on(event: string, callback: Callback<Any>): void;
      off(event: string, callback: Callback<Any>): void;

      start(callback: AsyncCallback<void>): void;
      start(): Promise<void>;
      pause(callback: AsyncCallback<void>): void;
      pause(): Promise<void>;
      resume(callback: AsyncCallback<void>): void;
      resume(): Promise<void>;
      stop(callback: AsyncCallback<void>): void;
      stop(): Promise<void>;
      setMaxSpeed(speed: long): Promise<void>;
    }

  export class TaskInner implements Task {
    static {
      loadLibrary("request_ani")
    }

    readonly tid: string;

    config: Config;

    native startSync(): void;

    native onEvent(event: string, callback: (progress: Progress) => void): void;

    native onResponseEvent(event: string, callback: Callback<HttpResponse>): void;

    native onFaultEvent(event: string, callback: Callback<Faults>): void;

    native offEvent(event: string, callback?: (progress: Progress) => void): void;

    native offResponseEvent(event: string, callback: Callback<HttpResponse>): void;

    native offFaultEvent(event: string, callback: Callback<Faults>): void;

    native offEvents(event: string): void;

    on(event: string, callback: Callback<Any>): void {
      if (event == "response") {
        this.onResponseEvent(event, callback as Callback<HttpResponse>);
      } else if (event == "faultOccur") {
        this.onFaultEvent(event, callback as Callback<Faults>);
      } else {
        this.onEvent(event, callback as Callback<Progress>);
      }
    }

    off(event: string, callback?: Callback<Any>): void {
      if (callback == undefined) {
        this.offEvents(event);
      } else if (event == "response") {
        this.offResponseEvent(event, callback as Callback<HttpResponse>);
      } else if (event == "faultOccur") {
        this.offFaultEvent(event, callback as Callback<Faults>);
      } else {
        this.offEvent(event, callback as Callback<Progress>);
      }
    }

    start(callback: AsyncCallback<void>): void {
        let p1 = taskpool.execute((): void => {
         return this.startSync();
        });
        p1.then(() => {
         callback(null, undefined);
        }, (err: Error): void => {
          callback(err as BusinessError, undefined);
        });
    }

    start(): Promise<void> {
        return new Promise<void>((resolve, reject) => {
            taskpool.execute(() => {
                this.startSync();
            }).then((content: NullishType) => {
                resolve(undefined);
            }, (err: Error): void => {
                reject(err as BusinessError);
            });
        });
    }

    native pauseSync(): void;

    pause(callback: AsyncCallback<void>): void {
        let p1 = taskpool.execute((): void => {
            return this.pauseSync();
        });
        p1.then(() => {
            callback(null, undefined);
        }, (err: Error): void => {
            callback(err as BusinessError, undefined);
        });
    }
    pause(): Promise<void> {
        return new Promise<void>((resolve, reject) => {
            taskpool.execute(() => {
                this.pauseSync();
            }).then((content: NullishType) => {
                resolve(undefined);
            }, (err: Error): void => {
                reject(err as BusinessError);
            });
        });
    }

    native resumeSync():void;

    resume(callback: AsyncCallback<void>): void {
        let p1 = taskpool.execute((): void => {
            return this.resumeSync();
        });
        p1.then(() => {
            callback(null, undefined);
        }, (err: Error): void => {
            callback(err as BusinessError, undefined);
        });
    }
    resume(): Promise<void> {
        return new Promise<void>((resolve, reject) => {
            taskpool.execute(() => {
                this.resumeSync();
            }).then((content: NullishType) => {
                resolve(undefined);
            }, (err: Error): void => {
                reject(err as BusinessError);
            });
        });
    }

    native stopSync(): void;

    stop(callback: AsyncCallback<void>): void {
        let p1 = taskpool.execute((): void => {
            return this.stopSync();
        });
        p1.then(() => {
            callback(null, undefined);
        }, (err: Error): void => {
            callback(err as BusinessError, undefined);
        });
    }
    stop(): Promise<void> {
        return new Promise<void>((resolve, reject) => {
            taskpool.execute(() => {
                this.stopSync();
            }).then((content: NullishType) => {
                resolve(undefined);
            }, (err: Error): void => {
                reject(err as BusinessError);
            });
        });
    }

    native setMaxSpeedSync(speed:long): void;

    setMaxSpeed(speed: long): Promise<void> {
        return new Promise<void>((resolve, reject) => {
            taskpool.execute(() => {
                this.setMaxSpeedSync(speed);
            }).then((content: NullishType) => {
                resolve(undefined);
            }, (err: Error): void => {
                reject(err as BusinessError);
            });
        });
    }
  }

    native function createSync(context: BaseContext, config: Config): Task;

    export function create(context: BaseContext, config: Config, callback: AsyncCallback<Task>): void {
      let p1 = taskpool.execute((): Task => {
        return createSync(context, config);
      });
      p1.then((content: NullishType) => {
        callback(null, content as Task);
      }, (err: Error): void => {
        callback(err as BusinessError,new TaskInner);
      });
    }

    export function create(context: BaseContext, config: Config): Promise<Task> {
      return new Promise<Task>((resolve, reject) => {
        taskpool.execute((): Task => {
          return createSync(context, config);
        }).then((content: NullishType) => {
          resolve(content as Task);
        }, (err: Error): void => {
          reject(err as BusinessError);
        });
      });
    }

    native function getTaskSync(context: BaseContext, id: string, token?: string): Task;

    export function getTask(context: BaseContext, id: string, token?: string): Promise<Task> {
      return new Promise<Task>((resolve, reject) => {
        taskpool.execute((): Task => {
          return getTaskSync(context, id, token);
        }).then((content: NullishType) => {
          resolve(content as Task);
        }, (err: Error): void => {
          reject(err as BusinessError);
        });
      });
    }

    native function removeSync(id: string): void;

    export function remove(id: string, callback: AsyncCallback<void>): void {
      let p1 = taskpool.execute((): void => {
        return removeSync(id);
      });
      p1.then(() => {
        callback(null, undefined);
      }, (err: Error): void => {
        callback(err as BusinessError, undefined);
      });
    }

    export function remove(id: string): Promise<void> {
      return new Promise<void>((resolve, reject) => {
        taskpool.execute((): void => {
          return removeSync(id);
        }).then(() => {
          resolve(undefined);
        }, (err: Error): void => {
          reject(err as BusinessError);
        });
      });
    }

    native function showSync(id: string): TaskInfo;

    export function show(id: string, callback: AsyncCallback<TaskInfo>): void {
      let p1 = taskpool.execute((): TaskInfo => {
        return showSync(id);
      });
      p1.then((content: NullishType) => {
        callback(null, content as TaskInfo);
      }, (err: Error): void => {
        callback(err as BusinessError, new TaskInfoInner);
      });
    }

    export function show(id: string): Promise<TaskInfo> {
      return new Promise<TaskInfo>((resolve, reject) => {
        taskpool.execute((): TaskInfo => {
          return showSync(id);
        }).then((content: NullishType) => {
          resolve(content as TaskInfo);
        }, (err: Error): void => {
          reject(err as BusinessError);
        });
      });
    }

    native function touchSync(id: string, token: string): TaskInfo;

    export function touch(id: string, token: string, callback: AsyncCallback<TaskInfo>): void {
      let p1 = taskpool.execute((): TaskInfo => {
        return touchSync(id, token);
      });
      p1.then((content: NullishType) => {
        callback(null, content as TaskInfo);
      }, (err: Error): void => {
        callback(err as BusinessError, new TaskInfoInner);
      });
    }

    export function touch(id: string, token: string): Promise<TaskInfo> {
      return new Promise<TaskInfo>((resolve, reject) => {
        taskpool.execute((): TaskInfo => {
          return touchSync(id, token);
        }).then((content: NullishType) => {
          resolve(content as TaskInfo);
        }, (err: Error): void => {
          reject(err as BusinessError);
        });
      });
    }

  native function searchSync(filter?: Filter): Array<string>;

    export function search(callback: AsyncCallback<Array<string>>): void {
      let p1 = taskpool.execute((): Array<string> => {
        return searchSync();
      });
      p1.then((content: NullishType) => {
        callback(null, content as Array<string>);
      }, (err: Error): void => {
        callback(err as BusinessError, undefined);
      });
    }

    export function search(filter: Filter, callback: AsyncCallback<Array<string>>): void {
      let p1 = taskpool.execute((): Array<string> => {
        return searchSync(filter);
      });
      p1.then((content: NullishType) => {
        callback(null, content as Array<string>);
      }, (err: Error): void => {
        callback(err as BusinessError, undefined);
      });
    }

    export function search(filter?: Filter): Promise<Array<string>> {
      return new Promise<Array<string>>((resolve, reject) => {
        taskpool.execute((): Array<string> => {
          return searchSync(filter)
        }).then((content: NullishType) => {
          resolve(content as Array<string>);
        }, (err: Error): void => {
          reject(err as BusinessError);
        });
      });
    }

    export native function querySync(id: string): TaskInfo;

    export function query(id: string, callback: AsyncCallback<TaskInfo>): void {
      let p1 = taskpool.execute((): TaskInfo => {
        return querySync(id);
      });
      p1.then((content: NullishType) => {
        callback(null, content as TaskInfo);
      }, (err: Error): void => {
        callback(err as BusinessError, new TaskInfoInner);
      });
    }

    export function query(id: string): Promise<TaskInfo> {
      return new Promise<TaskInfo>((resolve, reject) => {
        taskpool.execute((): TaskInfo => {
          return querySync(id);
        }).then((content: NullishType) => {
          resolve(content as TaskInfo);
        }, (err: Error): void => {
          reject(err as BusinessError);
        });
      });
    }

    interface GroupConfig {
      gauge?: boolean;
      notification: Notification;
    }

    export native function createGroupSync(config: GroupConfig): string;

    export function createGroup(config: GroupConfig): Promise<string> {
      return new Promise<string>((resolve, reject) => {
        taskpool.execute((): string => {
          return createGroupSync(config);
        }).then((content: NullishType) => {
          resolve(content as string);
        }, (err: Error): void => {
          reject(err as BusinessError);
        });
      });
    }

    native function attachGroupSync(gid: string, tids: string[]): void;

    export function attachGroup(gid: string, tids: string[]): Promise<void> {
      return new Promise<void>((resolve, reject) => {
        taskpool.execute((): void => {
          return attachGroupSync(gid, tids);
        }).then(() => {
          resolve(undefined);;
        }, (err: Error): void => {
          reject(err as BusinessError);
        });
      });
    }

    native function deleteGroupSync(gid: string, tids: string[]): void;

    export function deleteGroup(gid: string): Promise<void> {
      return new Promise<void>((resolve, reject) => {
        taskpool.execute((): void => {
          return deleteGroupSync(gid, []);
        }).then(() => {
          resolve(undefined);;
        }, (err: Error): void => {
          reject(err as BusinessError);
        });
      });
    }
  }
}
