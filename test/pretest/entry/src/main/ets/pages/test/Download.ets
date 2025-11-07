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

import router from '@ohos.router';
import common from '@ohos.app.ability.common';
import wifiManager from '@ohos.wifiManager';
import commonEvent from '@ohos.commonEventManager';
import { BusinessError, request } from '@kit.BasicServicesKit';
PersistentStorage.persistProp('testId', '')


@Entry
@Component
struct download {
  @State message: string = '';
  @State prgValue: number = 1;
  @State prgTotal: number = 100;
  @State taskState: string = '未开始';
  @State timeCard: string = "0";
  @State mode: request.agent.Mode = request.agent.Mode.FOREGROUND;
  @State method: string = 'GET';
  @State overwrite: boolean = true;
  @State retry: boolean = true;
  @State priority: number = 0;
  @State network: request.agent.Network = request.agent.Network.ANY
  controller: TextAreaController = new TextAreaController();
  private context = getContext(this) as common.UIAbilityContext;
  private task: undefined | request.agent.Task;
  @State tid: string = '';
  @State URL_30M: string = 'weekly.tar.gz';
  @State URL_411M: string = 'dayu200_img.tar.gz';
  @State Url_400k: string = 'test400K.txt';
  @State Url_1G: string = '1815-L2-SDK-MAC-M1-PUBLIC.tar.gz';
  @State config: request.agent.Config = {
    action: request.agent.Action.DOWNLOAD,
    url: 'weekly.tar.gz',
    title: 'intake_test',
    description: 'intake_test',
    saveas: './',
    mode: this.mode,
    overwrite: this.overwrite,
    retry: this.retry,
    priority: this.priority,
    token: undefined,
    network: this.network,
    gauge:true
  };
  @State taskLists: Array<request.agent.Task> = [];
  @StorageLink('testId') testId: string = ''

