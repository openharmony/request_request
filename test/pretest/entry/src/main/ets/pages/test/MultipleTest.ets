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

import request from '@ohos.request';
import taskpool from '@ohos.taskpool';
import { BusinessError } from '@ohos.base';
import featureAbility from '@ohos.ability.featureAbility'
import fileio from '@ohos.fileio';
import fs from '@ohos.file.fs'

@Entry
@Component
struct Uploadpage {
  @State value: string = "";
  @State n: number = 1;
  @State progress: string = '0%';
  @State Url_400k: string = 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt';
  @State task_num: number = 1000;

  build() {
    Row() {
      Column() {

        Text(this.progress)
          .fontSize(14)
          .fontWeight(FontWeight.Bold)
          .margin({
            left: 20
          })
        Button('tests callback')
          .width('50%')
          .margin({ top: 20 })
          .height(40)
          .onClick((event) => {
            this.test_callback();
          })
        Button('tests promise')
          .width('50%')
          .margin({ top: 20 })
          .height(40)
          .onClick((event) => {
            this.test_promise();
          })
        Button('search')
          .width('50%')
          .margin({ top: 20 })
          .height(40)
          .onClick((event) => {
            this.request1();
          })
      }
      .width('100%')
    }

    .height('100%')
  }

  completedCallback = (pro: request.agent.Progress) => {
    this.n++;
    this.progress = `${this.n / this.task_num * 100} %`;
  }

  async test_callback() {
    let config: request.agent.Config = {
      action: request.agent.Action.DOWNLOAD,
      url: this.Url_400k,
      gauge: true,
      overwrite: true,
      network: request.agent.Network.WIFI,
      mode: request.agent.Mode.FOREGROUND,
      retry: true,
    };
    let count = 0;
    this.n = 0;
    for (let i = 0; i < this.task_num; i++) {
      let t = 0;
      let p = 0;
      config.saveas = `./${i}`;
      request.agent.create(getContext(), config, (err: BusinessError, task: request.agent.Task) => {
        if (err) {
          console.error(`/Request to create a download task, Code: ${err.code}, message: ${err.message}`);
          return;
        }
        task.on('completed', this.completedCallback)
        task.start();
        count++;
      });
    }
    console.info("===>create task num is :" + count)
  }

  async test_promise() {
    let config: request.agent.Config = {
      action: request.agent.Action.DOWNLOAD,
      url: this.Url_400k,
      gauge: true,
      overwrite: true,
      network: request.agent.Network.WIFI,
      mode: request.agent.Mode.FOREGROUND,
      retry: true,
    };
    let count = 0;
    this.n = 0;
    for (let i = 0; i < this.task_num; i++) {
      let t = 0;
      let p = 0;
      config.saveas = `./${i}`;
      let task: request.agent.Task = await request.agent.create(getContext(), config);
      try {
        task.on('completed', this.completedCallback)
        await task.start();
        count++;
      } catch (err) {
        console.error(`/Request start fail, Code: ${err.code}, message: ${err.message}`);
      }
    }
    console.info("===>create task num is :" + count)
  }

  async request1() {
    let filter: request.agent.Filter = {
      state: request.agent.State.COMPLETED,
    }
    request.agent.search(filter, (err: BusinessError, data: Array<string>) => {
      if (err) {
        console.error(`Failed to search a upload task, Code: ${err.code}, message: ${err.message}`);
        return;
      }
      console.error(`/Request COMPLETED num1: ${data.length}`);
    });
    let filter2: request.agent.Filter = {
      state: request.agent.State.RUNNING,
    }
    request.agent.search(filter2, (err: BusinessError, data: Array<string>) => {
      if (err) {
        console.error(`Failed to search a upload task, Code: ${err.code}, message: ${err.message}`);
        return;
      }
      console.error(`/Request RUNNING num2: ${data.length}`);
    });
  }
}