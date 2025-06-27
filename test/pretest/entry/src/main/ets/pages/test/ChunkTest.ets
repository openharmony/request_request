import router from '@ohos.router';
import request from '@ohos.request';
import systemDateTime from '@ohos.systemDateTime';
import common from '@ohos.app.ability.common';
import wifiManager from '@ohos.wifiManager';

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
  controller: TextAreaController = new TextAreaController();
  private context = getContext(this) as common.UIAbilityContext;
  private task: undefined|request.agent.Task;
  @State config: request.agent.Config = {
    action: request.agent.Action.DOWNLOAD,
    url: this.downloadUrl,
    title: 'chunk_test',
    description: 'chunk_test',
    saveas: './',
    mode: this.mode,
    overwrite: this.overwrite,
    retry: this.retry,
    priority: this.priority,
    token: undefined
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
                console.info(`====>${this.downloadUrl}`)
              })
          }

          Row(){
            Progress({ value: this.prgValue, total: this.prgTotal, type: ProgressType.Linear })
              .color(Color.Orange)
          }.margin({ top: '1%'}).width('100%').align(Alignment.Center)

          Row(){
            Button('单任务验证')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async ()=>{
                await this.createTask(0);
              })
          }.margin({ top: '1%'}).width('100%').align(Alignment.Center)

          Row(){
            Button('多任务递减下载')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async ()=>{
                let count: number = 0
                setInterval(async()=>{
                  count += 1;
                  await this.createTask(count);
                  if(count==1000){
                    this.message = ''
                  }
                }, 500)
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
    // this.message += `====>completed progress.state: ${progress.state}\n`
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

  async createTask(index:number){
    try{
      this.config.saveas = `./chunkTest${index}.txt`
      console.info('====>task.config:' + JSON.stringify(this.config))
      let task  = await request.agent.create(this.context, this.config)
      task.on('completed', this.completedCallback)
      task.on('failed', this.failedCallback)
      await task.start()
      // this.message += `${index}====>start task success!\n`
    }catch(err){
      this.message += `====>create task err: ${err.message}\n`
    }
  }

}