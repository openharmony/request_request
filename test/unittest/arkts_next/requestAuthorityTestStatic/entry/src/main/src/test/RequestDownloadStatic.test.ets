'use static'

/**
 * Copyright (c) 2025 Huawei Device Co., Ltd.
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
import { describe, it, expect, TestType, Size, Level, beforeAll } from "../../../hypium/index";
import hilog from '@ohos.hilog'
import abilityDelegatorRegistry from '@ohos.app.ability.abilityDelegatorRegistry';
import UIAbility from '@ohos.app.ability.UIAbility';
import { BusinessError } from '@ohos.base';
import Utils from './Util.test';
import request from "@ohos.request";
import common from "@ohos.app.ability.common";
import fs from '@ohos.file.fs';

let domain: int = 0x0000; //日志标识,0x0000作为测试框架的业务标识
let tag: string = 'testTag'; //日志标识字符串,作为tag标识当前runner类下的测试行为
let baseContext: common.UIAbilityContext;

export default function RequestDownloadStaticTest() {

  describe("RequestDownloadStaticTest", (): void => {
    hilog.info(domain, tag, '%{public}s', 'RequestDownloadStaticTest start');

    beforeAll(() => {
      hilog.info(domain, tag, '%{public}s', 'beforeAll start');
      let abilityDelegator: abilityDelegatorRegistry.AbilityDelegator = abilityDelegatorRegistry.getAbilityDelegator()
      abilityDelegator.addAbilityMonitor({
        abilityName: "EntryAbility",
        moduleName: "entry",
        onAbilityCreate: (abilitys: UIAbility): void => {
          baseContext = abilitys.context
          hilog.info(domain, tag, '%{public}s', 'onAbilityCreate end');

        },
      }, (err: BusinessError | null): void => {
        if (err != null) {
          hilog.info(domain, tag, '%{public}s', '-----' + err.code);
        }
        hilog.info(domain, tag, '%{public}s', 'BusinessError  end');
      });
      await Utils.msSleep(2000)
      abilityDelegator.executeShellCommand("aa start -a EntryAbility -b com.acts.requestAuthority.static")
      await Utils.msSleep(2000)
      hilog.info(domain, tag, '%{public}s', 'beforeAll end');
    })

    it('SUB_REQUEST_downloadFile_STAGE_API_CALLBACK_0001', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("-----------------------SUB_REQUEST_downloadFile_STAGE_API_CALLBACK_0001 is starting-----------------------");
      let downloadFilePath = `${baseContext.tempDir}/SUB_REQUEST_downloadFile_STAGE_API_CALLBACK_0001.txt`
      console.info("====>downloadFileConfig:" +
        `${baseContext.tempDir}/SUB_REQUEST_downloadFile_STAGE_API_CALLBACK_0001.txt`)
      let downloadFileConfig: request.DownloadConfig = {
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        header: {
          'headers': 'http'
        } as Record<string, string>,
        enableMetered: false,
        enableRoaming: false,
        description: 'XTS download test!',
        networkType: request.NETWORK_WIFI,
        filePath: downloadFilePath,
        title: 'XTS download test!',
        background: false
      }
      request.downloadFile(baseContext, downloadFileConfig,
        (err: BusinessError | null, downloadTask: request.DownloadTask | undefined) => {
          console.info("====>SUB_REQUEST_downloadFile_STAGE_API_CALLBACK_0001 downloadTask: " + downloadTask);
          try {
            expect(downloadTask != undefined).assertEqual(true);
          } catch (e) {
            console.info("====>SUB_REQUEST_downloadFile_STAGE_API_CALLBACK_0001 except error: " + e);
          }
          // await downloadTask.delete();
          // fileio.unlinkSync(downloadFilePath);
          console.info("-----------------------SUB_REQUEST_downloadFile_STAGE_API_CALLBACK_0001 end-----------------------");
        });
    });

    it('SUB_REQUEST_downloadFile_STAGE_API_PROMISE_0001', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("-----------------------SUB_REQUEST_downloadFile_STAGE_API_PROMISE_0001 is starting-----------------------");
      let downloadFilePath = `${baseContext.tempDir}/SUB_REQUEST_downloadFile_STAGE_API_PROMISE_0001.txt`
      console.info("====>downloadFileConfig:" +
        `${baseContext.tempDir}/SUB_REQUEST_downloadFile_STAGE_API_PROMISE_0001.txt`)
      let downloadFileConfig: request.DownloadConfig = {
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        header: {
          'headers': 'http'
        } as Record<string, string>,
        enableMetered: false,
        enableRoaming: false,
        description: 'XTS download test!',
        networkType: request.NETWORK_WIFI,
        filePath: downloadFilePath,
        title: 'XTS download test!',
        background: false
      }
      try {
        request.downloadFile(baseContext, downloadFileConfig).then((downloadTask) => {
          console.info("====>SUB_REQUEST_downloadFile_STAGE_API_PROMISE_0001 downloadTask: " + downloadTask);
          expect(downloadTask != undefined).assertEqual(true);
          // await downloadTask.delete();
          // fileio.unlinkSync(downloadFilePath);
        }).catch((err: Error) => {
          console.error("====>SUB_REQUEST_downloadFile_STAGE_API_PROMISE_0001 error: " + err);
          // fileio.unlinkSync(downloadFilePath);
        })
      } catch (err) {
        console.error("====>SUB_REQUEST_downloadFile_STAGE_API_PROMISE_0001 catch error: " + err);
        // fileio.unlinkSync(downloadFilePath);
        // done();
      }
    });

    it('SUB_Misc_Request_Download_Notice_Task_001', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>---------------------SUB_Misc_Request_Download_Notice_Task_001 is starting---------------------");
      let downloadFilePath = `${baseContext.tempDir}/SUB_Misc_Request_Download_Notice_Task_001.txt`
      console.info("====>downloadFileConfig:" + `${baseContext.tempDir}/SUB_Misc_Request_Download_Notice_Task_001.txt`)
      let downloadFileConfig: request.DownloadConfig = {
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        header: {
          'headers': 'http'
        } as Record<string, string>,
        enableMetered: false,
        enableRoaming: false,
        description: 'XTS download test!',
        filePath: downloadFilePath,
        title: 'XTS download test!',
      }
      request.downloadFile(baseContext, downloadFileConfig, (err, downloadTask) => {
        console.info("====>SUB_Misc_Request_Download_Notice_Task_001 downloadTask: " + downloadTask);
        try {
          if (err) {
            console.info('====>SUB_Misc_Request_Download_Notice_Task_001 downloadFile fail: ' + JSON.stringify(err));
            expect().assertFail();
          }
          expect(downloadTask != undefined).assertEqual(true);
          downloadTask?.onComplete(() => {
            try {
              // downloadTask.off('complete');
              console.info('====>SUB_Misc_Request_Download_Notice_Task_001 complete success');
              fs.unlinkSync(downloadFilePath);
              console.info('====>SUB_Misc_Request_Download_Notice_Task_001 delete files success');
              // await downloadTask.delete();
              console.info('====>SUB_Misc_Request_Download_Notice_Task_001 delete downloadTask');
              expect(true).assertTrue();
              // done();
            } catch (err) {
              console.info("====>SUB_Misc_Request_Download_Notice_Task_001 complete error: " + JSON.stringify(err));
              // done();
            }
          });
        } catch (err) {
          console.info("====>SUB_Misc_Request_Download_Notice_Task_001 error: " + JSON.stringify(err));
          // await downloadTask.delete();
          fs.unlinkSync(downloadFilePath);
          // done();
        }
      })
    });
  })
}