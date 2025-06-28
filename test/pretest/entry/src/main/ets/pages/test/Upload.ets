import fs from '@ohos.file.fs';
import request from '@ohos.request';
import common from '@ohos.app.ability.common';
import wifiManager from '@ohos.wifiManager';

@Entry
@Component
struct upload {
  @State message: string = '';
  @State prgValue: number = 1;
  @State prgTotal: number = 100;
  private task: undefined | request.agent.Task;
  @State taskState: string = '未开始';
  @State responseHeader: string = '';
  @State upLoadUrl: string = '';
  @State method: string = 'POST';
  @State mode: request.agent.Mode = request.agent.Mode.FOREGROUND;
  @State token: boolean = false;
  @State multiple: boolean = false;
  @State retry: boolean = true;
  @State priority: number = 0;
  @State timeCard: string = "0";
  @State files: Array<request.agent.FormItem> = [];
  controller: TextAreaController = new TextAreaController()
  private context = getContext(this) as common.UIAbilityContext;
  @State config: request.agent.Config = {
    action: request.agent.Action.UPLOAD,
    url: this.upLoadUrl,
    title: 'intake_test',
    description: 'intake_test',
    mode: this.mode,
    retry: this.retry,
    priority: this.priority,
    method: this.method,
    gauge:true,
    // headers:{'Content-Type': 'application/octet-stream'}
  };

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
              this.config.url = value
              console.info(`====>${this.upLoadUrl}`)
            })
        }

        Row() {
          Button('生成测试文件')
            .width('40%')
            .onClick((event) => {
              this.fileWrite()
            })
            .margin({ left: '5%', right: '5%' })
        }.margin({ top: '1%' })

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

        Row() {
          Text('method:').width('25%')
          Row() {
            Radio({ value: 'PUT', group: 'method' }).checked(false)
              .radioStyle({
                checkedBackgroundColor: Color.Blue
              })
              .height(20)
              .onChange((isChecked: boolean) => {
                this.config.method = 'PUT'
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
                this.config.method = 'POST'
              })
            Text('POST')
          }.width('30%')
        }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

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
                this.config.token = undefined
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
                this.config.token = 'token888'
              })
            Text('有')
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

          Button('恢复')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(async () => {
              await this.resumeTask();
            })

          Button('删除')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(async () => {
              if (this.task != undefined) {
                await request.agent.remove(this.task.tid)
              } else {
                this.taskState = '任务为空'
              }
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
                console.info('====> catch err:' + JSON.stringify(err))
              }
            })

          Button('search')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(async () => {
              await this.searchTask()
            })
        }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

        Row() {
          Button('断开网络')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(() => {
              this.disableWifi();
            })

          Button('恢复网络')
            .fontSize(20)
            .fontWeight(FontWeight.Bold)
            .onClick(() => {
              this.enableWifi();
            })

          Text(this.timeCard).fontSize(20)
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

  async fileWrite() {
    let pathDir = ''
    try {
      pathDir = this.context.cacheDir
      console.debug('====>pathDir:' + pathDir)
      let count = 0
      let t = setInterval(() => {
        let filePath = pathDir + `/test_ten${count}.txt`;
        let file = fs.openSync(filePath, fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        let content = ''.padEnd(2 * 1024 * 1024, count.toString()) // 修改创建的大小
        if (count == 0) {
          content = ''.padEnd(2 * 1024 * 1024, count.toString())
        }
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
      console.debug('====>err:' + JSON.stringify(err.message))
    }
  }

  async prepareFiles() {
    let data: Array<request.agent.FormItem> = []
    let fileList: Array<request.agent.FileSpec> = []
    if (this.multiple) {
      for (let index = 0; index < 4; index++) {
        let file: request.agent.FileSpec = { path: `./test_ten${index}.txt` }
        fileList.push(file)
      }
    } else {
      let file: request.agent.FileSpec = { path: `./test_ten0.txt` }
      fileList.push(file)
    }
    data.push({ name: "multipleTest", value: fileList })
    this.config.data = data
    // this.config.data = [{"name": "multipleTest","value": [{"path": "./test3.txt"},{"path": "./test2.txt"}]}]
  }

  responseCallback = (response: request.agent.HttpResponse) => {
    response.headers.forEach((value, key) => {
      console.info(`====> key:${key}   value:${value}`)
    });
    this.message += `====>response.status: ${response.statusCode}==>headers.get:${response.headers.get('content-type')}\n`
    // this.message += `====>response.headers.get('content-type'): ${response.headers.get('content-type')}\n`
  }
  progressCallback = (progress: request.agent.Progress) => {
    this.prgValue = progress.processed;
    this.prgTotal = progress.sizes[0];
    console.info('====>on_progress progress.processed:' + progress.processed)
    this.message += `===>上传 size: + ${progress.processed} ===> total: + ${progress.sizes[0]} \n`
  }
  completedCallback = (progress: request.agent.Progress) => {
    console.info('====>on_complete_callback:=' + JSON.stringify(progress))
    let tag = false;
    let num: number = 0;
    if (progress.extras != undefined) {
      let extra: object = progress.extras;
      Object.keys(extra).forEach((key) => {
        num++;
        console.info("--->: " + key + "--" + extra[key])
        if (key == "body") {
          tag = true;
        }
      })
    }
    if (tag) {
      console.info("===>/Request body exist")
      this.message += `===>/Request body exist \n`
    } else {
      console.error("===>/Request no ResponseBody")
      this.message += `===>/Request no ResponseBody \n`
    }
    console.info("===>response num :" + num)
    this.message += `====>completed progress.state: ${progress.state}\n`
  }
  failedCallback = (progress: request.agent.Progress) => {
    console.info('====>on_failed_callback:' + JSON.stringify(progress))
    this.message += `====>failed progress.state: ${progress.state}\n`
  }
  pauseCallback = (progress: request.agent.Progress) => {
    console.info('====>on_pause_callback:' + JSON.stringify(progress))
    this.message += `====>pause progress.state: ${progress.state}\n`
  }
  removeCallback = (progress: request.agent.Progress) => {
    console.info('====>on_remove_callback:' + JSON.stringify(progress))
    this.message += `====>remove progress.state: ${progress.state}\n`
  }

  async createTask() {
    try {
      console.info('====>task.config:' + JSON.stringify(this.config))
      this.task = await request.agent.create(this.context, this.config)
      this.task.on('completed', this.completedCallback)
      this.task.on('response', this.responseCallback)
      this.task.on('progress', this.progressCallback)
      this.task.on('failed', this.failedCallback)
      this.task.on('pause', this.pauseCallback)
      this.task.on('remove', this.removeCallback)
      this.message += `====>create task success!\n`
    } catch (err) {
      this.message += `====>create task err: ${err.message}\n`
    }
  }

  async startTask() {
    try {
      if (this.task != undefined) {
        await this.task.start()
        this.message += `====>start task success!\n`
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
        this.message += `====>pause task success!\n`
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
        this.message += `====>resume task success!\n`
      } else {
        this.taskState = '暂停失败'
      }
    } catch (err) {
      this.message += `====>resume task err: ${err.message}\n`
    }
  }

  async removeTask(taskId: string) {
    try {
      await request.agent.remove(taskId)
      this.message += `====>remove task success!\n`
    } catch (err) {
      this.message += `====>remove task err: ${err.message}\n`
    }
  }

  async showTask(taskId: string) {
    try {
      let taskInfo = await request.agent.show(taskId)
      console.info('====>showTaskInfo:' + JSON.stringify(taskInfo))
      this.message += `====>show taskInfo.ctime: ${taskInfo.ctime.toString()}\n`
    } catch (err) {
      this.message += `====>show task err: ${err.message}\n`
    }
  }

  async touchTask(taskId: string, token: string) {
    try {
      let taskInfo = await request.agent.touch(taskId, token)
      console.info('====>touchTaskInfo:' + JSON.stringify(taskInfo))
      this.message += `====>touch taskInfo.mtime: ${taskInfo.mtime.toString()}\n`
    } catch (err) {
      this.message += `====>touch task err: ${err.message}\n`
    }
  }

  async searchTask() {
    try {
      let taskList = await request.agent.search()
      console.info('====>searchTaskList:' + JSON.stringify(taskList))
      this.message += JSON.stringify(taskList) + '\n'
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

}