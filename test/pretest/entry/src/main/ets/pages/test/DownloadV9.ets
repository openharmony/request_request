import router from '@ohos.router';
import request from '@ohos.request';
import common from '@ohos.app.ability.common';
import commonEvent from '@ohos.commonEventManager';
import wifiManager from '@ohos.wifiManager';
import { BusinessError } from '@kit.BasicServicesKit';
import fs from '@ohos.file.fs';

@Entry
@Component
struct download {
  @State message: string = '';
  @State prgValue: number = 1;
  @State prgTotal: number = 100;
  @State taskState: string = '未开始';
  @State timeCard: string = "0";
  @State method: string = 'GET';
  @State retry: boolean = true;
  @State enableRoaming: boolean = false;
  @State priority: number = 0;
  controller: TextAreaController = new TextAreaController();
  private context = getContext(this) as common.UIAbilityContext;
  private task: undefined | request.DownloadTask;
  @State URL_30M: string = 'weekly.tar.gz';
  @State URL_411M: string =  'img.tar.gz';
  @State Url_400k: string = '/test400K.txt';
  @State downloadConfig: request.DownloadConfig = {
    url: this.URL_30M,
    filePath: 'response_api9.txt',
    enableRoaming: this.enableRoaming,
  }

  build() {
    Row() {
      Column() {
        Flex({ direction: FlexDirection.Column, alignItems: ItemAlign.Center }) {
          Row() {
            Progress({ value: this.prgValue, total: this.prgTotal, type: ProgressType.Linear })
              .color(Color.Orange)
          }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

          //size
          Row() {
            Text('Size:').width('25%')
            Row() {
              Radio({ value: '400k', group: 'Size' })
                .checked(false)
                .radioStyle({ checkedBackgroundColor: Color.Blue })
                .height(20)
                .onChange((isChecked: boolean) => {
                  if (isChecked === true) {
                    this.downloadConfig.url = this.Url_400k;
                  }
                })
              Text('400k')
            }.width('25%')

            Row() {
              Radio({ value: '30M', group: 'Size' })
                .radioStyle({ checkedBackgroundColor: Color.Blue })
                .height(20)
                .checked(true)
                .onChange((isChecked: boolean) => {
                  if (isChecked === true) {
                    this.downloadConfig.url = this.URL_30M;
                  }
                })
              Text('30M')
            }.width('25%')

            Row() {
              Radio({ value: '411M', group: 'Size' })
                .checked(false)
                .radioStyle({ checkedBackgroundColor: Color.Blue })
                .height(20)
                .onChange((isChecked: boolean) => {
                  if (isChecked === true) {
                    this.downloadConfig.url = this.URL_411M;
                  }
                })
              Text('411M')
            }.width('25%')
          }.margin({ top: '1%' }).width('100%').align(Alignment.Center)


          Row() {
            Text('enableRoaming:').width('25%')
            Row() {
              Radio({ value: 'True', group: 'enableRoaming' })
                .checked(false)
                .radioStyle({
                  checkedBackgroundColor: Color.Blue
                })
                .height(20)
                .checked(this.enableRoaming)
                .onChange((isChecked: boolean) => {
                  if (isChecked === true) {
                    this.downloadConfig.enableRoaming = true;
                  }
                })
              Text('True')
            }.width('30%')

            Row() {
              Radio({ value: 'False', group: 'enableRoaming' })
                .checked(false)
                .radioStyle({
                  checkedBackgroundColor: Color.Blue
                })
                .height(20)
                .checked(!this.enableRoaming)
                .onChange((isChecked: boolean) => {
                  if (isChecked === true) {
                    this.downloadConfig.enableRoaming = false;
                  }
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
            Button('暂停')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                await this.suspendTask();
              })
            Button('恢复')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                await this.restoreTask();
              })
            Button('删除')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                await this.deleteTask();
              })
          }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

          Row() {
            Button('TaskInfo')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                await this.getTaskInfo()
              })

            Button('TaskMimeType')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                await this.getTaskMimeType()
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
            Button('DownAgain')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                await this.downTestTwice()
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

