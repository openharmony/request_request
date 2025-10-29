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
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or impligetProfileByAbilityEmptyAbilityCalled.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
import { describe, beforeAll, beforeEach, afterEach, afterAll, it, expect } from "@ohos/hypium"
import request from '@ohos.request';
import fs from '@ohos.file.fs';
import common from '@ohos.app.ability.common';
import wantAgent, { WantAgent } from '@ohos.app.ability.wantAgent';

export default function requestHttpConfigTest() {
  describe('requestHttpConfigTest', () => {
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
    let fileSpec: request.agent.FileSpec = {
      path: `./test.txt`
    };
    let formItem: request.agent.FormItem = {
      name: 'file',
      value: fileSpec
    };
    let fileSpec1: request.agent.FileSpec = {
      filename: 't1.txt',
      path: `./test.txt`
    };
    let fileSpec2: request.agent.FileSpec = {
      filename: 't2.txt',
      path: `./test.txt`
    };
    let fileSpec3: request.agent.FileSpec = {
      filename: 't3.txt',
      path: `./test.txt`
    };
    let formItem1: request.agent.FormItem = {
      name: 'file',
      value: [fileSpec1, fileSpec2, fileSpec3]
    };

    let wantAgentInfo: wantAgent.WantAgentInfo = {
      wants: [
        {
          deviceId: '',
          bundleName: 'com.example.request',
          abilityName: 'EntryAbility',
          action: '',
          entities: [],
          uri: '',
          parameters: {}
        }
      ],
      actionType: wantAgent.OperationType.START_ABILITY,
      requestCode: 0,
      wantAgentFlags:[wantAgent.WantAgentFlags.CONSTANT_FLAG]
    };
    /**
     * @tc.number: testTaskUrl001
     * @tc.name: testTaskUrl001
     * @tc.desc: Test create task when lack url
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a Record object (for type conversion) with action: DOWNLOAD, no url parameter.
     *           2. Try to call request.agent.create() with type-cast config; in the success callback 
     *             (unexpected), assert false and call done().
     *           3. Catch the thrown exception, assert err.code equals 401 (missing url error), 
     *             then call done().
     * @tc.require: issues#1573
     */
    it('testTaskUrl001', 0, async (done: Function) => {
      let conf: Record<string, string | number> = {
        "action": request.agent.Action.DOWNLOAD,
      };
      try {
        request.agent.create(context, conf as Object as request.agent.Config, (err) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });

    /**
     * @tc.number: testTaskUrl002
     * @tc.name: testTaskUrl002
     * @tc.desc: Test create task when url is empty
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a Config object with action: DOWNLOAD, url set to empty string ('').
     *           2. Try to call request.agent.create() to create a task; in the success callback 
     *             (unexpected), assert false and call done().
     *           3. Catch the thrown exception, assert err.code equals 401 (invalid url error), 
     *             then call done().
     * @tc.require: issues#1573
     */
    it('testTaskUrl002', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: '',
      };
      try {
        request.agent.create(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });

    /**
     * @tc.number: testTaskUrl003
     * @tc.name: testTaskUrl003
     * @tc.desc: Test create task when url is not support download
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a Config object with action: DOWNLOAD, url set to a non-download-supporting 
     *             address (https://gitee.com/openharmony/request_request).
     *           2. Call request.agent.create() with a callback; in the callback, assert err is null 
     *             (task created successfully).
     *           3. Clean up by calling request.agent.remove() to delete the task, then call done().
     * @tc.require: issues#1573
     */
    it('testTaskUrl003', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/openharmony/request_request',
      };
      request.agent.create(context, conf, async (err, data) => {
        expect(err === null).assertTrue();
        await request.agent.remove(data.tid);
        done();
      });
    });

    /**
     * @tc.number: testTaskUrl004
     * @tc.name: testTaskUrl004
     * @tc.desc: Test create task when url is not support upload
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. In the cache directory, create a test file (test.txt) with read-write permissions 
     *             and close it.
     *           2. Create a Config object with action: UPLOAD, url set to a non-upload-supporting 
     *             address, and data containing the test file.
     *           3. Call request.agent.create() with a callback; in the callback, assert err is null 
     *             (task created successfully).
     *           4. Clean up by calling request.agent.remove() to delete the task, then call done().
     * @tc.require: issues#1573
     */
    it('testTaskUrl004', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'https://gitee.com/openharmony/request_request',
        data: [formItem]
      };
      request.agent.create(context, conf, async (err, data) => {
        expect(err === null).assertTrue();
        await request.agent.remove(data.tid);
        done();
      });
    });

    /**
     * @tc.number: testTaskUrl005
     * @tc.name: testTaskUrl005
     * @tc.desc: Test create task when url is not http/https scheme.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a Config object with action: DOWNLOAD, url set to non-http/https scheme 
     *             (admin://127.0.0.1).
     *           2. Try to call request.agent.create() to create a task; in the success callback 
     *             (unexpected), assert false and call done().
     *           3. Catch the thrown exception, assert err.code equals 401 (invalid scheme error), 
     *             then call done().
     * @tc.require: issues#1573
     */
    it('testTaskUrl005', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'admin://127.0.0.1',
      };
      try {
        request.agent.create(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });

    /**
     * @tc.number: testTaskUrl006
     * @tc.name: testTaskUrl006
     * @tc.desc: Test create task when url length is 8193.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a Config object with action: DOWNLOAD, url composed of 'http://127.0.0.1/' 
     *             plus 8176 'a's (total length 8193), and saveas: testTaskUrl006.
     *           2. Try to call request.agent.create() to create a task; in the success callback 
     *             (unexpected), assert false and call done().
     *           3. Catch the thrown exception, assert err.code equals 401 (url too long error), 
     *             then call done().
     * @tc.require: issues#1573
     */
    it('testTaskUrl006', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'http://127.0.0.1/' + 'a'.repeat(8176),
        saveas: 'testTaskUrl006'
      };
      try {
        request.agent.create(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });

    /**
     * @tc.number: testTaskUrl007
     * @tc.name: testTaskUrl007
     * @tc.desc: Test create task when url length is 8192.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a Config object with action: DOWNLOAD, url composed of 'http://127.0.0.1/' 
     *             plus 8175 'a's (total length 8192), and saveas: testTaskUrl007.
     *           2. Call request.agent.create() with a callback; in the callback, assert true 
     *             (task created successfully).
     *           3. Clean up by calling request.agent.remove() to delete the task, then call done().
     *           4. In the outer catch, assert false (unexpected exception) and call done().
     * @tc.require: issues#1573
     */
    it('testTaskUrl007', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'http://127.0.0.1/' + 'a'.repeat(8175),
        saveas: 'testTaskUrl007'
      };
      try {
        request.agent.create(context, conf, async (err, data) => {
          expect(true).assertTrue();
          await request.agent.remove(data.tid);
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testTaskUrl008
     * @tc.name: testTaskUrl008
     * @tc.desc: Test upload when url length is 8192
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. In the cache directory, create a test file (test.txt) with read-write permissions 
     *             and close it.
     *           2. Create a Config object with action: UPLOAD, url composed of 'http://127.0.0.1/' 
     *             plus 8175 'a's (total length 8192), data containing the test file, and saveas: 
     *             testTaskUrl008.
     *           3. Call request.agent.create() with a callback; in the callback, assert true 
     *             (task created successfully), then call done().
     *           4. In the outer catch, assert false (unexpected exception) and call done().
     * @tc.require: issues#1573
     */
    it('testTaskUrl008', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1/' + 'a'.repeat(8175),
        data: [formItem],
        saveas: 'testTaskUrl008'
      };
      try {
        request.agent.create(context, conf, (err, data) => {
          expect(true).assertTrue();
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testTaskUrl009
     * @tc.name: testTaskUrl009
     * @tc.desc: Test upload when url length is 8193
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. In the cache directory, create a test file (test.txt) with read-write permissions 
     *             and close it.
     *           2. Create a Config object with action: UPLOAD, url composed of 'http://127.0.0.1/' 
     *             plus 8176 'a's (total length 8193), data containing the test file, and saveas: 
     *             testTaskUrl009.
     *           3. Try to call request.agent.create() to create a task; in the success callback 
     *             (unexpected), assert false and call done().
     *           4. Catch the thrown exception, assert err.code equals 401 (url too long error), 
     *             then call done().
     * @tc.require: issues#1573
     */
    it('testTaskUrl009', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1/' + 'a'.repeat(8176),
        data: [formItem],
        saveas: 'testTaskUrl009'
      };
      try {
        request.agent.create(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });

    /**
     * @tc.number: testTaskMethod001
     * @tc.name: testTaskMethod001
     * @tc.desc: Test create task when method is POST for upload
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. In the cache directory, create a test file (test.txt) with read-write permissions 
     *             and close it.
     *           2. Create a Config object with action: UPLOAD, url: http://127.0.0.1, data containing 
     *             the test file, and method: 'POST'.
     *           3. Call request.agent.create() to create an upload task instance.
     *           4. Call task.start() to execute the task; in the success callback, assert true 
     *             (task executed successfully) and call done().
     *           5. In the catch callback, assert false (task execution failed) and call done().
     * @tc.require: issues#1573
     */
    it('testTaskMethod001', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
          name: 'file',
          value: {
            path: `./test.txt`
          },
        }],
        method: 'POST'
      };
      let task = await request.agent.create(context, conf);
      task.start().then(() => {
        expect(true).assertTrue();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });

    /**
     * @tc.number: testTaskMethod002
     * @tc.name: testTaskMethod002
     * @tc.desc: Test create task when method is POST for download
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a Config object with action: DOWNLOAD, valid URL, saveas: testTaskMethod002.txt, 
     *             and method: 'POST'.
     *           2. Call request.agent.create() to create a download task instance.
     *           3. Call task.start() to execute the task; in the success callback, assert true 
     *             (task executed successfully).
     *           4. Clean up by calling request.agent.remove() to delete the task, then call done().
     *           5. In the catch callback, assert false (task execution failed) and call done().
     * @tc.require: issues#1573
     */
    it('testTaskMethod002', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskMethod002.txt',
        method: 'POST'
      };
      let task = await request.agent.create(context, conf);
      task.start().then(async () => {
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });

    /**
     * @tc.number: testTaskMethod003
     * @tc.name: testTaskMethod003
     * @tc.desc: Test create task when method is number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a Record object (for type conversion) with action: DOWNLOAD, valid URL, 
     *             saveas: testTaskMethod003.txt, and method: 123 (number type).
     *           2. Call request.agent.create() with type-cast config to create a download task instance.
     *           3. Call task.start() to execute the task; in the success callback, assert 
     *             task.config.method equals 123.
     *           4. Clean up by calling request.agent.remove() to delete the task, then call done().
     *           5. In the catch callback, assert false (task execution failed) and call done().
     * @tc.require: issues#1573
     */
    it('testTaskMethod003', 0, async (done: Function) => {
      let conf: Record<string, string | number> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "saveas": "testTaskMethod003.txt",
        "method": 123
      };
      let task = await request.agent.create(context, conf as Object as request.agent.Config);
      task.start().then(async () => {
        expect(task.config.method).assertEqual(123);
        await request.agent.remove(task.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });

    /**
     * @tc.number: testTaskMethod004
     * @tc.name: testTaskMethod004
     * @tc.desc: Test create task when method is empty
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a Config object with action: DOWNLOAD, valid URL, saveas: testTaskMethod004.txt, 
     *             and method: '' (empty string).
     *           2. Call request.agent.create() to create a download task instance.
     *           3. Call task.start() to execute the task; in the success callback, assert 
     *             task.config.method equals ''.
     *           4. Clean up by calling request.agent.remove() to delete the task, then call done().
     *           5. In the catch callback, assert false (task execution failed) and call done().
     * @tc.require: issues#1573
     */
    it('testTaskMethod004', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskMethod004.txt',
        method: ''
      };
      let task = await request.agent.create(context, conf);
      task.start().then(async () => {
        expect(task.config.method).assertEqual('');
        await request.agent.remove(task.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });

    /**
     * @tc.number: testTaskMethod005
     * @tc.name: testTaskMethod005
     * @tc.desc: Test create task when method is GET for upload
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. In the cache directory, create a test file (test.txt) with read-write permissions 
     *             and close it.
     *           2. Create a Config object with action: UPLOAD, url: http://127.0.0.1, data containing 
     *             the test file, and method: 'GET'.
     *           3. Call request.agent.create() to create an upload task instance.
     *           4. Call task.start() to execute the task; in the success callback, assert true 
     *             (task executed successfully).
     *           5. Clean up by calling request.agent.remove() to delete the task, then call done().
     *           6. In the catch callback, assert false (task execution failed) and call done().
     * @tc.require: issues#1573
     */
    it('testTaskMethod005', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
          name: 'file',
          value: {
            path: `./test.txt`
          },
        }],
        method: 'GET'
      };
      let task = await request.agent.create(context, conf);
      task.start().then(async () => {
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });

    /**
     * @tc.number: testTaskMethod006
     * @tc.name: testTaskMethod006
     * @tc.desc: Test create task when method is PUT for download
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Wait for 2 seconds (to avoid potential conflicts with previous tasks).
     *           2. Create a Config object with action: DOWNLOAD, valid URL, saveas: testTaskMethod006.txt, 
     *             and method: 'PUT'.
     *           3. Call request.agent.create() to create a download task instance.
     *           4. Call task.start() to execute the task; in the success callback, assert true 
     *             (task executed successfully).
     *           5. Clean up by calling request.agent.remove() to delete the task, then call done().
     *           6. In the catch callback, assert false (task execution failed) and call done().
     * @tc.require: issues#1573
     */
    it('testTaskMethod006', 0, async (done: Function) => {
      await sleep(2000);
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskMethod006.txt',
        method: 'PUT'
      };
      let task = await request.agent.create(context, conf);
      task.start().then(async () => {
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });

    /**
     * @tc.number: testTaskHeaders001
     * @tc.name: testTaskHeaders001
     * @tc.desc: Test create task when headers content-type is application/json but data is file for upload
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. In the cache directory, create a test file (test.txt) with read-write permissions 
     *             and close it.
     *           2. Create a Config object with action: UPLOAD, url: http://127.0.0.1, data containing 
     *             the test file, and headers: {'content-type': 'application/json'}.
     *           3. Call request.agent.create() to create an upload task instance.
     *           4. Call task.start() to execute the task; in the success callback, assert true 
     *             (task executed successfully) and call done().
     *           5. In the catch callback, assert false (task execution failed) and call done().
     * @tc.require: issues#1573
     */
    it('testTaskHeaders001', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
          name: 'file',
          value: {
            path: `./test.txt`
          },
        }],
        headers: {
          'content-type': 'application/json'
        },
      };
      let task = await request.agent.create(context, conf);
      task.start().then(() => {
        expect(true).assertTrue();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });

    /**
     * @tc.number: testTaskHeaders002
     * @tc.name: testTaskHeaders002
     * @tc.desc: Test create task when lack headers for upload
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. In the cache directory, create a test file (test.txt) with read-write permissions 
     *             and close it.
     *           2. Create a Config object with action: UPLOAD, url: http://127.0.0.1, data containing 
     *             the test file, no headers parameter.
     *           3. Call request.agent.create() to create an upload task instance.
     *           4. Call task.start() to execute the task; in the success callback, assert 
     *             typeof task.config.headers equals 'undefined' (no headers set) and call done().
     *           5. In the catch callback, assert false (task execution failed) and call done().
     * @tc.require: issues#1573
     */
    it('testTaskHeaders002', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
          name: 'file',
          value: {
            path: `./test.txt`
          },
        }],
      };
      let task = await request.agent.create(context, conf);
      task.start().then(() => {
        expect(typeof task.config.headers).assertEqual('undefined');
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });

    /**
     * @tc.number: testTaskHeaders003
     * @tc.name: testTaskHeaders003
     * @tc.desc: Test create task when lack headers for download
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a Config object with action: DOWNLOAD, valid URL, saveas: testTaskHeaders003.txt, 
     *             no headers parameter.
     *           2. Call request.agent.create() to create a download task instance.
     *           3. Call task.start() to execute the task; in the success callback, assert 
     *             typeof task.config.headers equals 'undefined' (no headers set).
     *           4. Clean up by calling request.agent.remove() to delete the task, then call done().
     *           5. In the catch callback, assert false (task execution failed) and call done().
     * @tc.require: issues#1573
     */
    it('testTaskHeaders003', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskHeaders003.txt',
      };
      let task = await request.agent.create(context, conf);
      task.start().then(async () => {
        expect(typeof task.config.headers).assertEqual('undefined');
        await request.agent.remove(task.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    
    /**
     * @tc.number: testTaskHeaders004
     * @tc.name: testTaskHeaders004
     * @tc.desc: Test create task when headers content-type is multipart/form-data but data is file for download
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a Config object with action: DOWNLOAD, valid URL, saveas: README.md, 
     *             mode: BACKGROUND, and headers: {'content-type': 'multipart/form-data'}.
     *           2. Call request.agent.create() to create a download task instance.
     *           3. Call task.start() to execute the task; in the success callback, assert true 
     *             (task executed successfully) and call done().
     *           4. In the catch callback, assert false (task execution failed) and call done().
     * @tc.require: issues#1573
     */
    it('testTaskHeaders004', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: `README.md`,
        mode: request.agent.Mode.BACKGROUND,
        headers: {
          'content-type': 'multipart/form-data'
        },
      };
      let task = await request.agent.create(context, conf);
      task.start().then(() => {
        expect(true).assertTrue();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });

    /**
     * @tc.number: testTaskProxy001
     * @tc.name: testTaskProxy001
     * @tc.desc: Test create task when proxy format error
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a Config object with action: DOWNLOAD, valid URL, proxy set to invalid format 
     *             (https://127.0.0.1:8080).
     *           2. Try to call request.agent.create() to create a task; in the success callback 
     *             (unexpected), assert false and call done().
     *           3. Catch the thrown exception, assert err.code equals 401 (invalid proxy format error), 
     *             then call done().
     * @tc.require: issues#1573
     */
    it('testTaskProxy001', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        proxy: 'https://127.0.0.1:8080'
      };
      try {
        request.agent.create(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });

    /**
     * @tc.number: testTaskProxy002
     * @tc.name: testTaskProxy002
     * @tc.desc: Test create task when proxy format error
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a Config object with action: DOWNLOAD, valid URL, proxy set to invalid format 
     *             (http://127.0.0.1:8080/proxy).
     *           2. Try to call request.agent.create() to create a task; in the success callback 
     *             (unexpected), assert false and call done().
     *           3. Catch the thrown exception, assert err.code equals 401 (invalid proxy format error), 
     *             then call done().
     * @tc.require: issues#1573
     */
    it('testTaskProxy002', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        proxy: 'http://127.0.0.1:8080/proxy'
      };
      try {
        request.agent.create(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });

    /**
     * @tc.number: testTaskProxy003
     * @tc.name: testTaskProxy003
     * @tc.desc: Test create task when proxy format error
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a Config object with action: DOWNLOAD, valid URL, proxy set to invalid format 
     *             (http://127.0.0.1:111111, port exceeds range).
     *           2. Try to call request.agent.create() to create a task; in the success callback 
     *             (unexpected), assert false and call done().
     *           3. Catch the thrown exception, assert err.code equals 401 (invalid proxy format error), 
     *             then call done().
     * @tc.require: issues#1573
     */
    it('testTaskProxy003', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        proxy: 'http://127.0.0.1:111111'
      };
      try {
        request.agent.create(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });

    /**
     * @tc.number: testTaskDescription001
     * @tc.name: testTaskDescription001
     * @tc.desc: Test create task when description is given
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a Config object with action: DOWNLOAD, valid URL, description: 'test upload', 
     *             and saveas: testTaskDescription001.txt.
     *           2. Call request.agent.create() to create a download task instance.
     *           3. Assert task.config.description equals 'test upload' (description set correctly).
     *           4. Clean up by calling request.agent.remove() to delete the task, then call done().
     * @tc.require: issues#1573
     */
    it('testTaskDescription001', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        description: 'test upload',
        saveas: 'testTaskDescription001.txt',
      };
      let task = await request.agent.create(context, conf);
      expect(task.config.description).assertEqual('test upload');
      await request.agent.remove(task.tid);
      done();
    });

    /**
     * @tc.number: testTaskDescription002
     * @tc.name: testTaskDescription002
     * @tc.desc: Test create task when description is number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a Record object (for type conversion) with action: DOWNLOAD, valid URL, 
     *             description: 123 (number type), and saveas: testTaskDescription002.txt.
     *           2. Call request.agent.create() with type-cast config and a callback; in the callback, 
     *             assert data.config.description equals 123 (description set correctly).
     *           3. Clean up by calling request.agent.remove() to delete the task, then call done().
     * @tc.require: issues#1573
     */
    it('testTaskDescription002', 0, async (done: Function) => {
      let conf: Record<string, string | number> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "description": 123,
        "saveas": "testTaskDescription002.txt",
      };
      request.agent.create(context, conf as Object as request.agent.Config, async (err, data) => {
        expect(data.config.description).assertEqual(123);
        await request.agent.remove(data.tid);
        done();
      });
    });

    /**
     * @tc.number: testTaskDescription003
     * @tc.name: testTaskDescription003
     * @tc.desc: Test create task when description length exceeds 1024
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a Config object with action: DOWNLOAD, valid URL, saveas: testTaskDescription003.txt, 
     *             and description: 1025 'a's (length exceeds limit).
     *           2. Try to call request.agent.create() to create a task; in the success callback 
     *             (unexpected), assert false and call done().
     *           3. Catch the thrown exception, assert err.code equals 401 (description too long error), 
     *             then call done().
     * @tc.require: issues#1573
     */
    it('testTaskDescription003', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskDescription003.txt',
        description: 'a'.repeat(1025)
      };
      try {
        request.agent.create(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });

    /**
     * @tc.number: testTaskDescription004
     * @tc.name: testTaskDescription004
     * @tc.desc: Test create task when description length is 1024
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a Config object with action: DOWNLOAD, valid URL, description: 1024 'a's 
     *             (length equals limit), and saveas: testTaskDescription004.txt.
     *           2. Call request.agent.create() with a callback; in the callback, assert err is null 
     *             (task created successfully).
     *           3. Clean up by calling request.agent.remove() to delete the task, then call done().
     * @tc.require: issues#1573
     */
    it('testTaskDescription004', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        description: 'a'.repeat(1024),
        saveas: 'testTaskDescription004.txt',
      };
      request.agent.create(context, conf, async (err, data) => {
        expect(err === null).assertTrue();
        await request.agent.remove(data.tid);
        done();
      });
    });
  })
}