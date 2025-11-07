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