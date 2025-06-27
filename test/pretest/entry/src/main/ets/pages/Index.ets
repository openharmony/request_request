import router from '@ohos.router';
import account_osAccount from '@ohos.account.osAccount';
import { osAccount } from '@kit.BasicServicesKit';
import { BusinessError } from '@ohos.base'

@Entry
@Component
struct Index {
  @State message: string = 'Hello World';

  build() {
    Row() {
      Column() {
        Button('上传下载')
          .fontSize(40)
          .fontWeight(FontWeight.Bold)
          .onClick(() => {
            router.pushUrl({ url: "pages/Request" })
          })
          .margin({ top: '1%' })
          .width('80%')
          .align(Alignment.Center)
      }
      .width('100%')
    }
    .height('100%')
  }
}