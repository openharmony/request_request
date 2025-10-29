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
     * @tc.step: 1. Create an empty Record object (for type conversion) as the download configuration.
     *           2. Try to call request.downloadFile() with the empty config; in the success callback (unexpected), assert false and call done().
     *           3. Catch the thrown exception, assert err.code equals 401 (missing URL error), then call done().
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a Record object with "url" set to undefined as the download configuration.
     *           2. Try to call request.downloadFile() with the config; in the success callback (unexpected), assert false and call done().
     *           3. Catch the thrown exception, assert err.code equals 401 (undefined URL error), then call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a DownloadConfig object with an URL that does not support download (gitee repository page) and filePath: testDownloadUrl003.txt.
     *           2. Call request.downloadFile() with the config; in the success callback, assert true (task created successfully).
     *           3. Get the download task from the callback result, call task.delete() to clean up, then call done().
     *           4. In the catch callback, assert false (task creation failed) and call done().
     * @tc.require: issues#1573
     */
    it('testDownloadUrl003', 0, (done: Function) => {
      let conf: request.DownloadConfig = {
        url: 'https://gitee.com/openharmony/request_request',
        filePath: 'testDownloadUrl003.txt',
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a DownloadConfig object with an URL using "admin://" scheme (non-http/https) and filePath: testDownloadUrl004.txt.
     *           2. Try to call request.downloadFile() with the config; in the success callback (unexpected), assert false and call done().
     *           3. Catch the thrown exception, assert err.code equals 401 (invalid URL scheme error), then call done().
     * @tc.require: issues#1573
     */
    it('testDownloadUrl004', 0, (done: Function) => {
      let conf: request.DownloadConfig = {
        url: 'admin://127.0.0.1',
        filePath: 'testDownloadUrl004.txt',
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Construct an URL with length 8193 (base "http://127.0.0.1/" + 8176 'a's) and set it in DownloadConfig with filePath: testDownloadUrl005.txt.
     *           2. Try to call request.downloadFile() with the config; in the success callback (unexpected), assert false and call done().
     *           3. Catch the thrown exception, assert err.code equals 401 (URL too long error), then call done().
     * @tc.require: issues#1573
     */
    it('testDownloadUrl005', 0, (done: Function) => {
      let conf: request.DownloadConfig = {
        url: 'http://127.0.0.1/' + 'a'.repeat(8176),
        filePath: 'testDownloadUrl005.txt',
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a DownloadConfig object with empty string URL and filePath: testDownloadUrl006.txt.
     *           2. Try to call request.downloadFile() with the config; in the success callback (unexpected), assert false and call done().
     *           3. Catch the thrown exception, assert err.code equals 401 (empty URL error), then call done().
     * @tc.require: issues#1573
     */
    it('testDownloadUrl006', 0, (done: Function) => {
      let conf: request.DownloadConfig = {
        url: '',
        filePath: 'testDownloadUrl006.txt',
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Construct an URL with length 8192 (base "http://127.0.0.1/" + 8175 'a's) and set it in DownloadConfig.
     *           2. Try to call request.downloadFile() with the config; in the success callback (unexpected), assert false and call done().
     *           3. Catch the thrown exception, assert err.code equals 13400001 (file path error), then call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Try to call request.downloadFile() with a string ("conf") type-cast to DownloadConfig.
     *           2. Catch the thrown exception, assert err.code equals 401 (invalid config type error), then call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a DownloadConfig object with valid URL and filePath: testV9_file02.txt.
     *           2. Call request.downloadFile() to create the first download task and wait for 2 seconds.
     *           3. Call request.downloadFile() again with the same config; in the success callback, call done().
     *           4. Catch the thrown exception, assert err.code equals 13400002 (duplicate download error), then call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a DownloadConfig object with valid URL and filePath: testV9_file03.txt.
     *           2. Call request.downloadFile() via Promise; in the success callback, get the task and assert true.
     *           3. Call task.delete() to clean up, then call done().
     *           4. In the catch callback, assert false (task creation failed) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a DownloadConfig object with valid URL and filePath: testV9_file04.txt.
     *           2. Call request.downloadFile() via callback; in the success callback, get the task and assert true.
     *           3. Call task.delete() to clean up, then call done().
     *           4. In the catch callback, assert false (task creation failed) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a DownloadConfig object with valid URL, filePath: 'file://media/Photo/1/test.txt' (user public directory), and background: false.
     *           2. Try to call request.downloadFile() via callback; in the success callback (unexpected), assert false and call done().
     *           3. Catch the thrown exception, assert true (expected error) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a DownloadConfig object with valid URL, filePath: testDownloadFile004.txt, and networkType: request.NETWORK_WIFI.
     *           2. Call request.downloadFile() via Promise; in the success callback, get the task and assert true.
     *           3. Call task.delete() to clean up, then call done().
     *           4. In the catch callback, assert false (task creation failed) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a DownloadConfig object with valid URL, filePath: testDownloadFile005.txt, and networkType: request.NETWORK_MOBILE.
     *           2. Call request.downloadFile() via Promise; in the success callback, get the task and assert true.
     *           3. Call task.delete() to clean up, then call done().
     *           4. In the catch callback, assert false (task creation failed) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Define the standard file path (cacheDir + '/test.txt'), delete it if it exists.
     *           2. Construct a file path with double slash (cacheDir + '//test.txt') and set it in DownloadConfig with valid URL.
     *           3. Call request.downloadFile() via callback; in the success callback, get the task and assert true.
     *           4. Bind the 'complete' event to the task: check if the file exists at the standard path, assert true, delete the file, unbind the event, then call done().
     *           5. In the catch callback, assert false (task creation failed) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create subdirectories cacheDir + '/A' and '/A/B' if they do not exist.
     *           2. Define the standard file path (cacheDir + '/A/B/test.txt'), delete it if it exists.
     *           3. Construct a file path with double slash (cacheDir + '/A//B/test.txt') and set it in DownloadConfig with valid URL.
     *           4. Call request.downloadFile() via callback; in the success callback, get the task and assert true.
     *           5. Bind the 'complete' event: check if the file exists at the standard path, assert true, delete the file, then call done().
     *           6. In the catch callback, assert false (task creation failed) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create subdirectories cacheDir + '/A' and '/A/B' if they do not exist.
     *           2. Define the standard file path (cacheDir + '/A/B/test.txt'), delete it if it exists.
     *           3. Construct a file path with multiple slashes (cacheDir + '/A///////B/test.txt') and set it in DownloadConfig with valid URL.
     *           4. Call request.downloadFile() via callback; in the success callback, get the task and assert true.
     *           5. Bind the 'complete' event: check if the file exists at the standard path, assert true, delete the file, then call done().
     *           6. In the catch callback, assert false (task creation failed) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create subdirectories cacheDir + '/A' and '/A/B' if they do not exist.
     *           2. Define the standard file path (cacheDir + '/A/B/test.txt'), delete it if it exists.
     *           3. Construct a file path with dots and multiple slashes (cacheDir + '/A//////./B/.//test.txt') and set it in DownloadConfig with valid URL.
     *           4. Call request.downloadFile() via callback; in the success callback, get the task and assert true.
     *           5. Bind the 'complete' event: check if the file exists at the standard path, assert true, delete the file, then call done().
     *           6. In the catch callback, assert false (task creation failed) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Define the file path (cacheDir + '/test.txt'), delete it if it exists.
     *           2. Create a DownloadConfig object with valid URL, call request.downloadFile() via Promise to get the task.
     *           3. Try to call task.on() with error event type 'test' (type-cast to 'progress'); in the success callback, do nothing.
     *           4. Catch the thrown exception, assert true (expected error), call task.delete() to clean up, then call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Define the file path (cacheDir + '/test.txt'), delete it if it exists.
     *           2. Create a DownloadConfig object with valid URL, call request.downloadFile() via Promise to get the task.
     *           3. Try to call task.on() with error event type 123 (type-cast to 'progress'); in the success callback, do nothing.
     *           4. Catch the thrown exception, assert true (expected error), call task.delete() to clean up, then call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a DownloadConfig object with an invalid URL (http://www.ylong_http.client.com/test.txt).
     *           2. Call request.downloadFile() via Promise to get the task.
     *           3. Call task.on() to bind the 'fail' event; in the event callback, assert true and call done().
     *           4. Catch the thrown exception, assert false (unexpected error) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Define the file path (cacheDir + '/test.txt'), delete it if it exists.
     *           2. Create a DownloadConfig object with valid URL, call request.downloadFile() via Promise to get the task.
     *           3. Call task.on() to bind the 'progress' event; in the event callback, assert true, check if total size equals 1042003, unbind the event, then call done().
     *           4. Catch the thrown exception, assert false (unexpected error) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Define the file path (cacheDir + '/test.txt'), delete it if it exists.
     *           2. Create a DownloadConfig object with valid URL, call request.downloadFile() via Promise to get the task.
     *           3. Call task.on() twice to bind the 'progress' event (two different callbacks).
     *           4. Assert true (event binding succeeded), call task.delete() to clean up, then call done().
     *           5. Catch the thrown exception, assert false (unexpected error) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Define the file path (cacheDir + '/test.txt'), delete it if it exists.
     *           2. Create a DownloadConfig object with valid URL, call request.downloadFile() via Promise to get the task.
     *           3. Call task.on() to bind the 'complete' event; in the event callback, assert true, unbind the event, then call done().
     *           4. Catch the thrown exception, assert false (unexpected error) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Define the file path (cacheDir + '/test.txt'), delete it if it exists.
     *           2. Create a DownloadConfig object with valid URL, call request.downloadFile() via Promise to get the task.
     *           3. Call task.on() to bind the 'pause' event; in the event callback, assert true.
     *           4. Wait for 1 second, call task.suspend() to pause the task, then call done().
     *           5. Catch the thrown exception, assert false (unexpected error) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Define the file path (cacheDir + '/test.txt'), delete it if it exists.
     *           2. Create a DownloadConfig object with valid URL, call request.downloadFile() via Promise to get the task.
     *           3. Call task.on() to bind the 'remove' event; in the event callback, assert true and call done().
     *           4. Call task.delete() to delete the task, triggering the 'remove' event.
     *           5. Catch the thrown exception, assert false (unexpected error) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Define the file path (cacheDir + '/test.txt'), delete it if it exists.
     *           2. Create a DownloadConfig object with valid URL, call request.downloadFile() via Promise to get the task.
     *           3. Try to call task.off() with error event type 'test' (type-cast to 'progress'); in the success callback, do nothing.
     *           4. Catch the thrown exception, assert true (expected error), call task.delete() to clean up, then call done().
     * @tc.require: issues#1573
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
     * @tc.step: 1. Define the file path (cacheDir + '/test.txt'), delete it if it exists.
     *           2. Create a DownloadConfig object with valid URL, call request.downloadFile() via Promise to get the task.
     *           3. Try to call task.off() with error event type 123 (type-cast to 'progress'); in the success callback, call done().
     *           4. Catch the thrown exception, assert true (expected error), call task.delete() to clean up, then call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Define the file path (cacheDir + '/test.txt'), delete it if it exists.
     *           2. Create a DownloadConfig object with valid URL, call request.downloadFile() via Promise to get the task.
     *           3. Bind the 'complete' event to the task; in the event callback, assert true and call done().
     *           4. Define a CompletedCallback (asserts false if triggered), bind it to 'complete', then call task.off() to unbind it.
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Define the file path (cacheDir + '/test.txt'), delete it if it exists.
     *           2. Create a DownloadConfig object with valid URL, call request.downloadFile() via Promise to get the task.
     *           3. Define two CompletedCallbacks (both assert false if triggered), bind them to 'complete'.
     *           4. Call task.off('complete') to unbind all 'complete' callbacks, wait for 2 seconds.
     *           5. Assert true (unbind succeeded), then call done().
     *           6. Catch the thrown exception, assert false (unexpected error) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Define the file path (cacheDir + '/test.txt'), delete it if it exists.
     *           2. Create a DownloadConfig object with valid URL, call request.downloadFile() via Promise to get the task.
     *           3. Call task.off('complete') twice (unbind even if no callbacks exist).
     *           4. Assert true (double unbind succeeded), call task.delete() to clean up, then call done().
     *           5. Catch the thrown exception, assert false (unexpected error) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Define the file path (cacheDir + '/test.txt'), delete it if it exists.
     *           2. Create a DownloadConfig object with valid URL, call request.downloadFile() via Promise to get the task.
     *           3. Bind 'progress' event (asserts true if triggered) and 'complete' event (asserts true if triggered) to the task.
     *           4. Define a ProgressCallback (asserts false if triggered), bind it to 'progress', then call task.off() to unbind it.
     *           5. Catch the thrown exception, assert false (unexpected error) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Define the file path (cacheDir + '/test.txt'), delete it if it exists.
     *           2. Create a DownloadConfig object with valid URL, call request.downloadFile() via Promise to get the task.
     *           3. Bind 'complete' event (asserts true if triggered) to the task.
     *           4. Define two ProgressCallbacks (both assert false if triggered), bind them to 'progress'.
     *           5. Call task.off('progress') to unbind all 'progress' callbacks.
     *           6. Catch the thrown exception, assert false (unexpected error) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Define the file path (cacheDir + '/test.txt'), delete it if it exists.
     *           2. Create a DownloadConfig object with valid URL, call request.downloadFile() via Promise to get the task.
     *           3. Bind 'pause' event (asserts true if triggered) to the task; in the event callback, call done().
     *           4. Define a PauseCallback (asserts false if triggered), bind it to 'pause', then call task.off() to unbind it.
     *           5. Call task.suspend() to pause the task, triggering the remaining 'pause' callback.
     *           6. Catch the thrown exception, assert false (unexpected error) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Define the file path (cacheDir + '/test.txt'), delete it if it exists.
     *           2. Create a DownloadConfig object with valid URL, call request.downloadFile() via Promise to get the task.
     *           3. Define two PauseCallbacks (both assert false if triggered), bind them to the 'pause' event.
     *           4. Call task.off('pause') to unbind all 'pause' event callbacks.
     *           5. Call task.suspend() to pause the task, wait for 2 seconds (ensure no callbacks are triggered).
     *           6. Assert true (unbind succeeded), call task.delete() to clean up, then call done().
     *           7. Catch the thrown exception, assert false (unexpected error) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Define the file path (cacheDir + '/test.txt'), delete it if it exists.
     *           2. Create a DownloadConfig object with valid URL, call request.downloadFile() via Promise to get the task.
     *           3. Bind a 'remove' event callback (asserts true if triggered) to the task.
     *           4. Define a RemoveCallback (asserts false if triggered), bind it to 'remove', then call task.off() to unbind it.
     *           5. Call task.delete() to delete the task, triggering the remaining 'remove' callback.
     *           6. Catch the thrown exception, assert false (unexpected error) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Define the file path (cacheDir + '/test.txt'), delete it if it exists.
     *           2. Create a DownloadConfig object with valid URL, call request.downloadFile() via Promise to get the task.
     *           3. Define two RemoveCallbacks (both assert false if triggered), bind them to 'remove'.
     *           4. Call task.off('remove') to unbind all 'remove' callbacks, then bind a new 'remove' callback (asserts true if triggered).
     *           5. Call task.delete() to delete the task, triggering the new 'remove' callback.
     *           6. Catch the thrown exception, assert false (unexpected error) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a DownloadConfig object with an invalid URL (http://www.ylong_http.client.com/test.txt).
     *           2. Call request.downloadFile() via Promise to get the task.
     *           3. Bind a 'fail' event callback (asserts true if triggered) to the task.
     *           4. Define a FailCallback (asserts false if triggered), bind it to 'fail', then call task.off() to unbind it.
     *           5. Catch the thrown exception, assert false (unexpected error) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Define the file path (cacheDir + '/test.txt'), delete it if it exists.
     *           2. Create a DownloadConfig object with an invalid URL (http://www.ylong_http.client.com/test.txt).
     *           3. Call request.downloadFile() via Promise to get the task.
     *           4. Define two FailCallbacks (both assert false if triggered), bind them to 'fail'.
     *           5. Call task.off('fail') to unbind all 'fail' callbacks, then bind a new 'fail' callback (asserts true if triggered).
     *           6. Catch the thrown exception, assert false (unexpected error) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Define the file path (cacheDir + '/test.txt'), delete it if it exists.
     *           2. Create a DownloadConfig object with valid URL, call request.downloadFile() via Promise to get the task.
     *           3. Call task.suspend() to pause the task.
     *           4. Call task.restore() via Promise; in the success callback, assert the result is true and call done().
     *           5. In the catch callback of restore(), assert false (restore failed) and call done().
     *           6. Catch the thrown exception, assert false (unexpected error) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Define the file path (cacheDir + '/test.txt'), delete it if it exists.
     *           2. Create a DownloadConfig object with valid URL, call request.downloadFile() via Promise to get the task.
     *           3. Call task.suspend() via await to ensure the task is paused.
     *           4. Call task.restore() via callback; in the callback, assert true (restore succeeded), call task.delete() to clean up, then call done().
     *           5. Catch the thrown exception, assert false (unexpected error) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Try to call request.downloadFile() with an empty string (type-cast to DownloadConfig) to get an invalid task.
     *           2. Call task.delete() on the invalid task.
     *           3. Catch the thrown exception, assert true (expected parameter error) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Define the file path (cacheDir + '/test.txt'), delete it if it exists.
     *           2. Create a DownloadConfig object with valid URL, call request.downloadFile() via Promise to get the task.
     *           3. Call task.delete() via Promise; in the success callback, assert true (delete succeeded) and call done().
     *           4. In the catch callback of delete(), assert false (delete failed) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Define the file path (cacheDir + '/test.txt'), delete it if it exists.
     *           2. Create a DownloadConfig object with valid URL, call request.downloadFile() via Promise to get the task.
     *           3. Call task.delete() via callback; in the callback, assert true (delete succeeded) and call done().
     *           4. Catch the thrown exception, assert false (unexpected error) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Define the file path (cacheDir + '/test.txt'), delete it if it exists.
     *           2. Create a DownloadConfig object with valid URL, call request.downloadFile() via Promise to get the task.
     *           3. Define a CompletedCallback, bind it to the 'complete' event.
     *           4. In CompletedCallback, call task.getTaskInfo() via Promise; assert downloadInfo.status equals 0 (completed) and call done().
     *           5. In the catch callback of getTaskInfo(), assert false (failed to get task info) and call done().
     *           6. Catch the thrown exception, assert false (unexpected error) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Define the file path (cacheDir + '/test.txt'), delete it if it exists.
     *           2. Create a DownloadConfig object with valid URL, call request.downloadFile() via Promise to get the task.
     *           3. Define a CompletedCallback, bind it to the 'complete' event.
     *           4. In CompletedCallback, call task.getTaskInfo() via callback; assert downloadInfo.status equals 0 (completed) and call done().
     *           5. Catch the thrown exception, assert false (unexpected error) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Define the file path (cacheDir + '/test.txt'), delete it if it exists.
     *           2. Create a DownloadConfig object with valid URL, call request.downloadFile() via Promise to get the task.
     *           3. Call task.suspend() via await to ensure the task is paused.
     *           4. Call task.getTaskInfo() via Promise; assert downloadInfo.status equals 3 (paused), call task.delete() to clean up, then call done().
     *           5. In the catch callback of getTaskInfo(), assert false (failed to get task info) and call done().
     *           6. Catch the thrown exception, assert false (unexpected error) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Define the file path (cacheDir + '/test.txt'), delete it if it exists.
     *           2. Create a DownloadConfig object with valid URL, call request.downloadFile() via Promise to get the task.
     *           3. Call task.suspend() via await to ensure the task is paused.
     *           4. Call task.getTaskInfo() via callback; assert downloadInfo.status equals 3 (paused), call task.delete() to clean up, then call done().
     *           5. Catch the thrown exception, assert false (unexpected error) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a DownloadConfig object with valid URL (large_test.zip) and filePath: DownloadGetTaskInfo003.
     *           2. Call request.downloadFile() via Promise to get the task.
     *           3. Define a ProgressCallback, bind it to the 'progress' event.
     *           4. In ProgressCallback: unbind the 'progress' event, call task.getTaskInfo() via Promise.
     *           5. Assert downloadInfo.status equals 1 (in progress), call task.delete() to clean up, then call done().
     *           6. In the catch callback of getTaskInfo(), assert false (failed to get task info) and call done().
     *           7. Catch the thrown exception, assert false (unexpected error) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a DownloadConfig object with valid URL (large_test.zip) and filePath: DownloadGetTaskInfo0031.
     *           2. Call request.downloadFile() via Promise to get the task.
     *           3. Bind a 'progress' event callback to the task.
     *           4. In the callback: unbind the 'progress' event, call task.getTaskInfo() via callback.
     *           5. Assert downloadInfo.status equals 1 (in progress), call task.delete() to clean up, then call done().
     *           6. Catch the thrown exception, assert false (unexpected error) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Define the file path (cacheDir + '/test.txt'), delete it if it exists.
     *           2. Create a DownloadConfig object with an invalid URL (http://www.ylong_http.client.com/test.txt).
     *           3. Call request.downloadFile() via Promise to get the task.
     *           4. Define a FailCallback, bind it to the 'fail' event.
     *           5. In FailCallback, call task.getTaskInfo() via Promise; assert downloadInfo.status equals 4 (failed), call task.delete() to clean up, then call done().
     *           6. In the catch callback of getTaskInfo(), assert false (failed to get task info) and call done().
     *           7. Catch the thrown exception, assert false (unexpected error) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Define the file path (cacheDir + '/test.txt'), delete it if it exists.
     *           2. Create a DownloadConfig object with an invalid URL (http://www.ylong_http.client.com/test.txt).
     *           3. Call request.downloadFile() via Promise to get the task.
     *           4. Define a FailCallback, bind it to the 'fail' event.
     *           5. In FailCallback, call task.getTaskInfo() via callback; assert downloadInfo.status equals 4 (failed), call task.delete() to clean up, then call done().
     *           6. Catch the thrown exception, assert false (unexpected error) and call done().
     * @tc.require: issues#1573
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