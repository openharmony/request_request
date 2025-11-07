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

import fileio from '@ohos.fileio';
import fs from '@ohos.file.fs';
import request from '@ohos.request';
import common from '@ohos.app.ability.common';
import wifiManager from '@ohos.wifiManager';
import { BusinessError } from '@kit.BasicServicesKit';


@Entry
@Component
struct upload {
  @State message: string = '';
  private uploadTask: undefined | request.UploadTask;
  @State upLoadUrl: string = '';
  @State files: Array<request.File> = [];
  @State method: string = 'POST';
  @State data: Array<request.RequestData> = [];
  private context = getContext(this) as common.UIAbilityContext;
  controller: TextAreaController = new TextAreaController()
  @State uploadConfig: request.UploadConfig = {
    url: this.upLoadUrl,
    header: { 'Accept': '*/*' },
    method: this.method,
    files: this.files,
    data: this.data,
  }
  @State token: boolean = false;
  @State multiple: boolean = false;
  @State retry: boolean = true;
  @State priority: number = 0;
  @State timeCard: string = "0";

  build() {
    Row() {
      Column() {
        Row() {
          TextInput({ text: this.upLoadUrl, placeholder: 'input your word...', controller: this.controller })
            .placeholderColor(Color.Grey)
            .placeholderFont({ size: 14, weight: 400 })
            .caretColor(Color.Blue)
            .width('95%')
            .height(40)
            .fontSize(14)
            .fontColor(Color.Black)
            .onChange((value: string) => {
              this.uploadConfig.url = value
              console.info(`====>${this.upLoadUrl}`)
            })
        }

        Row() {
          Button('生成测试文件')
            .width('40%')
            .height(40)
            .onClick((event) => {
              this.fileWrite()
            })
            .margin({ left: '5%', right: '5%' })
        }.margin({ top: '1%' })

        Row() {
          Text('method:').width('25%')
          Row() {
            Radio({ value: 'PUT', group: 'method' }).checked(false)
              .radioStyle({
                checkedBackgroundColor: Color.Blue
              })
              .height(20)
              .onChange((isChecked: boolean) => {
                this.uploadConfig.method = 'PUT'
              })
            Text('PUT')
          }.width('30%')

          Row() {
            Radio({ value: 'POST', group: 'method' })
              .checked(false)
              .radioStyle({
                checkedBackgroundColor: Color.Blue
              })
              .height(20)
              .checked(true)
              .onChange((isChecked: boolean) => {
                this.uploadConfig.method = 'POST'
              })
            Text('POST')
          }.width('30%')
        }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

        // Row() {
        //   Text('Token:').width('25%')
        //   Row() {
        //     Radio({ value: 'null', group: 'Token' })
        //       .checked(false)
        //       .radioStyle({
        //         checkedBackgroundColor: Color.Blue
        //       })
        //       .height(20)
        //       .checked(true)
        //       .onChange((isChecked: boolean) => {
        //         this.config.token = undefined
        //       })
        //     Text('无')
        //   }.width('30%')
        //
        //   Row() {
        //     Radio({ value: 'token888', group: 'Token' }).checked(false)
        //       .radioStyle({
        //         checkedBackgroundColor: Color.Blue
        //       })
        //       .height(20)
        //       .onChange((isChecked: boolean) => {
        //         this.config.token = 'token888'
        //       })
        //     Text('有')
        //   }.width('30%')
        // }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

        Row() {
          Text('Multiple:').width('25%')
          Row() {
            Radio({ value: 'True', group: 'OverWrite' })
              .checked(false)
              .radioStyle({
                checkedBackgroundColor: Color.Blue
              })
              .height(20)
              .checked(this.multiple)
              .onChange(async (isChecked: boolean) => {
                this.multiple = true
                await this.prepareFiles();
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
              .checked(!this.multiple)
              .onChange(async (isChecked: boolean) => {
                this.multiple = false
                await this.prepareFiles();
              })
            Text('False')
          }.width('30%')
        }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

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
                this.retry = true
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
                this.retry = false
              })
            Text('False')
          }.width('30%')
        }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

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
              this.priority = value
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

          Button('断开')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(async () => {
              await this.removeTask();
            })
          Button('删除')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(async () => {
              await this.deleteTask();
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
              await this.enableWifi();
            })

          Text(this.timeCard).fontSize(20)
        }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

        Row() {
          Button('创建空文件')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(async () => {
              await this.createEmptyFile();
            })
          Button('←上传')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(async () => {
              await this.uploadOneEmptyFile();
            })
          Button('←A10上传')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(async () => {
              await this.uploadOneEmptyFile10();
            })
        }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

