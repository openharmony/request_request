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

    it('SUB_Misc_Request_Download_Notice_Task_002', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>---------------------SUB_Misc_Request_Download_Notice_Task_002 is starting---------------------");
      let downloadFilePath = `${baseContext.tempDir}/SUB_Misc_Request_Download_Notice_Task_002.txt`
      console.info("====>downloadFileConfig:" + `${baseContext.tempDir}/SUB_Misc_Request_Download_Notice_Task_002.txt`);
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
        background: false
      }
      request.downloadFile(baseContext, downloadFileConfig, (err, downloadTask) => {
        console.info("====>SUB_Misc_Request_Download_Notice_Task_002 downloadTask: " + downloadTask);
        try {
          if (err) {
            console.info('====>SUB_Misc_Request_Download_Notice_Task_002 downloadFile fail: ' + JSON.stringify(err));
            expect().assertFail();
          }
          expect(downloadTask != undefined).assertEqual(true);
          downloadTask?.onComplete(() => {
            try {
              downloadTask?.offComplete();
              console.info('====>SUB_Misc_Request_Download_Notice_Task_002 complete success');
              fs.unlinkSync(downloadFilePath);
              console.info('====>SUB_Misc_Request_Download_Notice_Task_002 delete files success');
              // await downloadTask.delete();
              console.info('====>SUB_Misc_Request_Download_Notice_Task_002 delete downloadTask');
              expect(true).assertTrue();
              // done();
            } catch (err) {
              console.info("====>SUB_Misc_Request_Download_Notice_Task_002 complete error: " + JSON.stringify(err));
              // done();
            }
          });
        } catch (err) {
          console.info("====>SUB_Misc_Request_Download_Notice_Task_002 error: " + JSON.stringify(err));
          // await downloadTask.delete();
          fs.unlinkSync(downloadFilePath);
          // done();
        }
      })
    });


    it('SUB_Misc_Request_Download_Notice_Task_003', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>---------------------SUB_Misc_Request_Download_Notice_Task_003 is starting---------------------");
      let downloadFilePath = `${baseContext.tempDir}/SUB_Misc_Request_Download_Notice_Task_003.txt`
      console.info("====>downloadFileConfig:" + `${baseContext.tempDir}/SUB_Misc_Request_Download_Notice_Task_003.txt`);
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
        background: true
      }
      request.downloadFile(baseContext, downloadFileConfig, (err, downloadTask) => {
        console.info("====>SUB_Misc_Request_Download_Notice_Task_003 downloadTask: " + downloadTask);
        try {
          if (err) {
            console.info('====>SUB_Misc_Request_Download_Notice_Task_003 downloadFile fail: ' + JSON.stringify(err));
            expect().assertFail();
          }
          expect(downloadTask != undefined).assertEqual(true);
          downloadTask?.onComplete(() => {
            try {
              downloadTask?.offComplete();
              console.info('====>SUB_Misc_Request_Download_Notice_Task_003 complete success');
              fs.unlinkSync(downloadFilePath);
              console.info('====>SUB_Misc_Request_Download_Notice_Task_003 delete files success');
              // await downloadTask.delete();
              console.info('====>SUB_Misc_Request_Download_Notice_Task_003 delete downloadTask');
              expect(true).assertTrue();
              // done();
            } catch (err) {
              console.info("====>SUB_Misc_Request_Download_Notice_Task_003 complete error: " + JSON.stringify(err));
              // done();
            }
          });
        } catch (err) {
          console.info("====>SUB_Misc_Request_Download_Notice_Task_003 error: " + JSON.stringify(err));
          // await downloadTask.delete();
          fs.unlinkSync(downloadFilePath);
          // done();
        }
      })
    })

    it('SUB_Misc_Request_DownloadManagement_0003', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>---------------------SUB_Misc_Request_DownloadManagement_0003 is starting---------------------");
      let downloadFilePath = `${baseContext.tempDir}/SUB_Misc_Request_DownloadManagement_0003.txt`
      console.info("====>downloadFileConfig:" + `${baseContext.tempDir}/SUB_Misc_Request_DownloadManagement_0003.txt`);
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
      try {
        let file = fs.openSync(downloadFilePath, fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        let str: string = "hello, world";
        let writeLen = fs.writeSync(file.fd, str);
        console.info("====>SUB_Misc_Request_DownloadManagement_0003 write data to file succeed and size is:" +
          writeLen);
        fs.closeSync(file);
        await request.downloadFile(baseContext, downloadFileConfig);
        console.info("====>SUB_Misc_Request_DownloadManagement_0003 downloadFile success");
        expect().assertFail();
        // done();
      } catch (err) {
        console.info("====>SUB_Misc_Request_DownloadManagement_0003 error: " + JSON.stringify(err));
        fs.unlinkSync(downloadFilePath);
        // expect(err.code).assertEqual(13400002);
        // done();
      }
    });


    it('SUB_Misc_Request_DownloadManagement_0004', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>---------------------SUB_Misc_Request_DownloadManagement_0004 is starting---------------------");
      let downloadFilePath =
        `/data/app/el2/100/base/com.example.ddd/haps/entry_test/cache/SUB_Misc_Request_DownloadManagement_0004.txt`
      console.info("====>downloadFileConfig: " + `${downloadFilePath}`);
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
      try {
        await request.downloadFile(baseContext, downloadFileConfig);
        console.info("====>SUB_Misc_Request_DownloadManagement_0004 downloadFile success");
        expect().assertFail();
        // done();
      } catch (err) {
        console.info("====>SUB_Misc_Request_DownloadManagement_0004 error: " + JSON.stringify(err));
        // expect(err.code).assertEqual(13400001);
        // done();
      }
    });

    //
    it('SUB_Misc_Request_DownloadManagement_0004', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>SUB_Misc_Request_DownloadManagement_0005 is starting");
      let downloadFileConfig: request.DownloadConfig = {
        url: '/downloadTest/releases/download/v1.0/test.apk',
        enableMetered: false,
        enableRoaming: false,
        description: 'XTS download test!',
        filePath: "downloadFilePath",
        title: 'XTS download test!',
        background: false
      }
      try {
        await request.downloadFile(baseContext, downloadFileConfig)
        console.info('====>SUB_Misc_Request_DownloadManagement_0005 Download success.');
        expect().assertFail();
      } catch (err) {
        console.info("====>SUB_Misc_Request_DownloadManagement_0005 except error: " + JSON.stringify(err));
        // expect(err.code).assertEqual(401);
        // done();
      }
    });

    it('SUB_Misc_Request_Download_SupportSystem_DownloadSuccess_001', Level.LEVEL1,
      async (done: () => void): Promise<void> => {
        console.info("====>---------------------SUB_Misc_Request_Download_SupportSystem_DownloadSuccess_001 is starting---------------------");
        let downloadFileConfig: request.DownloadConfig = {
          url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
          header: {
            'headers': 'http'
          } as Record<string, string>,
          enableMetered: false,
          enableRoaming: false,
          description: 'XTS download test!',
          title: 'XTS download test!',
          background: false
        }
        request.downloadFile(baseContext, downloadFileConfig, (err, downloadTask) => {
          console.info("====>SUB_Misc_Request_Download_SupportSystem_DownloadSuccess_001 downloadTask: " +
            downloadTask);
          try {
            if (err) {
              console.info('====>SUB_Misc_Request_Download_SupportSystem_DownloadSuccess_001 downloadFile fail: ' +
              JSON.stringify(err));
              expect().assertFail();
            }
            expect(downloadTask != undefined).assertEqual(true);
            downloadTask?.onComplete(() => {
              try {
                downloadTask?.offComplete();
                console.info('====>SUB_Misc_Request_Download_SupportSystem_DownloadSuccess_001 complete success');
                // await downloadTask.delete();
                console.info('====>SUB_Misc_Request_Download_SupportSystem_DownloadSuccess_001 delete downloadTask');
                fs.unlinkSync(`${baseContext.cacheDir}/test.hap`);
                console.info('====>SUB_Misc_Request_Download_SupportSystem_DownloadSuccess_002 delete files success');
                expect(true).assertTrue();
                // done();
              } catch (err) {
                console.info("====>SUB_Misc_Request_Download_SupportSystem_DownloadSuccess_001 complete error: " +
                JSON.stringify(err));
                // done();
              }
            });
          } catch (err) {
            console.info("====>SUB_Misc_Request_Download_SupportSystem_DownloadSuccess_001 error: " +
            JSON.stringify(err));
            // await downloadTask.delete();
            // done();
          }
        })
      });

    it('SUB_Misc_Request_Download_SupportSystem_DownloadSuccess_002', Level.LEVEL1,
      async (done: () => void): Promise<void> => {
        console.info("====>---------------------SUB_Misc_Request_Download_SupportSystem_DownloadSuccess_002 is starting---------------------");
        let downloadFilePath = `${baseContext.tempDir}/SUB_Misc_Request_Download_SupportSystem_DownloadSuccess_002.txt`
        console.info("====>downloadFileConfig:" +
          `${baseContext.tempDir}/SUB_Misc_Request_Download_SupportSystem_DownloadSuccess_002.txt`);
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
          background: false
        }
        request.downloadFile(baseContext, downloadFileConfig, (err, downloadTask) => {
          console.info("====>SUB_Misc_Request_Download_SupportSystem_DownloadSuccess_002 downloadTask: " +
            downloadTask);
          try {
            if (err) {
              console.info('====>SUB_Misc_Request_Download_SupportSystem_DownloadSuccess_002 downloadFile fail: ' +
              JSON.stringify(err));
              expect().assertFail();
            }
            expect(downloadTask != undefined).assertEqual(true);
            downloadTask?.onComplete(() => {
              try {
                downloadTask?.offComplete();
                console.info('====>SUB_Misc_Request_Download_SupportSystem_DownloadSuccess_002 complete success');
                fs.unlinkSync(downloadFilePath);
                console.info('====>SUB_Misc_Request_Download_SupportSystem_DownloadSuccess_002 delete files success');
                // await downloadTask.delete();
                console.info('====>SUB_Misc_Request_Download_SupportSystem_DownloadSuccess_002 delete downloadTask');
                expect(true).assertTrue();
                // done();
              } catch (err) {
                console.info("====>SUB_Misc_Request_Download_SupportSystem_DownloadSuccess_002 complete error: " +
                JSON.stringify(err));
                // done();
              }
            });
          } catch (err) {
            console.info("====>SUB_Misc_Request_Download_SupportSystem_DownloadSuccess_002 error: " +
            JSON.stringify(err));
            // downloadTask.delete();
            fs.unlinkSync(downloadFilePath);
            // done();
          }
        })
      });

    it('SUB_Misc_Request_Download_SupportSystem_ConflictDownload_001', Level.LEVEL1,
      async (done: () => void): Promise<void> => {
        console.info("====>---------------------SUB_Misc_Request_Download_SupportSystem_ConflictDownload_001 is starting---------------------");
        let downloadFilePath = `${baseContext.tempDir}/SUB_Misc_Request_Download_SupportSystem_ConflictDownload_001.txt`
        console.info("====>downloadFileConfig:" +
          `${baseContext.tempDir}/SUB_Misc_Request_Download_SupportSystem_ConflictDownload_001.txt`);
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
          background: false
        }
        request.downloadFile(baseContext, downloadFileConfig, (err, downloadTask) => {
          console.info("====>SUB_Misc_Request_Download_SupportSystem_ConflictDownload_001 downloadTask: " +
            downloadTask);
          try {
            if (err) {
              console.info('====>SUB_Misc_Request_Download_SupportSystem_ConflictDownload_001 downloadFile fail: ' +
              JSON.stringify(err));
              expect().assertFail();
            }
            expect(downloadTask != undefined).assertEqual(true);
            downloadTask?.onComplete(() => {
              try {
                downloadTask?.offComplete();
                console.info('====>SUB_Misc_Request_Download_SupportSystem_ConflictDownload_001 complete off success');
                await request.downloadFile(baseContext, downloadFileConfig);
                console.info("====>SUB_Misc_Request_Download_SupportSystem_ConflictDownload_001 second downloadTask success");
                expect().assertFail();
                // done();
              } catch (err) {
                try {
                  fs.unlinkSync(downloadFilePath);
                  console.info("====>SUB_Misc_Request_Download_SupportSystem_ConflictDownload_001 second downloadTask fail: " +
                  JSON.stringify(err));
                  // expect(err.code).assertEqual(13400002);
                  // done();
                } catch (err) {
                  console.info("====>SUB_Misc_Request_Download_SupportSystem_ConflictDownload_001 catch err: " +
                  JSON.stringify(err));
                  // done();
                }
              }
            });
          } catch (err) {
            console.info("====>SUB_Misc_Request_Download_SupportSystem_ConflictDownload_001 error: " +
            JSON.stringify(err));
            // await downloadTask.delete();
            fs.unlinkSync(downloadFilePath);
          }
        })
      });

    it('SUB_Misc_REQUEST_Download_Promise_0030', Level.LEVEL1,
      async (done: () => void): Promise<void> => {
        console.info("====>SUB_Misc_REQUEST_Download_Promise_0030 is starting");
        let downloadFileConfig: request.DownloadConfig = {
          url: 'xxxxxxx.xxx',
          enableMetered: false,
          enableRoaming: false,
          description: 'XTS download test!',
          filePath: "downloadFilePath",
          title: 'XTS download test!',
          background: false
        }
        try {
          await request.downloadFile(baseContext, downloadFileConfig);
          console.info('====>SUB_Misc_REQUEST_Download_Promise_0030 Download success.');
          // expect().assertFail();
        } catch (err) {
          console.info("====>SUB_Misc_REQUEST_Download_Promise_0030 except error: " + JSON.stringify(err));
          // expect(err.code).assertEqual(401);
          // done();
        }
      });

    //
    it('Download_Test_0400', Level.LEVEL1,
      async (done: () => void): Promise<void> => {
        console.info("====>---------------------Download_Test_0400 is starting---------------------");
        let downloadFilePath = `${baseContext.tempDir}/Download_Test_0400.txt`
        console.info("====>downloadFileConfig:" + `${baseContext.tempDir}/Download_Test_0400.txt`)
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
          console.info("====>Download_Test_0400 downloadTask: " + downloadTask);
          try {
            if (err) {
              console.info('====>Download_Test_0400 downloadFile fail: ' + JSON.stringify(err));
              expect().assertFail();
            }
            expect(downloadTask != undefined).assertEqual(true);
            downloadTask?.onComplete(() => {
              try {
                downloadTask?.offComplete();
                console.info('====>Download_Test_0400 complete success');
                fs.unlinkSync(downloadFilePath);
                console.info('====>Download_Test_0400 delete files success');
                // await downloadTask.delete();
                console.info('====>Download_Test_0400 delete downloadTask');
                // expect(true).assertTrue();
                // done();
              } catch (err) {
                console.info("====>Download_Test_0400 complete error: " + JSON.stringify(err));
                // done();
              }
            });
          } catch (err) {
            console.info("====>Download_Test_0400 error: " + JSON.stringify(err));
            // await downloadTask.delete();
            fs.unlinkSync(downloadFilePath);
            // done();
          }
        })
      });
    
    it('Download_Test_0600', Level.LEVEL1,
      async (done: () => void): Promise<void> => {
        console.info("====>---------------------Download_Test_0600 is starting---------------------");
        let downloadFilePath = `${baseContext.tempDir}/Download_Test_0600.txt`
        console.info("====>downloadFileConfig:" + `${baseContext.tempDir}/Download_Test_0600.txt`);
        let downloadFileConfig: request.DownloadConfig = {
          url: 'https://xxx.com.apk',
          header: {
            'headers': 'http'
          } as Record<string, string>,
          enableMetered: false,
          enableRoaming: false,
          description: 'XTS download test!',
          filePath: downloadFilePath,
          title: 'XTS download test!',
        }
        let flag = true;
        request.downloadFile(baseContext, downloadFileConfig, (err, downloadTask) => {
          console.info("====>Download_Test_0600 downloadTask: " + downloadTask);
          try {
            if (err) {
              console.info('====>Download_Test_0600 downloadFile fail: ' + JSON.stringify(err));
              expect().assertFail();
            }
            expect(downloadTask != undefined).assertEqual(true);
            downloadTask?.onFail(() => {
              flag = false;
              console.info("====>Download_Test_0600 complete flag " + flag);
            });
            downloadTask?.offFail();
            if (flag) {
              console.info('====>Download_Test_0600 onFail success');
              fs.unlinkSync(downloadFilePath);
              console.info('====>Download_Test_0600 delete files success');
              // await downloadTask.delete();
              console.info('====>Download_Test_0600 delete downloadTask');
              expect(true).assertTrue();
            } else {
              console.info("====>Download_Test_0600 onFail error: " + JSON.stringify(err));
              expect().assertFail();
            }
          } catch (err) {
            console.info("====>Download_Test_0600 error: " + JSON.stringify(err));
            // await downloadTask.delete();
            fs.unlinkSync(downloadFilePath);
          }
        })
      });


    it('Download_Test_0700', Level.LEVEL1,
      async (done: () => void): Promise<void> => {
        console.info("====>---------------------Download_Test_0700 is starting---------------------");
        let downloadFilePath = `${baseContext.tempDir}/Download_Test_0700.txt`
        console.info("====>downloadFileConfig:" + `${baseContext.tempDir}/Download_Test_0700.txt`);
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
        let flag = true;
        request.downloadFile(baseContext, downloadFileConfig, (err, downloadTask) => {
          console.info("====>Download_Test_0700 downloadTask: " + downloadTask);
          try {
            if (err) {
              console.info('====>Download_Test_0700 downloadFile fail: ' + JSON.stringify(err));
              expect().assertFail();
            }
            expect(downloadTask != undefined).assertEqual(true);
            downloadTask?.onComplete(() => {
              flag = false;
              console.info("====>Download_Test_0700 complete flag " + flag);
            });
            downloadTask?.offComplete();
            downloadTask?.onProgress((size, totalSize) => {
              console.info(`====>size: ${size}; totalSize  ${totalSize}`);
              if (size === totalSize) {
                try {
                  console.info('====>Download_Test_0700 into expect');
                  downloadTask?.offProgress();
                  if (flag) {
                    console.info('====>Download_Test_0700 onFail success');
                    fs.unlinkSync(downloadFilePath);
                    console.info('====>Download_Test_0700 delete files success');
                    // await downloadTask.delete();
                    console.info('====>Download_Test_0700 delete downloadTask');
                    expect(true).assertTrue();
                    // done();
                  } else {
                    console.info("====>Download_Test_0700 onFail error: " + JSON.stringify(err));
                    expect().assertFail();
                    // done();/
                  }
                } catch (err) {
                  console.info('====>Download_Test_0700 catch err: ' + JSON.stringify(err));
                  // done();
                }
              }
            })
          } catch (err) {
            console.info("====>Download_Test_0700 error: " + JSON.stringify(err));
            // await downloadTask.delete();
            fs.unlinkSync(downloadFilePath);
            // done();
          }
        })
      });


    it('Download_Test_1100', Level.LEVEL1,
      async (done: () => void): Promise<void> => {
        console.info("====>---------------------Download_Test_1100 is starting---------------------");
        let downloadFilePath = `${baseContext.tempDir}/Download_Test_1100.txt`
        let downloadFilePath1 = `${baseContext.tempDir}/Download_Test1_1100.txt`
        let downloadFilePath2 = `${baseContext.tempDir}/Download_Test2_1100.txt`
        console.info("====>downloadFileConfig:" + `${baseContext.tempDir}/Download_Test_1100.txt`)
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
        }
        try {
          let task = await request.downloadFile(baseContext, downloadFileConfig);
          console.info('====>Download_Test_1100 task1 success');

          let downloadFileConfig1 = downloadFileConfig
          downloadFileConfig1.filePath = downloadFilePath1
          let task1 = await request.downloadFile(baseContext, downloadFileConfig1);
          console.info('====>Download_Test_1100 task2 success');

          let downloadFileConfig2 = downloadFileConfig
          downloadFileConfig2.filePath = downloadFilePath2
          let task2 = await request.downloadFile(baseContext, downloadFileConfig2);
          console.info('====>Download_Test_1100 task2 success');

          await task.delete();
          console.info('====>Download_Test_1100 task delete success');
          await task1.delete();
          console.info('====>Download_Test_1100 task1 delete success');
          await task2.delete();
          console.info('====>Download_Test_1100 task2 delete success');
          fs.unlinkSync(downloadFilePath);
          console.info('====>Download_Test_1100 task delete file success');
          fs.unlinkSync(downloadFilePath1);
          console.info('====>Download_Test_1100 task1 delete file success');
          fs.unlinkSync(downloadFilePath2);
          console.info('====>Download_Test_1100 task2 delete file success');
          expect(true).assertTrue();
          // done();
        } catch (err) {
          console.info("====>Download_Test_1100 error: " + JSON.stringify(err));
          expect().assertFail();
          // done();
        }
      });


    it('Download_Test_0100', Level.LEVEL1,
      async (done: () => void): Promise<void> => {
        console.info("====>Download_Test_0100 is starting");
        let downloadFileConfig: request.DownloadConfig = {
          url: 'https://mirror.bjtu.edu.cn/kernel/linux/devel/binutils/biniutils-2.11.90.0.23-2.11.90.0.24.sign',
          header: {} as Record<string, string>,
          enableMetered: true,
          enableRoaming: true,
          description: 'a2',
          networkType: 1,
          filePath: "/system/lib/abc2.txt",
          title: 'a2'
        }
        try {
          await request.downloadFile(baseContext, downloadFileConfig);
          console.info('====>Download_Test_0100 Download success.');
          expect().assertFail();
        } catch (err) {
          console.info("====>Download_Test_0100 except error: " + JSON.stringify(err));
          // expect(err.code).assertEqual(13400001);
          // done();
        }
      });

    it('SUB_REQUEST_DownloadManagement_Download_0500', Level.LEVEL1,
      async (done: () => void): Promise<void> => {
        console.info('-----------------------SUB_REQUEST_DownloadManagement_Download_0500 is starting-----------------------');
        let downloadFilePath = `${baseContext.tempDir}/SUB_REQUEST_DownloadManagement_Download_0500.txt`;
        console.info('====>downloadFileConfig:' +
          `${baseContext.tempDir}/SUB_REQUEST_DownloadManagement_Download_0500.txt`);
        let downloadFileConfig: request.DownloadConfig = {
          url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
          description: 'XTS download test!',
          networkType: request.NETWORK_WIFI,
          filePath: downloadFilePath,
          title: 'XTS download test!',
          background: false
        }
        request.downloadFile(baseContext, downloadFileConfig, (err, downloadTask) => {
          console.info('====>SUB_REQUEST_DownloadManagement_Download_0500 downloadTask: ' + downloadTask);
          let onProgress = (receivedSize: long, totalSize: long) => {
            try {
              downloadTask?.offProgress();
              console.info('====>SUB_REQUEST_DownloadManagement_Download_0500 onProgress success: ');
              // let ifDelete: boolean = await downloadTask.delete();
              // expect(ifDelete === true).assertTrue();
              // console.info('====>SUB_REQUEST_DownloadManagement_Download_0500 delete ifDelete: ' + ifDelete);
              fs.unlinkSync(downloadFilePath);
              console.info('====>SUB_REQUEST_DownloadManagement_Download_0500 unlinkSync success: ');
              // done();
            } catch (err) {
              console.info('====>SUB_REQUEST_DownloadManagement_Download_0500 onProgress err: ' + JSON.stringify(err));
              // done();
            }
          }
          try {
            if (err) {
              console.info('====>SUB_REQUEST_DownloadManagement_Download_0500 downloadFile fail: ' +
              JSON.stringify(err));
              expect().assertFail();
            }
            downloadTask?.onProgress(onProgress);
          } catch (error) {
            // await deleteTask(downloadTask, downloadFilePath)
            console.info('====>SUB_REQUEST_DownloadManagement_Download_0500 catch error: ' + error);
          }
        });
      });

    it('SUB_REQUEST_DownloadManagement_Download_0300', Level.LEVEL1,
      async (done: () => void): Promise<void> => {
        console.info('-----------------------SUB_REQUEST_DownloadManagement_Download_0300 is starting-----------------------');
        let downloadFilePath = `${baseContext.tempDir}/SUB_REQUEST_DownloadManagement_Download_0300.txt`;
        console.info('====>downloadFileConfig:' +
          `${baseContext.tempDir}/SUB_REQUEST_DownloadManagement_Download_0300.txt`);
        let downloadFileConfig: request.DownloadConfig = {
          url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
          description: 'XTS download test!',
          networkType: request.NETWORK_WIFI,
          filePath: downloadFilePath,
          title: 'XTS download test!',
          background: false
        }
        request.downloadFile(baseContext, downloadFileConfig, (err, downloadTask) => {
          console.info('====>SUB_REQUEST_DownloadManagement_Download_0300 downloadTask: ' + downloadTask);
          let onProgress = (receivedSize: long, totalSize: long) => {
            try {
              downloadTask?.offProgress();
              console.info('====>SUB_REQUEST_DownloadManagement_Download_0300 onProgress success: ');
              downloadTask?.suspend();
              console.info('====>SUB_REQUEST_DownloadManagement_Download_0300 downloadTask.pause success: ');
            } catch (err) {
              // await deleteTask(downloadTask, downloadFilePath);
              console.info('====>SUB_REQUEST_DownloadManagement_Download_0300 onProgress err: ' + JSON.stringify(err));
              // done();
            }
          }

          let pauseProgress = () => {
            try {

              console.info('====>SUB_REQUEST_DownloadManagement_Download_0300 pauseProgress success: ');
              // expect(downloadTask !== null).assertTrue();
              // done();
            } catch (err) {
              console.info('====>SUB_REQUEST_DownloadManagement_Download_0300 pauseProgress err: ' +
              JSON.stringify(err));
              // done();
            }
          }

          try {
            if (err) {
              console.info('====>SUB_REQUEST_DownloadManagement_Download_0300 downloadFile fail: ' +
              JSON.stringify(err));
              expect().assertFail();
            }
            downloadTask?.onProgress(onProgress);
            downloadTask?.onPause(pauseProgress);
            // await deleteTask(downloadTask, downloadFilePath);
            downloadTask?.offPause(pauseProgress);
          } catch (error) {
            // await deleteTask(downloadTask, downloadFilePath)
            console.info('====>SUB_REQUEST_DownloadManagement_Download_0300 catch error: ' + error);
          }
        });
      });

      it('SUB_REQUEST_DownloadManagement_Download_0300', Level.LEVEL1,
      async (done: () => void): Promise<void> => {
        console.info('-----------------------SUB_REQUEST_DownloadManagement_Download_0300 is starting-----------------------');
        let downloadFilePath = `${baseContext.tempDir}/SUB_REQUEST_DownloadManagement_Download_0300.txt`;
        console.info('====>downloadFileConfig:' +
          `${baseContext.tempDir}/SUB_REQUEST_DownloadManagement_Download_0300.txt`);
        let downloadFileConfig: request.DownloadConfig = {
          url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
          description: 'XTS download test!',
          networkType: request.NETWORK_WIFI,
          filePath: downloadFilePath,
          title: 'XTS download test!',
          background: false
        }
        request.downloadFile(baseContext, downloadFileConfig, (err, downloadTask) => {
          console.info('====>SUB_REQUEST_DownloadManagement_Download_0300 downloadTask: ' + downloadTask);
          let onProgress = (receivedSize: long, totalSize: long) => {
            try {
              downloadTask?.offProgress();
              console.info('====>SUB_REQUEST_DownloadManagement_Download_0300 onProgress success: ');
              downloadTask?.suspend();
              console.info('====>SUB_REQUEST_DownloadManagement_Download_0300 downloadTask.pause success: ');
            } catch (err) {
              // await deleteTask(downloadTask, downloadFilePath);
              console.info('====>SUB_REQUEST_DownloadManagement_Download_0300 onProgress err: ' + JSON.stringify(err));
              // done();
            }
          }

          let pauseProgress = () => {
            try {
              // await deleteTask(downloadTask, downloadFilePath);
              downloadTask?.offPause();
              console.info('====>SUB_REQUEST_DownloadManagement_Download_0300 pauseProgress success: ');
              // expect(downloadTask !== null).assertTrue();
              // done();
            } catch (err) {
              console.info('====>SUB_REQUEST_DownloadManagement_Download_0300 pauseProgress err: ' +
              JSON.stringify(err));
              // done();
            }
          }

          try {
            if (err) {
              console.info('====>SUB_REQUEST_DownloadManagement_Download_0300 downloadFile fail: ' +
              JSON.stringify(err));
              expect().assertFail();
            }
            downloadTask?.onProgress(onProgress);
            downloadTask?.onPause(pauseProgress);
          } catch (error) {
            // await deleteTask(downloadTask, downloadFilePath)
            console.info('====>SUB_REQUEST_DownloadManagement_Download_0300 catch error: ' + error);
          }
        });
      });

    it('SUB_REQUEST_DownloadManagement_DownloadSuccess_0100', Level.LEVEL1,
      async (done: () => void): Promise<void> => {
        console.info('-----------------------SUB_REQUEST_DownloadManagement_DownloadSuccess_0100 is starting-----------------------');
        let downloadFilePath = `${baseContext.tempDir}/SUB_REQUEST_DownloadManagement_DownloadSuccess_0100.txt`;
        console.info('====>downloadFileConfig:' +
          `${baseContext.tempDir}/SUB_REQUEST_DownloadManagement_DownloadSuccess_0100.txt`);
        let downloadFileConfig: request.DownloadConfig = {
          url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
          description: 'XTS download test!',
          networkType: request.NETWORK_WIFI,
          filePath: downloadFilePath,
          title: 'XTS download test!',
          background: false
        }
        request.downloadFile(baseContext, downloadFileConfig, (err, downloadTask) => {
          console.info('====>SUB_REQUEST_DownloadManagement_DownloadSuccess_0100 downloadTask: ' + downloadTask);
          let completeProgress = () => {
            try {
              // downloadTask.off('complete');
              console.info('====>SUB_REQUEST_DownloadManagement_DownloadSuccess_0100 completeProgress success: ');
              // await deleteTask(downloadTask, downloadFilePath);
              expect(downloadTask !== null).assertTrue();
              // done();
            } catch (err) {
              console.info('====>SUB_REQUEST_DownloadManagement_DownloadSuccess_0100 completeProgress err: ' +
              JSON.stringify(err));
              // done();
            }
          }

          try {
            if (err) {
              console.info('====>SUB_REQUEST_DownloadManagement_DownloadSuccess_0100 downloadFile fail: ' +
              JSON.stringify(err));
              expect().assertFail();
            }
            downloadTask?.onComplete(completeProgress);
          } catch (error) {
            // await deleteTask(downloadTask, downloadFilePath)
            console.info('====>SUB_REQUEST_DownloadManagement_DownloadSuccess_0100 catch error: ' + error);
          }
        });
      });

    it('SUB_REQUEST_SupportSystem_DownloadMultitask_0100', Level.LEVEL1,
      async (done: () => void): Promise<void> => {
        console.info('-----------------------SUB_REQUEST_SupportSystem_DownloadMultitask_0100 is starting-----------------------');
        let downloadFilePath1 = `${baseContext.tempDir}/downloadFilePath1.txt`;
        let downloadFilePath2 = `${baseContext.tempDir}/downloadFilePath2.txt`;
        let downloadFileConfig1: request.DownloadConfig = {
          url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
          description: 'XTS download test!',
          networkType: request.NETWORK_WIFI,
          filePath: downloadFilePath1,
          title: 'XTS download test!',
          background: false
        }

        let downloadFileConfig2: request.DownloadConfig = {
          url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
          description: 'XTS download test!',
          networkType: request.NETWORK_WIFI,
          filePath: downloadFilePath2,
          title: 'XTS download test!',
          background: false
        }

        let complete1: boolean = false;
        let complete2: boolean = false;
        let downloadTask1 = await request.downloadFile(baseContext, downloadFileConfig1);
        let downloadTask2 = await request.downloadFile(baseContext, downloadFileConfig2);
        console.info('====>SUB_REQUEST_SupportSystem_DownloadMultitask_0100 create tow tasks success');
        let completeProgress1 = () => {
          complete1 = true;
          downloadTask1?.offComplete();
          console.info('====>SUB_REQUEST_SupportSystem_DownloadMultitask_0100 completeProgress1 success');
          // await deleteTask(downloadTask1, downloadFilePath1);
          if (complete2) {
            console.info('====>SUB_REQUEST_SupportSystem_DownloadMultitask_0100 both complete1');
            expect(downloadTask1 !== null).assertTrue();
            // done();
          } else {
            console.info('====>SUB_REQUEST_SupportSystem_DownloadMultitask_0100 completeProgress1 complete');
          }
        }

        let completeProgress2 = () => {
          complete2 = true;
          downloadTask2?.offComplete();
          console.info('====>SUB_REQUEST_SupportSystem_DownloadMultitask_0100 completeProgress2 success: ');
          // await deleteTask(downloadTask2, downloadFilePath2);
          if (complete1) {
            console.info('====>SUB_REQUEST_SupportSystem_DownloadMultitask_0100 both complete2');
            expect(downloadTask1 !== null).assertTrue();
            // done();
          } else {
            console.info('====>SUB_REQUEST_SupportSystem_DownloadMultitask_0100 completeProgress2 complete');
          }
        }
        downloadTask1?.onComplete(completeProgress1);
        downloadTask2?.onComplete(completeProgress2);
      });


    it('SUB_REQUEST_SupportSystem_DownloadSuccess_0300', Level.LEVEL1,
      async (done: () => void): Promise<void> => {
        console.info('-----------------------SUB_REQUEST_SupportSystem_DownloadSuccess_0300 is starting-----------------------');
        let downloadFilePath = `${baseContext.tempDir}/SUB_REQUEST_SupportSystem_DownloadSuccess_0300.txt`
        console.info('====>downloadFileConfig:' +
          `${baseContext.tempDir}/SUB_REQUEST_SupportSystem_DownloadSuccess_0300.txt`)
        let downloadFileConfig: request.DownloadConfig = {
          url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
          header: {
            'Content-Type': 'application/octet-stream'
          } as Record<string, string>,
          description: 'XTS download test!',
          networkType: request.NETWORK_WIFI,
          filePath: downloadFilePath,
          title: 'XTS download test!',
          background: false
        }
        request.downloadFile(baseContext, downloadFileConfig, (err, downloadTask) => {
          console.info('====>SUB_REQUEST_SupportSystem_DownloadSuccess_0300 downloadTask: ' + downloadTask);
          let completeProgress = () => {
            try {
              downloadTask?.offComplete();
              console.info('====>SUB_REQUEST_SupportSystem_DownloadSuccess_0300 completeProgress success: ');
              // await deleteTask(downloadTask, downloadFilePath);
              expect(downloadTask !== null).assertTrue();
              // done();
            } catch (err) {
              console.info('====>SUB_REQUEST_SupportSystem_DownloadSuccess_0300 completeProgress err: ' +
              JSON.stringify(err));
              // done();
            }
          }

          try {
            if (err) {
              console.info('====>SUB_REQUEST_SupportSystem_DownloadSuccess_0300 downloadFile fail: ' +
              JSON.stringify(err));
              expect().assertFail();
            }
            downloadTask?.onComplete(completeProgress);
          } catch (error) {
            // await deleteTask(downloadTask, downloadFilePath)
            console.info('====>SUB_REQUEST_SupportSystem_DownloadSuccess_0300 catch error: ' + error);
          }
        });
      });


    it('SUB_Request_download_Default_0100', Level.LEVEL1,
      async (done: () => void): Promise<void> => {
        console.info('-----------------------SUB_Request_download_Default_0100 is starting-----------------------');
        let downloadFilePath = `${baseContext.cacheDir}/test.hap`
        console.info('====>downloadFileConfig:' + `${baseContext.cacheDir}/test.hap`)
        let downloadFileConfig: request.DownloadConfig = {
          url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
          filePath: undefined,
          header: undefined,
          enableMetered: undefined,
          enableRoaming: undefined,
          description: undefined,
          networkType: undefined,
          title: undefined,
          background: undefined,
        }
        request.downloadFile(baseContext, downloadFileConfig, (err, downloadTask) => {
          console.info('====>SUB_Request_download_Default_0100 downloadTask: ' + downloadTask);
          let completeProgress = () => {
            try {
              downloadTask?.offComplete();
              console.info('====>SUB_Request_download_Default_0100 completeProgress success: ');
              // await deleteTask(downloadTask, downloadFilePath);
              expect(downloadTask !== null).assertTrue();
              // done();
            } catch (err) {
              console.info('====>SUB_Request_download_Default_0100 completeProgress err: ' + JSON.stringify(err));
              // done();
            }
          }

          try {
            if (err) {
              console.info('====>SUB_Request_download_Default_0100 downloadFile fail: ' + JSON.stringify(err));
              expect().assertFail();
            }
            downloadTask?.onComplete(completeProgress);
          } catch (error) {
            // await deleteTask(downloadTask, downloadFilePath)
            console.info('====>SUB_Request_download_Default_0100 catch error: ' + error);
          }
        });
      });


    it('SUB_MISC_REQUEST_DOWNLOAD_removeNormal_promise_0010', Level.LEVEL1,
      async (done: () => void): Promise<void> => {
        console.info("====>-----------------------SUB_MISC_REQUEST_DOWNLOAD_removeNormal_promise_0010 is starting-----------------------");
        try {
          console.info("====>SUB_MISC_REQUEST_DOWNLOAD_removeNormal_promise_0010 remove");
          await request.agent.remove("1234567");
          expect().assertFail();
        } catch (err) {
          console.info("====>SUB_MISC_REQUEST_DOWNLOAD_removeNormal_promise_0010 catch error: " + JSON.stringify(err));
          // expect(err.code).assertEqual(13400003);
        }
        console.info("-----------------------SUB_MISC_REQUEST_DOWNLOAD_removeNormal_promise_0010 end-----------------------");
      });


    it('SUB_MISC_REQUEST_UPLOAD_removeNormal_callback_0010', Level.LEVEL1,
      async (done: () => void): Promise<void> => {
        console.info("====>-----------------------SUB_MISC_REQUEST_DOWNLOAD_removeNormal_callback_0010 is starting-----------------------");
        request.agent.remove("1234567", (err: BusinessError | null) => {
          try {
            if (err) {
              console.info("====>SUB_MISC_REQUEST_DOWNLOAD_removeNormal_callback_0010 err.message: " + err.message);
              console.info("====>SUB_MISC_REQUEST_DOWNLOAD_removeNormal_callback_0010 err.code: " + err.code);
              // expect(err.code).assertEqual(13400003);
            } else {
              expect().assertFail();
            }
            // done();
          } catch (err) {
            console.info("====>SUB_MISC_REQUEST_UPLOAD_removeNormal_callback_0010 remove fail: " + JSON.stringify(err));
            // done();
          }
        });
      });


    it('SUB_Request_Agent_ErrorDetailed_0500', Level.LEVEL1,
      async (done: () => void): Promise<void> => {
        console.info('====>-----------------------SUB_Request_Agent_ErrorDetailed_0500 is starting-----------------------');
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: 'https://singnn.com',
          title: 'createTest',
          description: 'XTS download test!',
          saveas: './SUB_Request_Agent_ErrorDetailed_0500',
          network: request.agent.Network.ANY,
          overwrite: true
        };
        try {
          let task: request.agent.Task = await request.agent.create(baseContext, config);
          task.on('failed', (pro: request.agent.Progress) => {
            try {
              task.off('failed');
              console.info('====>SUB_Request_Agent_ErrorDetailed_0500 failed pro: ' + JSON.stringify(pro));
              let info: request.agent.TaskInfo = await request.agent.show(task.tid);
              console.info('====>SUB_Request_Agent_ErrorDetailed_0500 info: ' + JSON.stringify(info));
              expect(info.faults).assertEqual(80);
              // done();
            } catch (err) {
              console.info('====>SUB_Request_Agent_ErrorDetailed_0500 err: ' + JSON.stringify(err));
            }
          })
          await task.start();
          console.info('====>SUB_Request_Agent_ErrorDetailed_0500 create success');
        } catch (err) {
          console.info('====>SUB_Request_Agent_ErrorDetailed_0500 create catch err: ' + JSON.stringify(err));
          expect().assertFail();
          // done();
        }
      });


    it('Download_Test_0300', Level.LEVEL1,
      async (done: () => void): Promise<void> => {
        console.info("====>---------------------Download_Test_0300 is starting---------------------");
        let downloadFilePath = `${baseContext.tempDir}/Download_Test_0300.txt`
        console.info("====>downloadFileConfig:" + `${baseContext.tempDir}/Download_Test_0300.txt`);
        let downloadFileConfig: request.DownloadConfig = {
          url: 'https://xxx.com.apk',
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
          console.info("====>Download_Test_0300 downloadTask: " + downloadTask);
          try {
            if (err) {
              console.info('====>Download_Test_0300 downloadFile fail: ' + JSON.stringify(err));
              expect().assertFail();
            }
            expect(downloadTask != undefined).assertEqual(true);
            downloadTask?.onFail(() => {
              try {
                downloadTask?.offFail();
                console.info('====>Download_Test_0300 fail success');
                fs.unlinkSync(downloadFilePath);
                console.info('====>Download_Test_0300 delete files success');
                // await downloadTask.delete();
                console.info('====>Download_Test_0300 delete downloadTask');
                expect(true).assertTrue();
                // done();
              } catch (err) {
                console.info("====>Download_Test_0300 fail error: " + JSON.stringify(err));
                // done();
              }
            });
          } catch (err) {
            console.info("====>Download_Test_0300 error: " + JSON.stringify(err));
            // await downloadTask?.delete();
            fs.unlinkSync(downloadFilePath);
            // done();
          }
        })
      });

    it('SUB_Request_Agent_TaskSaveas_0400', Level.LEVEL1,
      async (done: () => void): Promise<void> => {
        console.info("====>-----------------------SUB_Request_Agent_TaskSaveas_0400 is starting-----------------------");
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
          title: 'createTest',
          description: 'XTS download test!',
          saveas: 'SUB_Request_Agent_CreateTaskErrorCode_Callback_0100',
          network: request.agent.Network.WIFI,
          overwrite: true,
          mode: request.agent.Mode.BACKGROUND
        };
        try {
          for (let i = 0; i < 1002; i++) {
            request.agent.create(baseContext, config,
              (err: BusinessError | null, task: request.agent.Task | undefined) => {
                try {
                  if (err) {
                    console.info("====>SUB_Request_Agent_CreateTaskErrorCode_Callback_0100 create err: " +
                    JSON.stringify(err));
                    expect(err.code).assertEqual(21900004);
                    request.agent.search((err: BusinessError | null, tasksId: Array<string> | undefined) => {
                      let ids = (tasksId as Array<string>);
                      for (let i = 0; i < ids.length; i++) {
                        request.agent.remove(ids[i]);
                      }
                    });
                    // done();
                  } else {
                    console.info("====>SUB_Request_Agent_CreateTaskErrorCode_Callback_0100 create times: " + i);
                  }
                } catch (err) {
                  console.info("====>SUB_Request_Agent_CreateTaskErrorCode_Callback_0100 create catch err: " +
                  JSON.stringify(err));
                  // done();
                }
              });
          }
        } catch (err) {
          console.info("====>SUB_Request_Agent_CreateTaskErrorCode_Callback_0100 create catch err: " +
          JSON.stringify(err));
          expect().assertFail();
        }
      });

    it('SUB_Request_Download_on_faultOccur_0400', Level.LEVEL1,
      async (done: () => void): Promise<void> => {
        console.info("-----------------------SUB_Request_Download_on_faultOccur_0400 is starting-----------------------");
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: 'https://gitee111.com/chenzhixue/downloadTest/releases/download/V1.0/test1.txt',
          title: 'reeateTest',
          overwrite: true,
          description: 'XTS download test!',
          saveas: './SUB_Request_Download_on_faultOccur_0400.txt',
          network: request.agent.Network.ANY,
          mode: request.agent.Mode.BACKGROUND
        };
        let task: request.agent.Task = await request.agent.create(baseContext, config);
        let faultOccurCallBack = (faults: request.agent.Faults) => {
          try {
            task.off('faultOccur');
            console.info("====>SUB_Request_Download_on_faultOccur_0400 faults: " + faults);
            expect(faults).assertEqual(request.agent.Faults.DNS);
            // done();
          } catch (err) {
            console.info("====>SUB_Request_Download_on_faultOccur_0400 faultOccurCallBack err: " + JSON.stringify(err));
            // done();
          }
        }

        console.info("====>SUB_Request_Download_on_faultOccur_0400 create task success");
        task.on('faultOccur', faultOccurCallBack);
        await task.start();
      });

    it('SUB_Request_Download_on_faultOccur_0400', Level.LEVEL1,
      async (done: () => void): Promise<void> => {
        console.info("-----------------------SUB_Request_Download_on_faultOccur_0400 is starting-----------------------");
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: 'https://127.0.0.1',
          title: 'reeateTest',
          overwrite: true,
          description: 'XTS download test!',
          saveas: './SUB_Request_Download_on_faultOccur_0400.txt',
          network: request.agent.Network.ANY,
          mode: request.agent.Mode.BACKGROUND
        };
        let task: request.agent.Task = await request.agent.create(baseContext, config);
        let faultOccurCallBack = (faults: request.agent.Faults) => {
          try {
            task.off('faultOccur');
            console.info("====>SUB_Request_Download_on_faultOccur_0400 faults: " + faults);
            expect(faults).assertEqual(request.agent.Faults.TCP);
            // done();
          } catch (err) {
            console.info("====>SUB_Request_Download_on_faultOccur_0400 faultOccurCallBack err: " + JSON.stringify(err));
            // done();
          }
        }

        console.info("====>SUB_Request_Download_on_faultOccur_0400 create task success");
        task.on('faultOccur', faultOccurCallBack);
        await task.start();
      });

    it('SUB_REQUEST_FileUpload_UploadTaskCreation_0300', Level.LEVEL1,
      async (done: () => void): Promise<void> => {
        console.info('-----------------------SUB_REQUEST_FileUpload_UploadTaskCreation_0300 is starting-----------------------');
        let data: request.RequestData = {
          name: 'name',
          value: '123'
        }
        // let uploadConfig1: request.UploadConfig = JSON.parse(JSON.stringify(uploadConfig));
        let files: Array<request.File> = [{
          filename: 'test',
          name: 'test',
          uri: 'internal://cache/test.txt',
          type: 'txt'
        },
          {
            filename: 'test1',
            name: 'test1',
            uri: 'internal://cache/test1.txt',
            type: 'txt'
          }];
        let uploadConfig: request.UploadConfig = {
          url: 'http://127.0.0.1:8080',
          header: {
            'Accept': '*/*',
          },
          method: 'POST',
          files: files,
          data: [data]
        };
        request.uploadFile(baseContext, uploadConfig, (err, uploadTask) => {
          console.info('====>SUB_REQUEST_FileUpload_UploadTaskCreation_0300 uploadFile: ' + uploadTask);
          try {
            let completeCallback = (taskStates: Array<request.TaskState>) => {
              try {
                uploadTask?.offComplete();
                console.info('====>SUB_REQUEST_FileUpload_UploadTaskCreation_0300 complete: ' +
                JSON.stringify(taskStates));
                //expect(taskStates[0].responseCode).assertEqual(0);
                // await deleteTask(uploadTask);
                // done();
              } catch (err) {
                console.info('====>SUB_REQUEST_FileUpload_UploadTaskCreation_0300 catch err: ' + JSON.stringify(err));
                // await deleteTask(uploadTask);
                // done();
              }
            }
            if (err) {
              console.info('====>SUB_REQUEST_FileUpload_UploadTaskCreation_0300 downloadFile fail: ' +
              JSON.stringify(err));
              //expect().assertFail();
            }
            uploadTask?.onComplete(completeCallback);
            console.info('====>SUB_REQUEST_FileUpload_UploadTaskCreation_0300 on completeCallback success');
          } catch (err) {
            console.info('====>SUB_REQUEST_FileUpload_UploadTaskCreation_0300 except err: ' + err);
            // await deleteTask(uploadTask);/
            // done();
          }
        });
      });

    it('SUB_REQUEST_FileUpload_UploadTaskCreation_0300', Level.LEVEL1,
      async (done: () => void): Promise<void> => {
        console.info('-----------------------SUB_REQUEST_FileUpload_UploadTaskCreation_0300 is starting-----------------------');
        let data: request.RequestData = {
          name: 'name',
          value: '123'
        }
        // let uploadConfig1: request.UploadConfig = JSON.parse(JSON.stringify(uploadConfig));
        let files: Array<request.File> = [{
          filename: 'test',
          name: 'test',
          uri: 'internal://cache/test.txt',
          type: 'txt'
        }];
        let uploadConfig: request.UploadConfig = {
          url: 'http://127.0.0.1:8080',
          header: {
            'Accept': '*/*',
          },
          method: 'POST',
          files: files,
          data: [data]
        };
        request.uploadFile(baseContext, uploadConfig, (err, uploadTask) => {
          console.info('====>SUB_REQUEST_FileUpload_UploadTaskCreation_0400 uploadFile: ' + uploadTask);
          try {
            let completeCallback = (taskStates: Array<request.TaskState>) => {
              try {
                uploadTask?.offComplete();
                console.info('====>SUB_REQUEST_FileUpload_UploadTaskCreation_0400 complete: ' +
                JSON.stringify(taskStates));
                expect(taskStates[0].responseCode).assertEqual(0);
                // await deleteTask(uploadTask);
                // done();
              } catch (err) {
                console.info('====>SUB_REQUEST_FileUpload_UploadTaskCreation_0400 catch err: ' + JSON.stringify(err));
                // await deleteTask(uploadTask);
                // done();
              }
            }
            if (err) {
              console.info('====>SUB_REQUEST_FileUpload_UploadTaskCreation_0400 downloadFile fail: ' +
              JSON.stringify(err));
              expect().assertFail();
            }
            uploadTask?.onComplete(completeCallback);
            console.info('====>SUB_REQUEST_FileUpload_UploadTaskCreation_0400 on completeCallback success');
          } catch (err) {
            console.info('====>SUB_REQUEST_FileUpload_UploadTaskCreation_0400 except err: ' + err);
            // await deleteTask(uploadTask);/
            // done();
          }
        });
      })

    it('SUB_REQUEST_FileUpload_UploadTaskCreation_0500', Level.LEVEL1,
      async (done: () => void): Promise<void> => {
        console.info('-----------------------SUB_REQUEST_FileUpload_UploadTaskCreation_0500 is starting-----------------------');
        let data: request.RequestData = {
          name: 'name',
          value: '123'
        }
        // let uploadConfig1: request.UploadConfig = JSON.parse(JSON.stringify(uploadConfig));
        let files: Array<request.File> = [{
          filename: 'test',
          name: 'test',
          uri: 'internal://cache/test.txt',
          type: 'txt'
        },
          {
            filename: 'test',
            name: 'test',
            uri: 'internal://cache/test.jpg',
            type: 'jpg'
          },
          {
            filename: 'test',
            name: 'test',
            uri: 'internal://cache/test.mp4',
            type: 'mp4'
          },
          {
            filename: 'test',
            name: 'test',
            uri: 'internal://cache/test.zip',
            type: 'zip'
          },
          {
            filename: 'test',
            name: 'test',
            uri: 'internal://cache/test.hap',
            type: 'hap'
          },
          {
            filename: 'test',
            name: 'test',
            uri: 'internal://cache/test.pdf',
            type: 'pdf'
          }
        ];
        let uploadConfig: request.UploadConfig = {
          url: 'http://127.0.0.1:8080',
          header: {
            'Accept': '*/*',
          },
          method: 'POST',
          files: files,
          data: [data]
        };
        request.uploadFile(baseContext, uploadConfig, (err, uploadTask) => {
          console.info('====>SUB_REQUEST_FileUpload_UploadTaskCreation_0500 uploadFile: ' + uploadTask);
          try {
            let completeCallback = (taskStates: Array<request.TaskState>) => {
              try {
                uploadTask?.offComplete();
                console.info('====>SUB_REQUEST_FileUpload_UploadTaskCreation_0500 complete: ' +
                JSON.stringify(taskStates));
                //expect(taskStates[0].responseCode).assertEqual(0);
                // await deleteTask(uploadTask);
                // done();
              } catch (err) {
                console.info('====>SUB_REQUEST_FileUpload_UploadTaskCreation_0500 catch err: ' + JSON.stringify(err));
                // await deleteTask(uploadTask);
                // done();
              }
            }
            if (err) {
              console.info('====>SUB_REQUEST_FileUpload_UploadTaskCreation_0500 downloadFile fail: ' +
              JSON.stringify(err));
              //expect().assertFail();
            }
            uploadTask?.onComplete(completeCallback);
            console.info('====>SUB_REQUEST_FileUpload_UploadTaskCreation_0500 on completeCallback success');
          } catch (err) {
            console.info('====>SUB_REQUEST_FileUpload_UploadTaskCreation_0500 except err: ' + err);
            // await deleteTask(uploadTask);/
            // done();
          }
        });
      });

    it('SUB_REQUEST_FileUpload_UploadTaskCreation_0300', Level.LEVEL1,
      async (done: () => void): Promise<void> => {
        console.info('-----------------------SUB_REQUEST_FileUpload_UploadTaskCreation_0300 is starting-----------------------');
        let data: request.RequestData = {
          name: 'name',
          value: '123'
        }
        // let uploadConfig1: request.UploadConfig = JSON.parse(JSON.stringify(uploadConfig));
        let files: Array<request.File> = [{
          filename: 'test',
          name: 'test',
          uri: 'internal://cache/test.txt',
          type: 'txt'
        }];
        let uploadConfig: request.UploadConfig = {
          url: 'http://127.0.0.1:8080',
          header: {
            'Accept': '*/*',
          },
          method: 'POST',
          files: files,
          data: [data]
        };
        console.info('-----------------------SUB_REQUEST_FileUpload_UploadTaskCreation_0700 is starting-----------------------');
        let complete1: boolean = false;
        let complete2: boolean = false;
        let uploadTask1: request.UploadTask = await request.uploadFile(baseContext, uploadConfig);
        let uploadTask2: request.UploadTask = await request.uploadFile(baseContext, uploadConfig);
        console.info('====>SUB_REQUEST_FileUpload_UploadTaskCreation_0700 create tow tasks success');
        let completeProgress1 = () => {
          complete1 = true;
          uploadTask1.offComplete();
          console.info('====>SUB_REQUEST_FileUpload_UploadTaskCreation_0700 completeProgress1 success');
          // await deleteTask(uploadTask1);
          if (complete2) {
            console.info('====>SUB_REQUEST_FileUpload_UploadTaskCreation_0700 both complete1');
            //expect(uploadTask1 !== null).assertTrue();
            // done();
          } else {
            console.info('====>SUB_REQUEST_FileUpload_UploadTaskCreation_0700 completeProgress1 complete');
          }
        }

        let completeProgress2 = () => {
          complete2 = true;
          uploadTask2.offComplete();
          console.info('====>SUB_REQUEST_FileUpload_UploadTaskCreation_0700 completeProgress2 success: ');
          // await deleteTask(uploadTask2);
          if (complete1) {
            console.info('====>SUB_REQUEST_FileUpload_UploadTaskCreation_0700 both complete2');
            //expect(uploadTask1 !== null).assertTrue();
            // done();
          } else {
            console.info('====>SUB_REQUEST_FileUpload_UploadTaskCreation_0700 completeProgress2 complete');
          }
        }
        uploadTask1.onComplete(completeProgress1);
        uploadTask2.onComplete(completeProgress2);
        console.info('====>SUB_REQUEST_FileUpload_UploadTaskCreation_0700 on complete');
      });

    it('SUB_REQUEST_FileUpload_UploadTaskCreation_0300', Level.LEVEL1,
      async (done: () => void): Promise<void> => {
        console.info('-----------------------SUB_REQUEST_FileUpload_UploadTaskCreation_0300 is starting-----------------------');
        let data: request.RequestData = {
          name: 'name',
          value: '123'
        }
        // let uploadConfig1: request.UploadConfig = JSON.parse(JSON.stringify(uploadConfig));
        let files: Array<request.File> = [{
          filename: 'test',
          name: 'test',
          uri: 'internal://cache/test.txt',
          type: 'txt'
        }];
        let uploadConfig: request.UploadConfig = {
          url: 'http://127.0.0.1:8080',
          header: {
            'Accept': '*/*',
          },
          method: 'GET',
          files: files,
          data: [data]
        };
        request.uploadFile(baseContext, uploadConfig, (err, uploadTask) => {
          console.info('====>SUB_REQUEST_FileUpload_other_0100 uploadFile: ' + uploadTask);
          try {
            let completeCallback = (taskStates: Array<request.TaskState>) => {
              try {
                uploadTask?.offComplete();
                console.info('====>SUB_REQUEST_FileUpload_other_0100 complete: ' + JSON.stringify(taskStates));
                //expect(taskStates[0].responseCode).assertEqual(0);
                // await deleteTask(uploadTask);
                // done();
              } catch (err) {
                console.info('====>SUB_REQUEST_FileUpload_other_0100 catch err: ' + JSON.stringify(err));
                // await deleteTask(uploadTask);
                // done();
              }
            }
            if (err) {
              console.info('====>SUB_REQUEST_FileUpload_other_0100 downloadFile fail: ' + JSON.stringify(err));
              //expect().assertFail();
            }
            uploadTask?.onComplete(completeCallback);
            console.info('====>SUB_REQUEST_FileUpload_other_0100 on completeCallback success');
          } catch (err) {
            console.info('====>SUB_REQUEST_FileUpload_other_0100 except err: ' + err);
            // await deleteTask(uploadTask);/
            // done();
          }
        });
      })

    it('SUB_REQUEST_FileUpload_UploadTaskCreation_0300', Level.LEVEL1,
      async (done: () => void): Promise<void> => {
        console.info('-----------------------SUB_REQUEST_FileUpload_UploadTaskCreation_0300 is starting-----------------------');
        let data: request.RequestData = {
          name: 'name',
          value: '123'
        }
        // let uploadConfig1: request.UploadConfig = JSON.parse(JSON.stringify(uploadConfig));
        let files: Array<request.File> = [{
          filename: 'test',
          name: 'test',
          uri: 'internal://cache/test.txt',
          type: 'txt'
        }];
        let uploadConfig: request.UploadConfig = {
          url: 'http://127.0.0.1:8080',
          header: {
            'Accept': '*/*',
          },
          method: 'POST',
          files: files,
          data: [data]
        };
        request.uploadFile(baseContext, uploadConfig, (err, uploadTask) => {
          console.info('====>SUB_REQUEST_FileUpload_post_0100 uploadFile: ' + uploadTask);
          try {
            let completeCallback = (taskStates: Array<request.TaskState>) => {
              try {
                uploadTask?.offComplete();
                console.info('====>SUB_REQUEST_FileUpload_post_0100 complete: ' + JSON.stringify(taskStates));
                //expect(taskStates[0].responseCode).assertEqual(0);
                // await deleteTask(uploadTask);
                // done();
              } catch (err) {
                console.info('====>SUB_REQUEST_FileUpload_post_0100 catch err: ' + JSON.stringify(err));
                // await deleteTask(uploadTask);
                // done();
              }
            }
            if (err) {
              console.info('====>SUB_REQUEST_FileUpload_post_0100 downloadFile fail: ' + JSON.stringify(err));
              //expect().assertFail();
            }
            uploadTask?.onComplete(completeCallback);
            console.info('====>SUB_REQUEST_FileUpload_post_0100 on completeCallback success');
          } catch (err) {
            console.info('====>SUB_REQUEST_FileUpload_post_0100 except err: ' + err);
            // await deleteTask(uploadTask);/
            // done();
          }
        });
      });

    it('SUB_REQUEST_FileUpload_MonitoringProgress_0100', Level.LEVEL1,
      async (done: () => void): Promise<void> => {
        let data: request.RequestData = {
          name: 'name',
          value: '123'
        }
        // let uploadConfig1: request.UploadConfig = JSON.parse(JSON.stringify(uploadConfig));
        let files: Array<request.File> = [{
          filename: 'test',
          name: 'test',
          uri: 'internal://cache/test.txt',
          type: 'txt'
        }];
        let uploadConfig: request.UploadConfig = {
          url: 'http://127.0.0.1:8080',
          header: {
            'Accept': '*/*',
          },
          method: 'POST',
          files: files,
          data: [data]
        };
        console.info('-----------------------SUB_REQUEST_FileUpload_MonitoringProgress_0100 is starting-----------------------');
        request.uploadFile(baseContext, uploadConfig, (err, uploadTask) => {
          console.info('====>SUB_REQUEST_FileUpload_MonitoringProgress_0100 uploadFile: ' + uploadTask);
          try {
            let progressCallback = (uploadedSize: long, totalSize: long) => {
              try {
                uploadTask?.offProgress();
                console.info('====>SUB_REQUEST_FileUpload_MonitoringProgress_0100 uploadedSize: ' + uploadedSize +
                  ';totalSize: ' + totalSize);
                //expect(true).assertEqual(uploadTask != undefined);
                // await deleteTask(uploadTask);
                // done();
              } catch (err) {
                console.info('====>SUB_REQUEST_FileUpload_MonitoringProgress_0100 catch err: ' + JSON.stringify(err));
                // await deleteTask(uploadTask);
                // done();
              }
            }
            if (err) {
              console.info('====>SUB_REQUEST_FileUpload_MonitoringProgress_0100 downloadFile fail: ' +
              JSON.stringify(err));
              //expect().assertFail();
            }
            uploadTask?.onProgress(progressCallback);
            console.info('====>SUB_REQUEST_FileUpload_MonitoringProgress_0100 on progress success');
          } catch (err) {
            console.info('====>SUB_REQUEST_FileUpload_MonitoringProgress_0100 except err: ' + err);
            // await deleteTask(uploadTask);
            // done();
          }
        });
      });

    it('SUB_REQUEST_FileUpload_MonitoringProgress_0300', Level.LEVEL1,
      async (done: () => void): Promise<void> => {
        let data: request.RequestData = {
          name: 'name',
          value: '123'
        }
        // let uploadConfig1: request.UploadConfig = JSON.parse(JSON.stringify(uploadConfig));
        let files: Array<request.File> = [{
          filename: 'test',
          name: 'test',
          uri: 'internal://cache/test.txt',
          type: 'txt'
        }];
        let uploadConfig: request.UploadConfig = {
          url: 'http://127.0.0.1:8080',
          header: {
            'Accept': '*/*',
          },
          method: 'POST',
          files: files,
          data: [data]
        };
        console.info('-----------------------SUB_REQUEST_FileUpload_MonitoringProgress_0300 is starting-----------------------');
        request.uploadFile(baseContext, uploadConfig, (err, uploadTask) => {
          console.info('====>SUB_REQUEST_FileUpload_MonitoringProgress_0300 uploadFile: ' + uploadTask);
          console.info('====>SUB_REQUEST_FileUpload_MonitoringProgress_0300 uploadConfig: ' +
          JSON.stringify(uploadConfig));
          try {
            let headerCallback = (header: Record<String, String>) => {
              try {
                uploadTask?.offHeaderReceive();
                console.info('====>SUB_REQUEST_FileUpload_MonitoringProgress_0300 headerCallback: ' +
                JSON.stringify(header));
                // expect(header['body']).assertEqual('upload successful!');
                // await deleteTask(uploadTask);
                // done();
              } catch (err) {
                console.info('====>SUB_REQUEST_FileUpload_MonitoringProgress_0300 catch err: ' + JSON.stringify(err));
                // await deleteTask(uploadTask);
                // done();
              }
            }
            if (err) {
              console.info('====>SUB_REQUEST_FileUpload_MonitoringProgress_0300 downloadFile fail: ' +
              JSON.stringify(err));
              expect().assertFail();
            }
            uploadTask?.onHeaderReceive(headerCallback);
            console.info('====>SUB_REQUEST_FileUpload_MonitoringProgress_0300 on headerReceive success');
          } catch (err) {
            console.info('====>SUB_REQUEST_FileUpload_MonitoringProgress_0300 except err: ' + err);
            // await deleteTask(uploadTask);
            // done();
          }
        });
      });

    it('SUB_REQUEST_FileUpload_MonitoringProgress_0400', Level.LEVEL1,
      async (done: () => void): Promise<void> => {
        let data: request.RequestData = {
          name: 'name',
          value: '123'
        }
        // let uploadConfig1: request.UploadConfig = JSON.parse(JSON.stringify(uploadConfig));
        let files: Array<request.File> = [{
          filename: 'test',
          name: 'test',
          uri: 'internal://cache/test.txt',
          type: 'txt'
        }];
        let uploadConfig: request.UploadConfig = {
          url: 'http://127.0.0.1:8080',
          header: {
            'Accept': '*/*',
          },
          method: 'POST',
          files: files,
          data: [data]
        };
        console.info('-----------------------SUB_REQUEST_FileUpload_MonitoringProgress_0400 is starting-----------------------');
        request.uploadFile(baseContext, uploadConfig, (err, uploadTask) => {
          console.info('====>SUB_REQUEST_FileUpload_MonitoringProgress_0400 uploadFile: ' + uploadTask);
          console.info('====>SUB_REQUEST_FileUpload_MonitoringProgress_0400 uploadConfig: ' +
          JSON.stringify(uploadConfig));
          try {
            let headerCallback = (header: Record<String, String>) => {
              console.info('====>SUB_REQUEST_FileUpload_MonitoringProgress_0400 headerCallback: ' +
              JSON.stringify(header));
            }
            uploadTask?.onHeaderReceive(headerCallback);
            uploadTask?.offHeaderReceive();
            console.info('====>SUB_REQUEST_FileUpload_MonitoringProgress_0400 on headerReceive success');
          } catch (err) {
            console.info('====>SUB_REQUEST_FileUpload_MonitoringProgress_0400 except err: ' + err);
            // await deleteTask(uploadTask);
            // done();
          }
        });
      });

    it('SUB_REQUEST_FileUpload_MonitoringProgress_0400', Level.LEVEL1,
      async (done: () => void): Promise<void> => {
        let data: request.RequestData = {
          name: 'name',
          value: '123'
        }
        // let uploadConfig1: request.UploadConfig = JSON.parse(JSON.stringify(uploadConfig));
        let files: Array<request.File> = [{
          filename: 'test',
          name: 'test',
          uri: 'internal://cache/test.txt',
          type: 'txt'
        }];
        let uploadConfig: request.UploadConfig = {
          url: 'http://127.0.0.1:8080',
          header: {
            'Accept': '*/*',
          },
          method: 'POST',
          files: files,
          data: [data]
        };
        console.info('-----------------------SUB_REQUEST_FileUpload_MonitoringProgress_0400 is starting-----------------------');
        request.uploadFile(baseContext, uploadConfig, (err, uploadTask) => {
          console.info('====>SUB_REQUEST_FileUpload_MonitoringProgress_0400 uploadFile: ' + uploadTask);
          console.info('====>SUB_REQUEST_FileUpload_MonitoringProgress_0400 uploadConfig: ' +
          JSON.stringify(uploadConfig));
          try {
            let failCallback = (taskStates: Array<request.TaskState>) => {
              // flags = false;
              console.info('====>SUB_REQUEST_FileUpload_MonitoringProgress_0600 into failCallback');
            }
            uploadTask?.onFail(failCallback);
            uploadTask?.offFail();
            console.info('====>SUB_REQUEST_FileUpload_MonitoringProgress_0400 on headerReceive success');
          } catch (err) {
            console.info('====>SUB_REQUEST_FileUpload_MonitoringProgress_0400 except err: ' + err);
            // await deleteTask(uploadTask);
            // done();
          }
        });
      });

    it('SUB_REQUEST_FileUpload_OnFail_0100', Level.LEVEL1,
      async (done: () => void): Promise<void> => {
        console.info('-----------------------SUB_REQUEST_FileUpload_OnFail_0100 is starting-----------------------');
        let data: request.RequestData = {
          name: 'name',
          value: '123'
        }
        // let uploadConfig1: request.UploadConfig = JSON.parse(JSON.stringify(uploadConfig));
        let files: Array<request.File> = [{
          filename: 'test',
          name: 'test',
          uri: 'internal://cache/SUB_Misc_REQUEST_On_Download_Progress_0040.txt',
          type: 'txt'
        }];
        let uploadConfig: request.UploadConfig = {
          url: 'http://127.0.0.1:8080',
          header: {
            'Accept': '*/*',
          },
          method: 'POST',
          files: files,
          data: [data]
        };
        request.uploadFile(baseContext, uploadConfig, (err, uploadTask) => {
          console.info('====>SUB_REQUEST_FileUpload_OnFail_0100 uploadFile: ' + uploadTask);
          try {
            let failCallback = (taskStates: Array<request.TaskState>) => {
              uploadTask?.offFail();
              try {
                console.info("====>SUB_REQUEST_FileUpload_OnFail_0100 fail TaskState = " +
                JSON.stringify(taskStates[0]));
                // expect(typeof (taskStates[0].path) == "string").assertEqual(true);
                // expect(typeof (taskStates[0].responseCode) == "number").assertEqual(true);
                // expect(typeof (taskStates[0].message) == "string").assertEqual(true);
                // console.info("====>SUB_REQUEST_FileUpload_OnFail_0100 TaskState is correct");
                // done();
              } catch (err) {
                console.info("====>SUB_REQUEST_FileUpload_OnFail_0100 TaskState err: " + JSON.stringify(err));
                // done();
              }
            }

            if (err) {
              console.info('====>SUB_REQUEST_FileUpload_OnFail_0100 downloadFile fail: ' + JSON.stringify(err));
              // expect().assertFail();
            }

            uploadTask?.onFail(failCallback);
          } catch (err) {
            console.info('====>SUB_REQUEST_FileUpload_OnFail_0100 except err: ' + err);
            // await deleteTask(uploadTask);
            // done();
          }
        });
      });

    it('SUB_REQUEST_FileUpload_OffFail_0100', Level.LEVEL1,
      async (done: () => void): Promise<void> => {
        let data: request.RequestData = {
          name: 'name',
          value: '123'
        }
        // let uploadConfig1: request.UploadConfig = JSON.parse(JSON.stringify(uploadConfig));
        let files: Array<request.File> = [{
          filename: 'test',
          name: 'test',
          uri: 'internal://cache/SUB_Misc_REQUEST_On_Download_Progress_0040.txt',
          type: 'txt'
        }];
        let uploadConfig: request.UploadConfig = {
          url: 'http://127.0.0.1:8080',
          header: {
            'Accept': '*/*',
          },
          method: 'POST',
          files: files,
          data: [data]
        };
        console.info('-----------------------SUB_REQUEST_FileUpload_OffFail_0100 is starting-----------------------');
        request.uploadFile(baseContext, uploadConfig, (err, uploadTask) => {
          console.info('====>SUB_REQUEST_FileUpload_OffFail_0100 uploadFile: ' + uploadTask);
          console.info('====>SUB_REQUEST_FileUpload_OffFail_0100 uploadConfig: ' + JSON.stringify(uploadConfig));
          try {
            let failCallback = (taskStates: Array<request.TaskState>) => {
              // flags = false;
              console.info('====>SUB_REQUEST_FileUpload_OffFail_0100 into failCallback');
            }
            uploadTask?.onFail(failCallback);
            uploadTask?.offFail(failCallback);
            console.info('====>SUB_REQUEST_FileUpload_OffFail_0100 on headerReceive success');
          } catch (err) {
            console.info('====>SUB_REQUEST_FileUpload_OffFail_0100 except err: ' + err);
            // await deleteTask(uploadTask);
            // done();
          }
        });
      });

    it('SUB_REQUEST_FileUpload_offSuccess_0100', Level.LEVEL1,
      async (done: () => void): Promise<void> => {
        let data: request.RequestData = {
          name: 'name',
          value: '123'
        }
        // let uploadConfig1: request.UploadConfig = JSON.parse(JSON.stringify(uploadConfig));
        let files: Array<request.File> = [{
          filename: 'test',
          name: 'test',
          uri: 'internal://cache/SUB_Misc_REQUEST_On_Download_Progress_0040.txt',
          type: 'txt'
        }];
        let uploadConfig: request.UploadConfig = {
          url: 'http://127.0.0.1:8080',
          header: {
            'Accept': '*/*',
          },
          method: 'POST',
          files: files,
          data: [data]
        };
        console.info('-----------------------SUB_REQUEST_FileUpload_offSuccess_0100 is starting-----------------------');
        request.uploadFile(baseContext, uploadConfig, (err, uploadTask) => {
          console.info('====>SUB_REQUEST_FileUpload_offSuccess_0100 uploadFile: ' + uploadTask);
          // let flags: boolean = true;
          try {
            let completeCallback = (taskStates: Array<request.TaskState>) => {
              // flags = false;
              console.info('====>SUB_REQUEST_FileUpload_offSuccess_0100 into completeCallback');
            }
            if (err) {
              console.info('====>SUB_REQUEST_FileUpload_offSuccess_0100 downloadFile fail: ' + JSON.stringify(err));
              expect().assertFail();
            }
            //expect(true).assertEqual(uploadTask != undefined);
            uploadTask?.onComplete(completeCallback);
            // console.info('====>SUB_REQUEST_FileUpload_offSuccess_0100 onComplete');
            uploadTask?.offComplete(completeCallback);
            // console.info('====>SUB_REQUEST_FileUpload_offSuccess_0100 offComplete');
            // expect(flags).assertTrue();
            // await deleteTask(uploadTask);
            // done();
          } catch (err) {
            console.info('====>SUB_REQUEST_FileUpload_offSuccess_0100 except err: ' + err);
            // await deleteTask(uploadTask);
            // done();
          }
        });
      });
  })
}