  progressCallback = (receivedSize: number, totalSize: number) => {
    this.prgValue = receivedSize;
    this.prgTotal = totalSize;
    console.info("download receivedSize:" + receivedSize + " totalSize:" + totalSize);
    this.message += `===>下载progress size: + ${receivedSize} ===> total: + ${totalSize} \n`
  };
  completedCallback = () => {
    console.info('====>on_COMPLETE_callback:')
    this.message += `====>完成completed \n`
  }
  pauseCallback = () => {
    console.info('====>on_PAUSE_callback:')
    this.message += `====>暂停pause  \n`
  }
  removeCallback = () => {
    console.info('====>on_REMOVE_callback:')
    this.message += `====>删除remove \n`
  }
  failedCallback = (err: number) => {
    console.info('====>on_FAILED_callback: error code=' + err)
    this.message += `====>失败failed failed code:${err}\n`
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
      console.info('===>task.config:' + JSON.stringify(this.downloadConfig))
      this.task = await request.downloadFile(this.context, this.downloadConfig)

      this.task.on('progress', this.progressCallback)
      this.task.on('complete', this.completedCallback)
      this.task.on('pause', this.pauseCallback)
      this.task.on('remove', this.removeCallback)
      this.task.on('fail', this.failedCallback)

      let taskInfo=await this.task.getTaskInfo();
      this.message += `====>create task success ${taskInfo.downloadId} 开始!\n`
    } catch (err) {
      this.message += `====>create task err: ${err.message}\n`
    }
  }

  async suspendTask() {
    try {
      if (this.task != undefined) {
        let ret = await this.task.suspend()
        this.message += `====>暂停suspend task success! ${ret}\n`
      } else {
        this.taskState = '暂停失败'
      }
    } catch (err) {
      this.message += `====>暂停suspend task err: ${err.message}\n`
    }
  }

  async restoreTask() {
    try {
      if (this.task != undefined) {
        let ret = await this.task.restore()
        console.info('====>重启restore')
        this.message += `====>重启restore task success! ${ret} \n`
      } else {
        this.taskState = '重启失败'
      }
    } catch (err) {
      this.message += `====>重启restore task err: ${err.message}\n`
    }
  }

  async deleteTask() {
    try {
      if (this.task != undefined) {
        await this.task.delete().then((result: boolean) => {
          console.info(`Succeeded Delete. result: ${result}`);
        }).catch((err: BusinessError) => {
          console.error(`Failed Delete. Code: ${err.code}, message: ${err.message}`);
        });
        this.message += `====>删除delete task success!\n`
      } else {
        this.taskState = '删除失败'
      }
    } catch (err) {
      this.message += `====>删除delete task err: ${err.message}\n`
    }
  }

  async getTaskInfo() {
    try {
      if (this.task != undefined) {
        let info = await this.task.getTaskInfo()
        console.info('===>Succeeded in querying ' + JSON.stringify(info))
        this.message += `===>getTaskInfo: id:${info.downloadId} \n`
        this.message += `--file:${info.fileName} \n`
        this.message += `--status:${info.status}\n`
        this.message += `--> now:${info.downloadedBytes}--total:${info.downloadTotalBytes} \n`
      } else {
        this.taskState = '任务为空'
      }
    } catch (err) {
      this.message += `====>show task err: ${err.message}\n`
    }
  }

  async getTaskMimeType() {
    try {
      if (this.task != undefined) {
        let mimetype = await this.task.getTaskMimeType()
        console.info('===>get TaskMimeType:' + JSON.stringify(mimetype))
        this.message += `get task success mimetype: ${mimetype} \n`
      } else {
        this.taskState = '任务为空'
      }
    } catch (err) {
      this.message += `====>get TaskMimeType: ${err.message}\n`
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

  async downTestTwice() {
    let conf: request.DownloadConfig = {
      url: this.URL_30M,
      filePath: 'down2test.txt',
      header:{
        'Content-Type':'multipart/form-data',
      }
    }
    let path = this.context.cacheDir + '/' + conf.filePath;
    if (fs.accessSync(path)) {
      fs.unlinkSync(path);
    }
    try {
      this.task = await request.downloadFile(this.context, conf);
      console.info('===>create and start ');
      this.task.on('progress', this.progressCallback)
      this.task.on('complete', this.completedCallback)
      this.task.on('pause', this.pauseCallback)
      this.task.on('remove', this.removeCallback)
      this.task.on('fail', this.failedCallback)
      this.message += `====>create task success 开始!\n`

    } catch (err) {
      this.message += `====>download task err: ${err.message}\n`
      console.error(`Failed to request the download. err: ${JSON.stringify(err)}`);
    }
  }

}