        Row() {
          Button('test-range')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(async () => {
              await this.uploadTest();
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
      .width('100%')
    }
    .height('100%')
  }

  progressCallback = (receivedSize: number, totalSize: number) => {
    console.info('===>upload uploadedSize:' + receivedSize + '---total:' + totalSize)
    this.message += `===>上传 size: + ${receivedSize} ===> total: + ${totalSize} \n`
  }
  headerReceiveCallback = (headers: object) => {
    console.info("===>upOnHeader headers:" + JSON.stringify(headers));
    let num: number = 0;
    Object.keys(headers).forEach((key) => {
      if (key.toString() == "headers") {
        this.message += `====>receive headers: ${key}\n`
        Object.keys(headers[key]).forEach((head_key) => {
          console.info("===>headers: " + head_key + '--->' + headers[key][head_key]);
          num++;
        })
      }
      if (key.toString() == "body") {
        this.message += `====>receive body: ${key}\n`
        console.info("===>body:" + JSON.stringify(headers[key]))
      }
    })
    console.error("===>/Request headerNumbers is :" + num);
  }
  completedCallback = (taskStates: Array<request.TaskState>) => {
    taskStates.forEach((state) => {
      console.info('====>on_COMPLETE_callback:' + JSON.stringify(state))
      this.message += `====>upload completed state: ${state.responseCode}\n`
    })
  }
  failedCallback = (taskStates: Array<request.TaskState>) => {
    taskStates.forEach((state) => {
      console.info('====>on_FAILED_callback:' + JSON.stringify(state))
      this.message += `====>upload failed state: ${state}\n`
    })
  }

  async uploadApi9() {
    let uploadConfig: request.UploadConfig = {
      url: this.upLoadUrl,
      header: { 'Accept': '*/*' },
      method: "POST",
      files: [{
        filename: "test0",
        name: "test",
        uri: "internal://cache/test.txt",
        type: "txt"
      }],
      data: [{ name: "name123", value: "123" }],
    };
    try {
      request.uploadFile(getContext(), uploadConfig).then((data: request.UploadTask) => {
        this.uploadTask = data;
      }).catch((err: BusinessError) => {
        console.error(`Failed to request the upload. Code: ${err.code}, message: ${err.message}`);
      });
    } catch (err) {
      console.error(`Failed to request the upload. err: ${JSON.stringify(err)}`);
    }
  }

  async fileWrite() {
    let pathDir = ''
    try {
      pathDir = this.context.cacheDir
      console.debug('====>pathDir:' + pathDir)
      let count = 0
      let t = setInterval(() => {
        let filePath = pathDir + `/test_nine${count}.txt`;
        let file = fs.openSync(filePath, fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        let content = ''.padEnd(1 * 1024 * 1024, count.toString()) // 修改创建的大小
        fs.writeSync(file.fd, content)
        fs.closeSync(file);
        this.message = filePath
        count += 1
        if (count == 4) {
          clearInterval(t)
          this.message = '====>生成完成\n'
        }
      }, 200)
    } catch (err) {
      console.info('====>file write err:' + JSON.stringify(err.message))
    }
  }

  async prepareFiles() {
    let files: Array<request.File> = []
    if (this.multiple) {
      for (let index = 0; index < 4; index++) {
        let file: request.File = {
          filename: `test_nine${index}.txt`,
          name: `name${index}`,
          uri: `internal://cache/test_nine${index}.txt`,
          type: 'txt'
        }
        files.push(file)
      }
    } else {
      let file: request.File = {
        filename: `test_nine0.txt`,
        name: `name0`,
        uri: `internal://cache/test_nine0.txt`,
        type: 'txt'
      }
      files.push(file)
    }
    this.uploadConfig.files = files
    this.uploadConfig.data = [{ name: "uploadTest", value: "upload" }]
    // this.config.data = [{"name": "multipleTest","value": [{"path": "./test3.txt"},{"path": "./test2.txt"}]}]
  }

  async createTask() {
    try {
      console.info('====>task.config:' + JSON.stringify(this.uploadConfig))
      this.uploadTask = await request.uploadFile(this.context, this.uploadConfig)

      this.uploadTask.on('progress', this.progressCallback)
      this.uploadTask.on('headerReceive', this.headerReceiveCallback)
      this.uploadTask.on('complete', this.completedCallback)
      this.uploadTask.on('fail', this.failedCallback)

      this.message += `====>upload task create!\n`
    } catch (err) {
      this.message += `====>upload task err: ${err.message}\n`
      console.error(`Failed to request the upload. err: ${JSON.stringify(err)}`);
    }
  }

  async removeTask() {
    try {
      await this.uploadTask?.remove()
      this.message += `====>remove task success!\n`
    } catch (err) {
      this.message += `====>remove task err: ${err.message}\n`
    }
  }

  async deleteTask() {
    try {
      await this.uploadTask?.delete()
      this.message += `====>delete task success!\n`
    } catch (err) {
      this.message += `====>delete task err: ${err.message}\n`
    }
  }

  async createEmptyFile(){
    let pathDir = '';
    let files: Array<request.File> = [];

    try {
      pathDir = this.context.cacheDir;
      let filePath = pathDir + '/empty.txt';

      if (!fs.accessSync(filePath)) {
        //create a empty file in local cache
        console.info("===>create file");
        let fileTxt = fs.openSync(filePath, fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.close(fileTxt);
        this.message = filePath
      }
      this.message = '====>生成完成\n';
    }catch (err) {
      this.message += `====>file write err: ${err.message}\n`
      console.info('====>file write err:' + JSON.stringify(err.message))
    }
  }

  async uploadOneEmptyFile() {
    try {
      let uploadConfig: request.UploadConfig = {
        url: this.upLoadUrl,
        header: {},
        method: '',
        files: [{
          filename: 'empty.txt',
          name: 'empty0',
          uri: `internal://cache/empty.txt`,
          type: 'txt'
        }],
        data: [{ name: "uploadTest", value: "upload" }]
      }

      console.info('====>task.config:' + JSON.stringify(this.uploadConfig))
      this.uploadTask = await request.uploadFile(this.context, uploadConfig)
      this.uploadTask.on('progress', this.progressCallback)
      this.uploadTask.on('headerReceive', this.headerReceiveCallback)
      this.uploadTask.on('complete', this.completedCallback)
      this.uploadTask.on('fail', this.failedCallback)

      this.message += `====>upload task create!\n`
    } catch (err) {
      this.message += `====>upload task err: ${err.message}\n`
      console.error(`Failed to request the upload. err: ${JSON.stringify(err)}`);
    }

  }

  async uploadOneEmptyFile10() {
    let dataForm: Array<request.agent.FormItem> = []
    let fileList: Array<request.agent.FileSpec> = []
    let file: request.agent.FileSpec = { path: `./empty.txt` }
    fileList.push(file);
    dataForm.push({ name: "multipleTest", value: fileList })

    try {
      let config:request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: this.upLoadUrl,
        data: dataForm,
        method: 'POST',
      }

      console.info('====>task.config:' + JSON.stringify(this.uploadConfig))
      let task:request.agent.Task = await request.agent.create(this.context, config);
      task.on('progress', (p)=>{
        this.message += `==>progress${p.state} \n` ;
      })
      task.on('completed',(p)=>{
        this.message += `==>completed${p.state} \n` ;
      })
      task.on('failed', (p)=>{
        this.message += `==>failed${p.state} \n` ;
      })

      this.message += `====>upload task create!\n`
      task.start();
    } catch (err) {
      this.message += `====>upload task err: ${err.message}\n`
      console.error(`Failed to request the upload. err: ${JSON.stringify(err)}`);
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

  async uploadTest() {
    try {
      let config: request.UploadConfig = {
        url: this.upLoadUrl,
        header: { headers: 'https' },
        method: this.method,
        files: [],
        data: [],
        begins: 5,
        ends: 100,
      };

      let files: Array<request.File> = []

      let file: request.File = {
        filename: `test.txt`,
        name: `name`,
        uri: `internal://cache/test.txt`,
        type: 'txt'
      }
      files.push(file)

      config.files = files;
      config.data = [{ name: "uploadTest", value: "upload" }]

      console.info('====>task.config:' + JSON.stringify(config))

      this.uploadTask = await request.uploadFile(this.context, config);

      this.uploadTask.on('progress', this.progressCallback);
      this.uploadTask.on('headerReceive', this.headerReceiveCallback);
      this.uploadTask.on('complete', this.completedCallback);
      this.uploadTask.on('fail', this.failedCallback);
      this.message += `====>upload task create!\n`;
    } catch (err) {
      this.message += `====>upload task err: ${err.message}\n`
      console.error(`Failed to request the upload. err: ${JSON.stringify(err)}`);
    }
  }
}