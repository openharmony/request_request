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

@Entry
@Component
struct systemTime {
  @State message: string = 'Hello World';

  build() {
    Row() {
      Column() {

        //API9
        Text('API9')
          .textAlign(TextAlign.Center)
          .fontSize(25)
          .padding(10)
        Button('DownLoad')
          .fontSize(25)
          .fontWeight(FontWeight.Bold)
          .onClick(()=>{
            router.pushUrl({url:"pages/test/download_V9"})
          })
          .margin({ top: '1%'}).width('50%').align(Alignment.Center)

        Button('UpLoad')
          .fontSize(25)
          .fontWeight(FontWeight.Bold)
          .onClick(()=>{
            router.pushUrl({url:"pages/test/upload_V9"})
          })
          .margin({ top: '1%'}).width('50%').align(Alignment.Center)
        Button('9test')
          .fontSize(25)
          .fontWeight(FontWeight.Bold)
          .onClick(()=>{
            router.pushUrl({url:"pages/test/test"})
          })
          .margin({ top: '1%'}).width('50%').align(Alignment.Center)

        //API10
        Text('API10')
          .textAlign(TextAlign.Center)
          .fontSize(25)
          .width('100%')
          .margin({ top: '5%'}).width('50%').align(Alignment.Center)

        Button('DownLoad')
          .fontSize(25)
          .fontWeight(FontWeight.Bold)
          .onClick(()=>{
            router.pushUrl({url:"pages/test/download"})
          })
          .margin({ top: '1%'}).width('50%').align(Alignment.Center)

        Button('UpLoad')
          .fontSize(25)
          .fontWeight(FontWeight.Bold)
          .onClick(()=>{
            router.pushUrl({url:"pages/test/upload"})
          })
          .margin({ top: '1%'}).width('50%').align(Alignment.Center)

        Button('1000任务下载压力测试')
          .fontSize(25)
          .fontWeight(FontWeight.Bold)
          .onClick(()=>{
            router.pushUrl({url:"pages/test/multipleTask"})
          })
          .margin({ top: '1%'}).width('50%').align(Alignment.Center)

        Button('ChunkTest')
          .fontSize(25)
          .fontWeight(FontWeight.Bold)
          .onClick(()=>{
            router.pushUrl({url:"pages/test/chunktest"})
          })
          .margin({ top: '1%'}).width('50%').align(Alignment.Center)

        Button('任务代理和证书')
          .fontSize(25)
          .fontWeight(FontWeight.Bold)
          .onClick(()=>{
            router.pushUrl({url:"pages/test/feature50"})
          })
          .margin({ top: '1%'}).width('50%').align(Alignment.Center)

        Button('1k/2k任务特殊测试')
          .fontSize(25)
          .fontWeight(FontWeight.Bold)
          .onClick(()=>{
            router.pushUrl({url:"pages/test/stressTask"})
          })
          .margin({ top: '1%'}).width('50%').align(Alignment.Center)

        Button('TaskPoolTest')
          .fontSize(25)
          .fontWeight(FontWeight.Bold)
          .onClick(()=>{
            router.pushUrl({url:"pages/test/taskpooltest"})
          })
          .margin({ top: '1%'}).width('50%').align(Alignment.Center)


      }
      .width('100%')
    }
    .height('100%')
  }
}