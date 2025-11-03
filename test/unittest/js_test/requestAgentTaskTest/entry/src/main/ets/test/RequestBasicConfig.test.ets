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

export default function requestBasicConfigTest() {
  describe('requestBasicConfigTest', () => {
    beforeAll(() => {
      console.info('beforeAll called');
    });
    afterAll(() => {
      console.info('afterAll called');
    });
    beforeEach(() => {
      console.info('beforeEach called');
      if (fs.accessSync(cacheDir + '/test.txt')) {
        fs.unlinkSync(cacheDir + '/test.txt');
      }
      if (fs.accessSync(cacheDir + '/test1.txt')) {
        fs.unlinkSync(cacheDir + '/test1.txt');
      }
    });
    afterEach(async () => {
      console.info('afterEach called');
      if (fs.accessSync(cacheDir + '/test.txt')) {
        fs.unlinkSync(cacheDir + '/test.txt');
      }
      if (fs.accessSync(cacheDir + '/test1.txt')) {
        fs.unlinkSync(cacheDir + '/test1.txt');
      }
    });

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
     * @tc.number: testTaskAction001
     * @tc.name: testTaskAction001
     * @tc.desc: Test create task when lack action
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a config object with only url and saveas (no action field).
     *           2. Call request.agent.create() with the config, attempt to create a task.
     *           3. Verify the call throws an error, and the error code is 401.
     * @tc.require: issues#1573
     */
    it('testTaskAction001', 0, (done: Function) => {
      let conf: Record<string, string> = {
        "url": "http://127.0.0.1",
        "saveas": "testTaskAction001"
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
     * @tc.number: testTaskAction002
     * @tc.name: testTaskAction002
     * @tc.desc: Test create task when action is string
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a config object with action set to string "UPLOAD", plus url and saveas.
     *           2. Call request.agent.create() with the config, attempt to create a task.
     *           3. Verify the call throws an error: code 401 and error message contains "action".
     * @tc.require: issues#1573
     */
    it('testTaskAction002', 0, async (done: Function) => {
      let conf: Record<string, string> = {
        "action": "UPLOAD",
        "url": "http://127.0.0.1",
        "saveas": "testTaskAction002"
      };
      try {
        request.agent.create(context, conf as Object as request.agent.Config, (err) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.message.toLowerCase().includes('action')).assertTrue();
        expect(err.code).assertEqual(401);
        done();
      }
    });

    /**
     * @tc.number: testTaskAction003
     * @tc.name: testTaskAction003
     * @tc.desc: Test create task when action is 2
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a Config object with action set to number 2, plus url and saveas.
     *           2. Call request.agent.create() with the config, attempt to create a task.
     *           3. Verify the call throws an error, and the error code is 401.
     * @tc.require: issues#1573
     */
    it('testTaskAction003', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: 2,
        url: 'http://127.0.0.1',
        saveas: "testTaskAction003"
      };
      try {
        request.agent.create(context, conf, (err) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });

    /**
     * @tc.number: testTaskAction004
     * @tc.name: testTaskAction004
     * @tc.desc: Test create task when action is UPLOAD
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a test.txt file in cacheDir, then close the file.
     *           2. Create a Config object with action set to request.agent.Action.UPLOAD, plus url and data.
     *           3. Call request.agent.create() to create a task; verify no error (err is null).
     *           4. Call request.agent.remove() with the task's tid to clean up, then finish the test.
     * @tc.require: issues#1573
     */
    it('testTaskAction004', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [formItem]
      };
      request.agent.create(context, conf, async (err, data) => {
        expect(err === null).assertTrue();
        await request.agent.remove(data.tid);
        done();
      });
    });

    /**
     * @tc.number: testTaskAction005
     * @tc.name: testTaskAction005
     * @tc.desc: Test create task when action is DOWNLOAD
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a Config object with action set to request.agent.Action.DOWNLOAD, 
     *              plus valid test URL, saveas and gauge: true.
     *           2. Call request.agent.create() with the config, use then() to handle success.
     *           3. Verify the target file exists via fs.accessSync(), then call request.agent.remove() to clean up.
     *           4. If catch() is triggered, assert failure; otherwise finish the test.
     * @tc.require: issues#1573
     */
    it('testTaskAction005', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: `./testTaskAction005`,
        gauge: true
      };
      request.agent.create(context, conf).then(async (data) => {
        if (fs.accessSync(`${cacheDir}/testTaskAction005`)) {
          expect(true).assertTrue();
          await request.agent.remove(data.tid);
          done();
        }
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });

    /**
     * @tc.number: testTaskAction006
     * @tc.name: testTaskAction006
     * @tc.desc: Test create task when action is DOWNLOAD
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a Config object with action set to number 0 (representing DOWNLOAD), 
     *              plus valid test URL, saveas and gauge: true.
     *           2. Call request.agent.create() with the config, use then() to handle success.
     *           3. Verify the target file exists via fs.accessSync(), then call request.agent.remove() to clean up.
     *           4. If catch() is triggered, assert failure; otherwise finish the test.
     * @tc.require: issues#1573
     */
    it('testTaskAction006', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: 0,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: `./testTaskAction006`,
        gauge: true
      };
      request.agent.create(context, conf).then(async (data) => {
        if (fs.accessSync(`${cacheDir}/testTaskAction006`)) {
          expect(true).assertTrue();
          await request.agent.remove(data.tid);
          done();
        }
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });

    /**
     * @tc.number: testTaskAction007
     * @tc.name: testTaskAction007
     * @tc.desc: Test create task when action is 1
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a test.txt file in cacheDir, then close the file.
     *           2. Create a Config object with action set to number 1 (representing UPLOAD), 
     *              plus url and data.
     *           3. Call request.agent.create() to create a task; verify no error (err is null).
     *           4. Call request.agent.remove() with the task's tid to clean up, then finish the test.
     * @tc.require: issues#1573
     */
    it('testTaskAction007', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: 1,
        url: 'http://127.0.0.1',
        data: [formItem]
      };
      request.agent.create(context, conf, async (err, data) => {
        expect(err === null).assertTrue();
        await request.agent.remove(data.tid);
        done();
      });
    });

    /**
     * @tc.number: testTaskMode001
     * @tc.name: testTaskMode001
     * @tc.desc: Test create task when mode is FOREGROUND
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a Config object with action: DOWNLOAD, valid test URL, saveas, 
     *              and mode set to request.agent.Mode.FOREGROUND.
     *           2. Call request.agent.create() with the config; use then() to handle success.
     *           3. Assert success (true === true), call request.agent.remove() to clean up, then finish the test.
     *           4. If catch() is triggered, assert failure.
     * @tc.require: issues#1573
     */
    it('testTaskMode001', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskMode001.txt',
        mode: request.agent.Mode.FOREGROUND
      };
      request.agent.create(context, conf).then(async (data) => {
        expect(true).assertTrue();
        await request.agent.remove(data.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });

    /**
     * @tc.number: testTaskMode002
     * @tc.name: testTaskMode002
     * @tc.desc: Test create task when mode is BACKGROUND
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a Config object with action: DOWNLOAD, valid test URL, saveas, 
     *              and mode set to request.agent.Mode.BACKGROUND.
     *           2. Call request.agent.create() with the config; use then() to handle success.
     *           3. Assert success (true === true), call request.agent.remove() to clean up, then finish the test.
     *           4. If catch() is triggered, assert failure.
     * @tc.require: issues#1573
     */
    it('testTaskMode002', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskMode002.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      request.agent.create(context, conf).then(async (data) => {
        expect(true).assertTrue();
        await request.agent.remove(data.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });

    /**
     * @tc.number: testTaskMode003
     * @tc.name: testTaskMode003
     * @tc.desc: Test create task when mode is string
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a config object with action: DOWNLOAD, valid test URL, saveas, and mode as "BACKGROUND".
     *           2. Await request.agent.create() with the config (cast to valid type) to get task instance.
     *           3. Call task.start(), on success assert true, clean up with remove(), then finish test.
     *           4. If start() fails, assert false and finish test.
     * @tc.require: issues#1573
     */
    it('testTaskMode003', 0, async (done: Function) => {
      let conf: Record<string, string | number> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "saveas": "testTaskMode003.txt",
        "mode": "BACKGROUND"
      };
      let task = await request.agent.create(context, conf as Object as request.agent.Config);
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
     * @tc.number: testTaskMode004
     * @tc.name: testTaskMode004
     * @tc.desc: Test create task when mode is 0
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a Config object with action: DOWNLOAD, valid test URL, saveas, and mode: 0.
     *           2. Call request.agent.create() with the config, on success assert true.
     *           3. Clean up with request.agent.remove() using task tid, then finish test.
     *           4. If create() fails, assert false and finish test.
     * @tc.require: issues#1573
     */
    it('testTaskMode004', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskMode004.txt',
        mode: 0
      };
      request.agent.create(context, conf).then(async (data) => {
        expect(true).assertTrue();
        await request.agent.remove(data.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });

    /**
     * @tc.number: testTaskMode005
     * @tc.name: testTaskMode005
     * @tc.desc: Test create task when mode is 1
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a Config object with action: DOWNLOAD, valid test URL, saveas, and mode: 1.
     *           2. Call request.agent.create() with the config, on success assert true.
     *           3. Clean up with request.agent.remove() using task tid, then finish test.
     *           4. If create() fails, assert false and finish test.
     * @tc.require: issues#1573
     */
    it('testTaskMode005', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskMode005.txt',
        mode: 1
      };
      request.agent.create(context, conf).then(async (data) => {
        expect(true).assertTrue();
        await request.agent.remove(data.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });

    /**
     * @tc.number: testTaskMode006
     * @tc.name: testTaskMode006
     * @tc.desc: Test create task when mode is 3
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a Config object with action: DOWNLOAD, valid test URL, saveas, and mode: 3.
     *           2. Await request.agent.create() to get task instance.
     *           3. Call task.start(), on success assert true, clean up with remove(), then finish test.
     *           4. If start() fails, assert false and finish test.
     * @tc.require: issues#1573
     */
    it('testTaskMode006', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskMode006.txt',
        mode: 3
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
     * @tc.number: testTaskMode007
     * @tc.name: testTaskMode007
     * @tc.desc: Test create task when mode is other type
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a config with action: DOWNLOAD, valid URL, saveas, mode as request.agent.Action.DOWNLOAD.
     *           2. Await request.agent.create() with cast config to get task instance.
     *           3. Call task.start(), on success assert true, clean up with remove(), then finish test.
     *           4. If start() fails, assert false and finish test.
     * @tc.require: issues#1573
     */
    it('testTaskMode007', 0, async (done: Function) => {
      let conf: Record<string, string | number> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        "saveas": 'testTaskMode007.txt',
        "mode": request.agent.Action.DOWNLOAD
      };
      let task = await request.agent.create(context, conf as Object as request.agent.Config);
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
     * @tc.number: testTaskMode008
     * @tc.name: testTaskMode008
     * @tc.desc: Test create task when mode is empty
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a Config object with action: DOWNLOAD, valid test URL, saveas (no mode field).
     *           2. Call request.agent.create() with the config, on success check data.config.mode is undefined.
     *           3. Assert true, clean up with remove(), then finish test.
     *           4. If create() fails, assert false and finish test.
     * @tc.require: issues#1573
     */
    it('testTaskMode008', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskMode008.txt'
      };
      request.agent.create(context, conf).then(async (data) => {
        expect(typeof data.config.mode).assertEqual('undefined');
        expect(true).assertTrue();
        await request.agent.remove(data.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });

    /**
     * @tc.number: testTaskNetwork001
     * @tc.name: testTaskNetwork001
     * @tc.desc: Test create task when network is 3
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create test.txt in cacheDir, close the file.
     *           2. Create Config with action: UPLOAD, url, data (file info), network: 3.
     *           3. Call request.agent.create() with config, on success assert true.
     *           4. Clean up with remove(), then finish test. If failed, assert false.
     * @tc.require: issues#1573
     */
    it('testTaskNetwork001', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
          name: 'file',
          value: {
            path: `./test.txt`,
          }
        }],
        network: 3
      };
      request.agent.create(context, conf).then(async (data) => {
        expect(true).assertTrue();
        await request.agent.remove(data.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });

    /**
     * @tc.number: testTaskNetwork002
     * @tc.name: testTaskNetwork002
     * @tc.desc: Test create task when network is string
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a config with action: DOWNLOAD, valid URL, saveas, network: "ANY".
     *           2. Call request.agent.create() with cast config, on success assert true.
     *           3. Clean up with remove(), then finish test. If failed, assert false.
     * @tc.require: issues#1573
     */
    it('testTaskNetwork002', 0, async (done: Function) => {
      let conf: Record<string, string | number> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "saveas": "testTaskNetwork002.txt",
        "network": "ANY"
      };
      request.agent.create(context, conf as Object as request.agent.Config).then(async (data) => {
        expect(true).assertTrue();
        await request.agent.remove(data.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });

    /**
     * @tc.number: testTaskNetwork003
     * @tc.name: testTaskNetwork003
     * @tc.desc: Test create task when network is WIFI for DOWNLOAD
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, saveas, network: WIFI.
     *           2. Call request.agent.create() with config, on success assert true.
     *           3. Clean up with remove(), then finish test. If failed, assert false.
     * @tc.require: issues#1573
     */
    it('testTaskNetwork003', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskNetwork003.txt',
        network: request.agent.Network.WIFI
      };
      request.agent.create(context, conf).then(async (data) => {
        expect(true).assertTrue();
        await request.agent.remove(data.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });

    /**
     * @tc.number: testTaskNetwork004
     * @tc.name: testTaskNetwork004
     * @tc.desc: Test create task when network is WIFI for UPLOAD
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create test.txt in cacheDir, close the file.
     *           2. Create Config with action: UPLOAD, url, data (file info), network: WIFI.
     *           3. Await request.agent.create() to get task instance, call task.start().
     *           4. On start() success assert true, finish test. If failed, assert false.
     * @tc.require: issues#1573
     */
    it('testTaskNetwork004', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
          name: 'file',
          value: {
            path: `./test.txt`,
          }
        }],
        network: request.agent.Network.WIFI
      };
      let task = await request.agent.create(context, conf);
      task.start().then(async () => {
        expect(true).assertTrue();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });

    /**
     * @tc.number: testTaskNetwork005
     * @tc.name: testTaskNetwork005
     * @tc.desc: Test create task when network is ANY for DOWNLOAD
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, saveas, network: ANY.
     *           2. Call request.agent.create() with config, on success assert true.
     *           3. Clean up with remove(), then finish test. If failed, assert false.
     * @tc.require: issues#1573
     */
    it('testTaskNetwork005', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskNetwork005.txt',
        network: request.agent.Network.ANY
      };
      request.agent.create(context, conf).then(async (data) => {
        expect(true).assertTrue();
        await request.agent.remove(data.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });

    /**
     * @tc.number: testTaskNetwork006
     * @tc.name: testTaskNetwork006
     * @tc.desc: Test create task when network is 0
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, saveas, network: 0.
     *           2. Call request.agent.create() with config, on success assert true.
     *           3. Clean up with remove(), then finish test. If failed, assert false.
     * @tc.require: issues#1573
     */
    it('testTaskNetwork006', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskNetwork006.txt',
        network: 0
      };
      request.agent.create(context, conf).then(async (data) => {
        expect(true).assertTrue();
        await request.agent.remove(data.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });

    /**
     * @tc.number: testTaskNetwork007
     * @tc.name: testTaskNetwork007
     * @tc.desc: Test create task when network is 1
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, saveas, network: 1.
     *           2. Call request.agent.create() with config, on success assert true.
     *           3. Clean up with remove(), then finish test. If failed, assert false.
     * @tc.require: issues#1573
     */
    it('testTaskNetwork007', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskNetwork007.txt',
        network: 1
      };
      request.agent.create(context, conf).then(async (data) => {
        expect(true).assertTrue();
        await request.agent.remove(data.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });

    /**
     * @tc.number: testTaskNetwork008
     * @tc.name: testTaskNetwork008
     * @tc.desc: Test create task when network is CELLULAR for DOWNLOAD
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, saveas, network: CELLULAR.
     *           2. Call request.agent.create() with config, on success assert true.
     *           3. Clean up with remove(), then finish test. If failed, assert false.
     * @tc.require: issues#1573
     */
    it('testTaskNetwork008', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskNetwork008.txt',
        network: request.agent.Network.CELLULAR
      };
      request.agent.create(context, conf).then(async (data) => {
        expect(true).assertTrue();
        await request.agent.remove(data.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });

    /**
     * @tc.number: testTaskNetwork009
     * @tc.name: testTaskNetwork009
     * @tc.desc: Test create task when network is 2
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, saveas, network: 2.
     *           2. Call request.agent.create() with config, on success assert true.
     *           3. Clean up with remove(), then finish test. If failed, assert false.
     * @tc.require: issues#1573
     */
    it('testTaskNetwork009', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskNetwork009.txt',
        network: 2
      };
      request.agent.create(context, conf).then(async (data) => {
        expect(true).assertTrue();
        await request.agent.remove(data.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });

    /**
     * @tc.number: testTaskNetwork010
     * @tc.name: testTaskNetwork010
     * @tc.desc: Test create task when network is empty
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, saveas (no network field).
     *           2. Call request.agent.create() with config, on success assert true.
     *           3. Clean up with remove(), then finish test. If failed, assert false.
     * @tc.require: issues#1573
     */
    it('testTaskNetwork010', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskNetwork010.txt'
      };
      request.agent.create(context, conf).then(async (data) => {
        expect(true).assertTrue();
        await request.agent.remove(data.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });

    /**
     * @tc.number: testTaskRetry001
     * @tc.name: testTaskRetry001
     * @tc.desc: Test create task when retry is true for FOREGROUND
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, url: http://xxx, mode: FOREGROUND, saveas, retry: true.
     *           2. Await request.agent.create() to get task instance, call task.start().
     *           3. On start() success assert true, clean up with remove(), then finish test.
     *           4. If start() fails, assert false and finish test.
     * @tc.require: issues#1573
     */
    it('testTaskRetry001', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'http://xxx',
        mode: request.agent.Mode.FOREGROUND,
        saveas: 'testTaskRetry001.txt',
        retry: true
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
     * @tc.number: testTaskRetry002
     * @tc.name: testTaskRetry002
     * @tc.desc: Test create task when retry is true for background
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, mode: BACKGROUND, saveas, retry: true, gauge: true.
     *           2. Await request.agent.create() to get task instance, call task.start().
     *           3. On start() success assert true, clean up with remove(), then finish test.
     *           4. If start() fails, assert false and finish test.
     * @tc.require: issues#1573
     */
    it('testTaskRetry002', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.BACKGROUND,
        saveas: 'testTaskRetry002.txt',
        retry: true,
        gauge: true
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
     * @tc.number: testTaskRetry003
     * @tc.name: testTaskRetry003
     * @tc.desc: Test create task when retry is string
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create config with action: DOWNLOAD, url: http://xxx, mode: BACKGROUND, saveas, retry: "true", overwrite: true.
     *           2. Await request.agent.create() with cast config to get task instance.
     *           3. Listen to 'failed' event, on trigger assert true and finish test.
     *           4. Call task.start() to execute the task.
     * @tc.require: issues#1573
     */
    it('testTaskRetry003', 0, async (done: Function) => {
      let conf: Record<string, string | number | boolean> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "http://xxx",
        "mode": request.agent.Mode.BACKGROUND,
        "saveas": 'testTaskRetry003.txt',
        "retry": "true",
        "overwrite": true
      };
      let task = await request.agent.create(context, conf as Object as request.agent.Config);
      task.on('failed', () => {
        expect(true).assertTrue();
        done();
      });
      await task.start();
    });

    /**
     * @tc.number: testTaskRetry004
     * @tc.name: testTaskRetry004
     * @tc.desc: Test create task when retry is false for FOREGROUND
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, target URL, mode: FOREGROUND, saveas, retry: false.
     *           2. Await request.agent.create() to get task instance.
     *           3. Listen to 'failed' event, on trigger assert true and finish test.
     *           4. Call task.start() to execute the task.
     * @tc.require: issues#1573
     */
    it('testTaskRetry004', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test1.txt',
        mode: request.agent.Mode.FOREGROUND,
        saveas: 'testTaskRetry004.txt',
        retry: false
      };
      let task = await request.agent.create(context, conf);
      task.on('failed', () => {
        expect(true).assertTrue();
        done();
      });
      await task.start();
    });

    /**
     * @tc.number: testTaskRetry005
     * @tc.name: testTaskRetry005
     * @tc.desc: Test create task when retry is false for background
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, target URL, mode: BACKGROUND, saveas, retry: false, overwrite: true.
     *           2. Await request.agent.create() to get task instance.
     *           3. Listen to 'failed' event, on trigger assert true and finish test.
     *           4. Call task.start() to execute the task.
     * @tc.require: issues#1573
     */
    it('testTaskRetry005', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test1.txt',
        mode: request.agent.Mode.BACKGROUND,
        saveas: 'testTaskRetry005.txt',
        retry: false,
        overwrite: true
      };
      let task = await request.agent.create(context, conf);
      task.on('failed', () => {
        expect(true).assertTrue();
        done();
      });
      await task.start();
    });
    
    /**
     * @tc.number: testTaskTitle001
     * @tc.name: testTaskTitle001
     * @tc.desc: Test create task when title is given
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, title: 'download test.txt', saveas.
     *           2. Await request.agent.create() to get task instance.
     *           3. Assert task.config.title equals 'download test.txt', clean up with remove().
     *           4. Finish the test after cleanup.
     * @tc.require: issues#1573
     */
    it('testTaskTitle001', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        title: 'download test.txt',
        saveas: 'testTaskTitle001.txt',
      };
      let task = await request.agent.create(context, conf);
      expect(task.config.title).assertEqual('download test.txt');
      await request.agent.remove(task.tid);
      done();
    });

    /**
     * @tc.number: testTaskTitle002
     * @tc.name: testTaskTitle002
     * @tc.desc: Test create task when title is number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create config with action: DOWNLOAD, valid URL, title: 123, saveas.
     *           2. Await request.agent.create() with cast config to get task instance.
     *           3. Assert task.config.title equals 123, clean up with remove().
     *           4. Finish the test after cleanup.
     * @tc.require: issues#1573
     */
    it('testTaskTitle002', 0, async (done: Function) => {
      let conf: Record<string, string | number> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "title": 123,
        "saveas": "testTaskTitle002.txt",
      };
      let task = await request.agent.create(context, conf as Object as request.agent.Config);
      expect(task.config.title).assertEqual(123);
      await request.agent.remove(task.tid);
      done();
    });

    /**
     * @tc.number: testTaskTitle003
     * @tc.name: testTaskTitle003
     * @tc.desc: Test create task when title length exceeds 256
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, title: 'a' repeated 257 times, saveas.
     *           2. Try to call request.agent.create() with the config; expect it to throw an error.
     *           3. In catch block, assert error code is 401, then finish test.
     *           4. If no error is thrown, assert false and finish test.
     * @tc.require: issues#1573
     */
    it('testTaskTitle003', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        title: 'a'.repeat(257),
        saveas: 'testTaskTitle003.txt',
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
     * @tc.number: testTaskTitle004
     * @tc.name: testTaskTitle004
     * @tc.desc: Test create task when title length is 256
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, title: 'a' repeated 256 times, saveas.
     *           2. Try to await request.agent.create() to create task; if success, call done().
     *           3. If create() throws error, assert true and call done().
     * @tc.require: issues#1573
     */
    it('testTaskTitle004', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        title: 'a'.repeat(256),
        saveas: 'testTaskTitle004.txt',
      };
      try {
        let task = await request.agent.create(context, conf);
        done();
      } catch (err) {
        expect(true).assertTrue();
        done();
      }
    });
    
    /**
     * @tc.number: testDownloadNormalConfig001
     * @tc.name: testDownloadNormalConfig001
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, other parameters set to undefined.
     *           2. Call request.agent.create() with config; assert err is null.
     *           3. Call request.agent.show() with task tid, verify multiple info fields match expectations.
     *           4. Clean up with request.agent.remove(), print success log, call done().
     *           5. Catch any errors, assert false, print fail log, call done().
     * @tc.require: issues#1573
     */
    it('testDownloadNormalConfig001', 0, (done: Function) => {
      console.info("===> testDownloadNormalConfig001 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
      try {
        request.agent.create(context, config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/haps/testModule/cache/test.txt');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual('download');
            expect(info.description).assertEqual("");
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(false);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadNormalConfig001 test success!");
            done();
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadNormalConfig001 test fail!");
            done();
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadNormalConfig001 test fail!");
        done();
      }
    });

    /**
     * @tc.number: testDownloadConfigTitle001
     * @tc.name: testDownloadConfigTitle001
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, title to "title",
     and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, title: "title", other params undefined.
     *           2. Call request.agent.create() with config; assert err is null.
     *           3. Call request.agent.show() to verify info.title equals "title" and other fields match.
     *           4. Clean up with remove(), print success log, call done().
     *           5. Catch errors, assert false, print fail log, call done().
     * @tc.require: issues#1573
     */
    it('testDownloadConfigTitle001', 0, (done: Function) => {
      console.info("===> testDownloadConfigTitle001 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: "title",
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
      try {
        request.agent.create(context, config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/haps/testModule/cache/test.txt');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual("title");
            expect(info.description).assertEqual("");
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(false);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadConfigTitle001 test success!");
            done();
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadConfigTitle001 test fail!");
            done();
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadConfigTitle001 test fail!");
        done();
      }
    });

    /**
     * @tc.number: testDownloadConfigTitle002
     * @tc.name: testDownloadConfigTitle002
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, title to "",
     and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, title: "", other params undefined.
     *           2. Call request.agent.create() with config; assert err is null.
     *           3. Call request.agent.show() to verify info.title equals "download" and other fields match.
     *           4. Clean up with remove(), print success log, call done().
     *           5. Catch errors, assert false, print fail log, call done().
     * @tc.require: issues#1573
     */
    it('testDownloadConfigTitle002', 0, (done: Function) => {
      console.info("===> testDownloadConfigTitle002 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: "",
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
      try {
        request.agent.create(context, config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/haps/testModule/cache/test.txt');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual('download');
            expect(info.description).assertEqual("");
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(false);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadConfigTitle002 test success!");
            done();
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadConfigTitle002 test fail!");
            done();
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadConfigTitle002 test fail!");
        done();
      }
    });

    /**
     * @tc.number: testDownloadConfigTitle003
     * @tc.name: testDownloadConfigTitle003
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, title to 'a'.repeat(256),
     and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, title: 'a'×256, other params undefined.
     *           2. Call request.agent.create() with config; assert err is null.
     *           3. Call request.agent.show() to verify info.title equals 'a'×256 and other fields match.
     *           4. Clean up with remove(), print success log, call done().
     *           5. Catch errors, assert false, print fail log, call done().
     * @tc.require: issues#1573
     */
    it('testDownloadConfigTitle003', 0, (done: Function) => {
      console.info("===> testDownloadConfigTitle003 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: 'a'.repeat(256),
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
      try {
        request.agent.create(context, config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/haps/testModule/cache/test.txt');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual('a'.repeat(256));
            expect(info.description).assertEqual("");
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(false);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadConfigTitle003 test success!");
            done();
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadConfigTitle003 test fail!");
            done();
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadConfigTitle003 test fail!");
        done();
      }
    });

    /**
     * @tc.number: testDownloadConfigTitle004
     * @tc.name: testDownloadConfigTitle004
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, title to 'a'.repeat(257),
     and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, title: 'a'×257, other params undefined.
     *           2. Try to call request.agent.create() with config; expect it to throw error.
     *           3. In catch block, assert err.code equals 401, print success log, call done().
     *           4. If create() succeeds, print fail log, assert false, call done().
     * @tc.require: issues#1573
     */
    it('testDownloadConfigTitle004', 0, (done: Function) => {
      console.info("===> testDownloadConfigTitle004 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: 'a'.repeat(257),
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }

      try {
        request.agent.create(context, config, async (err, data) => {
          console.info("===> testDownloadConfigTitle004 test fail!");
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        console.info("===> testDownloadConfigTitle004 test success!");
        done();
      }
    });

    /**
     * @tc.number: testDownloadConfigTitle005
     * @tc.name: testDownloadConfigTitle005
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, title to int,
     and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Record config with action: DOWNLOAD, valid URL, title: 1, other params undefined.
     *           2. Call request.agent.create() with cast config; assert err is null.
     *           3. Call request.agent.show() to verify info.title equals "download" and other fields match.
     *           4. Clean up with remove(), print success log, call done().
     *           5. Catch errors, assert false, print fail log, call done().
     * @tc.require: issues#1573
     */
    it('testDownloadConfigTitle005', 0, (done: Function) => {
      console.info("===> testDownloadConfigTitle005 test start!");

      let config: Record<string, string | number | undefined> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "title": 1,
        "description": undefined,
        "mode": undefined,
        "overwrite": undefined,
        "method": undefined,
        "headers": undefined,
        //data: undefined,
        "saveas": undefined,
        "network": undefined,
        "metered": undefined,
        "roaming": undefined,
        "retry": undefined,
        "redirect": undefined,
        "proxy": undefined,
        "index": undefined,
        "begins": undefined,
        "ends": undefined,
        "gauge": undefined,
        "precise": undefined,
        "token": undefined,
        "priority": undefined,
        "extras": undefined,
        "multipart": undefined,
        "notification": undefined,
        "minSpeed": undefined,
        "timeout": undefined,
      }

      try {
        request.agent.create(context, config as Object as request.agent.Config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/haps/testModule/cache/test.txt');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual('download');
            expect(info.description).assertEqual("");
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(false);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadConfigTitle005 test success!");
            done();
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadConfigTitle005 test fail!");
            done();
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadConfigTitle003 test fail!");
        done();
      }
    });

    /**
     * @tc.number: testDownloadConfigDescription001
     * @tc.name: testDownloadConfigDescription001
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, description to "desc",
     and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, description: "desc", other params undefined.
     *           2. Call request.agent.create() with config; assert err is null.
     *           3. Call request.agent.show() to verify info.description equals "desc" and other fields match.
     *           4. Clean up with remove(), print success log, call done().
     *           5. Catch errors, assert false, print fail log, call done().
     * @tc.require: issues#1573
     */
    it('testDownloadConfigDescription001', 0, (done: Function) => {
      console.info("===> testDownloadConfigDescription001 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: "desc",
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
      try {
        request.agent.create(context, config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/haps/testModule/cache/test.txt');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual('download');
            expect(info.description).assertEqual("desc");
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(false);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadConfigDescription001 test success!");
            done();
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadConfigDescription001 test fail!");
            done();
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadConfigDescription001 test fail!");
        done();
      }
    });

    /**
     * @tc.number: testDownloadConfigDescription002
     * @tc.name: testDownloadConfigDescription002
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, title to "",
     and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, description: "", other params undefined.
     *           2. Call request.agent.create() with config; assert err is null.
     *           3. Call request.agent.show() to verify info.description equals "" and other fields match.
     *           4. Clean up with remove(), print success log, call done().
     *           5. Catch errors, assert false, print fail log, call done().
     * @tc.require: issues#1573
     */
    it('testDownloadConfigDescription002', 0, (done: Function) => {
      console.info("===> testDownloadConfigDescription002 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: "",
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
      try {
        request.agent.create(context, config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/haps/testModule/cache/test.txt');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual('download');
            expect(info.description).assertEqual("");
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(false);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadConfigDescription002 test success!");
            done();
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadConfigDescription002 test fail!");
            done();
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadConfigDescription002 test fail!");
        done();
      }
    });

    /**
     * @tc.number: testDownloadConfigDescription003
     * @tc.name: testDownloadConfigDescription003
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, description to 'a'.repeat(1024),
     and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, description: 'a'×1024, other params undefined.
     *           2. Call request.agent.create() with config; assert err is null.
     *           3. Call request.agent.show() to verify info.description equals 'a'×1024 and other fields match.
     *           4. Clean up with remove(), print success log, call done().
     *           5. Catch errors, assert false, print fail log, call done().
     * @tc.require: issues#1573
     */
    it('testDownloadConfigDescription003', 0, (done: Function) => {
      console.info("===> testDownloadConfigDescription003 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: 'a'.repeat(1024),
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
      try {
        request.agent.create(context, config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/haps/testModule/cache/test.txt');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual('download');
            expect(info.description).assertEqual('a'.repeat(1024));
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(false);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadConfigDescription003 test success!");
            done();
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadConfigDescription003 test fail!");
            done();
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadConfigDescription003 test fail!");
        done();
      }
    });

    /**
     * @tc.number: testDownloadConfigDescription004
     * @tc.name: testDownloadConfigDescription004
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, description to 'a'.repeat(1025),
     and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, description: 'a'×1025, other params undefined.
     *           2. Try to call request.agent.create() with config; expect it to throw error.
     *           3. In catch block, assert err.code equals 401, print success log, call done().
     *           4. If create() succeeds, print fail log, assert false, call done().
     * @tc.require: issues#1573
     */
    it('testDownloadConfigDescription004', 0, (done: Function) => {
      console.info("===> testDownloadConfigDescription004 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: 'a'.repeat(1025),
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }

      try {
        request.agent.create(context, config, async (err, data) => {
          console.info("===> testDownloadConfigDescription004 test fail!");
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        console.info("===> testDownloadConfigDescription004 test success!");
        done();
      }
    });

    /**
     * @tc.number: testDownloadConfigDescription005
     * @tc.name: testDownloadConfigDescription005
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, description to int,
     and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Record config with action: DOWNLOAD, valid URL, description: 1, other params undefined.
     *           2. Call request.agent.create() with cast config; assert err is null.
     *           3. Call request.agent.show() to verify info.description equals "" and other fields match.
     *           4. Clean up with remove(), print success log, call done().
     *           5. Catch errors, assert false, print fail log, call done().
     * @tc.require: issues#1573
     */
    it('testDownloadConfigDescription005', 0, (done: Function) => {
      console.info("===> testDownloadConfigDescription005 test start!");

      let config: Record<string, string | number | undefined> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "title": undefined,
        "description": 1,
        "mode": undefined,
        "overwrite": undefined,
        "method": undefined,
        "headers": undefined,
        //data: undefined,
        "saveas": undefined,
        "network": undefined,
        "metered": undefined,
        "roaming": undefined,
        "retry": undefined,
        "redirect": undefined,
        "proxy": undefined,
        "index": undefined,
        "begins": undefined,
        "ends": undefined,
        "gauge": undefined,
        "precise": undefined,
        "token": undefined,
        "priority": undefined,
        "extras": undefined,
        "multipart": undefined,
        "notification": undefined,
        "minSpeed": undefined,
        "timeout": undefined,
      }

      try {
        request.agent.create(context, config as Object as request.agent.Config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/haps/testModule/cache/test.txt');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual('download');
            expect(info.description).assertEqual("");
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(false);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadConfigDescription005 test success!");
            done();
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadConfigDescription005 test fail!");
            done();
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadConfigDescription005 test fail!");
        done();
      }
    });

    /**
     * @tc.number: testDownloadConfigMode001
     * @tc.name: testDownloadConfigMode001
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, mode to BACKGROUND,
     and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, mode: BACKGROUND, other params undefined.
     *           2. Call request.agent.create() with config; assert err is null.
     *           3. Call request.agent.show() to verify info.mode equals BACKGROUND and other fields match.
     *           4. Clean up with remove(), print success log, call done().
     *           5. Catch errors, assert false, print fail log, call done().
     * @tc.require: issues#1573
     */
    it('testDownloadConfigMode001', 0, (done: Function) => {
      console.info("===> testDownloadConfigMode001 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: request.agent.Mode.BACKGROUND,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
      try {
        request.agent.create(context, config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/haps/testModule/cache/test.txt');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual('download');
            expect(info.description).assertEqual("");
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(false);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadConfigMode001 test success!");
            done();
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadConfigMode001 test fail!");
            done();
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadConfigMode001 test fail!");
        done();
      }
    });

    /**
     * @tc.number: testDownloadConfigMode002
     * @tc.name: testDownloadConfigMode002
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, mode to FOREGROUND,
     and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, mode: FOREGROUND, other params undefined.
     *           2. Call request.agent.create() with config; assert err is null.
     *           3. Call request.agent.show() to verify info.mode equals FOREGROUND and other fields match.
     *           4. Clean up with remove(), print success log, call done().
     *           5. Catch errors, assert false, print fail log, call done().
     * @tc.require: issues#1573
     */
    it('testDownloadConfigMode002', 0, (done: Function) => {
      console.info("===> testDownloadConfigMode002 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: request.agent.Mode.FOREGROUND,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
      try {
        request.agent.create(context, config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/haps/testModule/cache/test.txt');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual('download');
            expect(info.description).assertEqual("");
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.FOREGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(false);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadConfigMode002 test success!");
            done();
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadConfigMode002 test fail!");
            done();
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadConfigMode002 test fail!");
        done();
      }
    });

    /**
     * @tc.number: testDownloadConfigMode003
     * @tc.name: testDownloadConfigMode003
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, mode to int,
     and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Record config with action: DOWNLOAD, valid URL, mode: 1, other params undefined.
     *           2. Call request.agent.create() with cast config; assert err is null.
     *           3. Call request.agent.show() to verify info.mode equals FOREGROUND and other fields match.
     *           4. Clean up with remove(), print success log, call done().
     *           5. Catch errors, assert false, print fail log, call done().
     * @tc.require: issues#1573
     */
    it('testDownloadConfigMode003', 0, (done: Function) => {
      console.info("===> testDownloadConfigMode003 test start!");

      let config: Record<string, string | number | undefined> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "title": undefined,
        "description": undefined,
        "mode": 1,
        "overwrite": undefined,
        "method": undefined,
        "headers": undefined,
        //data: undefined,
        "saveas": undefined,
        "network": undefined,
        "metered": undefined,
        "roaming": undefined,
        "retry": undefined,
        "redirect": undefined,
        "proxy": undefined,
        "index": undefined,
        "begins": undefined,
        "ends": undefined,
        "gauge": undefined,
        "precise": undefined,
        "token": undefined,
        "priority": undefined,
        "extras": undefined,
        "multipart": undefined,
        "notification": undefined,
        "minSpeed": undefined,
        "timeout": undefined,
      }

      try {
        request.agent.create(context, config as Object as request.agent.Config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/haps/testModule/cache/test.txt');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual('download');
            expect(info.description).assertEqual("");
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.FOREGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(false);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadConfigMode003 test success!");
            done();
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadConfigMode003 test fail!");
            done();
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadConfigMode003 test fail!");
        done();
      }
    });

    /**
     * @tc.number: testDownloadConfigOverwrite001
     * @tc.name: testDownloadConfigOverwrite001
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, overwrite to true,
     and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, overwrite: true, other params undefined.
     *           2. Call request.agent.create() with config; assert err is null.
     *           3. Call request.agent.show() to verify other fields match expectations (overwrite not checked here).
     *           4. Clean up with remove(), print success log, call done().
     *           5. Catch errors, assert false, print fail log, call done().
     * @tc.require: issues#1573
     */
    it('testDownloadConfigOverwrite001', 0, (done: Function) => {
      console.info("===> testDownloadConfigOverwrite001 test start!");
      
      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: true,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
      try {
        request.agent.create(context, config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/haps/testModule/cache/test.txt');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual('download');
            expect(info.description).assertEqual("");
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(false);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadConfigOverwrite001 test success!");
            done();
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadConfigOverwrite001 test fail!");
            done();
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadConfigOverwrite001 test fail!");
        done();
      }
    });

    /**
     * @tc.number: testDownloadConfigOverwrite002
     * @tc.name: testDownloadConfigOverwrite002
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, overwrite to false,
     and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, overwrite: false, other params undefined.
     *           2. Call request.agent.create() with config; assert err is null.
     *           3. Call request.agent.show() with task tid, verify multiple info fields match expectations.
     *           4. Clean up with request.agent.remove(), print success log, call done().
     *           5. Catch any errors, assert false, print fail log, call done().
     * @tc.require: issues#1573
     */
    it('testDownloadConfigOverwrite002', 0, (done: Function) => {
      console.info("===> testDownloadConfigOverwrite002 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: false,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
      try {
        request.agent.create(context, config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/haps/testModule/cache/test.txt');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual('download');
            expect(info.description).assertEqual("");
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(false);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadConfigOverwrite002 test success!");
            done();
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadConfigOverwrite002 test fail!");
            done();
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadConfigOverwrite002 test fail!");
        done();
      }
    });

    /**
     * @tc.number: testDownloadConfigMethod001
     * @tc.name: testDownloadConfigMethod001
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, method to GET,
     and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, method: 'GET', other params undefined.
     *           2. Call request.agent.create() with config; assert err is null.
     *           3. Call request.agent.show() with task tid, verify multiple info fields match expectations.
     *           4. Clean up with request.agent.remove(), print success log, call done().
     *           5. Catch any errors, assert false, print fail log, call done().
     * @tc.require: issues#1573
     */
    it('testDownloadConfigMethod001', 0, (done: Function) => {
      console.info("===> testDownloadConfigMethod001 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: 'GET',
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
      try {
        request.agent.create(context, config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/haps/testModule/cache/test.txt');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual('download');
            expect(info.description).assertEqual("");
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(false);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadConfigMethod001 test success!");
            done();
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadConfigMethod001 test fail!");
            done();
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadConfigMethod001 test fail!");
        done();
      }
    });

    /**
     * @tc.number: testDownloadConfigMethod002
     * @tc.name: testDownloadConfigMethod002
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, method to POST,
     and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, method: 'POST', other params undefined.
     *           2. Call request.agent.create() with config; assert err is null.
     *           3. Call request.agent.show() with task tid, verify multiple info fields match expectations.
     *           4. Clean up with request.agent.remove(), print success log, call done().
     *           5. Catch any errors, assert false, print fail log, call done().
     * @tc.require: issues#1573
     */
    it('testDownloadConfigMethod002', 0, (done: Function) => {
      console.info("===> testDownloadConfigMethod002 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: 'POST',
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
      try {
        request.agent.create(context, config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/haps/testModule/cache/test.txt');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual('download');
            expect(info.description).assertEqual("");
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(false);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadConfigMethod002 test success!");
            done();
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadConfigMethod002 test fail!");
            done();
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadConfigMethod002 test fail!");
        done();
      }
    });

    /**
     * @tc.number: testDownloadConfigMethod003
     * @tc.name: testDownloadConfigMethod003
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, method to PUT,
     and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, method: 'PUT', other params undefined.
     *           2. Call request.agent.create() with config; assert err is null.
     *           3. Call request.agent.show() with task tid, verify multiple info fields match expectations.
     *           4. Clean up with request.agent.remove(), print success log, call done().
     *           5. Catch any errors, assert false, print fail log, call done().
     * @tc.require: issues#1573
     */
    it('testDownloadConfigMethod003', 0, (done: Function) => {
      console.info("===> testDownloadConfigMethod003 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: 'PUT',
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
      try {
        request.agent.create(context, config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/haps/testModule/cache/test.txt');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual('download');
            expect(info.description).assertEqual("");
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(false);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadConfigMethod003 test success!");
            done();
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadConfigMethod003 test fail!");
            done();
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadConfigMethod003 test fail!");
        done();
      }
    });

    /**
     * @tc.number: testDownloadConfigMethod004
     * @tc.name: testDownloadConfigMethod004
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     method to 1, and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Record config with action: DOWNLOAD, valid URL, method: 1, other params undefined.
     *           2. Call request.agent.create() with cast config; assert err is null.
     *           3. Call request.agent.show() with task tid, verify multiple info fields match expectations.
     *           4. Clean up with request.agent.remove(), print success log, call done().
     *           5. Catch any errors, assert false, print fail log, call done().
     * @tc.require: issues#1573
     */
    it('testDownloadConfigMethod004', 0, (done: Function) => {
      console.info("===> testDownloadConfigMethod004 test start!");

      let config: Record<string, string | number | undefined> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "title": undefined,
        "description": undefined,
        "mode": undefined,
        "overwrite": undefined,
        "method": 1,
        "headers": undefined,
        //data: undefined,
        "saveas": undefined,
        "network": undefined,
        "metered": undefined,
        "roaming": undefined,
        "retry": undefined,
        "redirect": undefined,
        "proxy": undefined,
        "index": undefined,
        "begins": undefined,
        "ends": undefined,
        "gauge": undefined,
        "precise": undefined,
        "token": undefined,
        "priority": undefined,
        "extras": undefined,
        "multipart": undefined,
        "notification": undefined,
        "minSpeed": undefined,
        "timeout": undefined,
      }

      try {
        request.agent.create(context, config as Object as request.agent.Config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/haps/testModule/cache/test.txt');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual('download');
            expect(info.description).assertEqual("");
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(false);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadConfigMethod004 test success!");
            done();
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadConfigMethod004 test fail!");
            done();
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadConfigMethod004 test fail!");
        done();
      }
    });

    /**
     * @tc.number: testDownloadConfigHeaders001
     * @tc.name: testDownloadConfigHeaders001
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     headers to 'application/json' and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, headers: {'content-type': 'application/json'}, other params undefined.
     *           2. Call request.agent.create() with config; assert err is null.
     *           3. Call request.agent.show() with task tid, verify multiple info fields match expectations.
     *           4. Clean up with request.agent.remove(), print success log, call done().
     *           5. Catch any errors, assert false, print fail log, call done().
     * @tc.require: issues#1573
     */
    it('testDownloadConfigHeaders001', 0, (done: Function) => {
      console.info("===> testDownloadConfigHeaders001 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: {
          'content-type': 'application/json'
        },
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
      try {
        request.agent.create(context, config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/haps/testModule/cache/test.txt');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual('download');
            expect(info.description).assertEqual("");
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(false);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadConfigHeaders001 test success!");
            done();
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadConfigHeaders001 test fail!");
            done();
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadConfigHeaders001 test fail!");
        done();
      }
    });
    
    /**
     * @tc.number: testDownloadConfigHeaders002
     * @tc.name: testDownloadConfigHeaders002
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     headers to 'multipart/form-data' and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL,
     *              headers: {'content-type': 'multipart/form-data'}, other params undefined.
     *           2. Call request.agent.create() with config; assert err is null.
     *           3. Call request.agent.show() with task tid, verify multiple info fields match expectations.
     *           4. Clean up with request.agent.remove(), print success log, call done().
     *           5. Catch any errors, assert false, print fail log, call done().
     * @tc.require: issues#1573
     */
    it('testDownloadConfigHeaders002', 0, (done: Function) => {
      console.info("===> testDownloadConfigHeaders002 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: {
          'content-type': 'multipart/form-data'
        },
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
      try {
        request.agent.create(context, config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/haps/testModule/cache/test.txt');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual('download');
            expect(info.description).assertEqual("");
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(false);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadConfigHeaders002 test success!");
            done();
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadConfigHeaders002 test fail!");
            done();
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadConfigHeaders002 test fail!");
        done();
      }
    });

    /**
     * @tc.number: testDownloadConfigHeaders003
     * @tc.name: testDownloadConfigHeaders003
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     headers to 1, and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Record config with action: DOWNLOAD, valid URL, headers: 1, other params undefined.
     *           2. Try to call request.agent.create() with cast config; expect it to throw error.
     *           3. In catch block, assert true, print success log, call done().
     *           4. If create() succeeds, assert false, print success log (code logic), call done().
     *           5. Catch any other errors, assert false, print fail log, call done().
     * @tc.require: issues#1573
     */
    it('testDownloadConfigHeaders003', 0, (done: Function) => {
      console.info("===> testDownloadConfigHeaders003 test start!");

      let config: Record<string, string | number | undefined> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "title": undefined,
        "description": undefined,
        "mode": undefined,
        "overwrite": undefined,
        "method": undefined,
        "headers": 1,
        //data: undefined,
        "saveas": undefined,
        "network": undefined,
        "metered": undefined,
        "roaming": undefined,
        "retry": undefined,
        "redirect": undefined,
        "proxy": undefined,
        "index": undefined,
        "begins": undefined,
        "ends": undefined,
        "gauge": undefined,
        "precise": undefined,
        "token": undefined,
        "priority": undefined,
        "extras": undefined,
        "multipart": undefined,
        "notification": undefined,
        "minSpeed": undefined,
        "timeout": undefined,
      }

      try {
        request.agent.create(context, config as Object as request.agent.Config, async (err, data) => {
          expect(false).assertTrue();
          console.info("===> testDownloadConfigHeaders003 test success!");
          done();
        });
      } catch (err) {
        expect(true).assertTrue();
        console.info("===> testDownloadConfigHeaders003 test success!");
        done();
      }
    });

    /**
     * @tc.number: testDownloadConfigSaveas001
     * @tc.name: testDownloadConfigSaveas001
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     saveas to 'test.txt' and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testDownloadConfigSaveas001', 0, (done: Function) => {
      console.info("===> testDownloadConfigSaveas001 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: 'test.txt',
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
      try {
        request.agent.create(context, config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/haps/testModule/cache/test.txt');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual('download');
            expect(info.description).assertEqual("");
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(false);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadConfigSaveas001 test success!");
            done();
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadConfigSaveas001 test fail!");
            done();
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadConfigSaveas001 test fail!");
        done();
      }
    });

    /**
     * @tc.number: testDownloadConfigSaveas002
     * @tc.name: testDownloadConfigSaveas002
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     saveas to 1, and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testDownloadConfigSaveas002', 0, (done: Function) => {
      console.info("===> testDownloadConfigSaveas002 test start!");

      let config: Record<string, string | number | undefined> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "title": undefined,
        "description": undefined,
        "mode": undefined,
        "overwrite": undefined,
        "method": undefined,
        "headers": undefined,
        //data: undefined,
        "saveas": 1,
        "network": undefined,
        "metered": undefined,
        "roaming": undefined,
        "retry": undefined,
        "redirect": undefined,
        "proxy": undefined,
        "index": undefined,
        "begins": undefined,
        "ends": undefined,
        "gauge": undefined,
        "precise": undefined,
        "token": undefined,
        "priority": undefined,
        "extras": undefined,
        "multipart": undefined,
        "notification": undefined,
        "minSpeed": undefined,
        "timeout": undefined,
      }

      try {
        request.agent.create(context, config as Object as request.agent.Config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/haps/testModule/cache/test.txt');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual('download');
            expect(info.description).assertEqual("");
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(false);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadConfigSaveas002 test success!");
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadConfigSaveas002 test fail!");
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadConfigSaveas002 test fail!");
      }
    });

    /**
     * @tc.number: testDownloadConfigSaveas003
     * @tc.name: testDownloadConfigSaveas003
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     saveas to './' and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testDownloadConfigSaveas003', 0, (done: Function) => {
      console.info("===> testDownloadConfigSaveas003 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: './',
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
      try {
        request.agent.create(context, config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/haps/testModule/cache/test.txt');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual('download');
            expect(info.description).assertEqual("");
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(false);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadConfigSaveas003 test success!");
            done();
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadConfigSaveas003 test fail!");
            done();
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadConfigSaveas003 test fail!");
        done();
      }
    });

    /**
     * @tc.number: testDownloadConfigSaveas004
     * @tc.name: testDownloadConfigSaveas004
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     saveas to ./filename and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testDownloadConfigSaveas004', 0, (done: Function) => {
      console.info("===> testDownloadConfigSaveas004 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: './test004.txt',
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
      try {
        request.agent.create(context, config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/haps/testModule/cache/test004.txt');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual('download');
            expect(info.description).assertEqual("");
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(false);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadConfigSaveas004 test success!");
            done();
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadConfigSaveas004 test fail!");
            done();
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadConfigSaveas004 test fail!");
        done();
      }
    });

    /**
     * @tc.number: testDownloadConfigSaveas005
     * @tc.name: testDownloadConfigSaveas005
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     saveas to pathname/filename and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testDownloadConfigSaveas005', 0, (done: Function) => {
      console.info("===> testDownloadConfigSaveas005 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: 'a/b/test005.txt',
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
      try {
        request.agent.create(context, config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/haps/testModule/cache/a/b/test005');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual('download');
            expect(info.description).assertEqual("");
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(false);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadConfigSaveas005 test success!");
            done();
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadConfigSaveas005 test fail!");
            done();
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadConfigSaveas005 test fail!");
        done();
      }
    });

    /**
     * @tc.number: testDownloadConfigSaveas006
     * @tc.name: testDownloadConfigSaveas006
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     saveas to internal://cache/filename and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testDownloadConfigSaveas006', 0, (done: Function) => {
      console.info("===> testDownloadConfigSaveas006 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: 'internal://cache/test006',
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
      try {
        request.agent.create(context, config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/haps/testModule/cache/test006');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual('download');
            expect(info.description).assertEqual("");
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(false);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadConfigSaveas006 test success!");
            done();
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadConfigSaveas006 test fail!");
            done();
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadConfigSaveas006 test fail!");
        done();
      }
    });

    /**
     * @tc.number: testDownloadConfigSaveas007
     * @tc.name: testDownloadConfigSaveas007
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     saveas is whole path which contains error .. and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testDownloadConfigSaveas007', 0, (done: Function) => {
      console.info("===> testDownloadConfigSaveas007 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: '/data/storage/../../el2/base/test_saveas_007',
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
      try {
        request.agent.create(context, config as Object as request.agent.Config, async (err, data) => {
          expect(false).assertTrue();
          console.info("===> testDownloadConfigSaveas007 test success!");
          done();
        });
      } catch (err) {
        expect(true).assertTrue();
        console.info("===> testDownloadConfigSaveas007 test success!");
        done();
      }
    });

    /**
     * @tc.number: testDownloadConfigSaveas008
     * @tc.name: testDownloadConfigSaveas008
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     saveas is file:// with correct package name and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testDownloadConfigSaveas008', 0, (done: Function) => {
      console.info("===> testDownloadConfigSaveas008 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: 'file://com.acts.request/data/storage/el2/base/test008',
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
      try {
        request.agent.create(context, config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/test008');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual('download');
            expect(info.description).assertEqual("");
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(false);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadConfigSaveas008 test success!");
            done();
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadConfigSaveas008 test fail!");
            done();
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadConfigSaveas008 test fail!");
        done();
      }
    });

    /**
     * @tc.number: testDownloadConfigSaveas009
     * @tc.name: testDownloadConfigSaveas009
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     saveas is file:// with error package name and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testDownloadConfigSaveas009', 0, (done: Function) => {
      console.info("===> testDownloadConfigSaveas009 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: 'file://com.acts.request1111/data/storage/el2/base/test009',
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
      try {
        request.agent.create(context, config as Object as request.agent.Config, async (err, data) => {
          expect(false).assertTrue();
          console.info("===> testDownloadConfigSaveas009 test success!");
          done();
        });
      } catch (err) {
        expect(true).assertTrue();
        console.info("===> testDownloadConfigSaveas009 test success!");
        done();
      }
    });

    /**
     * @tc.number: testDownloadConfigMetered001
     * @tc.name: testDownloadConfigMetered001
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     metered is true and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testDownloadConfigMetered001', 0, (done: Function) => {
      console.info("===> testDownloadConfigMetered001 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: true,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
      try {
        request.agent.create(context, config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/haps/testModule/cache/test.txt');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual('download');
            expect(info.description).assertEqual("");
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(false);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadConfigMetered001 test success!");
            done();
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadConfigMetered001 test fail!");
            done();
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadConfigMetered001 test fail!");
        done();
      }
    });

    /**
     * @tc.number: testDownloadConfigMetered002
     * @tc.name: testDownloadConfigMetered002
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     metered is 1 and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testDownloadConfigMetered002', 0, (done: Function) => {
      console.info("===> testDownloadConfigMetered002 test start!");

      let config: Record<string, string | number | undefined> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "title": undefined,
        "description": undefined,
        "mode": undefined,
        "overwrite": undefined,
        "method": undefined,
        "headers": undefined,
        //data: undefined,
        "saveas": undefined,
        "network": undefined,
        "metered": 1,
        "roaming": undefined,
        "retry": undefined,
        "redirect": undefined,
        "proxy": undefined,
        "index": undefined,
        "begins": undefined,
        "ends": undefined,
        "gauge": undefined,
        "precise": undefined,
        "token": undefined,
        "priority": undefined,
        "extras": undefined,
        "multipart": undefined,
        "notification": undefined,
        "minSpeed": undefined,
        "timeout": undefined,
      }

      try {
        request.agent.create(context, config as Object as request.agent.Config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/haps/testModule/cache/test.txt');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual('download');
            expect(info.description).assertEqual("");
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(false);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadConfigMetered002 test success!");
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadConfigMetered002 test fail!");
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadConfigMetered002 test fail!");
      }
    });

    /**
     * @tc.number: testDownloadConfigRoaming001
     * @tc.name: testDownloadConfigRoaming001
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     roaming is true and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testDownloadConfigRoaming001', 0, (done: Function) => {
      console.info("===> testDownloadConfigRoaming001 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: true,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
      try {
        request.agent.create(context, config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/haps/testModule/cache/test.txt');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual('download');
            expect(info.description).assertEqual("");
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(false);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadConfigRoaming001 test success!");
            done();
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadConfigRoaming001 test fail!");
            done();
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadConfigRoaming001 test fail!");
        done();
      }
    });

    /**
     * @tc.number: testDownloadConfigRoaming002
     * @tc.name: testDownloadConfigRoaming002
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     roaming is 1 and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testDownloadConfigMetered002', 0, (done: Function) => {
      console.info("===> testDownloadConfigMetered002 test start!");

      let config: Record<string, string | number | undefined> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "title": undefined,
        "description": undefined,
        "mode": undefined,
        "overwrite": undefined,
        "method": undefined,
        "headers": undefined,
        //data: undefined,
        "saveas": undefined,
        "network": undefined,
        "metered": undefined,
        "roaming": 1,
        "retry": undefined,
        "redirect": undefined,
        "proxy": undefined,
        "index": undefined,
        "begins": undefined,
        "ends": undefined,
        "gauge": undefined,
        "precise": undefined,
        "token": undefined,
        "priority": undefined,
        "extras": undefined,
        "multipart": undefined,
        "notification": undefined,
        "minSpeed": undefined,
        "timeout": undefined,
      }

      try {
        request.agent.create(context, config as Object as request.agent.Config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/haps/testModule/cache/test.txt');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual('download');
            expect(info.description).assertEqual("");
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(false);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadConfigRoaming002 test success!");
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadConfigRoaming002 test fail!");
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadConfigRoaming002 test fail!");
      }
    });

    /**
     * @tc.number: testDownloadConfigRetry001
     * @tc.name: testDownloadConfigRetry001
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     retry is true and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testDownloadConfigRetry001', 0, (done: Function) => {
      console.info("===> testDownloadConfigRetry001 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: true,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
      try {
        request.agent.create(context, config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/haps/testModule/cache/test.txt');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual('download');
            expect(info.description).assertEqual("");
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(true);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadConfigRetry001 test success!");
            done();
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadConfigRetry001 test fail!");
            done();
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadConfigRetry001 test fail!");
        done();
      }
    });

    /**
     * @tc.number: testDownloadConfigRetry002
     * @tc.name: testDownloadConfigRetry002
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     retry is 1 and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testDownloadConfigRetry002', 0, (done: Function) => {
      console.info("===> testDownloadConfigRetry002 test start!");

      let config: Record<string, string | number | undefined> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "title": undefined,
        "description": undefined,
        "mode": undefined,
        "overwrite": undefined,
        "method": undefined,
        "headers": undefined,
        //data: undefined,
        "saveas": undefined,
        "network": undefined,
        "metered": undefined,
        "roaming": undefined,
        "retry": 1,
        "redirect": undefined,
        "proxy": undefined,
        "index": undefined,
        "begins": undefined,
        "ends": undefined,
        "gauge": undefined,
        "precise": undefined,
        "token": undefined,
        "priority": undefined,
        "extras": undefined,
        "multipart": undefined,
        "notification": undefined,
        "minSpeed": undefined,
        "timeout": undefined,
      }

      try {
        request.agent.create(context, config as Object as request.agent.Config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/haps/testModule/cache/test.txt');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual('download');
            expect(info.description).assertEqual("");
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(false);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadConfigRetry002 test success!");
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadConfigRetry002 test fail!");
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadConfigRetry002 test fail!");
      }
    });

    /**
     * @tc.number: testDownloadConfigRedirect001
     * @tc.name: testDownloadConfigRedirect001
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     redirect is true and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testDownloadConfigRedirect001', 0, (done: Function) => {
      console.info("===> testDownloadConfigRedirect001 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: true,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
      try {
        request.agent.create(context, config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/haps/testModule/cache/test.txt');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual('download');
            expect(info.description).assertEqual("");
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(true);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadConfigRedirect001 test success!");
            done();
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadConfigRedirect001 test fail!");
            done();
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadConfigRedirect001 test fail!");
        done();
      }
    });

    /**
     * @tc.number: testDownloadConfigRedirect002
     * @tc.name: testDownloadConfigRedirect002
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     redirect is 1 and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testDownloadConfigRedirect002', 0, (done: Function) => {
      console.info("===> testDownloadConfigRedirect002 test start!");

      let config: Record<string, string | number | undefined> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "title": undefined,
        "description": undefined,
        "mode": undefined,
        "overwrite": undefined,
        "method": undefined,
        "headers": undefined,
        //data: undefined,
        "saveas": undefined,
        "network": undefined,
        "metered": undefined,
        "roaming": undefined,
        "retry": undefined,
        "redirect": 1,
        "proxy": undefined,
        "index": undefined,
        "begins": undefined,
        "ends": undefined,
        "gauge": undefined,
        "precise": undefined,
        "token": undefined,
        "priority": undefined,
        "extras": undefined,
        "multipart": undefined,
        "notification": undefined,
        "minSpeed": undefined,
        "timeout": undefined,
      }

      try {
        request.agent.create(context, config as Object as request.agent.Config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/haps/testModule/cache/test.txt');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual('download');
            expect(info.description).assertEqual("");
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(false);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadConfigRedirect002 test success!");
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadConfigRedirect002 test fail!");
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadConfigRedirect002 test fail!");
      }
    });

    /**
     * @tc.number: testDownloadConfigProxy001
     * @tc.name: testDownloadConfigProxy001
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
    proxy to 'http://127.0.0.1:8080', and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testDownloadConfigProxy001', 0, (done: Function) => {
      console.info("===> testDownloadConfigProxy001 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: 'https://127.0.0.1:8080',
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }

      try {
        request.agent.create(context, config, async (err, data) => {
          console.info("===> testDownloadConfigProxy001 test fail!");
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        console.info("===> testDownloadConfigProxy001 test success!");
        done();
      }
    });

    /**
     * @tc.number: testDownloadConfigProxy002
     * @tc.name: testDownloadConfigProxy002
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     proxy to 1, and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testDownloadConfigProxy002', 0, (done: Function) => {
      console.info("===> testDownloadConfigProxy002 test start!");

      let config: Record<string, string | number | undefined> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "title": undefined,
        "description": undefined,
        "mode": undefined,
        "overwrite": undefined,
        "method": undefined,
        "headers": undefined,
        //data: undefined,
        "saveas": undefined,
        "network": undefined,
        "metered": undefined,
        "roaming": undefined,
        "retry": undefined,
        "redirect": undefined,
        "proxy": 1,
        "index": undefined,
        "begins": undefined,
        "ends": undefined,
        "gauge": undefined,
        "precise": undefined,
        "token": undefined,
        "priority": undefined,
        "extras": undefined,
        "multipart": undefined,
        "notification": undefined,
        "minSpeed": undefined,
        "timeout": undefined,
      }

      try {
        request.agent.create(context, config as Object as request.agent.Config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/haps/testModule/cache/test.txt');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual('download');
            expect(info.description).assertEqual("");
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(false);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadConfigProxy002 test success!");
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadConfigProxy002 test fail!");
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadConfigProxy002 test fail!");
      }
    });

    /**
     * @tc.number: testDownloadConfigIndex001
     * @tc.name: testDownloadConfigIndex001
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
    index to 1, and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testDownloadConfigIndex001', 0, (done: Function) => {
      console.info("===> testDownloadConfigIndex001 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: 1,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }

      try {
        request.agent.create(context, config as Object as request.agent.Config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/haps/testModule/cache/test.txt');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual('download');
            expect(info.description).assertEqual("");
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(false);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadConfigIndex001 test success!");
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadConfigIndex001 test fail!");
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadConfigIndex001 test fail!");
      }
    });

    /**
     * @tc.number: testDownloadConfigIndex002
     * @tc.name: testDownloadConfigIndex002
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     index to 'test', and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testDownloadConfigIndex002', 0, (done: Function) => {
      console.info("===> testDownloadConfigIndex002 test start!");

      let config: Record<string, string | number | undefined> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "title": undefined,
        "description": undefined,
        "mode": undefined,
        "overwrite": undefined,
        "method": undefined,
        "headers": undefined,
        //data: undefined,
        "saveas": undefined,
        "network": undefined,
        "metered": undefined,
        "roaming": undefined,
        "retry": undefined,
        "redirect": undefined,
        "proxy": undefined,
        "index": 'test',
        "begins": undefined,
        "ends": undefined,
        "gauge": undefined,
        "precise": undefined,
        "token": undefined,
        "priority": undefined,
        "extras": undefined,
        "multipart": undefined,
        "notification": undefined,
        "minSpeed": undefined,
        "timeout": undefined,
      }

      try {
        request.agent.create(context, config as Object as request.agent.Config, async (err, task) => {
          expect(err === null).assertTrue();
          request.agent.show(task.tid).then(async (info) => {
            expect(info.uid).assertEqual(undefined);
            expect(info.bundle).assertEqual(undefined);
            expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
            expect(info.data).assertEqual("");
            expect(info.saveas).assertEqual('/data/storage/el2/base/haps/testModule/cache/test.txt');
            expect(info.tid).assertEqual(task.tid);
            expect(info.title).assertEqual('download');
            expect(info.description).assertEqual("");
            expect(info.action).assertEqual(request.agent.Action.DOWNLOAD);
            expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
            expect(info.mimeType).assertEqual("");
            expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
            expect(info.progress.index).assertEqual(0);
            expect(info.progress.sizes.length).assertEqual(1);
            expect(info.faults).assertEqual(null);
            expect(info.gauge).assertEqual(false);
            expect(typeof info.ctime).assertEqual("number");
            expect(typeof info.mtime).assertEqual("number");
            expect(info.retry).assertEqual(false);
            expect(info.tries).assertEqual(0);
            expect(typeof info.reason).assertEqual("string");
            await request.agent.remove(task.tid);
            console.info("===> testDownloadConfigIndex002 test success!");
          }).catch((err: Error) => {
            expect(false).assertTrue();
            console.info("===> testDownloadConfigIndex002 test fail!");
          });
        });
      } catch (err) {
        expect(false).assertTrue();
        console.info("===> testDownloadConfigIndex002 test fail!");
      }
    });
  })
}