  build() {
    Row() {
      Column() {
        Flex({ direction: FlexDirection.Column, alignItems: ItemAlign.Center }) {
          Row() {
            Progress({ value: this.prgValue, total: this.prgTotal, type: ProgressType.Linear })
              .color(Color.Orange)
          }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

          //url
          Row() {
            Text('Size:').width('25%')
            Row() {
              Radio({ value: '400k', group: 'Size' })
                .radioStyle({
                  checkedBackgroundColor: Color.Blue
                })
                .height(20)
                .checked(false)
                .onChange((isChecked: boolean) => {
                  if (isChecked === true) {
                    this.config.url = this.Url_400k;
                  }
                })
              Text('400k')
            }.width('25%')

            Row() {
              Radio({ value: '30M', group: 'Size' })
                .radioStyle({
                  checkedBackgroundColor: Color.Blue
                })
                .height(20)
                .checked(true)
                .onChange((isChecked: boolean) => {
                  if (isChecked === true) {
                    this.config.url = this.URL_30M;
                  }
                })
              Text('30M')
            }.width('25%')

            Row() {
              Radio({ value: '411M', group: 'Size' })
                .checked(false)
                .radioStyle({
                  checkedBackgroundColor: Color.Blue
                })
                .height(20)
                .onChange((isChecked: boolean) => {
                  if (isChecked === true) {
                    this.config.url = this.URL_411M;
                  }
                })
              Text('411M')
            }.width('25%')
          }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

          //mode
          Row() {
            Text('Mode:').width('25%')
            Row() {
              Radio({ value: 'ForeGround', group: 'ModeGroup' })
                .radioStyle({
                  checkedBackgroundColor: Color.Blue
                })
                .height(20)
                .checked(true)
                .onChange((isChecked: boolean) => {
                  if (isChecked === true) {
                    this.config.mode = request.agent.Mode.FOREGROUND;
                  }
                })
              Text('ForeGround')
            }.width('30%')

            Row() {
              Radio({ value: 'BackGround', group: 'ModeGroup' })
                .checked(false)
                .radioStyle({
                  checkedBackgroundColor: Color.Blue
                })
                .height(20)
                .onChange((isChecked: boolean) => {
                  if (isChecked === true) {
                    this.config.mode = request.agent.Mode.BACKGROUND
                  }
                })
              Text('BackGround')
            }.width('30%')
          }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

          //method
          Row() {
            Text('method:').width('25%')
            Row() {
              Radio({ value: 'GET', group: 'method' })
                .checked(false)
                .radioStyle({
                  checkedBackgroundColor: Color.Blue
                })
                .height(20)
                .checked(true)
                .onChange((isChecked: boolean) => {
                  if (isChecked === true) {
                    this.config.method = 'GET'
                  }
                })
              Text('GET')
            }.width('30%')

            Row() {
              Radio({ value: 'POST', group: 'method' })
                .checked(false)
                .radioStyle({
                  checkedBackgroundColor: Color.Blue
                })
                .height(20)
                .onChange((isChecked: boolean) => {
                  if (isChecked === true) {
                    this.config.method = 'POST'
                  }
                })
              Text('POST')
            }.width('30%')
          }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

          //token
          Row() {
            Text('Token:').width('25%')
            Row() {
              Radio({ value: 'null', group: 'Token' })
                .checked(false)
                .radioStyle({
                  checkedBackgroundColor: Color.Blue
                })
                .height(20)
                .checked(true)
                .onChange((isChecked: boolean) => {
                  if (isChecked === true) {
                    this.config.token = undefined
                  }
                })
              Text('无')
            }.width('30%')

            Row() {
              Radio({ value: 'token888', group: 'Token' }).checked(false)
                .radioStyle({
                  checkedBackgroundColor: Color.Blue
                })
                .height(20)
                .onChange((isChecked: boolean) => {
                  if (isChecked === true) {
                    this.config.token = 'token888'
                  }
                })
              Text('有')
            }.width('30%')
          }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

          //overwrite
          Row() {
            Text('OverWrite:').width('25%')
            Row() {
              Radio({ value: 'True', group: 'OverWrite' })
                .checked(false)
                .radioStyle({
                  checkedBackgroundColor: Color.Blue
                })
                .height(20)
                .checked(this.overwrite)
                .onChange((isChecked: boolean) => {
                  if (isChecked === true) {
                    this.config.overwrite = true
                  }
                })
              Text('True')
            }.width('30%')

            Row() {
              Radio({ value: 'False', group: 'OverWrite' })
                .checked(false)
                .radioStyle({
                  checkedBackgroundColor: Color.Blue
                })
                .height(20)
                .checked(!this.overwrite)
                .onChange((isChecked: boolean) => {
                  if (isChecked === true) {
                    this.config.overwrite = false
                  }
                })
              Text('False')
            }.width('30%')
          }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

          //retry
          Row() {
            Text('retry:').width('25%')
            Row() {
              Radio({ value: 'True', group: 'retry' })
                .checked(false)
                .radioStyle({
                  checkedBackgroundColor: Color.Blue
                })
                .height(20)
                .checked(this.retry)
                .onChange((isChecked: boolean) => {
                  if (isChecked === true) {
                    this.config.retry = true
                  }
                })
              Text('True')
            }.width('30%')

            Row() {
              Radio({ value: 'False', group: 'retry' })
                .checked(false)
                .radioStyle({
                  checkedBackgroundColor: Color.Blue
                })
                .height(20)
                .checked(!this.retry)
                .onChange((isChecked: boolean) => {
                  if (isChecked === true) {
                    this.config.retry = false
                  }
                })
              Text('False')
            }.width('30%')
          }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

          //network
          Row() {
            Text('network:').width('25%')
            Row() {
              Radio({ value: 'any', group: 'network' })
                .checked(false)
                .radioStyle({
                  checkedBackgroundColor: Color.Blue
                })
                .height(20)
                .checked(this.retry)
                .onChange((isChecked: boolean) => {
                  if (isChecked === true) {
                    this.config.network = request.agent.Network.ANY
                  }
                })
              Text('any')
            }.width('30%')

            Row() {
              Radio({ value: 'WIFI', group: 'network' })
                .checked(false)
                .radioStyle({
                  checkedBackgroundColor: Color.Blue
                })
                .height(20)
                .checked(!this.retry)
                .onChange((isChecked: boolean) => {
                  if (isChecked === true) {
                    this.config.network = request.agent.Network.WIFI
                  }
                })
              Text('WIFI')
            }.width('30%')

            Row() {
              Radio({ value: '蜂窝', group: 'network' })
                .checked(false)
                .radioStyle({
                  checkedBackgroundColor: Color.Blue
                })
                .height(20)
                .checked(!this.retry)
                .onChange((isChecked: boolean) => {
                  if (isChecked === true) {
                    this.config.network = request.agent.Network.CELLULAR
                  }
                })
              Text('蜂窝')
            }.width('30%')
          }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

          //priority
          Row() {
            Text('priority:').width('25%')
            Slider({
              value: this.priority,
              min: 0,
              max: 100,
              style: SliderStyle.OutSet
            })
              .showTips(true)
              .onChange((value: number, mode: SliderChangeMode) => {
                this.config.priority = value
                console.info('value:' + value + 'mode:' + mode.toString())
              })
              .width('65%')
            // toFixed(0)将滑动条返回值处理为整数精度
            Text(this.priority.toFixed(0)).fontSize(20)
          }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

          Row() {
            Button('创建')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                await this.createTask();
              })

            Button('开始')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                await this.startTask();
              })

            Button('暂停')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                await this.pauseTask()
              })
          }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

          Row() {
            Button('恢复')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                await this.resumeTask();
              })
            Button('停止')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                await this.stopTask();
              })
            Button('删除')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                await this.removeTask();
              })
          }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

          Row() {
            Button('show')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                try {
                  if (this.task != undefined) {
                    await this.showTask(this.task.tid)
                  } else {
                    this.taskState = '任务为空'
                  }
                } catch (err) {
                  console.info('====> catch err:' + JSON.stringify(err))
                }
              })

            Button('touch')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                try {
                  if (this.task != undefined) {
                    await this.touchTask(this.task.tid, "token888")
                  } else {
                    this.taskState = '任务为空'
                  }
                } catch (err) {
                  console.info('====>touch catch err:' + JSON.stringify(err))
                }
              })

            Button('search')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                await this.searchTask()
              })

            Button('query')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                try {
                  if (this.task != undefined) {
                    await this.queryTask(this.task.tid)
                  } else {
                    this.taskState = '任务为空'
                  }
                } catch (err) {
                  console.info('====>query catch err:' + JSON.stringify(err))
                }
              })
          }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

          Row() {
            Button('断开网络')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                this.disableWifi();
              })

            Button('恢复网络')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                this.enableWifi();
              })

            Text(this.timeCard).fontSize(20)
          }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

          Row() {
            Button('100前')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                await this.createTasksFore();
              })
            Button('100后')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                await this.createTasksBack();
              })
            Button('开始')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                await this.tasksStart();
              })
            Button('删除')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                await this.removeTasks();
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
            Button('挂回调')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                await this.taskOnCallback();
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
            Button('下文件')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                await this.downloadSaveasErr();
              })
            Button('test')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                await this.testTask();
              })
          }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

          TextArea({
            text: this.message,
            placeholder: '默认全量订阅',
            controller: this.controller
          })
            .placeholderColor(Color.Grey)
            .placeholderFont({ size: 14, weight: 400 })
            .caretColor(Color.Blue)
            .width(400)
            .height('70%')
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

  responseCallback = (response: request.agent.HttpResponse) => {
    this.message += `====>response.statusCode: ${response.statusCode}\n`
    this.message += `====>response.headers.get('content-type'): ${response.headers.get('content-type')}\n`
    response.headers.forEach((v, k) => {
      console.info(`====> key:${k}   value:${v}`)
    })
  }
  progressCallback = (progress: request.agent.Progress) => {
    this.prgValue = progress.processed;
    this.prgTotal = progress.sizes[0]
    console.info('====>on_progress_callback:' + JSON.stringify(progress))
    this.message += `===>下载progress size:${this.prgValue} ->total:${this.prgTotal} \n`
  }
  completedCallback = (progress: request.agent.Progress) => {
    console.info('====>on_COMPLETE_callback:' + JSON.stringify(progress))
    this.message += `====>完成completed progress.state: ${progress.state}\n`
    request.agent.remove(this.task?.tid)
  }
  failedCallback = (progress: request.agent.Progress) => {
    console.info('====>on_FAILED_callback:' + JSON.stringify(progress))
    this.message += `====>失败failed progress.state: ${progress.state}\n`
  }
  pauseCallback = (progress: request.agent.Progress) => {
    console.info('====>on_PAUSE_callback:' + JSON.stringify(progress))
    this.message += `====>暂停pause progress.state: ${progress.state}\n`
  }
  resumeCallback = (progress: request.agent.Progress) => {
    console.info('====>on_RESUME_callback:' + JSON.stringify(progress))
    this.message += `====>重启resume progress.state: ${progress.state}\n`
  }
  removeCallback = (progress: request.agent.Progress) => {
    console.info('====>on_REMOVE_callback:' + JSON.stringify(progress))
    this.message += `====>删除remove progress.state: ${progress.state}\n`
  }

  async subscribe() {
    let subscriberCallback = (err: BusinessError, data: commonEvent.CommonEventData) => {
      try {
        console.info("====>subscriberCallback data:" + JSON.stringify(data));
        commonEvent.unsubscribe(subscriber, unSubscriberCallback);
        console.info("====>data.code: " + data.code);
        this.message += `====>BroadcastEvent.COMPLETE data.code: ${data.code} \n`
      } catch (err) {
        console.info("====>subscriberCallback error: " + JSON.stringify(err));
        this.message += `====>subscriberCallback error: ${err.message} \n`
      }
    }
    let unSubscriberCallback = (err: BusinessError) => {
      console.info("====>unSubscriberCallback start");
      if (err) {
        console.info("====>unSubscriberCallback failed:" +
        JSON.stringify(err));
      } else {
        console.info("====>unSubscriberCallback finish");
      }
    }
    let commonEventSubscribeInfo: commonEvent.CommonEventSubscribeInfo = {
      events: [request.agent.BroadcastEvent.COMPLETE]
    }
    let subscriber: commonEvent.CommonEventSubscriber = await commonEvent.createSubscriber(commonEventSubscribeInfo);
    await commonEvent.subscribe(subscriber, subscriberCallback);
  }

  async createTask() {
    try {
      this.config.saveas = `./on_response.txt`
      console.info('====>task.config:' + JSON.stringify(this.config))
      this.task = await request.agent.create(this.context, this.config)

      this.taskLists.push(this.task)
      this.task.on('completed', this.completedCallback)
      this.task.on('resume', this.resumeCallback)
      this.task.on('response', this.responseCallback)
      this.task.on('progress', this.progressCallback)
      this.task.on('failed', this.failedCallback)
      this.task.on('pause', this.pauseCallback)
      this.task.on('remove', this.removeCallback)
      await this.subscribe();
      this.message += `====>创建tid:${this.task.tid}！create task success!\n`
    } catch (err) {
      this.message += `====>create task err: ${err.message}\n`
    }
  }

  async createTasksFore() {
    let count = 0;
    let fa = 0;
    let failedCall = (progress: request.agent.Progress) => {
      fa++;
      this.message += `====>失败failed progress.state: ${progress.state}---${fa}\n`
    }
    try {
      this.config.url = this.URL_30M;
      this.config.mode = request.agent.Mode.FOREGROUND;
      for (let index = 0; index < 100; index++) {
        try {
          this.config.saveas = `./on_${index}.txt`
          let task: request.agent.Task = await request.agent.create(getContext(), this.config);
          this.taskLists.push(task);
          task.on('failed', failedCall);
          count++;
          this.message += `====>创建${index} ${task.tid}！create task success!\n`
        } catch (err) {
          this.message += `====>create ${index} task err: ${err.message}\n`
        }
      }
    } catch (err) {
      this.message += `==>create err!:${err.message} \n`
    }
    console.info("===>create task num is :" + count)
  }

  async createTasksBack() {
    let count = 0;
    let fa = 0;
    let failedCall = (progress: request.agent.Progress) => {
      fa++;
      this.message += `====>失败failed progress.state: ${progress.state}---${fa}\n`
    }
    try {
      this.config.url = this.Url_400k;
      this.config.mode = request.agent.Mode.BACKGROUND;
      for (let index = 0; index < 100; index++) {
        try {
          this.config.saveas = `./on_${index}.txt`
          let task: request.agent.Task = await request.agent.create(getContext(), this.config);
          this.taskLists.push(task);
          task.on('failed', failedCall);
          count++;
          this.message += `====>创建${index} ${task.tid}！create task success!\n`
        } catch (err) {
          this.message += `====>create ${index} task err: ${err.message}\n`
        }
      }
    } catch (err) {
      this.message += `==>create err!:${err.message} \n`
    }
    console.info("===>create task num is :" + count)
  }

  async tasksStart() {
    for (let i = 0; i < this.taskLists.length; i++) {
      let task = this.taskLists[i];
      try {
        await task.start();
        this.message += `===>start task${i}\n`;
      } catch (err) {
        console.error(`/Request start fail, Code: ${err.code}, message: ${err.message}`);
      }
    }
    console.info('===>start 500')
  }

  async removeTasks() {
    try {
      for (let index = 0; index < this.taskLists.length; index++) {
        try {
          this.removeIt(this.taskLists[index].tid)
        } catch (err) {
          this.message += `====>remove ${index} task err: ${err.message}\n`
        }
      }
    } catch (err) {
      this.message += `====>remove task err: ${err.message}\n`
    }
  }

  async removeIt(tid: string) {
    try {
      await request.agent.remove(tid)
      this.message += `====>删除tid: ${tid}！remove task success!\n`
    } catch (err) {
      this.message += `====>remove task err: ${err.message}\n`
    }
  }

  async startTask() {
    try {
      if (this.task != undefined) {
        console.info('====>启动！task.config:' + JSON.stringify(this.config))
        await this.task.start()
        this.message += `====>启动tid:${this.task.tid}！start task success!\n`;
      } else {
        this.taskState = '任务为空'
      }
    } catch (err) {
      this.message += `====>start task err: ${err.message}\n`
    }
  }

  async pauseTask() {
    try {
      if (this.task != undefined) {
        await this.task.pause()
        this.message += `====>暂停tid:${this.task.tid}！pause task success!\n`;
      } else {
        this.taskState = '暂停失败'
      }
    } catch (err) {
      this.message += `====>pause task err: ${err.message}\n`
    }
  }

  async resumeTask() {
    try {
      if (this.task != undefined) {
        await this.task.resume()
        this.message += `====>恢复tid:${this.task.tid}！resume task success!\n`;
      } else {
        this.taskState = '恢复失败'
      }
    } catch (err) {
      this.message += `====>resume task err: ${err.message}\n`
    }
  }

  async stopTask() {
    try {
      if (this.task != undefined) {
        await this.task.stop()
        this.message += `====>停止tid:${this.task.tid}！stop task success!\n`;
      } else {
        this.taskState = '停止失败'
      }
    } catch (err) {
      this.message += `====>stop task err: ${err.message}\n`
    }
  }

  async removeTask() {
    try {
      if (this.task != undefined) {
        await request.agent.remove(this.task.tid)
        this.message += `====>删除tid: ${this.task.tid}！remove task success!\n`;
      } else {
        this.taskState = '任务为空'
        this.message += `===>no task!`
      }
    } catch (err) {
      this.message += `====>remove task err: ${err.message}\n`
    }
  }

  async showTask(taskId: string) {
    try {
      let taskInfo = await request.agent.show(taskId)

      let testId = taskId;
      console.info('testId' + JSON.stringify(testId))
      AppStorage.setOrCreate('testId',testId)

      console.info('====>showTaskInfo:' + JSON.stringify(taskInfo))
      this.message += `====>show${taskId}！show task success!\n`;

      this.message += `====>taskInfo.ctime: ${taskInfo.ctime.toString()}, state: ${taskInfo.progress.state}, reason: ${taskInfo.reason}\n`;
    } catch (err) {
      this.message += `====>show task err: ${err.message}\n`
    }
  }

  async touchTask(taskId: string, token: string) {
    try {
      let taskInfo = await request.agent.touch(taskId, token)
      console.info('====>touchTaskInfo:' + JSON.stringify(taskInfo))
      this.message += `====>touch${taskId}！touch task success!\n`;
      this.message += `====>touch tid${taskId} taskInfo.mtime: ${taskInfo.mtime.toString()}\n`
    } catch (err) {
      this.message += `====>touch task err: ${err.message}\n`
    }
  }

  async searchTask() {
    try {
      let taskList = await request.agent.search()
      console.info('====>searchTaskList:' + JSON.stringify(taskList))
      this.message += `====>search all task success!\n`;
      this.message += JSON.stringify(taskList) + '\n'
      console.info('====>Length:' + taskList.length)
      this.message += `len:` + taskList.length + '\n'
    } catch (err) {
      this.message += `====>search task err: ${err.message}\n`
    }
  }

  async queryTask(taskId: string) {
    try {
      let task_info = await request.agent.query(taskId)
      console.info('====>searchTaskList:' + JSON.stringify(task_info));
      this.message += `====>query task uid: ${JSON.stringify(task_info.uid)}\n`
      this.message += `====>query task bundle: ${JSON.stringify(task_info.bundle)}\n`
      this.message += `====>query task success!\n`;
    } catch (err) {
      this.message += `====>search task err: ${err.message}\n`
    }
  }

  async enableWifi() {
    try {
      wifiManager.enableWifi();
      this.timeCard = '服务是否启动';
    } catch (err) {
      this.message += `====>enableWifi err: ${err.message}\n`;
    }
  }

  async disableWifi() {
    try {
      wifiManager.disableWifi();
      let count = 0;
      let t = setInterval(() => {
        count += 1
        this.timeCard = count.toString()
        if (count >= 65) {
          this.timeCard = '服务是否退出'
          clearInterval(t)
        }
      }, 1000)
    } catch (err) {
      this.message += `====>disableWifi err: ${err.message}\n`;
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

  async getTaskInfo(tid: string) {
    try {
      this.testId = AppStorage.get('testId') + ''
      console.info('testId' + JSON.stringify(this.testId))

      let taskGet = await request.agent.getTask(this.context, this.testId);
      this.task = taskGet;
      console.info('===>Succeeded get---config:' + JSON.stringify(taskGet.config))
      this.message += `>task ${tid}！get_task info success \n`
    } catch (err) {
      this.message += `====>search task ${tid} err: ${err.message}\n`
    }
  }

  async taskOnCallback() {
    try {
      if (this.task != undefined) {
        this.task.on('completed', this.completedCallback)
        this.task.on('resume', this.resumeCallback)
        this.task.on('response', this.responseCallback)
        this.task.on('progress', this.progressCallback)
        this.task.on('failed', this.failedCallback)
        this.task.on('pause', this.pauseCallback)
        this.task.on('remove', this.removeCallback)
        this.message += `task ${this.task.tid} get on success \n`
        console.info('===>task' + this.task.tid + 'has on')
      } else {
        this.taskState = '任务为空'
        this.message += `===>no task!`
      }
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
      this.message += `===>task=>total_len:` + len + '\n'

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
      this.message += `===>task=>total_len:` + len + '\n'

    } catch (err) {
      this.message += `====>search task err: ${err.message}\n`
    }
  }

  async downloadSaveasErr() {
    let config: request.agent.Config = {
      action: request.agent.Action.DOWNLOAD,
      url: this.URL_30M,
      title: 'test_folder',
      description: 'test_folder',
      saveas: './down',
      mode: this.mode,
      overwrite: this.overwrite,
      retry: this.retry,
      priority: this.priority,
      token: undefined,
      network: this.network
    }
    try {
      this.task = await request.agent.create(this.context, config);
      console.info('==Succeeded get---config:' + JSON.stringify(this.task.config));

      this.task.on('resume', this.resumeCallback)
      this.task.on('response', this.responseCallback)
      this.task.on('progress', this.progressCallback)
      this.task.on('failed', this.failedCallback)
      this.task.on('pause', this.pauseCallback)
      this.task.on('remove', this.removeCallback)
      await this.subscribe();
      this.message += `====>创建！create task:${this.task.tid} success!\n`

      this.task.start();
      console.info('===>task start !')

    } catch (err) {
      this.message += `====>create task err: ${err.message}\n`
    }
  }

  async testTask() {
    try {
      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        title: 'intake_test',
        description: 'intake_test',
        saveas: '/data',
        mode: this.mode,
        overwrite: this.overwrite,
        retry: this.retry,
        priority: this.priority,
        token: undefined,
        network: this.network
      }
      // this.task = await request.agent.create(this.context, config);
      // console.info('==Succeeded get---config:' + JSON.stringify(this.task.config));
      // this.task.on('response', this.responseCallback)
      // await this.task.start();
      request.agent.create(getContext(),config, (err:BusinessError,task:request.agent.Task)=>{
        task.start();
        request.agent.remove(task.tid);
      })


    } catch (err) {
      this.message += `====>test task err: ${err.message}\n`
    }
  }

}