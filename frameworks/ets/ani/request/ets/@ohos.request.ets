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
  const EXCEPTION_PERMISSION: number;
  const EXCEPTION_PARAMCHECK: number;
  const EXCEPTION_UNSUPPORTED: number;
  const EXCEPTION_FILEIO: number;
  const EXCEPTION_FILEPATH: number;
  const EXCEPTION_SERVICE: number;
  const EXCEPTION_OTHERS: number;

  const NETWORK_MOBILE: number;

  const NETWORK_WIFI: number;

  const ERROR_CANNOT_RESUME: number;

  const ERROR_DEVICE_NOT_FOUND: number;

  const ERROR_FILE_ALREADY_EXISTS: number;

  const ERROR_FILE_ERROR: number;

  const ERROR_HTTP_DATA_ERROR: number;

  const ERROR_INSUFFICIENT_SPACE: number;

  const ERROR_TOO_MANY_REDIRECTS: number;

  const ERROR_UNHANDLED_HTTP_CODE: number;

  const ERROR_UNKNOWN: number;

  const ERROR_OFFLINE: number;

  const ERROR_UNSUPPORTED_NETWORK_TYPE: number;

  const PAUSED_QUEUED_FOR_WIFI: number;

  const PAUSED_WAITING_FOR_NETWORK: number;

  const PAUSED_WAITING_TO_RETRY: number;

  const PAUSED_BY_USER: number;

  const PAUSED_UNKNOWN: number;

  const SESSION_SUCCESSFUL: number;

  const SESSION_RUNNING: number;

  const SESSION_PENDING: number;

  const SESSION_PAUSED: number;

  const SESSION_FAILED: number;

  native function downloadSync(config: DownloadConfig): DownloadTask;

  export function download(config: DownloadConfig, callback: AsyncCallback<DownloadTask>): void {
    let p1 = taskpool.execute((): DownloadTask => {
        return downloadSync(config);
    })
    p1.then((content: NullishType) => {
        callback(new BusinessError(), content as DownloadTask);
    }, (err: BusinessError): void => {
        callback(err, new DownloadTaskInner);
    });
  }

  native function downloadFileSync(context: BaseContext, config: DownloadConfig): DownloadTask;

  export function downloadFile(context: BaseContext, config: DownloadConfig, callback: AsyncCallback<DownloadTask>): void {
    let p1 = taskpool.execute((): DownloadTask => {
        return downloadFileSync(context, config);
    })
    p1.then((content: NullishType) => {
        callback(new BusinessError(), content as DownloadTask);
    }, (err: BusinessError): void => {
        callback(err, new DownloadTaskInner);
    });
  }

  export function download(config: DownloadConfig): Promise<DownloadTask> {
    return new Promise<DownloadTask>((resolve, reject) => {
      taskpool.execute((): DownloadTask => {
          return downloadSync(config);
      }).then((content: NullishType) => {
          resolve(content as DownloadTask);
      }, (err: BusinessError): void => {
          reject(err);
      });
    });
  }

  export function downloadFile(context: BaseContext, config: DownloadConfig): Promise<DownloadTask> {
    return new Promise<DownloadTask>((resolve, reject) => {
      taskpool.execute((): DownloadTask => {
          return downloadFileSync(context, config);
      }).then((content: NullishType) => {
          resolve(content as DownloadTask);
      }, (err: BusinessError): void => {
          reject(err);
      });
    });
  }

  native function uploadSync(config: UploadConfig): UploadTask;

  export function upload(config: UploadConfig, callback: AsyncCallback<UploadTask>): void {
    let p1 = taskpool.execute((): UploadTask => {
        return uploadSync(config);
    })
    p1.then((content: NullishType) => {
        callback(new BusinessError(), content as UploadTask);
    }, (err: BusinessError): void => {
        callback(err, new UploadTaskInner);
    });
  }

  native function uploadFileSync(context: BaseContext, config: UploadConfig): UploadTask;

  export function uploadFile(context: BaseContext, config: UploadConfig, callback: AsyncCallback<UploadTask>): void {
    let p1 = taskpool.execute((): UploadTask => {
        return uploadFileSync(context,config);
    })
    p1.then((content: NullishType) => {
        callback(new BusinessError(), content as UploadTask);
    }, (err: BusinessError): void => {
        callback(err, new UploadTaskInner);
    });
  }

  export function upload(config: UploadConfig): Promise<UploadTask> {
    return new Promise<UploadTask>((resolve, reject) => {
      taskpool.execute((): UploadTask => {
          return uploadSync(config);
      }).then((content: NullishType) => {
          resolve(content as UploadTask);
      }, (err: BusinessError): void => {
          reject(err);
      });
    });
  }

  export function uploadFile(context: BaseContext, config: UploadConfig): Promise<UploadTask> {
    return new Promise<UploadTask>((resolve, reject) => {
      taskpool.execute((): UploadTask => {
          return uploadFileSync(context, config);
      }).then((content: NullishType) => {
          resolve(content as UploadTask);
      }, (err: BusinessError): void => {
          reject(err);
      });
    });
  }

  interface DownloadConfig {
    url: string;
    header?: Object;
    enableMetered?: boolean;
    enableRoaming?: boolean;
    description?: string;
    networkType?: number;
    filePath?: string;
    title?: string;
    background?: boolean;
  }

  interface DownloadInfo {
    description: string;
    downloadedBytes: number;
    downloadId: number;
    failedReason: number;
    fileName: string;
    filePath: string;
    pausedReason: number;
    status: number;
    targetURI: string;
    downloadTitle: string;
    downloadTotalBytes: number;
  }

  class DownloadInfoInner implements DownloadInfo{
    description: string;
    downloadedBytes: number;
    downloadId: number;
    failedReason: number;
    fileName: string;
    filePath: string;
    pausedReason: number;
    status: number;
    targetURI: string;
    downloadTitle: string;
    downloadTotalBytes: number;
  }

  interface DownloadTask {
    // on(type: 'progress', callback: (receivedSize: number, totalSize: number) => void): void;

    // off(type: 'progress', callback?: (receivedSize: number, totalSize: number) => void): void;

    // on(type: 'complete' | 'pause' | 'remove', callback: () => void): void;

    // off(type: 'complete' | 'pause' | 'remove', callback?: () => void): void;

    // on(type: 'fail', callback: (err: number) => void): void;

    // off(type: 'fail', callback?: (err: number) => void): void;

    remove(callback: AsyncCallback<boolean>): void;

    remove(): Promise<boolean>;

    pause(callback: AsyncCallback<void>): void;

    pause(): Promise<void>;

    resume(callback: AsyncCallback<void>): void;

    resume(): Promise<void>;

    query(callback: AsyncCallback<DownloadInfo>): void;

    query(): Promise<DownloadInfo>;

    queryMimeType(callback: AsyncCallback<string>): void;

    queryMimeType(): Promise<string>;

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

  class DownloadTaskInner implements DownloadTask {
    native removeSync(): boolean;

    remove(callback: AsyncCallback<boolean>): void {
      let p1 = taskpool.execute((): boolean => {
        return this.removeSync();
      })
      p1.then((content: NullishType) => {
          callback(new BusinessError(), content as boolean);
      }, (err: BusinessError): void => {
          callback(err, false);
      });
    }

    remove(): Promise<boolean> {
      return new Promise<boolean>((resolve, reject) => {
        taskpool.execute((): boolean => {
            return this.removeSync();
        }).then((content: NullishType) => {
            resolve(content as boolean);
        }, (err: BusinessError): void => {
            reject(err);
        });
      });
    }

    native pauseSync(): void;

    pause(callback: AsyncCallback<void>): void {
        let p1 = taskpool.execute((): undefined => {
            return this.pauseSync();
        })
        p1.then(() => {
            callback(new BusinessError(), undefined);
        }, (err: BusinessError): void => {
            callback(err, undefined);
        });
    }

    pause(): Promise<void> {
        return new Promise<void>((resolve, reject) => {
            taskpool.execute((): undefined => {
              return this.pauseSync();
            }).then(() => {
                resolve(undefined);
            }, (err: BusinessError): void => {
                reject(err);
            });
        });
    }

    native resumeSync(): void;

    resume(callback: AsyncCallback<void>): void {
        let p1 = taskpool.execute((): undefined => {
            return this.resumeSync();
        })
        p1.then(() => {
            callback(new BusinessError(), undefined);
        }, (err: BusinessError): void => {
            callback(err, undefined);
        });
    }

    resume(): Promise<void> {
        return new Promise<void>((resolve, reject) => {
            taskpool.execute((): undefined => {
                return this.resumeSync();
            }).then(() => {
                resolve(undefined);
            }, (err: BusinessError): void => {
                reject(err);
            });
        });
    }

    native querySync(): DownloadInfo;

    query(callback: AsyncCallback<DownloadInfo>): void {
        let p1 = taskpool.execute((): DownloadInfo => {
            return this.querySync();
        })
        p1.then((content: NullishType) => {
            callback(new BusinessError(), content as DownloadInfo);
        }, (err: BusinessError): void => {
            callback(err, new DownloadInfoInner);
        });
    }

    query(): Promise<DownloadInfo> {
        return new Promise<DownloadInfo>((resolve, reject) => {
            taskpool.execute((): DownloadInfo => {
                return this.querySync();
            }).then((content: NullishType) => {
                resolve(content as DownloadInfo);
            }, (err: BusinessError): void => {
                reject(err);
            });
        });
    }

    native queryMimeTypeSync(): string;

    queryMimeType(callback: AsyncCallback<string>): void {
        let p1 = taskpool.execute((): string => {
            return this.queryMimeTypeSync();
        })
        p1.then((content: NullishType) => {
            callback(new BusinessError(), content as string);
        }, (err: BusinessError): void => {
            callback(err, "");
        });
    }

    queryMimeType(): Promise<string> {
        return new Promise<string>((resolve, reject) => {
            taskpool.execute((): string => {
                return this.queryMimeTypeSync();
            }).then((content: NullishType) => {
                resolve(content as string);
            }, (err: BusinessError): void => {
                reject(err);
            });
        });
    }

    native deleteSync(): boolean;

    delete(callback: AsyncCallback<boolean>): void {
        let p1 = taskpool.execute((): boolean => {
            return this.deleteSync();
        })
        p1.then((content: NullishType) => {
            callback(new BusinessError(), content as boolean);
        }, (err: BusinessError): void => {
            callback(err, false);
        });
    }

    delete(): Promise<boolean> {
        return new Promise<boolean>((resolve, reject) => {
            taskpool.execute((): boolean => {
                return this.deleteSync();
            }).then((content: NullishType) => {
                resolve(content as boolean);
            }, (err: BusinessError): void => {
                reject(err);
            });
        });
    }

    native suspendSync(): boolean;

    suspend(callback: AsyncCallback<boolean>): void {
        let p1 = taskpool.execute((): boolean => {
            return this.suspendSync();
        })
        p1.then((content: NullishType) => {
            callback(new BusinessError(), content as boolean);
        }, (err: BusinessError): void => {
            callback(err, false);
        });
    }

    suspend(): Promise<boolean> {
        return new Promise<boolean>((resolve, reject) => {
            taskpool.execute((): boolean => {
                return this.suspendSync();
            }).then((content: NullishType) => {
                resolve(content as boolean);
            }, (err: BusinessError): void => {
                reject(err);
            });
        });
    }

    native restoreSync(): boolean;

    restore(callback: AsyncCallback<boolean>): void {
        let p1 = taskpool.execute((): boolean => {
            return this.restoreSync();
        })
        p1.then((content: NullishType) => {
            callback(new BusinessError(), content as boolean);
        }, (err: BusinessError): void => {
            callback(err, false);
        });
    }

    restore(): Promise<boolean> {
        return new Promise<boolean>((resolve, reject) => {
            taskpool.execute((): boolean => {
                return this.restoreSync();
            }).then((content: NullishType) => {
                resolve(content as boolean);
            }, (err: BusinessError): void => {
                reject(err);
            });
        });
    }

    native getTaskInfoSync(): DownloadInfo;

    getTaskInfo(callback: AsyncCallback<DownloadInfo>): void {
        let p1 = taskpool.execute((): DownloadInfo => {
            return this.getTaskInfoSync();
        })
        p1.then((content: NullishType) => {
            callback(new BusinessError(), content as DownloadInfo);
        }, (err: BusinessError): void => {
            callback(err, new DownloadInfoInner);
        });
    }

    getTaskInfo(): Promise<DownloadInfo> {
        return new Promise<DownloadInfo>((resolve, reject) => {
            taskpool.execute((): DownloadInfo => {
                return this.getTaskInfoSync();
            }).then((content: NullishType) => {
                resolve(content as DownloadInfo);
            }, (err: BusinessError): void => {
                reject(err);
            });
        });
    }

    native getTaskMimeTypeSync(): string;

    getTaskMimeType(callback: AsyncCallback<string>): void {
        let p1 = taskpool.execute((): string => {
            return this.getTaskMimeTypeSync();
        })
        p1.then((content: NullishType) => {
            callback(new BusinessError(), content as string);
        }, (err: BusinessError): void => {
            callback(err, "");
        });
    }

    getTaskMimeType(): Promise<string> {
        return new Promise<string>((resolve, reject) => {
            taskpool.execute((): string => {
                return this.getTaskMimeTypeSync();
            }).then((content: NullishType) => {
                resolve(content as string);
            }, (err: BusinessError): void => {
                reject(err);
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
    header: Object;
    method: string;
    index?: number;
    begins?: number;
    ends?: number;
    files: Array<File>;
    data: Array<RequestData>;
  }

  interface TaskState {
    path: string;
    responseCode: number;
    message: string;
  }

  interface UploadTask {
    // on(type: 'progress', callback: (uploadedSize: number, totalSize: number) => void): void;

    // off(type: 'progress', callback?: (uploadedSize: number, totalSize: number) => void): void;

    // on(type: 'headerReceive', callback: (header: object) => void): void;

    // off(type: 'headerReceive', callback?: (header: object) => void): void;

    // on(type: 'complete' | 'fail', callback: Callback<Array<TaskState>>): void;

    // off(type: 'complete' | 'fail', callback?: Callback<Array<TaskState>>): void;

    remove(callback: AsyncCallback<boolean>): void;

    remove(): Promise<boolean>;

    delete(callback: AsyncCallback<boolean>): void;

    delete(): Promise<boolean>;
  }

  class UploadTaskInner implements UploadTask {

    native removeSync(): boolean;

    remove(callback: AsyncCallback<boolean>): void {
        let p1 = taskpool.execute((): boolean => {
            return this.removeSync();
        })
        p1.then((content: NullishType) => {
            callback(new BusinessError(), content as boolean);
        }, (err: BusinessError): void => {
            callback(err, false);
        });
    }

    remove(): Promise<boolean> {
      return new Promise<boolean>((resolve, reject) => {
          taskpool.execute((): boolean => {
              return this.removeSync();
          }).then((content: NullishType) => {
              resolve(content as boolean);
          }, (err: BusinessError): void => {
              reject(err);
          });
      });
    }

    native deleteSync(): boolean;

    delete(callback: AsyncCallback<boolean>): void {
      let p1 = taskpool.execute((): boolean => {
          return this.deleteSync();
      })
      p1.then((content: NullishType) => {
          callback(new BusinessError(), content as boolean);
      }, (err: BusinessError): void => {
          callback(err, false);
      });
    }

    delete(): Promise<boolean> {
      return new Promise<boolean>((resolve, reject) => {
          taskpool.execute((): boolean => {
              return this.deleteSync();
          }).then((content: NullishType) => {
              resolve(content as boolean);
          }, (err: BusinessError): void => {
              reject(err);
          });
      });
    }
  }

  namespace agent {
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
      mimeType?: string;
      contentType?: string;
      filename?: string;
      extras?: object;
    }

    interface FormItem {
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
      headers?: object;
      data?: string | Array<FormItem>;
      saveas?: string;
      network?: Network;
      metered?: boolean;
      roaming?: boolean;
      retry?: boolean;
      redirect?: boolean;
      proxy?: string;
      index?: number;
      begins?: number;
      ends?: number;
      gauge?: boolean;
      precise?: boolean;
      token?: string;
      priority?: number;
      extras?: object;
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
      readonly index: number;
      readonly processed: number;
      readonly sizes: Array<number>;
      readonly extras?: object;
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
      before?: number;
      after?: number;
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
      readonly priority: number;
      readonly mimeType: string;
      readonly progress: Progress;
      readonly gauge: boolean;
      readonly ctime: number;
      readonly mtime: number;
      readonly retry: boolean;
      readonly tries: number;
      readonly faults: Faults;
      readonly reason: string;
      readonly extras?: object;
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
      readonly priority: number;
      readonly mimeType: string;
      readonly progress: Progress;
      readonly gauge: boolean;
      readonly ctime: number;
      readonly mtime: number;
      readonly retry: boolean;
      readonly tries: number;
      readonly faults: Faults;
      readonly reason: string;
      readonly extras?: object;
    }

    interface HttpResponse {
      readonly version: string,
      readonly statusCode: number,
      readonly reason: string,
      readonly headers: Map<string, Array<string>>,
    }

    interface Task {
      readonly tid: string;
      config: Config;

      // on(event: 'progress', callback: (progress: Progress) => void): void;
      // off(event: 'progress', callback?: (progress: Progress) => void): void;
      // on(event: 'completed', callback: (progress: Progress) => void): void;
      // off(event: 'completed', callback?: (progress: Progress) => void): void;
      // on(event: 'failed', callback: (progress: Progress) => void): void;
      // off(event: 'failed', callback?: (progress: Progress) => void): void;
      // on(event: 'pause', callback: (progress: Progress) => void): void;
      // off(event: 'pause', callback?: (progress: Progress) => void): void;
      // on(event: 'resume', callback: (progress: Progress) => void): void;
      // off(event: 'resume', callback?: (progress: Progress) => void): void;
      // on(event: 'remove', callback: (progress: Progress) => void): void;
      // off(event: 'remove', callback?: (progress: Progress) => void): void;
      // on(event: 'response', callback: Callback<HttpResponse>): void;
      // off(event: 'response', callback?: Callback<HttpResponse>): void;

      start(callback: AsyncCallback<void>): void;
      start(): Promise<void>;
      pause(callback: AsyncCallback<void>): void;
      pause(): Promise<void>;
      resume(callback: AsyncCallback<void>): void;
      resume(): Promise<void>;
      stop(callback: AsyncCallback<void>): void;
      stop(): Promise<void>;
      setMaxSpeed(speed: number): Promise<void>;
    }

  class TaskInner implements Task {
    readonly tid: string;
    config: Config;

    native startSync(): void;

    start(callback: AsyncCallback<void>): void {
        let p1 = taskpool.execute((): void => {
         return this.startSync();
        });
        p1.then(() => {
         callback(new BusinessError(), undefined);
        }, (err: BusinessError): void => {
          callback(err, undefined);
        });
    }

    start(): Promise<void> {
        return new Promise<void>((resolve, reject) => {
            taskpool.execute((): undefined => {
              return this.startSync();
            }).then((content: NullishType) => {
                resolve(undefined);
            }, (err: BusinessError): void => {
                reject(err);
            });
        });
    }

    native pauseSync(): void;

    pause(callback: AsyncCallback<void>): void {
        let p1 = taskpool.execute((): void => {
            return this.pauseSync();
        });
        p1.then(() => {
            callback(new BusinessError(), undefined);
        }, (err: BusinessError): void => {
            callback(err, undefined);
        });
    }
    pause(): Promise<void> {
        return new Promise<void>((resolve, reject) => {
            taskpool.execute((): undefined => {
              return this.pauseSync();
            }).then((content: NullishType) => {
                resolve(undefined);
            }, (err: BusinessError): void => {
                reject(err);
            });
        });
    }

    native resumeSync():void;

    resume(callback: AsyncCallback<void>): void {
        let p1 = taskpool.execute((): void => {
            return this.resumeSync();
        });
        p1.then(() => {
            callback(new BusinessError(), undefined);
        }, (err: BusinessError): void => {
            callback(err, undefined);
        });
    }
    resume(): Promise<void> {
        return new Promise<void>((resolve, reject) => {
            taskpool.execute((): undefined => {
              return this.resumeSync();
            }).then((content: NullishType) => {
                resolve(undefined);
            }, (err: BusinessError): void => {
                reject(err);
            });
        });
    }

    native stopSync(): void;

    stop(callback: AsyncCallback<void>): void {
        let p1 = taskpool.execute((): void => {
            return this.stopSync();
        });
        p1.then(() => {
            callback(new BusinessError(), undefined);
        }, (err: BusinessError): void => {
            callback(err, undefined);
        });
    }
    stop(): Promise<void> {
        return new Promise<void>((resolve, reject) => {
            taskpool.execute((): undefined => {
              return this.stopSync();
            }).then((content: NullishType) => {
                resolve(undefined);
            }, (err: BusinessError): void => {
                reject(err);
            });
        });
    }

    native setMaxSpeedSync(speed:number): void;

    setMaxSpeed(speed: number): Promise<void> {
        return new Promise<void>((resolve, reject) => {
            taskpool.execute((): undefined => {
              return this.setMaxSpeedSync(speed);
            }).then((content: NullishType) => {
                resolve(undefined);
            }, (err: BusinessError): void => {
                reject(err);
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
        callback(new BusinessError(), content as Task);
      }, (err: BusinessError): void => {
        callback(err,new TaskInner);
      });
    }

    export function create(context: BaseContext, config: Config): Promise<Task> {
      return new Promise<Task>((resolve, reject) => {
        taskpool.execute((): Task => {
          return createSync(context, config);
        }).then((content: NullishType) => {
          resolve(content as Task);
        }, (err: BusinessError): void => {
          reject(err);
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
        }, (err: BusinessError): void => {
          reject(err);
        });
      });
    }

    native function removeSync(id: string): void;

    export function remove(id: string, callback: AsyncCallback<void>): void {
      let p1 = taskpool.execute((): void => {
        return removeSync(id);
      });
      p1.then(() => {
        callback(new BusinessError(), undefined);
      }, (err: BusinessError): void => {
        callback(err, undefined);
      });
    }

    export function remove(id: string): Promise<void> {
      return new Promise<void>((resolve, reject) => {
        taskpool.execute((): void => {
          return removeSync(id);
        }).then(() => {
          resolve(undefined);
        }, (err: BusinessError): void => {
          reject(err);
        });
      });
    }

    native function showSync(id: string): TaskInfo;

    export function show(id: string, callback: AsyncCallback<TaskInfo>): void {
      let p1 = taskpool.execute((): TaskInfo => {
        return showSync(id);
      });
      p1.then((content: NullishType) => {
        callback(new BusinessError(), content as TaskInfo);
      }, (err: BusinessError): void => {
        callback(err, new TaskInfoInner);
      });
    }

    export function show(id: string): Promise<TaskInfo> {
      return new Promise<TaskInfo>((resolve, reject) => {
        taskpool.execute((): TaskInfo => {
          return showSync(id);
        }).then((content: NullishType) => {
          resolve(content as TaskInfo);
        }, (err: BusinessError): void => {
          reject(err);
        });
      });
    }

    native function touchSync(id: string, token: string): TaskInfo;

    export function touch(id: string, token: string, callback: AsyncCallback<TaskInfo>): void {
      let p1 = taskpool.execute((): TaskInfo => {
        return touchSync(id, token);
      });
      p1.then((content: NullishType) => {
        callback(new BusinessError(), content as TaskInfo);
      }, (err: BusinessError): void => {
        callback(err, new TaskInfoInner);
      });
    }

    export function touch(id: string, token: string): Promise<TaskInfo> {
      return new Promise<TaskInfo>((resolve, reject) => {
        taskpool.execute((): TaskInfo => {
          return touchSync(id, token);
        }).then((content: NullishType) => {
          resolve(content as TaskInfo);
        }, (err: BusinessError): void => {
          reject(err);
        });
      });
    }

    native function searchSync(filter?: Filter): Array<string>;

    export function search(callback: AsyncCallback<Array<string>>): void {
      let p1 = taskpool.execute((): Array<string> => {
        return searchSync();
      });
      p1.then((content: NullishType) => {
        callback(new BusinessError(), content as Array<string>);
      }, (err: BusinessError): void => {
        callback(err, []);
      });
    }

    export function search(filter: Filter, callback: AsyncCallback<Array<string>>): void {
      let p1 = taskpool.execute((): Array<string> => {
        return searchSync(filter);
      });
      p1.then((content: NullishType) => {
        callback(new BusinessError(), content as Array<string>);
      }, (err: BusinessError): void => {
        callback(err, []);
      });
    }

    export function search(filter?: Filter): Promise<Array<string>> {
      return new Promise<Array<string>>((resolve, reject) => {
        taskpool.execute((): Array<string> => {
          return searchSync(filter);
        }).then((content: NullishType) => {
          resolve(content as Array<string>);
        }, (err: BusinessError): void => {
          reject(err);
        });
      });
    }

    native function querySync(id: string): TaskInfo;

    export function query(id: string, callback: AsyncCallback<TaskInfo>): void {
      let p1 = taskpool.execute((): TaskInfo => {
        return querySync(id);
      });
      p1.then((content: NullishType) => {
        callback(new BusinessError(), content as TaskInfo);
      }, (err: BusinessError): void => {
        callback(err, new TaskInfoInner);
      });
    }

    export function query(id: string): Promise<TaskInfo> {
      return new Promise<TaskInfo>((resolve, reject) => {
        taskpool.execute((): TaskInfo => {
          return querySync(id);
        }).then((content: NullishType) => {
          resolve(content as TaskInfo);
        }, (err: BusinessError): void => {
          reject(err);
        });
      });
    }

    interface GroupConfig {
      gauge?: boolean;
      notification: Notification;
    }

    native function createGroupSync(config: GroupConfig): string;

    export function createGroup(config: GroupConfig): Promise<string> {
      return new Promise<string>((resolve, reject) => {
        taskpool.execute((): string => {
          return createGroupSync(config);
        }).then((content: NullishType) => {
          resolve(content as string);
        }, (err: BusinessError): void => {
          reject(err);
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
        }, (err: BusinessError): void => {
          reject(err);
        });
      });
    }

    native function detachGroupSync(gid: string, tids: string[]): void;

    export function deleteGroup(gid: string): Promise<void> {
      return new Promise<void>((resolve, reject) => {
        taskpool.execute((): void => {
          return detachGroupSync(gid, []);
        }).then(() => {
          resolve(undefined);;
        }, (err: BusinessError): void => {
          reject(err);
        });
      });
    }
  }
}