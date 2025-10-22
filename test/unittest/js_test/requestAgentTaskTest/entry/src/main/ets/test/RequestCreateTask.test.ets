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
import wantAgent, { WantAgent } from '@ohos.app.ability.wantAgent';

export default function requestCreateTaskTest() {
  describe('requestCreateTaskTest', () => {
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
     * @tc.number: testTaskToken001
     * @tc.name: testTaskToken001
     * @tc.desc: Test create task when token is 7 bytes
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskToken001', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskToken001.txt',
        token: 'abcdef'
      };
      try {
        request.agent.create(context, conf).then((data) => {
          expect(false).assertTrue();
          done();
        }).catch((err: Error) => {
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testTaskToken002
     * @tc.name: testTaskToken002
     * @tc.desc: Test create task when token is 2049 bytes
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskToken002', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskToken002.txt',
        token: 'a'.padStart(2049, 'a')
      };
      try {
        request.agent.create(context, conf).then((data) => {
          expect(false).assertTrue();
          done();
        }).catch((err: Error) => {
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testTaskToken003
     * @tc.name: testTaskToken003
     * @tc.desc: Test create task when token is number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskToken003', 0, async (done: Function) => {
      let conf: Record<string, string | number> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "saveas": "testTaskToken003.txt",
        "token": 666
      };
      request.agent.create(context, conf as Object as request.agent.Config).then(async (data) => {
        expect(data.config.token).assertEqual(666);
        await request.agent.remove(data.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskToken004
     * @tc.name: testTaskToken004
     * @tc.desc: Test create task when token is 2048 bytes
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskToken004', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskToken004.txt',
        token: 'a'.padStart(2048, 'a')
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
     * @tc.number: testTaskToken005
     * @tc.name: testTaskToken005
     * @tc.desc: Test create task when token is 8 bytes
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskToken005', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskToken005.txt',
        token: '11111111'
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
     * @tc.number: testCreate001
     * @tc.name: testCreate001
     * @tc.desc: Test create for conf is string type
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testCreate001', 0, async (done: Function) => {
      try {
        await request.agent.create(context, "conf" as Object as request.agent.Config);
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testCreate002
     * @tc.name: testCreate002
     * @tc.desc: Test create
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testCreate002', 0, async (done: Function) => {
      try {
        expect(request.agent.Action.DOWNLOAD).assertEqual(0);
        expect(request.agent.Action.UPLOAD).assertEqual(1);
        expect(request.agent.Mode.BACKGROUND).assertEqual(0);
        expect(request.agent.Mode.FOREGROUND).assertEqual(1);
        expect(request.agent.Network.ANY).assertEqual(0);
        expect(request.agent.Network.WIFI).assertEqual(1);
        expect(request.agent.Network.CELLULAR).assertEqual(2);
        expect(request.agent.State.INITIALIZED).assertEqual(0x00);
        expect(request.agent.State.WAITING).assertEqual(0x10);
        expect(request.agent.State.RUNNING).assertEqual(0x20);
        expect(request.agent.State.RETRYING).assertEqual(0x21);
        expect(request.agent.State.PAUSED).assertEqual(0x30);
        expect(request.agent.State.STOPPED).assertEqual(0x31);
        expect(request.agent.State.COMPLETED).assertEqual(0x40);
        expect(request.agent.State.FAILED).assertEqual(0x41);
        expect(request.agent.State.REMOVED).assertEqual(0x50);
        expect(request.agent.Faults.OTHERS).assertEqual(0xFF);
        expect(request.agent.Faults.DISCONNECTED).assertEqual(0x00);
        expect(request.agent.Faults.TIMEOUT).assertEqual(0x10);
        expect(request.agent.Faults.PROTOCOL).assertEqual(0x20);
        expect(request.agent.Faults.FSIO).assertEqual(0x40);
        expect(request.agent.WaitingReason.TASK_QUEUE_FULL).assertEqual(0x00);
        expect(request.agent.WaitingReason.NETWORK_NOT_MATCH).assertEqual(0x01);
        expect(request.agent.WaitingReason.APP_BACKGROUND).assertEqual(0x02);
        expect(request.agent.WaitingReason.USER_INACTIVATED).assertEqual(0x03);
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testTaskVisibility001
     * @tc.name: testTaskVisibility001
     * @tc.desc: Test create task with valid visibility COMPLETION using enum
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskVisibility001', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskVisibility001.txt',
        notification: {
          visibility: request.agent.VISIBILITY_COMPLETION,
        },
      };
      try {
        let task = await request.agent.create(context, conf);
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testTaskVisibility002
     * @tc.name: testTaskVisibility002
     * @tc.desc: Test create task with valid visibility PROGRESS using enum
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskVisibility002', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskVisibility002.txt',
        notification: {
          visibility: request.agent.VISIBILITY_PROGRESS,
        },
      };
      try {
        let task = await request.agent.create(context, conf);
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testTaskVisibility003
     * @tc.name: testTaskVisibility003
     * @tc.desc: Test create task with valid visibility ANY (COMPLETION | PROGRESS)
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskVisibility003', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskVisibility003.txt',
        notification: {
          visibility: request.agent.VISIBILITY_COMPLETION | request.agent.VISIBILITY_PROGRESS,
        },
      };
      try {
        let task = await request.agent.create(context, conf);
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testTaskVisibility004
     * @tc.name: testTaskVisibility004
     * @tc.desc: Test create task when fail by invalid visibility value 0b00
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskVisibility004', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskVisibility004.txt',
        notification: {
          visibility: 0b00, // Invalid value
        },
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
     * @tc.number: testTaskVisibility005
     * @tc.name: testTaskVisibility005
     * @tc.desc: Test create task when fail by invalid visibility value
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskVisibility005', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskVisibility005.txt',
        notification: {
          visibility: 0b100, // Invalid value
        },
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
     * @tc.number: testTaskVisibility006
     * @tc.name: testTaskVisibility006
     * @tc.desc: Test create task when fail by invalid visibility type
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskVisibility006', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskVisibility006.txt',
        notification: {
          visibility: 'invalid' as Object as number, // Invalid type
        },
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
     * @tc.number: testTaskVisibility007
     * @tc.name: testTaskVisibility007
     * @tc.desc: Test create task with valid visibility PROGRESS using enum and gauge recover larger
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskVisibility007', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskVisibility007.txt',
        gauge: true,
        notification: {
          visibility: request.agent.VISIBILITY_PROGRESS,
        },
      };
      try {
        let task = await request.agent.create(context, conf);
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testTaskVisibility008
     * @tc.name: testTaskVisibility008
     * @tc.desc: Test create task with valid visibility PROGRESS using enum and gauge recover smaller
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskVisibility008', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskVisibility008.txt',
        gauge: false,
        notification: {
          visibility: request.agent.VISIBILITY_COMPLETION | request.agent.VISIBILITY_PROGRESS,
        },
      };
      try {
        let task = await request.agent.create(context, conf);
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testTaskWantAgent001
     * @tc.name: testTaskWantAgent001
     * @tc.desc: Test create task with valid wantAgent
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskWantAgent001', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskWantAgent001.txt',
        gauge: false,
        notification: {
          visibility: request.agent.VISIBILITY_COMPLETION | request.agent.VISIBILITY_PROGRESS,
          wantAgent: await wantAgent.getWantAgent(wantAgentInfo),
        }
      };
      try {
        let task = await request.agent.create(context, conf);
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testTaskWantAgent002
     * @tc.name: testTaskWantAgent002
     * @tc.desc: Test create task with invalid wantAgent
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskWantAgent002', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskWantAgent002.txt',
        gauge: false,
        notification: {
          visibility: request.agent.VISIBILITY_COMPLETION | request.agent.VISIBILITY_PROGRESS,
          wantAgent: {},
        }
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
     * @tc.number: testTaskWantAgent003
     * @tc.name: testTaskWantAgent003
     * @tc.desc: Test create task with undefined wantAgent
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskWantAgent003', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskWantAgent003.txt',
        gauge: false,
        notification: {
          visibility: request.agent.VISIBILITY_COMPLETION | request.agent.VISIBILITY_PROGRESS,
          wantAgent: undefined,
        }
      };
      try {
        let task = await request.agent.create(context, conf);
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testTaskWantAgent004
     * @tc.name: testTaskWantAgent004
     * @tc.desc: Test create task with no wantAgent
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskWantAgent004', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskWantAgent004.txt',
        gauge: false,
        notification: {
          visibility: request.agent.VISIBILITY_COMPLETION | request.agent.VISIBILITY_PROGRESS,
        }
      };
      try {
        let task = await request.agent.create(context, conf);
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testTaskNotiTitle001
     * @tc.name: testTaskNotiTitle001
     * @tc.desc: Test create task with no notification title 
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskNotiTitle001', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskNotiTitle001.txt',
        gauge: false,
        notification: {}
      };
      try {
        let task = await request.agent.create(context, conf);
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testTaskNotiTitle002
     * @tc.name: testTaskNotiTitle002
     * @tc.desc: Test create task with notification title is string
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskNotiTitle002', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskNotiTitle002.txt',
        gauge: false,
        notification: {
          title: 'testTaskNotiTitle002',
        }
      };
      try {
        let task = await request.agent.create(context, conf);
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testTaskNotiTitle003
     * @tc.name: testTaskNotiTitle003
     * @tc.desc: Test create task with notification title length is 1024
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskNotiTitle003', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskNotiTitle003.txt',
        gauge: false,
        notification: {
          title: 'a'.repeat(1024),
        }
      };
      try {
        let task = await request.agent.create(context, conf);
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testTaskNotiTitle004
     * @tc.name: testTaskNotiTitle004
     * @tc.desc: Test create task with notification title length exceeds 1024
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskNotiTitle004', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskNotiTitle004.txt',
        gauge: false,
        notification: {
          title: 'a'.repeat(1025),
        }
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
     * @tc.number: testTaskNotiText001
     * @tc.name: testTaskNotiText001
     * @tc.desc: Test create task with no notification text
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskNotiText001', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskNotiText001.txt',
        gauge: false,
        notification: {}
      };
      try {
        let task = await request.agent.create(context, conf);
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testTaskNotiText002
     * @tc.name: testTaskNotiText002
     * @tc.desc: Test create task with notification text is string
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskNotiText002', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskNotiText002.txt',
        gauge: false,
        notification: {
          text: 'testTaskNotiText002',
        }
      };
      try {
        let task = await request.agent.create(context, conf);
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testTaskNotiText003
     * @tc.name: testTaskNotiText003
     * @tc.desc: Test create task with notification title length is 3072
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskNotiText003', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskNotiText003.txt',
        gauge: false,
        notification: {
          text: 'a'.repeat(3072),
        }
      };
      try {
        let task = await request.agent.create(context, conf);
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testTaskNotiText004
     * @tc.name: testTaskNotiText004
     * @tc.desc: Test create task with notification title length exceeds 3072
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskNotiText004', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskNotiText004.txt',
        gauge: false,
        notification: {
          text: 'a'.repeat(3073),
        }
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
  })
}