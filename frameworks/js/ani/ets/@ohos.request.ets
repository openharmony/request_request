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
import hilog from '@ohos.hilog';

export default namespace request {
  namespace agent {
    loadLibraryWithPermissionCheck("requestmodule_ani", "@ohos.request");

    interface FileSpec {
      path: string;
      filename?: string;
    }

    interface FormItem {
      name: string;
      value: string | FileSpec | Array<FileSpec>;
    }

    enum Action {
      DOWNLOAD,
      UPLOAD
    }

    interface HttpResponse {
      readonly version: string;
      readonly statusCode: number;
      readonly reason: string;
    }

    export class HttpResponseImpl implements HttpResponse {
      readonly version: string;
      readonly statusCode: number;
      readonly reason: string;

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
    }
    const StateMap: Record<number, State> = {
      0x00: State.INITIALIZED,
      0x10: State.WAITING,
      0x20: State.RUNNING,
      0x21: State.RETRYING,
      0x30: State.PAUSED,
      0x31: State.STOPPED,
      0x40: State.COMPLETED,
      0x41: State.FAILED,
      0x50: State.REMOVED
    };
    export class ProgressImpl implements Progress {
      readonly state: State;
      readonly index: number;
      readonly processed: number;

      constructor(state: number, index: number, processed: number) {
        const resolvedState = StateMap[state];
        if (resolvedState !== undefined) {
          this.state = resolvedState;
        } else {
          this.state = State.INITIALIZED;
        }
        this.index = index;
        this.processed = processed;
      }
    }

    interface Config {
      action: Action;
      url: string;
      method?: string;
      data?: string | Array<FormItem>;
      saveas?: string;
    }

    interface Task {
      start(callback: AsyncCallback<void>): void;
      start(): Promise<void>;
      on(event: 'response' | 'completed', callback: Callback<HttpResponse | Progress>): void;
    };

    class Cleaner {
      static callback(cleaner: Cleaner): void {
          hilog.info(0x0000, 'request', "enter Cleaner.callback");
          cleaner.clean()
      }

      constructor(targetPtr: long) {
          this.targetPtr = targetPtr
      }

      native clean(): void

      private targetPtr: long = 0;
    }

    class FinalizationAgent<T extends Object> {
      static register<T extends Object>(obj: T, ptr: long): FinalizationAgent<T> {
          let instance: FinalizationAgent<T> = new FinalizationAgent<T>();
          instance.init(obj, ptr);
          return instance;
      }

      unregister(): void {
          finalizer.unregister(this.unregisterToken);
      }

      private init(obj: T, ptr: long): void {
          this.unregisterToken = {};
          this.cleaner = new Cleaner(ptr);
          finalizer.register(obj, this.cleaner!, this.unregisterToken);
      }

      private cleaner: Cleaner | null = null;
      private unregisterToken: object;
    }

    let finalizer = new FinalizationRegistry<Cleaner>(Cleaner.callback)

    export class TaskImpl implements Task {
      nativePtr: long = 0;
      private fzAgent: FinalizationAgent<TaskImpl>;
      constructor(context: long) {
          hilog.info(0x0000, 'request', "in constructor, context is " + context)
          this.nativePtr = context;
          this.fzAgent = FinalizationAgent.register<TaskImpl>(this, this.nativePtr);
      }

      unregisterCleaner(): void {
        this.fzAgent.unregister();
      }

      on(event: 'response' | 'completed', callback: Callback<HttpResponse | Progress>): void {
        this.onSync(event, callback);
      };

      start(callback: AsyncCallback<void>): void {
        let p1 = taskpool.execute((): void => {
          this.startSync();
        });
        p1.then((data: NullishType): void => {
          let e = new BusinessError<void>;
          callback(e, undefined);
        }).catch((error: NullishType) => {
          let err =  error as BusinessError<void>;
          callback(err, undefined);
        });
      };

      start(): Promise<void> {
        let p = new Promise<void>((resolve: (v: undefined) => void, reject: (error: Error) => void) : void => {
            let p1 = taskpool.execute((): void => {
              this.startSync();
            });
            p1.then((e: NullishType) : void => {
              resolve(undefined);
            }).catch((err: Error) : void => {
              reject(err);
            });
        });
        return p;
      }

      native onSync(event: string, callback: Callback<HttpResponse | Progress>): void;
      native startSync(): void;
    };

    export native function createSync(context: BaseContext, config: Config): Task;

    export function create(context: BaseContext, config: Config, callback: AsyncCallback<Task>): void {
      let p1 = taskpool.execute(createSync, context, config);
      p1.then((data: NullishType) => {
        let r =  data as Task;
        let err = new BusinessError<void>;
        callback(err, r);
      }).catch((error: NullishType) => {
        let err =  error as BusinessError<void>;
        let data: Task;
        callback(err, data);
      });
    };

    export function create(context: BaseContext, config: Config): Promise<Task> {
      let p = new Promise<Task>((resolve: (v: Task) => void,
        reject: (error: Error) => void): void => {
          let p1 = taskpool.execute(createSync, context, config);
          p1.then((e: NullishType): void => {
            resolve(e as Task);
          }).catch((err: Error) : void => {
            reject(err);
          });
      });
      return p;
    }
  }
}
