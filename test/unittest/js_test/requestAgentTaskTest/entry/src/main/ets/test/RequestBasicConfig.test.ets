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
    });
    afterEach(async () => {
      console.info('afterEach called');
      if (fs.accessSync(cacheDir + '/test.txt')) {
        fs.unlinkSync(cacheDir + '/test.txt');
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
        if (fs.accessSync(`${cacheDir}/testTaskAction005.txt`)) {
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
        if (fs.accessSync(`${cacheDir}/testTaskAction006.txt`)) {
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

  })
}