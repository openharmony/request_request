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

import { AsyncCallback, Callback } from '@ohos.base';
import { BusinessError } from '@ohos.base';
import BaseContext from 'application.BaseContext';

export default namespace request {
  namespace agent {
    loadLibrary("requestmodule_ani");

    interface FileSpec {
      path: string;
      mimeType?: string;
      filename?: string;
      extras?: object;
    }

    interface FormItem {
      name: string;
      value: string | FileSpec | Array<FileSpec>;
    }

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

    interface HttpResponse {
      readonly version: string,
      readonly statusCode: number,
      readonly reason: string,
      readonly headers: Map<string, Array<string>>,
    }

    export class HttpResponseImpl implements HttpResponse {
      readonly version: string;
      readonly statusCode: number;
      readonly reason: string;
      readonly headers: Map<string, Array<string>>;

      constructor(version: string, statusCode: number, reason: string) {
          this.version = version;
          this.statusCode = statusCode;
          this.reason = reason;
      }
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
    }

    interface Task {
      start(): Promise<void>;
      on(event: 'response', callback: Callback<HttpResponse>): void;
    };

    export class TaskImpl implements Task {
      nativePtr: long = 0;
      constructor(context: long) {
          console.log("in constructor, context is " + context)
          this.nativePtr = context;
      }

      on(event: 'response', callback: Callback<HttpResponse>): void {
        this.onSync(event, callback);
      };

      start(callback: AsyncCallback<void>): void {
        let p1 = taskpool.execute((): void => {
          this.startSync();
        });
        p1.then((data: NullishType): void => {
          let e: BusinessError<void>;
          callback(e, undefined);
        }).catch((error: NullishType) => {
          let err =  error as BusinessError<void>;
          callback(err, undefined);
        });
      };

      start(): Promise<void> {
        let p = new Promise<void>((resolve: (v: undefined) => void, reject: (error: Object) => void) : void => {
            let p1 = taskpool.execute((): void => {
              this.startSync();
            });
            p1.then((e: NullishType) : void => {
              console.log("in start Promise then. e is ", e);
              resolve(undefined);
            }).catch((err: Error) : void => {
              reject(err);
            });
        });
        return p;
      }

      native function onSync(event: 'response', callback: Callback<HttpResponse>): void;
      native function startSync(): void;
    };
  
    export native function createSync(context: BaseContext, config: Config): Task;
  
    export function create(context: BaseContext, config: Config, callback: AsyncCallback<Task>): void {
      let p1 = taskpool.execute(createSync, context, config);
      p1.then((data: NullishType) => {
        let r =  data as Task;
        let err: BusinessError<void>;
        callback(err, r);
      }).catch((error: NullishType) => {
        let err =  error as BusinessError<void>;
        let data: Task;
        callback(err, data);
      });
    };
  
    export function create(context: BaseContext, config: Config): Promise<Task> {
      let p = new Promise<Task>((resolve: (v: Task) => void,
        reject: (error: Object) => void): void => {
          let p1 = taskpool.execute(createSync, context, config);
          p1.then((e: NullishType): void => {
            console.log("in create Promise then. e is ", e)
            resolve(e as Task);
          }).catch((err: Error) : void => {
            reject(err);
          });
      });
      return p;
    }
  }
}
