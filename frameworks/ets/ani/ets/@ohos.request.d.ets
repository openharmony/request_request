/*
 * Copyright (c) 2025 Huawei Device Co., Ltd.
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

import type { AsyncCallback, Callback } from '@ohos.base';
import { BusinessError  }  from '@ohos.base';
// import BaseContext from 'BaseContext';

class Result<T, E>{
    result:T;
    error:E;
    constructor(result: T , error: E) {
        this.result = result;
        this.error = error;
    }
}


export default namespace request {
  loadLibrary("request_ani") 

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

  export native function downloadSync(config: DownloadConfig): DownloadTask;

  function download(config: DownloadConfig, callback: AsyncCallback<DownloadTask>): void {
      let p1 = taskpool.execute(downloadSync, config);
      p1.then((e :NullishType)=>{
          let r = e as Result<DownloadTask, BusinessError>;;
          callback(r.error, r.result)
      });
  }
  
  // function downloadFile(context: BaseContext, config: DownloadConfig, callback: AsyncCallback<DownloadTask>): void;
  
  // function download(config: DownloadConfig): Promise<DownloadTask>;
  
  // function downloadFile(context: BaseContext, config: DownloadConfig): Promise<DownloadTask>;
  
  // function upload(config: UploadConfig, callback: AsyncCallback<UploadTask>): void;
  
  // function uploadFile(context: BaseContext, config: UploadConfig, callback: AsyncCallback<UploadTask>): void;
  
  // function upload(config: UploadConfig): Promise<UploadTask>;
  
  // function uploadFile(context: BaseContext, config: UploadConfig): Promise<UploadTask>;
  
  interface DownloadConfig {
    url: string;
    header?: Record<string, string>;
    enableMetered?: boolean;
    enableRoaming?: boolean;
    description?: string;
    networkType?: number;
    filePath?: string;
    title?: string;
    background?: boolean;
  }

  export class DownloadConfigInner implements DownloadConfig {
    url: string;
    header?: Record<string, string>;
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

  export class DownloadInfoInner implements DownloadInfo {
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
      on(type: 'progress' | 'complete' | 'pause' | 'remove' | 'fail', callback: ((receivedSize: number, totalSize: number) => void) | (() => void)| ((err: number) => void)): void;
    
      off(type: 'progress' | 'complete' | 'pause' | 'remove' | 'fail', callback?: ((receivedSize: number, totalSize: number) => void) | (() => void) | ((err: number) => void)): void;
    
  //  remove(callback: AsyncCallback<boolean>): void;
    
  //  remove(): Promise<boolean>;
    
  //  pause(callback: AsyncCallback<void>): void;
    
  //  pause(): Promise<void>;
    
  //  resume(callback: AsyncCallback<void>): void;
    
  //  resume(): Promise<void>;
    
  //  query(callback: AsyncCallback<DownloadInfo>): void;
    
  //  query(): Promise<DownloadInfo>;
    
  //  queryMimeType(callback: AsyncCallback<string>): void;
    
  //  queryMimeType(): Promise<string>;
    
  //  delete(callback: AsyncCallback<boolean>): void;
    
  //  delete(): Promise<boolean>;

  //  suspend(callback: AsyncCallback<boolean>): void;
    
  //  suspend(): Promise<boolean>;

  //  restore(callback: AsyncCallback<boolean>): void;
    
  //  restore(): Promise<boolean>;
    
  //  getTaskInfo(callback: AsyncCallback<DownloadInfo>): void;
    
  //  getTaskInfo(): Promise<DownloadInfo>;
    
  //  getTaskMimeType(callback: AsyncCallback<string>): void;
    
  //  getTaskMimeType(): Promise<string>;

  }

  interface File {
    filename: string;
    name: string;
    uri: string;
    type: string;
  }
  
  class FileInner implements File {
    filename: string;
    name: string;
    uri: string;
    type: string;
  }

  interface RequestData {
    name: string;
    value: string;
  }
  
  class RequestDataInner implements RequestData {
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
  
  class UploadConfigInner implements UploadConfig {
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

  class TaskStateInner implements TaskState {
    path: string;
    
    responseCode: number;
    
    message: string;
  }

  interface UploadTask {
   on(type: 'progress' | 'headerReceive' |  'complete' | 'fail', callback: ((uploadedSize: number, totalSize: number) => void) | ((header: object) => void) | Callback<Array<TaskState>>): void;
    
   off(type: 'progress' | 'headerReceive' |  'complete' | 'fail', callback?: ((uploadedSize: number, totalSize: number) => void) | ((header: object) => void) | Callback<Array<TaskState>> ): void;
    
  //  remove(callback: AsyncCallback<boolean>): void;
    
  //  remove(): Promise<boolean>;
    
  //  delete(callback: AsyncCallback<boolean>): void;
    
  //  delete(): Promise<boolean>;
  }
  
  export namespace agent {
    loadLibrary("request_ani")
    
    export interface Config {
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

    class ConfigInner implements Config {
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
    
    export enum Action {
      DOWNLOAD,
      UPLOAD
    }
    export enum Mode {
      BACKGROUND,
      FOREGROUND
    }
    export enum Network {
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
    
    class FileSpecInner implements FileSpec{
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
    
    class FormItemInner implements FormItem {
      
      name: string;
      
      value: string | FileSpec | Array<FileSpec>;
    }

    interface Notification {
      
      title?: string;
      
      text?: string;
    }

    class NotificationInner implements Notification {
      
      title?: string;
      
      text?: string;
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

    export interface Progress {
      readonly state: State;
      
      readonly index: number;
      
      readonly processed: number;
      
      readonly sizes: Array<number>;
      
      readonly extras?: object;
    }

    class ProgressInner implements Progress {
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

    class FilterInner implements Filter{
      bundle?: string;
      
      before?: number;
      
      after?: number;
      
      state?: State;
      
      action?: Action;
      
      mode?: Mode;
    }

    interface HttpResponse {
      
      readonly version: string;
      
      readonly statusCode: number; 
      
      readonly reason: string;
      
      readonly headers: Map<string, Array<string>>;
    }

    class HttpResponseInner implements HttpResponse {
      readonly version: string;
      
      readonly statusCode: number;
      
      readonly reason: string;
      
      readonly headers: Map<string, Array<string>>;
    }

    interface Task {
      tid: string;

      // config: Config;
            
      on(event: 'progress' | 'completed'| 'failed'| 'pause' | 'resume' | 'remove' | 'response', callback: ((progress: Progress) => void) | Callback<HttpResponse>): void;
      
      // off(event: 'progress' | 'completed'| 'failed'| 'pause' | 'resume' | 'remove' | 'response', callback?: ((progress: Progress) => void) |Callback<HttpResponse>): void;

      // start(callback: AsyncCallback<void>): void;
      
      // start(): Promise<void>;
      
    //  pause(callback: AsyncCallback<void>): void;
      
    //  pause(): Promise<void>;
      
    //  resume(callback: AsyncCallback<void>): void;
      
    //  resume(): Promise<void>;
      
    //  stop(callback: AsyncCallback<void>): void;
      
    //  stop(): Promise<void>;
      
    //  setMaxSpeed(speed: number): Promise<void>;
    }
    
    class TaskInner implements Task {
      tid: string;

      native function startSync(): void;

      native function on(event: 'progress' | 'completed'| 'failed'| 'pause' | 'resume' | 'remove' | 'response', callback: ((progress: Progress) => void) | Callback<HttpResponse>): void;
    }

    export native function createSync( config: Config): TaskInner;

    export function create(config: Config, callback: AsyncCallback<Task>): void {
      let p1 = taskpool.execute(createSync,  config);
      p1.then((e :NullishType)=>{
          let r = e as Result<Task, BusinessError>;;
          callback(r.error, r.result)
      });
    }
  
    export function create( config: Config): Promise<Task> {
        let p = new Promise<Task>((resolve: (v: Task) => void, reject: (error: BusinessError) => void) => {
          let p1 = taskpool.execute(createSync, config);
          p1.then((e :NullishType)=>{
              let r =  e as Result<Task, BusinessError>;
              let err = r.error;
              if(err.code != 0){
                  reject(err);
              }else{
                  resolve(r.result)
              }
          });
      
      });
      return p;
    }
    
  //   function getTask(context: BaseContext, id: string, token?: string): Promise<Task>;
    
  //   function remove(id: string, callback: AsyncCallback<void>): void;
    
  //   function remove(id: string): Promise<void>;
    
  //   function show(id: string, callback: AsyncCallback<TaskInfo>): void;
    
  //   function show(id: string): Promise<TaskInfo>;
    
  //   function touch(id: string, token: string, callback: AsyncCallback<TaskInfo>): void;
    
  //   function touch(id: string, token: string): Promise<TaskInfo>;
    
  //   function search(callback: AsyncCallback<Array<string>>): void;
    
  //   function search(filter: Filter, callback: AsyncCallback<Array<string>>): void;
    
  //   function search(filter?: Filter): Promise<Array<string>>;
    
  //   function query(id: string, callback: AsyncCallback<TaskInfo>): void;
    
  //   function query(id: string): Promise<TaskInfo>;
    
    interface GroupConfig {
      
      gauge?: boolean;
      
      notification: Notification;
    }
    
  //   function createGroup(config: GroupConfig): Promise<string>;
    
  //   function attachGroup(gid: string, tids: string[]): Promise<void>;
    
  //   function deleteGroup(gid: string): Promise<void>;
  }
}

function main (): void {
  let config : request.agent.Config = {
    url: "http://cidownload.openharmony.cn/version/Daily_Version/OpenHarmony_5.1.0.58/20250418_020536/version-Daily_Version-OpenHarmony_5.1.0.58-20250418_020536-dayu200_0328_img.tar.gz",
    action: request.agent.Action.DOWNLOAD,
  }
  let task = request.agent.createSync(config);
  task.on('progress', (progress: request.agent.Progress) => {
  })
  task.startSync()
}