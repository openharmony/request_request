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

import common from '@ohos.app.ability.common';
import wifiManager from '@ohos.wifiManager';
import commonEvent from '@ohos.commonEventManager';
import { BusinessError, request, systemDateTime } from '@kit.BasicServicesKit';
import abilityAccessCtrl, { PermissionRequestResult, Permissions } from '@ohos.abilityAccessCtrl';
import bundleManager from '@ohos.bundle.bundleManager';
import fs from '@ohos.file.fs';
import photoAccessHelper from '@ohos.file.photoAccessHelper';
import picker from '@ohos.file.picker';

@Entry
@Component
struct feature50 {
  @State downloadNum: number = 0;
  @State message: string = "";
  @State url: string = 'http://192.168.175.242:8080/hfs/test.txt';
  controller: TextAreaController = new TextAreaController()
  @State Url_400k: string = 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt';
  @State taskLists: Array<request.agent.Task> = [];
  @State cost_time: number = 0;
  private context = getContext(this) as common.UIAbilityContext;
  @State tid: string = '';
  private task: undefined | request.agent.Task;

  build() {
    Row() {
      Column() {
        Row() {
          TextInput({ text: this.url, placeholder: 'input your word...', controller: this.controller })
            .placeholderColor(Color.Grey)
            .placeholderFont({ size: 14, weight: 400 })
            .caretColor(Color.Blue)
            .width('95%')
            .height(40)
            .fontSize(14)
            .fontColor(Color.Black)
            .onChange((value: string) => {
              this.url = value
              console.info(`====>${this.url}`)
            })
        }

        Flex({ direction: FlexDirection.Column, alignItems: ItemAlign.Center }) {
          Row() {
            Button('2kFore')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                this.downloadOutFore();
              })
            Button('1kBack')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                this.downloadOutBack();
              })
          }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

