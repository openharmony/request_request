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
  @State upLoadUrl: string = 'http://192.168.0.118:8080/hfs/';
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
              await this.removeTask()
            })

        }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

        Row() {
          Button('ok')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(async () => {
              await this.uploadOK()
            })
          Button('url1')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(async () => {
              await this.uploadUrl1();
            })
          Button('u2')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(async () => {
              await this.uploadUrl2();
            })
          Button('u3')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(async () => {
              await this.uploadUrl3();
            })
          Button('met1')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(async () => {
              await this.uploadMethod1()
            })
          Button('met2')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(async () => {
              await this.uploadMethod2()
            })
        }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

        Row() {
          Button('head1')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(async () => {
              await this.uploadHeader1()
            })
          Button('head2')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(async () => {
              await this.uploadHeader2()
            })
          Button('head3')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(async () => {
              await this.uploadHeader3()
            })
        }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

        Row() {
          Button('ind1')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(async () => {
              await this.uploadIndex1()
            })
          Button('ind2')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(async () => {
              await this.uploadIndex2()
            })
          Button('ind3')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(async () => {
              await this.uploadIndex3()
            })
          Button('ind4')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(async () => {
              await this.uploadIndex4()
            })
        }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

        Row() {
          Button('beg1')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(async () => {
              await this.uploadBegin1()
            })
          Button('beg2')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(async () => {
              await this.uploadBegin2()
            })
          Button('beg3')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(async () => {
              await this.uploadBegin3()
            })
        }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

        Row() {
          Button('end1')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(async () => {
              await this.uploadEnd1()
            })
          Button('end2')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(async () => {
              await this.uploadEnd2()
            })
          Button('end3')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(async () => {
              await this.uploadEnd3()
            })
          Button('beg/end')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(async () => {
              await this.uploadBeginEnd()
            })
        }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

        Row() {
          Button('nof')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(async () => {
              await this.uploadTestFile1ef()
            })
          Button('non')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(async () => {
              await this.uploadTestFile2en()
            })
          Button('nou')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(async () => {
              await this.uploadTestFile3eu()
            })
          Button('noeu')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(async () => {
              await this.uploadTestFile4neu()
            })
          Button('efo')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(async () => {
              await this.uploadTestFile5fo()
            })
          Button('edata')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(async () => {
              await this.uploadTestFile6eData()
            })
        }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

        Row() {
          Button('testDir')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(async () => {
              await this.testDir()
            })
          Button('test')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(async () => {
              await this.uploadTest()
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
    console.info('====>header receive in====')
    console.info("upOnHeader headers:" + JSON.stringify(headers));
    this.message += `====>receive progress.state: ${headers}\n`
  }
  completedCallback = (taskStates: Array<request.TaskState>) => {
    taskStates.forEach((state) => {
      console.info('====>on_COMPLETE_callback:' + JSON.stringify(state))
      this.message += `====>upload completed state: ${state}\n`
    })
  }
  failedCallback = (taskStates: Array<request.TaskState>) => {
    taskStates.forEach((state) => {
      console.info('====>on_FAILED_callback:' + JSON.stringify(state))
      this.message += `====>upload failed state: ${state}\n`
    })
  }

  async fileWrite() {
    let pathDir = ''
    try {
      pathDir = this.context.cacheDir
      console.debug('====>pathDir:' + pathDir)
      let count = 0
      let t = setInterval(() => {
        let filePath = pathDir + `/test${count}.txt`;
        let file = fs.openSync(filePath, fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        let content = ''.padEnd(1 * 1024 * 1024, count.toString()) // 修改创建的大小
        if (count == 9) {
          content = ''.padEnd(5 * 1024 * 1024, count.toString())
        }
        fs.writeSync(file.fd, content)
        fs.closeSync(file);
        this.message = filePath
        count += 1
        if (count == 10) {
          clearInterval(t)
          this.message = '====>生成完成\n'
        }
      }, 200)
    } catch (err) {
      console.debug('====>err:' + JSON.stringify(err.message))
    }
  }

  async prepareFiles() {
    let files: Array<request.File> = []
    if (this.multiple) {
      for (let index = 0; index < 10; index++) {
        let file: request.File = {
          filename: `test${index}.txt`,
          name: `name${index}`,
          uri: `internal://cache/test${index}.txt`,
          type: 'txt'
        }
        files.push(file)
      }
    } else {
      let file: request.File = {
        filename: `test0.txt`,
        name: `name0`,
        uri: `internal://cache/test0.txt`,
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

  async uploadOK() {
    try {
      let config: request.UploadConfig = {
        url: this.upLoadUrl,
        header: { headers: 'https' },
        method: this.method,
        files: [],
        data: [],
      };

      let files: Array<request.File> = []

      for (let index = 0; index < 10; index++) {
        let file: request.File = {
          filename: `test${index}.txt`,
          name: `name${index}`,
          uri: `internal://cache/test${index}.txt`,
          type: 'txt'
        }
        files.push(file)
      }

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

  async uploadTest() {
    try {
      let config: request.UploadConfig = {
        url: this.upLoadUrl,
        header: { headers: 'https' },
        method: this.method,
        files: [],
        data: [],
      };

      let files: Array<request.File> = []

      let path = getContext().cacheDir + '/test0.txt'
      let f = fs.openSync(path, fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE)
      fs.close(f);

      //添加上传文件参数
      let file: request.File = {
        filename: `test0`,
        name: `name0`,
        uri: `internal://cache/test0.txt`,
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

  async uploadTestFile1ef() {
    try {
      this.uploadConfig = {
        url: this.upLoadUrl,
        header: { 'Accept': '*/*' },
        method: this.method,
        files: this.files,
        data: [],
      };
      let files: Array<request.File> = []

      let file: request.File = {
        filename: ``,
        name: `name-9`,
        uri: `internal://cache/test9.txt`,
        type: 'txt'
      }
      files.push(file)

      this.uploadConfig.files = files
      this.uploadConfig.data = [{ name: "uploadTest", value: "upload" }]
      console.info('====>task.config:' + JSON.stringify(this.uploadConfig))

      this.uploadTask = await request.uploadFile(this.context, this.uploadConfig);

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

  async uploadTestFile2en() {
    try {
      this.uploadConfig = {
        url: this.upLoadUrl,
        header: { 'Accept': '*/*' },
        method: this.method,
        files: this.files,
        data: [],
      };
      let files: Array<request.File> = []

      let file: request.File = {
        filename: `test_9.txt`,
        name: ``,
        uri: `internal://cache/test9.txt`,
        type: 'txt'
      }
      files.push(file)

      this.uploadConfig.files = files
      this.uploadConfig.data = [{ name: "uploadTest", value: "upload" }]
      console.info('====>task.config:' + JSON.stringify(this.uploadConfig))

      this.uploadTask = await request.uploadFile(this.context, this.uploadConfig);

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

  async uploadTestFile3eu() {
    try {
      this.uploadConfig = {
        url: this.upLoadUrl,
        header: { 'Accept': '*/*' },
        method: this.method,
        files: this.files,
        data: [],
      };
      let files: Array<request.File> = []

      let file: request.File = {
        filename: `test_9.txt`,
        name: `name-9`,
        uri: ``,
        type: 'txt'
      }
      files.push(file)

      this.uploadConfig.files = files
      this.uploadConfig.data = [{ name: "uploadTest", value: "upload" }]
      console.info('====>task.config:' + JSON.stringify(this.uploadConfig))

      this.uploadTask = await request.uploadFile(this.context, this.uploadConfig);

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

  async uploadTestFile4neu() {
    try {
      this.uploadConfig = {
        url: this.upLoadUrl,
        header: { 'Accept': '*/*' },
        method: this.method,
        files: this.files,
        data: [],
      };
      let files: Array<request.File> = []

      let file: request.File = {
        filename: `test_9.txt`,
        name: `name-9`,
        uri: `internal://cache/not-exist-test.txt`,
        type: 'txt'
      }
      files.push(file)

      this.uploadConfig.files = files
      this.uploadConfig.data = [{ name: "uploadTest", value: "upload" }]
      console.info('====>task.config:' + JSON.stringify(this.uploadConfig))

      this.uploadTask = await request.uploadFile(this.context, this.uploadConfig);

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

  async uploadTestFile5fo() {
    try {
      this.uploadConfig = {
        url: this.upLoadUrl,
        header: { 'Accept': '*/*' },
        method: this.method,
        files: this.files,
        data: [],
      };
      let files: Array<request.File> = []

      let file: request.File = {
        filename: `test_9`,
        name: `name-9`,
        uri: `internal://cache`,
        type: 'txt'
      }
      files.push(file)

      this.uploadConfig.files = files
      this.uploadConfig.data = [{ name: "uploadTest", value: "upload" }]
      console.info('====>task.config:' + JSON.stringify(this.uploadConfig))

      this.uploadTask = await request.uploadFile(this.context, this.uploadConfig);

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

  async uploadTestFile6eData() {
    try {
      this.uploadConfig = {
        url: this.upLoadUrl,
        header: { 'Accept': '*/*' },
        method: this.method,
        files: this.files,
        data: [],
      };
      let files: Array<request.File> = []

      let file: request.File = {
        filename: `test_9.txt`,
        name: `name-9`,
        uri: `internal://cache/test9.txt`,
        type: 'txt'
      }
      files.push(file)

      this.uploadConfig.files = files
      this.uploadConfig.data = [{ name: '', value: 'upload' }]
      console.info('====>task.config:' + JSON.stringify(this.uploadConfig))

      this.uploadTask = await request.uploadFile(this.context, this.uploadConfig);

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

  async uploadUrl1() {
    try {
      this.uploadConfig = {
        url: 'http://127.0.0.1/' + 'a'.repeat(2048),
        header: { 'Accept': '*/*' },
        method: this.method,
        files: this.files,
        data: this.data,
      };
      this.prepareFiles();
      console.info('====>task.config:' + JSON.stringify(this.uploadConfig))

      this.uploadTask = await request.uploadFile(this.context, this.uploadConfig);

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

  async uploadUrl2() {
    try {
      this.uploadConfig = {
        url: 'admin://127.0.0.1',
        header: { 'Accept': '*/*' },
        method: this.method,
        files: this.files,
        data: this.data,
      };
      this.prepareFiles();
      console.info('====>task.config:' + JSON.stringify(this.uploadConfig))

      this.uploadTask = await request.uploadFile(this.context, this.uploadConfig);

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

  async uploadUrl3() {
    try {
      this.uploadConfig = {
        url: 'https://gitee.com/openharmony/request_request',
        header: { 'Accept': '*/*' },
        method: this.method,
        files: this.files,
        data: this.data,
      };
      this.prepareFiles();
      console.info('====>task.config:' + JSON.stringify(this.uploadConfig))

      this.uploadTask = await request.uploadFile(this.context, this.uploadConfig);

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

  async uploadV10Url4() {
    try {
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'https://gitee.com/openharmony/request_request',
      }
      this.uploadConfig = {
        url: 'https://gitee.com/openharmony/request_request',
        header: { 'Accept': '*/*' },
        method: this.method,
        files: this.files,
        data: this.data,
      };
      this.prepareFiles();
      console.info('====>task.config:' + JSON.stringify(this.uploadConfig))

      this.uploadTask = await request.uploadFile(this.context, this.uploadConfig);

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

  async uploadHeader1() {
    try {
      this.uploadConfig = {
        url: this.upLoadUrl,
        header: {},
        method: this.method,
        files: this.files,
        data: this.data,
      };
      this.prepareFiles();
      console.info('====>task.config:' + JSON.stringify(this.uploadConfig))

      this.uploadTask = await request.uploadFile(this.context, this.uploadConfig);

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

  async uploadHeader2() {
    try {
      this.uploadConfig = {
        url: this.upLoadUrl,
        header: { '': '*/*' },
        method: this.method,
        files: this.files,
        data: this.data,
      };
      this.prepareFiles();
      console.info('====>task.config:' + JSON.stringify(this.uploadConfig))

      this.uploadTask = await request.uploadFile(this.context, this.uploadConfig);

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

  async uploadHeader3() {
    try {
      this.uploadConfig = {
        url: this.upLoadUrl,
        header: { 'Accept': '' },
        method: this.method,
        files: this.files,
        data: this.data,
      };
      this.prepareFiles();
      console.info('====>task.config:' + JSON.stringify(this.uploadConfig))

      this.uploadTask = await request.uploadFile(this.context, this.uploadConfig);

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

  async uploadMethod1() {
    try {
      this.uploadConfig = {
        url: this.upLoadUrl,
        header: { 'Accept': '*/*' },
        method: 'HEAD',
        files: this.files,
        data: this.data,
      };
      this.prepareFiles();
      console.info('====>task.config:' + JSON.stringify(this.uploadConfig))

      this.uploadTask = await request.uploadFile(this.context, this.uploadConfig);

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

  async uploadMethod2() {
    try {
      this.uploadConfig = {
        url: this.upLoadUrl,
        header: { 'Accept': '*/*' },
        method: '',
        files: this.files,
        data: this.data,
      };
      this.prepareFiles();
      console.info('====>task.config:' + JSON.stringify(this.uploadConfig))

      this.uploadTask = await request.uploadFile(this.context, this.uploadConfig);

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

  async uploadIndex1() {
    try {
      this.uploadConfig = {
        url: this.upLoadUrl,
        header: { 'Accept': '*/*' },
        method: this.method,
        files: this.files,
        data: this.data,
        index: 100,
      };
      this.prepareFiles();
      console.info('====>task.config:' + JSON.stringify(this.uploadConfig))

      this.uploadTask = await request.uploadFile(this.context, this.uploadConfig);

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

  async uploadIndex2() {
    try {
      this.uploadConfig = {
        url: this.upLoadUrl,
        header: { 'Accept': '*/*' },
        method: this.method,
        files: this.files,
        data: this.data,
        index: -1,
      };
      this.prepareFiles();
      console.info('====>task.config:' + JSON.stringify(this.uploadConfig))

      this.uploadTask = await request.uploadFile(this.context, this.uploadConfig);

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

  async uploadIndex3() {
    try {
      // this.multiple = true;
      this.uploadConfig = {
        url: this.upLoadUrl,
        header: { 'Accept': '*/*' },
        method: this.method,
        files: this.files,
        data: [],
        index: 5,
      };
      this.prepareFiles();
      console.info('====>task.config:' + JSON.stringify(this.uploadConfig))

      this.uploadTask = await request.uploadFile(this.context, this.uploadConfig);

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

  async uploadIndex4() {
    try {
      // this.multiple = true;
      this.uploadConfig = {
        url: this.upLoadUrl,
        header: { 'Accept': '*/*' },
        method: this.method,
        files: this.files,
        data: [],
        index: 5,
        begins: 1024,
        ends: 500 * 1024,
      };
      this.prepareFiles();
      console.info('====>task.config:' + JSON.stringify(this.uploadConfig))

      this.uploadTask = await request.uploadFile(this.context, this.uploadConfig);

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

  async uploadBegin1() {
    try {
      let config: request.UploadConfig = {
        url: this.upLoadUrl,
        header: { headers: 'https' },
        method: this.method,
        files: [],
        data: [],
        begins: 10,
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

  async uploadBegin2() {
    try {
      this.uploadConfig = {
        url: this.upLoadUrl,
        header: { 'Accept': '*/*' },
        method: this.method,
        files: this.files,
        data: this.data,
        begins: 100 * 999 * 1024,
      };
      this.prepareFiles();
      console.info('====>task.config:' + JSON.stringify(this.uploadConfig))

      this.uploadTask = await request.uploadFile(this.context, this.uploadConfig);

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

  async uploadBegin3() {
    try {
      this.uploadConfig = {
        url: this.upLoadUrl,
        header: { 'Accept': '*/*' },
        method: this.method,
        files: this.files,
        data: this.data,
        begins: -1,
      };
      this.prepareFiles();
      console.info('====>task.config:' + JSON.stringify(this.uploadConfig))

      this.uploadTask = await request.uploadFile(this.context, this.uploadConfig);

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

  async uploadTestFileD2f() {
    try {
      this.uploadConfig = {
        url: this.upLoadUrl,
        header: { 'Accept': '*/*' },
        method: this.method,
        files: this.files,
        data: [],
      };
      let files: Array<request.File> = []

      let file: request.File = {
        filename: `test_9.txt`,
        name: `name-9`,
        uri: `internal://cache/test9.txt`,
        type: 'txt'
      }
      files.push(file)

      this.uploadConfig.files = files
      this.uploadConfig.data = [{ name: "uploadTest", value: "upload" }]
      console.info('====>task.config:' + JSON.stringify(this.uploadConfig))

      this.uploadTask = await request.uploadFile(this.context, this.uploadConfig);

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

  async uploadEnd1() {
    try {
      let config: request.UploadConfig = {
        url: this.upLoadUrl,
        header: { headers: 'https' },
        method: this.method,
        files: [],
        data: [],
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

  async uploadEnd2() {
    try {
      this.uploadConfig = {
        url: this.upLoadUrl,
        header: { 'Accept': '*/*' },
        method: this.method,
        files: this.files,
        data: this.data,
        ends: 10 * 1024 * 1024,
      };
      this.prepareFiles();
      console.info('====>task.config:' + JSON.stringify(this.uploadConfig))

      this.uploadTask = await request.uploadFile(this.context, this.uploadConfig);

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

  async uploadEnd3() {
    try {
      this.uploadConfig = {
        url: this.upLoadUrl,
        header: { 'Accept': '*/*' },
        method: this.method,
        files: this.files,
        data: this.data,
        ends: -10,
      };
      this.prepareFiles();
      console.info('====>task.config:' + JSON.stringify(this.uploadConfig))

      this.uploadTask = await request.uploadFile(this.context, this.uploadConfig);

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

  async uploadBeginEnd() {
    try {
      this.uploadConfig = {
        url: this.upLoadUrl,
        header: { 'Accept': '*/*' },
        method: this.method,
        files: this.files,
        data: this.data,
        index: 2,
        begins: 400 * 1024,
        ends: 100 * 1024,
      };
      let files: Array<request.File> = []

      for (let index = 0; index < 10; index++) {
        let file: request.File = {
          filename: `test${index}.txt`,
          name: `name${index}`,
          uri: `internal://cache/test${index}.txt`,
          type: 'txt'
        }
        files.push(file)
      }
      this.uploadConfig.files = files
      this.uploadConfig.data = [{ name: "uploadTest", value: "upload" }]
      console.info('====>task.config:' + JSON.stringify(this.uploadConfig))

      this.uploadTask = await request.uploadFile(this.context, this.uploadConfig);

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

  async testDir() {
    let path = getContext().cacheDir + '/testDir'

    let config: request.UploadConfig = {
      url: this.upLoadUrl,
      header: { headers: 'https' },
      method: this.method,
      files: [],
      data: [],
    };
    let files: Array<request.File> = []

    try {
      // fs.statSync(path).isDirectory()

      fs.mkdirSync(path)
      let file: request.File = {
        filename: `test`,
        name: `name`,
        uri: `internal://cache/testDir`,
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
      fs.rmdirSync(path)
      this.message += `====>upload task err: ${err.message}\n`
      console.error(`Failed to request the upload. err: ${JSON.stringify(err)}`);
    }
  }
}