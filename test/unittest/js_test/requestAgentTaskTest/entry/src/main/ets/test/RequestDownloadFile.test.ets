/**
 * Copyright (c) 2023 Huawei Device Co., Ltd.
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or impligetProfileByAbilityEmptyAbilityCalled.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

import { describe, beforeAll, beforeEach, afterEach, afterAll, it, expect } from "@ohos/hypium"
import request from '@ohos.request';
import fs from '@ohos.file.fs';
import common from '@ohos.app.ability.common';

export default function requestDownloadFileTest() {
  describe('requestDownloadFileTest', () => {
    beforeAll(() => {
      console.info('beforeAll called');
    });
    afterAll(() => {
      console.info('afterAll called');
    });
    beforeEach(() => {
      console.info('beforeEach called');
    });
    afterEach(async () => {
      console.info('afterEach called');
      if (fs.accessSync(cacheDir + '/test.txt')) {
        fs.unlinkSync(cacheDir + '/test.txt');
      }
    });

    let sleep: (ms: number) => Promise<void> = async (ms: number): Promise<void> => {
      return new Promise((resolve) => {
        setTimeout(resolve, ms);
      });
    };

    let context: common.UIAbilityContext = globalThis.abilityContext;
    let cacheDir: String = globalThis.cacheDir;
    let globalDownloadConf: request.DownloadConfig = {
      url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      header: {
        headers: 'http'
      },
      enableMetered: false,
      enableRoaming: false,
      description: 'downloadV9 test!',
      networkType: request.NETWORK_WIFI,
      filePath: '/data/storage/el2/base/haps/entry/files/test' + new Date().getTime() + '.txt',
      title: 'downloadV9 test!',
      background: false
    };
    /**
     * @tc.number: testDownloadUrl001
     * @tc.name: testDownloadUrl001
     * @tc.desc: Test downloadFile when lack url
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadUrl001', 0, (done: Function) => {
      let conf: Record<string, string> = {};
      try {
        request.downloadFile(context, conf as Object as request.DownloadConfig, (err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testDownloadUrl002
     * @tc.name: testDownloadUrl002
     * @tc.desc: Test downloadFile when url is undefined
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadUrl002', 0, (done: Function) => {
      let conf: Record<string, undefined> = {
        "url": undefined,
      };
      try {
        request.downloadFile(context, conf as Object as request.DownloadConfig, (err, data) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testDownloadUrl003
     * @tc.name: testDownloadUrl003
     * @tc.desc: Test downloadFile when url is not support download
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadUrl003', 0, (done: Function) => {
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/openharmony/request_request',
        filePath: 'test.txt',
      };
      request.downloadFile(context, conf).then(async (data) => {
        expect(true).assertTrue();
        let task = data;
        await task.delete();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testDownloadUrl004
     * @tc.name: testDownloadUrl004
     * @tc.desc: Test downloadFile when url is not http/https scheme.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadUrl004', 0, (done: Function) => {
      let conf: request.DownloadConfig = {
        url: 'admin://127.0.0.1',
        filePath: 'test.txt',
      };
      try {
        request.downloadFile(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testDownloadUrl005
     * @tc.name: testDownloadUrl005
     * @tc.desc: Test downloadFile when url length is 8193.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadUrl005', 0, (done: Function) => {
      let conf: request.DownloadConfig = {
        url: 'http://127.0.0.1/' + 'a'.repeat(8176),
      };
      try {
        request.downloadFile(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testDownloadUrl006
     * @tc.name: testDownloadUrl006
     * @tc.desc: Test downloadFile when url is empty
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadUrl006', 0, (done: Function) => {
      let conf: request.DownloadConfig = {
        url: '',
      };
      try {
        request.downloadFile(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testDownloadUrl007
     * @tc.name: testDownloadUrl007
     * @tc.desc: Test downloadFile when url length is 8192.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadUrl007', 0, (done: Function) => {
      let conf: request.DownloadConfig = {
        url: 'http://127.0.0.1/' + 'a'.repeat(8175),
      };
      try {
        request.downloadFile(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(13400001);
        done();
      }
    });
    /**
     * @tc.number: testDownloadFile001
     * @tc.name: testDownloadFile001
     * @tc.desc: Test create for conf is string type
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFile001', 0, async (done: Function) => {
      try {
        await request.downloadFile(context, "conf" as Object as request.DownloadConfig);
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testDownloadFile002
     * @tc.name: testDownloadFile002
     * @tc.desc: Test create for download again failed
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFile002', 0, async (done: Function) => {
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        filePath: 'testV9_file02.txt'
      };
      try {
        await request.downloadFile(context, conf);
        await sleep(2000);
        await request.downloadFile(context, conf);
        done();
      } catch (err) {
        expect(err.code).assertEqual(13400002);
        done();
      }
    });
    /**
     * @tc.number: testDownloadFile003
     * @tc.name: testDownloadFile003
     * @tc.desc: Test create for downloadfile correctly for promise
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFile003', 0, async (done: Function) => {
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        filePath: 'testV9_file03.txt'
      };
      try {
        let task = await request.downloadFile(context, conf);
        expect(true).assertTrue();
        await task.delete();
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testDownloadFile0031
     * @tc.name: testDownloadFile0031
     * @tc.desc: Test create for downloadfile correctly for callback
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFile0031', 0, async (done: Function) => {
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        filePath: 'testV9_file04.txt'
      };
      try {
        request.downloadFile(context, conf, async (err, data) => {
          expect(true).assertTrue();
          let task = data;
          await task.delete();
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testDownloadFile0032
     * @tc.name: testDownloadFile0032
     * @tc.desc: Test create for downloadfile of user file.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFile0032', 0, async (done: Function) => {
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        filePath: 'file://media/Photo/1/test.txt',
        background: false,
      };
      try {
        request.downloadFile(context, conf, async (err, data) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(true).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testDownloadFile004
     * @tc.name: testDownloadFile004
     * @tc.desc: Test create for downloadfile when networkType is NETWORK_WIFI
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFile004', 0, async (done: Function) => {
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        filePath: 'testDownloadFile004.txt',
        networkType: request.NETWORK_WIFI
      };
      try {
        let task = await request.downloadFile(context, conf);
        expect(true).assertTrue();
        await task.delete();
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testDownloadFile005
     * @tc.name: testDownloadFile005
     * @tc.desc: Test create for downloadfile when networkType is NETWORK_MOBILE
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFile005', 0, async (done: Function) => {
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        filePath: 'testDownloadFile005.txt',
        networkType: request.NETWORK_MOBILE
      };
      try {
        let task = await request.downloadFile(context, conf);
        expect(true).assertTrue();
        await task.delete();
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testDownloadFilePath0001
     * @tc.name: testDownloadFilePath0001
     * @tc.desc: Test create for downloadfile correctly for callback
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFilePath0001', 0, async (done: Function) => {
      let standardizePath = cacheDir + '/test.txt';
      if (fs.accessSync(standardizePath)) {
        fs.unlinkSync(standardizePath);
      }
      let filePath = cacheDir + '//test.txt';
      let conf: request.DownloadConfig  = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        filePath: filePath,
      }
      try {
        request.downloadFile(context, conf, async (err, data) => {
          expect(true).assertTrue();
          let task = data;
          task.on('complete', () => {
            if (fs.accessSync(standardizePath)) {
              expect(true).assertTrue();
              fs.unlinkSync(standardizePath);
            } else {
              expect(false).assertTrue();
            }
            task.off('complete');
            done();
          });
        })
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testDownloadFilePath0002
     * @tc.name: testDownloadFilePath0002
     * @tc.desc: Test create for downloadfile correctly for callback
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFilePath0002', 0, async (done: Function) => {
      if (!fs.accessSync(cacheDir + '/A')) {
        fs.mkdirSync(cacheDir + '/A');
      }
      if (!fs.accessSync(cacheDir + '/A/B')) {
        fs.mkdirSync(cacheDir + '/A/B');
      }
      let standardizePath = cacheDir + '/A/B/test.txt';
      if (fs.accessSync(standardizePath)) {
        fs.unlinkSync(standardizePath);
      }
      let filePath = cacheDir + '/A//B/test.txt';
      let conf: request.DownloadConfig  = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        filePath: filePath,
      }
      try {
        request.downloadFile(context, conf, async (err, data) => {
          expect(true).assertTrue();
          let task = data;
          task.on('complete', () => {
            if (fs.accessSync(standardizePath)) {
              expect(true).assertTrue();
              fs.unlinkSync(standardizePath);
            } else {
              expect(false).assertTrue();
            }
            done();
          });
        })
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testDownloadFilePath0003
     * @tc.name: testDownloadFilePath0003
     * @tc.desc: Test create for downloadfile correctly for callback
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFilePath0003', 0, async (done: Function) => {
      if (!fs.accessSync(cacheDir + '/A')) {
        fs.mkdirSync(cacheDir + '/A');
      }
      if (!fs.accessSync(cacheDir + '/A/B')) {
        fs.mkdirSync(cacheDir + '/A/B');
      }
      let standardizePath = cacheDir + '/A/B/test.txt';
      if (fs.accessSync(standardizePath)) {
        fs.unlinkSync(standardizePath);
      }
      let filePath = cacheDir + '/A///////B/test.txt';
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        filePath: filePath,
      }
      try {
        request.downloadFile(context, conf, async (err, data) => {
          expect(true).assertTrue();
          let task = data;
          task.on('complete', () => {
            if (fs.accessSync(standardizePath)) {
              expect(true).assertTrue();
              fs.unlinkSync(standardizePath);
            } else {
              expect(false).assertTrue();
            }
            done();
          });
        })
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testDownloadFilePath0004
     * @tc.name: testDownloadFilePath0004
     * @tc.desc: Test create for downloadfile correctly for callback
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFilePath0004', 0, async (done: Function) => {
      if (!fs.accessSync(cacheDir + '/A')) {
        fs.mkdirSync(cacheDir + '/A');
      }
      if (!fs.accessSync(cacheDir + '/A/B')) {
        fs.mkdirSync(cacheDir + '/A/B');
      }
      let standardizePath = cacheDir + '/A/B/test.txt';
      if (fs.accessSync(standardizePath)) {
        fs.unlinkSync(standardizePath);
      }
      let filePath = cacheDir + '/A//////./B/.//test.txt';
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        filePath: filePath,
      }
      try {
        request.downloadFile(context, conf, async (err, data) => {
          expect(true).assertTrue();
          let task = data;
          task.on('complete', () => {
            if (fs.accessSync(standardizePath)) {
              expect(true).assertTrue();
              fs.unlinkSync(standardizePath);
            } else {
              expect(false).assertTrue();
            }
            done();
          });
        })
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testDownloadFileOn001
     * @tc.name: testDownloadFileOn001
     * @tc.desc: Test downloadFile task on for error type like 'test'
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFileOn001', 0, async (done: Function) => {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      };
      let task = await request.downloadFile(context, conf);
      try {
        task.on('test' as Object as 'progress', () => {
        });
      } catch (err) {
        expect(true).assertTrue();
        await task.delete();
        done();
      }
    });
    /**
     * @tc.number: testDownloadFileOn002
     * @tc.name: testDownloadFileOn002
     * @tc.desc: Test downloadFile task on for error type like number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFileOn002', 0, async (done: Function) => {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      };
      let task = await request.downloadFile(context, conf);
      try {
        task.on(123 as Object as 'progress', () => {
        });
      } catch (err) {
        expect(true).assertTrue();
        await task.delete();
        done();
      }
    });
    /**
     * @tc.number: testDownloadFileOn003
     * @tc.name: testDownloadFileOn003
     * @tc.desc: Test downloadFile task on for fail
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFileOn003', 0, async (done: Function) => {
      let conf: request.DownloadConfig = {
        url: 'http://www.ylong_http.client.com/test.txt',
      };
      let task = await request.downloadFile(context, conf);
      try {
        task.on('fail', () => {
          expect(true).assertTrue();
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testDownloadFileOn004
     * @tc.name: testDownloadFileOn004
     * @tc.desc: Test downloadFile task on for progress
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFileOn004', 0, async (done: Function) => {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      };
      let task = await request.downloadFile(context, conf);
      try {
        task.on('progress', (size, total) => {
          expect(true).assertTrue();
          expect(total).assertEqual(1042003);
          task.off('progress');
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testDownloadFileOn005
     * @tc.name: testDownloadFileOn005
     * @tc.desc: Test downloadFile task on for progress twice
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFileOn005', 0, async (done: Function) => {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      };
      let task = await request.downloadFile(context, conf);
      try {
        task.on('progress', (size, total) => {
        });
        task.on('progress', (size, total) => {
        });
        expect(true).assertTrue();
        await task.delete();
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testDownloadFileOn006
     * @tc.name: testDownloadFileOn006
     * @tc.desc: Test downloadFile task on for complete
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFileOn006', 0, async (done: Function) => {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      };
      let task = await request.downloadFile(context, conf);
      try {
        task.on('complete', () => {
          expect(true).assertTrue();
          task.off('complete');
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testDownloadFileOn007
     * @tc.name: testDownloadFileOn007
     * @tc.desc: Test downloadFile task on for pause
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFileOn007', 0, async (done: Function) => {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      };
      let task = await request.downloadFile(context, conf);
      try {
        task.on('pause', () => {
          expect(true).assertTrue();
        });
        await sleep(1000);
        await task.suspend();
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testDownloadFileOn008
     * @tc.name: testDownloadFileOn008
     * @tc.desc: Test downloadFile task on for remove
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFileOn008', 0, async (done: Function) => {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      };
      let task = await request.downloadFile(context, conf);
      try {
        task.on('remove', () => {
          expect(true).assertTrue();
          done();
        });
        await task.delete();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testDownloadFileOff001
     * @tc.name: testDownloadFileOff001
     * @tc.desc: Test downloadFile task off for error type like 'test'
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFileOff001', 0, async (done: Function) => {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      };
      let task = await request.downloadFile(context, conf);
      try {
        task.off('test' as Object as 'progress', () => {
        });
      } catch (err) {
        expect(true).assertTrue();
        await task.delete();
        done();
      }
    });
    /**
     * @tc.number: testDownloadFileOff002
     * @tc.name: testDownloadFileOff002
     * @tc.desc: Test downloadFile task off for error type like number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFileOff002', 0, async (done: Function) => {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      };
      let task = await request.downloadFile(context, conf);
      try {
        task.off(123 as Object as 'progress', () => {
        });
        done();
      } catch (err) {
        expect(true).assertTrue();
        await task.delete();
        done();
      }
    });
    /**
     * @tc.number: testDownloadFileOff003
     * @tc.name: testDownloadFileOff003
     * @tc.desc: Test downloadFile task off for complete
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFileOff003', 0, async (done: Function) => {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      };
      let task = await request.downloadFile(context, conf);
      task.on('complete', () => {
        expect(true).assertTrue();
        done();
      });

      let CompletedCallback: () => void = () : void => {
        expect(false).assertTrue();
        done();
      }

      task.on('complete', CompletedCallback);
      task.off('complete', CompletedCallback);
    });
    /**
     * @tc.number: testDownloadFileOff0031
     * @tc.name: testDownloadFileOff0031
     * @tc.desc: Test downloadFile task off for complete
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFileOff0031', 0, async (done: Function) => {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      };
      let task = await request.downloadFile(context, conf);
      try {

        let CompletedCallback1: () => void = () : void => {
          expect(false).assertTrue();
          done();
        }

        let CompletedCallback2: () => void = () : void => {
          expect(false).assertTrue();
          done();
        }

        task.on('complete', CompletedCallback1);
        task.on('complete', CompletedCallback2);
        task.off('complete');
        await sleep(2000);
        expect(true).assertTrue();
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testDownloadFileOff004
     * @tc.name: testDownloadFileOff004
     * @tc.desc: Test downloadFile task off task twice
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFileOff004', 0, async (done: Function) => {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      };
      let task = await request.downloadFile(context, conf);
      try {
        task.off('complete');
        task.off('complete');
        expect(true).assertTrue();
        await task.delete();
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testDownloadFileOff005
     * @tc.name: testDownloadFileOff005
     * @tc.desc: Test downloadFile task off for progress
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFileOff005', 0, async (done: Function) => {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      };
      let task = await request.downloadFile(context, conf);
      try {
        task.on('progress', (progress) => {
          expect(true).assertTrue();
          done();
        });
        task.on('complete', () => {
          expect(true).assertTrue();
          done();
        });

        let ProgressCallback: (receivedSize: number, totalSize: number) => void = (receivedSize: number, totalSize: number) : void => {
          expect(false).assertTrue();
          done();
        }

        task.on('progress', ProgressCallback);
        task.off('progress', ProgressCallback);
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testDownloadFileOff0051
     * @tc.name: testDownloadFileOff0051
     * @tc.desc: Test downloadFile task off for progress
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFileOff0051', 0, async (done: Function) => {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      };
      let task = await request.downloadFile(context, conf);
      try {

        let ProgressCallback1: (receivedSize: number, totalSize: number) => void = (receivedSize: number, totalSize: number) : void => {
          expect(false).assertTrue();
          done();
        }

        let ProgressCallback2: (receivedSize: number, totalSize: number) => void = (receivedSize: number, totalSize: number) : void => {
          expect(false).assertTrue();
          done();
        }

        task.on('complete', () => {
          expect(true).assertTrue();
          done();
        });
        task.on('progress', ProgressCallback1);
        task.on('progress', ProgressCallback2);
        task.off('progress');
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testDownloadFileOff006
     * @tc.name: testDownloadFileOff006
     * @tc.desc: Test downloadFile task off for pause
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFileOff006', 0, async (done: Function) => {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      };
      let task = await request.downloadFile(context, conf);
      try {
        task.on('pause', () => {
          expect(true).assertTrue();
          done();
        });

        let PauseCallback: () => void = () : void => {
          expect(false).assertTrue();
          done();
        }

        task.on('pause', PauseCallback);
        task.off('pause', PauseCallback);
        await task.suspend();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testDownloadFileOff007
     * @tc.name: testDownloadFileOff007
     * @tc.desc: Test downloadFile task off for pause
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFileOff007', 0, async (done: Function) => {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      };
      let task = await request.downloadFile(context, conf);
      try {

        let PauseCallback1: () => void = () : void => {
          expect(false).assertTrue();
          done();
        }

        let PauseCallback2: () => void = () : void => {
          expect(false).assertTrue();
          done();
        }

        task.on('pause', PauseCallback1);
        task.on('pause', PauseCallback2);
        task.off('pause');
        await task.suspend();
        await sleep(2000);
        expect(true).assertTrue();
        await task.delete();
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testDownloadFileOff008
     * @tc.name: testDownloadFileOff008
     * @tc.desc: Test downloadFile task off for remove
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFileOff008', 0, async (done: Function) => {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      };
      let task = await request.downloadFile(context, conf);
      try {
        task.on('remove', () => {
          expect(true).assertTrue();
          done();
        });

        let RemoveCallback: () => void = () : void => {
          expect(false).assertTrue();
          done();
        }

        task.on('remove', RemoveCallback);
        task.off('remove', RemoveCallback);
        await task.delete();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testDownloadFileOff009
     * @tc.name: testDownloadFileOff009
     * @tc.desc: Test downloadFile task off for remove
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFileOff009', 0, async (done: Function) => {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      };
      let task = await request.downloadFile(context, conf);
      try {

        let RemoveCallback1: () => void = () : void => {
          expect(false).assertTrue();
          done();
        }

        let RemoveCallback2: () => void = () : void => {
          expect(false).assertTrue();
          done();
        }

        task.on('remove', RemoveCallback1);
        task.on('remove', RemoveCallback2);
        task.off('remove');
        task.on('remove', () => {
          expect(true).assertTrue();
          done();
        });
        await task.delete();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testDownloadFileOff010
     * @tc.name: testDownloadFileOff010
     * @tc.desc: Test downloadFile task off for fail
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFileOff010', 0, async (done: Function) => {
      let conf: request.DownloadConfig = {
        url: 'http://www.ylong_http.client.com/test.txt',
      };
      let task = await request.downloadFile(context, conf);
      try {
        task.on('fail', () => {
          expect(true).assertTrue();
          done();
        });

        let FailCallback: (err: number) => void = (err: number) : void => {
          expect(false).assertTrue();
          done();
        }

        task.on('fail', FailCallback);
        task.off('fail', FailCallback);
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testDownloadFileOff011
     * @tc.name: testDownloadFileOff011
     * @tc.desc: Test downloadFile task off for fail
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFileOff011', 0, async (done: Function) => {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf: request.DownloadConfig = {
        url: 'http://www.ylong_http.client.com/test.txt',
      };
      let task = await request.downloadFile(context, conf);
      try {

        let FailCallback1: (err: number) => void = (err: number) : void => {
          expect(false).assertTrue();
          done();
        }

        let FailCallback2: (err: number) => void = (err: number) : void => {
          expect(false).assertTrue();
          done();
        }

        task.on('fail', FailCallback1);
        task.on('fail', FailCallback2);
        task.off('fail');
        task.on('fail', () => {
          expect(true).assertTrue();
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testDownloadTaskRestore001
     * @tc.name: testDownloadTaskRestore001
     * @tc.desc: Test downloadTask restore for promise
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadTaskRestore001', 0, async (done: Function) => {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      };
      let task = await request.downloadFile(context, conf);
      task.suspend();
      try {
        task.restore().then((res) => {
          expect(res).assertTrue();
          done();
        }).catch((err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testDownloadTaskRestore0011
     * @tc.name: testDownloadTaskRestore0011
     * @tc.desc: Test downloadTask restore for callback
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadTaskRestore0011', 0, async (done: Function) => {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      };
      let task = await request.downloadFile(context, conf);
      await task.suspend();
      try {
        task.restore((err, res) => {
          expect(true).assertTrue();
          task.delete();
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testDownloadTaskDelete001
     * @tc.name: testDownloadTaskDelete001
     * @tc.desc: Test downloadTask delete for parameter error
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadTaskDelete001', 0, async (done: Function) => {
      try {
        let task = await request.downloadFile(context, '' as Object as request.DownloadConfig);
        task.delete();
      } catch (err) {
        expect(true).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testDownloadTaskDelete002
     * @tc.name: testDownloadTaskDelete002
     * @tc.desc: Test downloadTask delete for promise
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadTaskDelete002', 0, async (done: Function) => {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      };
      let task = await request.downloadFile(context, conf);
      task.delete().then(() => {
        expect(true).assertTrue();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testDownloadTaskDelete0021
     * @tc.name: testDownloadTaskDelete0021
     * @tc.desc: Test downloadTask delete for callback
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadTaskDelete0021', 0, async (done: Function) => {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      };
      let task = await request.downloadFile(context, conf);
      try {
        task.delete(async (err: Error) => {
          expect(true).assertTrue();
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testDownloadGetTaskInfo001
     * @tc.name: testDownloadGetTaskInfo001
     * @tc.desc: Test downloadTask for promise when completed
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadGetTaskInfo001', 0, async (done: Function) => {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      };
      let task = await request.downloadFile(context, conf);
      try {

        let CompletedCallback: () => void = () : void => {
          task.getTaskInfo().then((downloadInfo) => {
            expect(downloadInfo.status).assertEqual(0);
            done();
          }).catch((err: Error) => {
            expect(false).assertTrue();
            done();
          });
        }

        task.on('complete', CompletedCallback);
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testDownloadGetTaskInfo0011
     * @tc.name: testDownloadGetTaskInfo0011
     * @tc.desc: Test downloadTask for callback when completed
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadGetTaskInfo0011', 0, async (done: Function) => {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      };
      let task = await request.downloadFile(context, conf);
      try {

        let CompletedCallback: () => void = () : void => {
          task.getTaskInfo((err, downloadInfo) => {
            expect(downloadInfo.status).assertEqual(0);
            done();
          });
        }

        task.on('complete', CompletedCallback);
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testDownloadGetTaskInfo002
     * @tc.name: testDownloadGetTaskInfo002
     * @tc.desc: Test downloadTask for promise when pause
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadGetTaskInfo002', 0, async (done: Function) => {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      };
      let task = await request.downloadFile(context, conf);
      await task.suspend();
      try {
        task.getTaskInfo().then(async (downloadInfo) => {
          expect(downloadInfo.status).assertEqual(3);
          await task.delete();
          done();
        }).catch((err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testDownloadGetTaskInfo0021
     * @tc.name: testDownloadGetTaskInfo0021
     * @tc.desc: Test downloadTask for callback when pause
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadGetTaskInfo0021', 0, async (done: Function) => {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      };
      let task = await request.downloadFile(context, conf);
      await task.suspend();
      try {
        task.getTaskInfo(async (err, downloadInfo) => {
          expect(downloadInfo.status).assertEqual(3);
          await task.delete();
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testDownloadGetTaskInfo003
     * @tc.name: testDownloadGetTaskInfo003
     * @tc.desc: Test downloadTask for promise when progress
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadGetTaskInfo003', 0, async (done: Function) => {
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/large_test.zip',
        filePath: "DownloadGetTaskInfo003"
      };
      let task = await request.downloadFile(context, conf);
      try {

        let ProgressCallback: (receivedSize: number, totalSize: number) => void = (receivedSize: number, totalSize: number) : void => {
          task.off('progress');
          task.getTaskInfo().then((downloadInfo) => {
            expect(downloadInfo.status).assertEqual(1);
            task.delete();
            done();
          }).catch((err: Error) => {
            expect(false).assertTrue();
            done();
          });
        }

        task.on('progress', ProgressCallback);
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testDownloadGetTaskInfo0031
     * @tc.name: testDownloadGetTaskInfo0031
     * @tc.desc: Test downloadTask for callback when progress
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadGetTaskInfo0031', 0, async (done: Function) => {
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/large_test.zip',
        filePath: "DownloadGetTaskInfo0031",
      };
      let task = await request.downloadFile(context, conf);
      try {
        task.on('progress', (size, total) => {
          task.off('progress');
          task.getTaskInfo((err, downloadInfo) => {
            expect(downloadInfo.status).assertEqual(1);
            task.delete();
            done();
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testDownloadGetTaskInfo004
     * @tc.name: testDownloadGetTaskInfo004
     * @tc.desc: Test downloadTask for promise when fail
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadGetTaskInfo004', 0, async (done: Function) => {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf: request.DownloadConfig = {
        url: 'http://www.ylong_http.client.com/test.txt',
      };
      let task = await request.downloadFile(context, conf);
      try {

        let FailCallback: (err: number) => void = (err: number) : void => {
          task.getTaskInfo().then((downloadInfo) => {
            expect(downloadInfo.status).assertEqual(4);
            task.delete();
            done();
          }).catch((err: Error) => {
            expect(false).assertTrue();
            done();
          });
        }

        task.on('fail', FailCallback);
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testDownloadGetTaskInfo0041
     * @tc.name: testDownloadGetTaskInfo0041
     * @tc.desc: Test downloadTask for callback when fail
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadGetTaskInfo0041', 0, async (done: Function) => {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf: request.DownloadConfig = {
        url: 'http://www.ylong_http.client.com/test.txt',
      };
      let task = await request.downloadFile(context, conf);
      try {

        let FailCallback: () => void = () : void => {
          task.getTaskInfo(async (err, downloadInfo) => {
            expect(downloadInfo.status).assertEqual(4);
            await task.delete();
            done();
          });
        }

        task.on('fail', FailCallback);
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
  })
}