          Row() {
            Button('开始')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                this.tasksStart();
              })
            Button('暂停')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                this.tasksPause();
              })
            Button('恢复')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                this.tasksResume();
              })
            Button('停止')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                this.tasksStop();
              })
            Button('移除')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                this.tasksRemove();
              })
          }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

          Row() {
            Button('1fore')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                this.taskTimeFore();
              })
            Button('1back')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                this.taskTimeBack();
              })
            Button('查询')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                this.tasksSearch();
              })
          }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

          Row() {
            Button('移除')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                await this.searchRemoveTask();
              })
            Button('失败')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                await this.searchFailedTask();
              })
            Button('完成')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                await this.searchCompletedTask();
              })
            Button('停止')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                await this.searchStopTask();
              })
          }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

          Row() {
            Button('初始')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                await this.searchInitializedTask();
              })
            Button('暂停')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                await this.searchPauseTask();
              })
            Button('重试')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                await this.searchRetryTask();
              })
            Button('等待')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                await this.searchWaitingTask();
              })
            Button('运行')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                await this.searchRunningTask();
              })
          }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

          Row() {
            Button('前台')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                await this.searchTaskFore();
              })
            Button('前队')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                await this.searchTaskForeQueue();
              })
            Button('后台')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                await this.searchTaskBack();
              })
            Button('后队')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                await this.searchTaskBackQueue();
              })
          }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

          Row() {
            TextInput({ text: this.task?.tid, placeholder: 'input your word...', controller: this.controller })
              .placeholderColor(Color.Grey)
              .placeholderFont({ size: 14, weight: 400 })
              .caretColor(Color.Blue)
              .width('40%')
              .height(40)
              .fontSize(14)
              .fontColor(Color.Black)
              .onChange(async (value: string) => {
                this.tid = value;
              })
            Button('getTask')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                try {
                  await this.getTaskInfo(this.tid);
                } catch (err) {
                  console.info('====> catch err:' + JSON.stringify(err))
                }
              })
          }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

          TextArea({
            text: this.message,
            placeholder: 'console',
            controller: this.controller
          })
            .placeholderColor(Color.Grey)
            .placeholderFont({ size: 14, weight: 400 })
            .caretColor(Color.Blue)
            .width(400)
            .height('40%')
            .margin(20)
            .fontSize(14)
            .focusable(false)
            .fontColor(Color.Black)
            .onChange((value: string) => {
              this.message = value;
              this.controller.caretPosition(this.message.length)
            })
            .margin({ top: '5%' })
        }
      }
      .width('100%')
    }
    .height('100%')
  }

  completedCallback = (progress: request.agent.Progress) => {
    this.downloadNum += 1;
    console.info('====>on_COMPLETE_callback:' + JSON.stringify(progress))
    this.message += `====>完成completed progress.state: ${progress.state} -- ${this.downloadNum}\n`
  }

  failedCallback = (progress: request.agent.Progress) => {
    console.info('====>on_Failed_callback:' + JSON.stringify(progress))
    this.message += `====>失败failed progress.state: ${progress.state} \n`
  }

  async getTaskInfo(tid: string) {
    try {
      let taskGet = await request.agent.getTask(this.context, tid);
      console.info('===>Succeeded get---config:' + JSON.stringify(taskGet.config))
      let taskInfo = await request.agent.show(tid)
      console.info('===>task:' + tid + ' info:' + JSON.stringify(taskInfo))
      this.message += `>get task ${tid} info OK state ${taskInfo.progress.state}:${taskInfo.reason} \n`
    } catch (err) {
      this.message += `====>search task ${tid} err: ${err.message}\n`
    }
  }

  //前台2000后台1000任务限制
  async downloadOutFore() {
    let config: request.agent.Config = {
      action: request.agent.Action.DOWNLOAD,
      url: this.Url_400k,
      mode: request.agent.Mode.FOREGROUND,
      overwrite: true,
      retry: true,
      network: request.agent.Network.ANY
    };
    let count = 0;
    this.downloadNum = 0;
    this.message += `start create ${count} task \n`
    for (let i = 0; i < 2000; i++) {
      let t = 0;
      let p = 0;
      config.saveas = `./${i}`;
      try {
        let task: request.agent.Task = await request.agent.create(getContext(), config);
        task.on('completed', this.completedCallback)
        task.on('failed',this.failedCallback)
        this.taskLists.push(task)
        count++;
      } catch (err) {
        console.error(`===/Request start fail, Code: ${err.code}, message: ${err.message}`);
      }
    }
    console.info("===>create task num is :" + count)
    this.message += `create ${count} task done \n`
  }

  async downloadOutBack() {
    let config: request.agent.Config = {
      action: request.agent.Action.DOWNLOAD,
      url: this.Url_400k,
      mode: request.agent.Mode.BACKGROUND,
      overwrite: true,
      retry: true,
      network: request.agent.Network.ANY
    };
    let count = 0;
    this.downloadNum = 0;
    this.message += `start create ${count} task \n`
    for (let i = 0; i < 1000; i++) {
      let t = 0;
      let p = 0;
      config.saveas = `./${i}`;
      try {
        let task: request.agent.Task = await request.agent.create(getContext(), config);
        task.on('completed', this.completedCallback)
        task.on('failed',this.failedCallback)
        this.taskLists.push(task)
        count++;
      } catch (err) {
        console.error(`===/Request start fail, Code: ${err.code}, message: ${err.message}`);
      }
    }
    console.info("===>create task num is :" + count)
    this.message += `create ${count} task done \n`
  }

  async tasksStart() {
    for (let i = 0; i < this.taskLists.length; i++) {
      let task = this.taskLists[i];
      try {
        await task.start();
      } catch (err) {
        console.error(`===/Request start fail, Code: ${err.code}, message: ${err.message}`);
      }
    }
    console.info('===>start 2000')
  }

  async tasksRemove() {
    for (let i = 0; i < this.taskLists.length; i++) {
      let task = this.taskLists[i];
      try {
        await request.agent.remove(task.tid);
      } catch (err) {
        console.error(`===/Request remove fail, Code: ${err.code}, message: ${err.message}`);
      }
    }
    console.info('===>remove all task')
  }
  async tasksStop() {
    for (let i = 0; i < this.taskLists.length; i++) {
      let task = this.taskLists[i];
      try {
        await task.stop();
      } catch (err) {
        console.error(`===/Request stop fail, Code: ${err.code}, message: ${err.message}`);
      }
    }
    console.info('===>stop all task')
  }
  async tasksPause() {
    for (let i = 0; i < this.taskLists.length; i++) {
      let task = this.taskLists[i];
      try {
        await task.pause();
      } catch (err) {
        console.error(`===/Request pause fail, Code: ${err.code}, message: ${err.message}`);
      }
    }
    console.info('===>pause all task')
  }
  async tasksResume() {
    for (let i = 0; i < this.taskLists.length; i++) {
      let task = this.taskLists[i];
      try {
        await task.resume();
      } catch (err) {
        console.error(`===/Request resume fail, Code: ${err.code}, message: ${err.message}`);
      }
    }
    console.info('===>resume all task')
  }

  async tasksSearch() {
    try {
      let taskList = await request.agent.search()
      console.info('====>searchTaskList:' + JSON.stringify(taskList))
      console.info('====>Length:' + taskList.length)
      this.message += `任务总数len: ${taskList.length} \n`
    } catch (err) {
      this.message += `====>search task err: ${err.message} \n`
    }
  }

  async taskTimeFore() {
    let config: request.agent.Config = {
      action: request.agent.Action.DOWNLOAD,
      url: this.Url_400k,
      mode: request.agent.Mode.FOREGROUND,
      overwrite: true,
      retry: true,
      network: request.agent.Network.ANY
    };
    try {
      this.cost_time = systemDateTime.getTime();
      let task = await request.agent.create(getContext(), config);
      this.message += `===> task ${task.tid} create \n`
      task.on('completed', (progress: request.agent.Progress) => {
        let taskTime = systemDateTime.getTime() - this.cost_time;
        console.info('====>on_COMPLETE_callback:' + JSON.stringify(progress) +
          (taskTime))
        this.message += `====>完成completed progress.state: ${progress.state}\n`
        this.message += `===>耗时:${taskTime} \n`
      });
      let cti = systemDateTime.getTime() - this.cost_time;
      console.info('===>create time :' + cti);
      this.message += `===>create ${cti} \n`
      await task.start();
      let sti = systemDateTime.getTime() - this.cost_time;
      console.info('===>create time :' + sti);
      this.message += `===>create ${sti} \n`
    } catch (err) {
      console.error(`===/Request start fail, Code: ${err.code}, message: ${err.message}`);
      this.message += `start err ${err}\n`
    }
  }

  async taskTimeBack() {
    let config: request.agent.Config = {
      action: request.agent.Action.DOWNLOAD,
      url: this.Url_400k,
      mode: request.agent.Mode.BACKGROUND,
      overwrite: true,
      retry: true,
      network: request.agent.Network.ANY
    };
    try {
      this.cost_time = systemDateTime.getTime();
      let task = await request.agent.create(getContext(), config);
      this.message += `===> task ${task.tid} create \n`
      task.on('completed', (progress: request.agent.Progress) => {
        let taskTime = systemDateTime.getTime() - this.cost_time;
        console.info('====>on_COMPLETE_callback:' + JSON.stringify(progress) +
          (taskTime))
        this.message += `====>完成completed progress.state: ${progress.state}\n`
        this.message += `===>耗时:${taskTime} \n`
      });
      let cti = systemDateTime.getTime() - this.cost_time;
      console.info('===>create time :' + cti);
      this.message += `===>create ${cti} \n`
      await task.start();
      let sti = systemDateTime.getTime() - this.cost_time;
      console.info('===>create time :' + sti);
      this.message += `===>create ${sti} \n`
    } catch (err) {
      console.error(`===/Request start fail, Code: ${err.code}, message: ${err.message}`);
      this.message += `start err ${err}\n`
    }
  }

  async searchRemoveTask() {
    try {
      let filter: request.agent.Filter = {
        action: request.agent.Action.DOWNLOAD,
        state: request.agent.State.REMOVED
      }
      let taskList = await request.agent.search(filter)
      console.info('==>remove 已删除的==>list:' + JSON.stringify(taskList))
      this.message += `==>已删除的==>list:` + JSON.stringify(taskList) + `\n`
      console.info('====>Length:' + taskList.length)
      this.message += `len:` + taskList.length + '\n'
    } catch (err) {
      this.message += `====>search task err: ${err.message}\n`
    }
  }

  async searchCompletedTask() {
    try {
      let filter: request.agent.Filter = {
        action: request.agent.Action.DOWNLOAD,
        state: request.agent.State.COMPLETED
      }
      let taskList = await request.agent.search(filter)
      console.info('==>completed 完成的==>list:' + JSON.stringify(taskList))
      this.message += `==>完成的==>list:` + JSON.stringify(taskList) + `\n`
      console.info('====>Length:' + taskList.length)
      this.message += `len:` + taskList.length + '\n'
    } catch (err) {
      this.message += `====>search task err: ${err.message}\n`
    }
  }

  async searchFailedTask() {
    try {
      let filter: request.agent.Filter = {
        action: request.agent.Action.DOWNLOAD,
        state: request.agent.State.FAILED
      }
      let taskList = await request.agent.search(filter)
      console.info('==>failed 失败的==>list:' + JSON.stringify(taskList))
      this.message += `==>失败的==>list:` + JSON.stringify(taskList) + `\n`
      console.info('====>Length:' + taskList.length)
      this.message += `len:` + taskList.length + '\n'
    } catch (err) {
      this.message += `====>search task err: ${err.message}\n`
    }
  }

  async searchInitializedTask() {
    try {
      let filter: request.agent.Filter = {
        action: request.agent.Action.DOWNLOAD,
        state: request.agent.State.INITIALIZED
      }
      let taskList = await request.agent.search(filter)
      console.info('==>INITIALIZED 失败的==>list:' + JSON.stringify(taskList))
      this.message += `==>INITIALIZED的==>list:` + JSON.stringify(taskList) + `\n`
      console.info('====>Length:' + taskList.length)
      this.message += `len:` + taskList.length + '\n'
    } catch (err) {
      this.message += `====>search task err: ${err.message}\n`
    }
  }

  async searchRunningTask() {
    try {
      let filter: request.agent.Filter = {
        action: request.agent.Action.DOWNLOAD,
        state: request.agent.State.RUNNING
      }
      let taskList = await request.agent.search(filter)
      console.info('==>RUNNING 失败的==>list:' + JSON.stringify(taskList))
      this.message += `==>RUNNING的==>list:` + JSON.stringify(taskList) + `\n`
      console.info('====>Length:' + taskList.length)
      this.message += `len:` + taskList.length + '\n'
    } catch (err) {
      this.message += `====>search task err: ${err.message}\n`
    }
  }

  async searchPauseTask() {
    try {
      let filter: request.agent.Filter = {
        action: request.agent.Action.DOWNLOAD,
        state: request.agent.State.PAUSED
      }
      let taskList = await request.agent.search(filter)
      console.info('==>PAUSED 失败的==>list:' + JSON.stringify(taskList))
      this.message += `==>PAUSED的==>list:` + JSON.stringify(taskList) + `\n`
      console.info('====>Length:' + taskList.length)
      this.message += `len:` + taskList.length + '\n'
    } catch (err) {
      this.message += `====>search task err: ${err.message}\n`
    }
  }

  async searchStopTask() {
    try {
      let filter: request.agent.Filter = {
        action: request.agent.Action.DOWNLOAD,
        state: request.agent.State.STOPPED
      }
      let taskList = await request.agent.search(filter)
      console.info('==>STOPPED 失败的==>list:' + JSON.stringify(taskList))
      this.message += `==>STOPPED的==>list:` + JSON.stringify(taskList) + `\n`
      console.info('====>Length:' + taskList.length)
      this.message += `len:` + taskList.length + '\n'
    } catch (err) {
      this.message += `====>search task err: ${err.message}\n`
    }
  }

  async searchWaitingTask() {
    try {
      let filter: request.agent.Filter = {
        action: request.agent.Action.DOWNLOAD,
        state: request.agent.State.WAITING
      }
      let taskList = await request.agent.search(filter)
      console.info('==>WAITING 等待的==>list:' + JSON.stringify(taskList))
      this.message += `==>WAITING==>list:` + JSON.stringify(taskList) + `\n`
      console.info('====>Length:' + taskList.length)
      this.message += `len:` + taskList.length + '\n'
    } catch (err) {
      this.message += `====>search task err: ${err.message}\n`
    }
  }

  async searchRetryTask() {
    try {
      let filter: request.agent.Filter = {
        action: request.agent.Action.DOWNLOAD,
        state: request.agent.State.RETRYING
      }
      let taskList = await request.agent.search(filter)
      console.info('==>RETRYING 失败的==>list:' + JSON.stringify(taskList))
      this.message += `==>RETRYING的==>list:` + JSON.stringify(taskList) + `\n`
      console.info('====>Length:' + taskList.length)
      this.message += `len:` + taskList.length + '\n'
    } catch (err) {
      this.message += `====>search task err: ${err.message}\n`
    }
  }

  async searchTaskFore() {
    try {
      let filter: request.agent.Filter = {
        action: request.agent.Action.DOWNLOAD,
        mode: request.agent.Mode.FOREGROUND
      }
      let taskList = await request.agent.search(filter)
      console.info('==>FOREGROUND的==>list:' + JSON.stringify(taskList))
      this.message += `==>FOREGROUND==>list:` + JSON.stringify(taskList) + `\n`
      console.info('====>Length:' + taskList.length)
      this.message += `len:` + taskList.length + '\n'
    } catch (err) {
      this.message += `====>search task err: ${err.message}\n`
    }
  }

  async searchTaskForeQueue() {
    let len = 0;
    try {
      let filter1: request.agent.Filter = {
        action: request.agent.Action.DOWNLOAD,
        mode: request.agent.Mode.FOREGROUND,
        state: request.agent.State.INITIALIZED
      }
      let taskList = await request.agent.search(filter1)
      console.info('==>INITIALIZED的==>list:' + JSON.stringify(taskList))
      this.message += `==>INITIALIZED==>list:` + JSON.stringify(taskList) + `\n`
      console.info('====>Length:' + taskList.length)
      this.message += `len:` + taskList.length + '\n'
      len += taskList.length;

      let filter2: request.agent.Filter = {
        action: request.agent.Action.DOWNLOAD,
        mode: request.agent.Mode.FOREGROUND,
        state: request.agent.State.RUNNING
      }
      taskList = await request.agent.search(filter2)
      console.info('==>RUNNING的==>list:' + JSON.stringify(taskList))
      this.message += `==>RUNNING==>list:` + JSON.stringify(taskList) + `\n`
      console.info('====>Length:' + taskList.length)
      this.message += `len:` + taskList.length + '\n'
      len += taskList.length

      let filter3: request.agent.Filter = {
        action: request.agent.Action.DOWNLOAD,
        mode: request.agent.Mode.FOREGROUND,
        state: request.agent.State.WAITING
      }
      taskList = await request.agent.search(filter3)
      console.info('==>WAITING的==>list:' + JSON.stringify(taskList))
      this.message += `==>WAITING==>list:` + JSON.stringify(taskList) + `\n`
      console.info('====>Length:' + taskList.length)
      this.message += `len:` + taskList.length + '\n'
      len += taskList.length

      let filter4: request.agent.Filter = {
        action: request.agent.Action.DOWNLOAD,
        mode: request.agent.Mode.FOREGROUND,
        state: request.agent.State.PAUSED
      }
      taskList = await request.agent.search(filter4)
      console.info('==>PAUSED的==>list:' + JSON.stringify(taskList))
      this.message += `==>PAUSED==>list:` + JSON.stringify(taskList) + `\n`
      console.info('====>Length:' + taskList.length)
      this.message += `len:` + taskList.length + '\n'
      len += taskList.length

      let filter5: request.agent.Filter = {
        action: request.agent.Action.DOWNLOAD,
        mode: request.agent.Mode.FOREGROUND,
        state: request.agent.State.RETRYING
      }
      taskList = await request.agent.search(filter5)
      console.info('==>RETRYING的==>list:' + JSON.stringify(taskList))
      this.message += `==>RETRYING==>list:` + JSON.stringify(taskList) + `\n`
      console.info('====>Length:' + taskList.length)
      this.message += `len:` + taskList.length + '\n'
      len += taskList.length;
      console.info('====>total-Length:' + len)
      this.message += `===>task-fore=>total_len:` + len + '\n'

    } catch (err) {
      this.message += `====>search task err: ${err.message}\n`
    }
  }

  async searchTaskBack() {
    try {
      let filter: request.agent.Filter = {
        action: request.agent.Action.DOWNLOAD,
        mode: request.agent.Mode.BACKGROUND
      }
      let taskList = await request.agent.search(filter)
      console.info('==>BACKGROUND的==>list:' + JSON.stringify(taskList))
      this.message += `==>BACKGROUND==>list:` + JSON.stringify(taskList) + `\n`
      console.info('====>Length:' + taskList.length)
      this.message += `len:` + taskList.length + '\n'
    } catch (err) {
      this.message += `====>search task err: ${err.message}\n`
    }
  }

  async searchTaskBackQueue() {
    let len = 0;
    try {
      let filter1: request.agent.Filter = {
        action: request.agent.Action.DOWNLOAD,
        mode: request.agent.Mode.BACKGROUND,
        state: request.agent.State.INITIALIZED
      }
      let taskList = await request.agent.search(filter1)
      console.info('==>INITIALIZED的==>list:' + JSON.stringify(taskList))
      this.message += `==>INITIALIZED==>list:` + JSON.stringify(taskList) + `\n`
      console.info('====>Length:' + taskList.length)
      this.message += `len:` + taskList.length + '\n'
      len += taskList.length;

      let filter2: request.agent.Filter = {
        action: request.agent.Action.DOWNLOAD,
        mode: request.agent.Mode.BACKGROUND,
        state: request.agent.State.RUNNING
      }
      taskList = await request.agent.search(filter2)
      console.info('==>RUNNING的==>list:' + JSON.stringify(taskList))
      this.message += `==>RUNNING==>list:` + JSON.stringify(taskList) + `\n`
      console.info('====>Length:' + taskList.length)
      this.message += `len:` + taskList.length + '\n'
      len += taskList.length

      let filter3: request.agent.Filter = {
        action: request.agent.Action.DOWNLOAD,
        mode: request.agent.Mode.BACKGROUND,
        state: request.agent.State.WAITING
      }
      taskList = await request.agent.search(filter3)
      console.info('==>WAITING的==>list:' + JSON.stringify(taskList))
      this.message += `==>WAITING==>list:` + JSON.stringify(taskList) + `\n`
      console.info('====>Length:' + taskList.length)
      this.message += `len:` + taskList.length + '\n'
      len += taskList.length

      let filter4: request.agent.Filter = {
        action: request.agent.Action.DOWNLOAD,
        mode: request.agent.Mode.BACKGROUND,
        state: request.agent.State.PAUSED
      }
      taskList = await request.agent.search(filter4)
      console.info('==>PAUSED的==>list:' + JSON.stringify(taskList))
      this.message += `==>PAUSED==>list:` + JSON.stringify(taskList) + `\n`
      console.info('====>Length:' + taskList.length)
      this.message += `len:` + taskList.length + '\n'
      len += taskList.length

      let filter5: request.agent.Filter = {
        action: request.agent.Action.DOWNLOAD,
        mode: request.agent.Mode.BACKGROUND,
        state: request.agent.State.RETRYING
      }
      taskList = await request.agent.search(filter5)
      console.info('==>RETRYING的==>list:' + JSON.stringify(taskList))
      this.message += `==>RETRYING==>list:` + JSON.stringify(taskList) + `\n`
      console.info('====>Length:' + taskList.length)
      this.message += `len:` + taskList.length + '\n'
      len += taskList.length;
      console.info('====>total-Length:' + len)
      this.message += `===>task-back=>total_len:` + len + '\n'

    } catch (err) {
      this.message += `====>search task err: ${err.message}\n`
    }
  }
}