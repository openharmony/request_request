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
import request from '@ohos.request';
import common from '@ohos.app.ability.common';
import taskpool from '@ohos.taskpool';
import systemDateTime from '@ohos.systemDateTime';

let context = getContext(this) as common.UIAbilityContext;

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
  @State downloadUrl: string = 'http://192.168.3.2:35620';
  @State retry: boolean = true;
  @State priority: number = 0;
  @State startTime: number = 0;
  @State endTime: number = 0;
  controller: TextAreaController = new TextAreaController();
  private context = getContext(this) as common.UIAbilityContext;
  private task: undefined|request.agent.Task;
  taskGroup: taskpool.TaskGroup = new taskpool.TaskGroup();
  @State config: request.agent.Config = {
    action: request.agent.Action.DOWNLOAD,
    url: 'http://192.168.232.162:35620',
    title: 'taskpool_test',
    description: 'taskpool_test',
    saveas: './',
    mode: request.agent.Mode.FOREGROUND,
    overwrite: true,
    retry: true,
  };

  build() {
    Row() {
      Column() {
        Flex({ direction: FlexDirection.Column, alignItems: ItemAlign.Center}){
          Row(){
            TextInput({ text: this.downloadUrl, placeholder: 'input your word...', controller: this.controller })
              .placeholderColor(Color.Grey)
              .placeholderFont({ size: 14, weight: 400 })
              .caretColor(Color.Blue)
              .width('95%')
              .height(40)
              .fontSize(14)
              .fontColor(Color.Black)
              .onChange((value: string) => {
                this.config.url = value
              })
          }

          Row(){
            Button('创建并发任务')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async ()=>{
                this.taskAdd();
              })

            Button('开始并发任务')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async ()=>{
                this.startTime = systemDateTime.getTime();
                this.message += `====>任务开始时间：${this.startTime} (ms)\n`
                this.TaskGroupExec();
              })
          }.margin({ top: '1%'}).width('100%').align(Alignment.Center)

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
            .margin({top:'5%'})
        }
      }
      .width('100%')
    }
    .height('100%')
  }

  responseCallback = (response: request.agent.HttpResponse)=>{
    response.headers.forEach((value, key) => {
      console.info(`====> key:${key}   value:${value}`)
    });
    this.message += `====>response.statusCode: ${response.statusCode}\n`
    this.message += `====>response.headers.get('content-type'): ${response.headers.get('content-type')}\n`

  }

  progressCallback = (progress: request.agent.Progress) => {
    this.prgValue = progress.processed;
    this.prgTotal = progress.sizes[0]
    console.info('====>on_progress progress.processed:' + progress.processed)
  }

  completedCallback = (progress: request.agent.Progress) => {
    console.info('====>on_complete_callback:' + JSON.stringify(progress))
    this.message += `====>completed progress.state: ${progress.state}\n`
  }

  failedCallback = (progress: request.agent.Progress) => {
    console.info('====>on_failed_callback:' +JSON.stringify(progress))
    this.message += `====>failed progress.state: ${progress.state}\n`
  }

  pauseCallback = (progress: request.agent.Progress) => {
    console.info('====>on_pause_callback:' +JSON.stringify(progress))
    this.message += `====>pause progress.state: ${progress.state}\n`
  }

  removeCallback = (progress: request.agent.Progress) => {
    console.info('====>on_remove_callback:' +JSON.stringify(progress))
    this.message += `====>remove progress.state: ${progress.state}\n`
  }

  async taskAdd(): Promise<void> {
    try {
      let task1: taskpool.Task = new taskpool.Task(createTask, this.context, 1, this.downloadUrl);
      this.taskGroup.addTask(task1);

      let task2: taskpool.Task = new taskpool.Task(createTask, this.context, 2, this.downloadUrl);
      this.taskGroup.addTask(task2);

      let task3: taskpool.Task = new taskpool.Task(createTask, this.context, 3, this.downloadUrl);
      this.taskGroup.addTask(task3);

      let task4: taskpool.Task = new taskpool.Task(createTask, this.context, 4, this.downloadUrl);
      this.taskGroup.addTask(task4);

      let task5: taskpool.Task = new taskpool.Task(createTask, this.context, 5, this.downloadUrl);
      this.taskGroup.addTask(task5);
    } catch (err) {
      console.info('====>taskAdd throw err:' + JSON.stringify(err.message))
    }
  }

  async TaskGroupExec(): Promise<void> {
    try {
      let res: string[] = await taskpool.execute(this.taskGroup) as string[];
      this.endTime = systemDateTime.getTime();
      this.message += `====>任务结束时间：${this.endTime} (ms)\n`
      this.message += `====>任务用时：${this.endTime- this.startTime} (ms)\n`
    } catch(err) {
      console.info('====>TaskGroupExec throw err:' + JSON.stringify(err.message))
    }
  }

}

@Concurrent
async function createTask(context: common.Context, index: number, downloadUrl: string) {
  let downloadFilePath = context.cacheDir
  try{
    let downloadConfig: request.DownloadConfig = {
      url: downloadUrl,
      header: {
        headers: 'http'
      },
      enableMetered: false,
      enableRoaming: false,
      description: 'XTS download test!',
      networkType: request.NETWORK_WIFI,
      filePath: `${downloadFilePath}/`,
      title: 'XTS download test!',
      background: false
    }
    for(let task_index=1; task_index<=240; task_index++){
      downloadConfig.filePath = `${downloadFilePath}/taskpool_${index}_${task_index}.txt`
      console.info('====>task.config:' + JSON.stringify(downloadConfig))
      let task  = await request.downloadFile(context, downloadConfig)
    }

  }catch(err){
    console.info('====>createTask catch err:' + JSON.stringify(err.message))
  }
}