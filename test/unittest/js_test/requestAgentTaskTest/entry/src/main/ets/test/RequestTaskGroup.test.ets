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

export default function requestTaskGroupTest() {
  describe('requestTaskGroupTest', () => {
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
    /**
     * @tc.number: testcreateGroup001
     * @tc.name: testcreateGroup001
     * @tc.desc: Test for createGroup when succeed
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testcreateGroup001', 0, async (done: Function) => {
      let config: request.agent.GroupConfig = {
        gauge: true,
        notification: {},
      };
      try {
        let gid = await request.agent.createGroup(config);
        expect(true).assertTrue();
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testcreateGroup002
     * @tc.name: testcreateGroup002
     * @tc.desc: Test for createGroup when fail by other type param
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testcreateGroup002', 0, async (done: Function) => {
      try {
        let gid = await request.agent.createGroup(123 as Object as request.agent.GroupConfig);
        expect(false).assertTrue();
        done();
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testcreateGroup003
     * @tc.name: testcreateGroup003
     * @tc.desc: Test for createGroup when title length is 1024
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testcreateGroup003', 0, async (done: Function) => {
      let config: request.agent.GroupConfig = {
        gauge: true,
        notification: {
          title: 'a'.repeat(1024),
        },
      };
      try {
        let gid = await request.agent.createGroup(config);
        expect(true).assertTrue();
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testcreateGroup004
     * @tc.name: testcreateGroup004
     * @tc.desc: Test for createGroup when fail by title length more than 1024
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testcreateGroup004', 0, async (done: Function) => {
      let config: request.agent.GroupConfig = {
        gauge: true,
        notification: {
          title: '1' + 'a'.repeat(1024),
        },
      };
      try {
        let gid = await request.agent.createGroup(config);
        expect(false).assertTrue();
        done();
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testcreateGroup005
     * @tc.name: testcreateGroup005
     * @tc.desc: Test for createGroup when text length is 3072
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testcreateGroup005', 0, async (done: Function) => {
      let config: request.agent.GroupConfig = {
        gauge: true,
        notification: {
          text: 'a'.repeat(3072),
        },
      };
      try {
        let gid = await request.agent.createGroup(config);
        expect(true).assertTrue();
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testcreateGroup006
     * @tc.name: testcreateGroup006
     * @tc.desc: Test for createGroup when fail by text length more than 3072
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testcreateGroup006', 0, async (done: Function) => {
      let config: request.agent.GroupConfig = {
        gauge: true,
        notification: {
          text: '1' + 'a'.repeat(3072),
        },
      };
      try {
        let gid = await request.agent.createGroup(config);
        expect(false).assertTrue();
        done();
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testcreateGroup007
     * @tc.name: testcreateGroup007
     * @tc.desc: Test for createGroup with valid visibility COMPLETION using enum
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testcreateGroup007', 0, async (done: Function) => {
      let config: request.agent.GroupConfig = {
        notification: {
          visibility: request.agent.VISIBILITY_COMPLETION,
        },
      };
      try {
        let gid = await request.agent.createGroup(config);
        expect(true).assertTrue();
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testcreateGroup008
     * @tc.name: testcreateGroup008
     * @tc.desc: Test for createGroup with valid visibility PROGRESS using enum
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testcreateGroup008', 0, async (done: Function) => {
      let config: request.agent.GroupConfig = {
        notification: {
          visibility: request.agent.VISIBILITY_PROGRESS,
        },
      };
      try {
        let gid = await request.agent.createGroup(config);
        expect(true).assertTrue();
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testcreateGroup009
     * @tc.name: testcreateGroup009
     * @tc.desc: Test for createGroup with valid visibility ANY (COMPLETION | PROGRESS)
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testcreateGroup009', 0, async (done: Function) => {
      let config: request.agent.GroupConfig = {
        notification: {
          visibility: request.agent.VISIBILITY_COMPLETION | request.agent.VISIBILITY_PROGRESS,
        },
      };
      try {
        let gid = await request.agent.createGroup(config);
        expect(true).assertTrue();
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testcreateGroup010
     * @tc.name: testcreateGroup010
     * @tc.desc: Test for createGroup when fail by invalid visibility value 0b00
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testcreateGroup010', 0, async (done: Function) => {
      let config: request.agent.GroupConfig = {
        notification: {
          visibility: 0b00, // Invalid value
        },
      };
      try {
        let gid = await request.agent.createGroup(config);
        expect(false).assertTrue();
        done();
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testcreateGroup011
     * @tc.name: testcreateGroup011
     * @tc.desc: Test for createGroup when fail by invalid visibility value
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testcreateGroup011', 0, async (done: Function) => {
      let config: request.agent.GroupConfig = {
        notification: {
          visibility: 0b100, // Invalid value
        },
      };
      try {
        let gid = await request.agent.createGroup(config);
        expect(false).assertTrue();
        done();
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testcreateGroup012
     * @tc.name: testcreateGroup012
     * @tc.desc: Test for createGroup when fail by invalid visibility type
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testcreateGroup012', 0, async (done: Function) => {
      let config: request.agent.GroupConfig = {
        notification: {
          visibility: 'invalid' as Object as number, // Invalid type
        },
      };
      try {
        let gid = await request.agent.createGroup(config);
        expect(false).assertTrue();
        done();
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testcreateGroup013
     * @tc.name: testcreateGroup013
     * @tc.desc: Test for createGroup with valid visibility COMPLETION using enum and gauge recover larger
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testcreateGroup013', 0, async (done: Function) => {
      let config: request.agent.GroupConfig = {
        gauge: true,
        notification: {
          visibility: request.agent.VISIBILITY_COMPLETION,
        },
      };
      try {
        let gid = await request.agent.createGroup(config);
        expect(true).assertTrue();
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testcreateGroup014
     * @tc.name: testcreateGroup014
     * @tc.desc: Test for createGroup with valid visibility COMPLETION using enum and gauge recover smaller
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testcreateGroup014', 0, async (done: Function) => {
      let config: request.agent.GroupConfig = {
        gauge: false,
        notification: {
          visibility: request.agent.VISIBILITY_COMPLETION | request.agent.VISIBILITY_PROGRESS,
        },
      };
      try {
        let gid = await request.agent.createGroup(config);
        expect(true).assertTrue();
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testattachGroup001
     * @tc.name: testattachGroup001
     * @tc.desc: Test for attachGroup when succeed with tid is empty
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testattachGroup001', 0, async (done: Function) => {
      let config: request.agent.GroupConfig = {
        gauge: true,
        notification: {},
      };
      try {
        let gid = await request.agent.createGroup(config);
        let tids = [];
        await request.agent.attachGroup(gid, tids);
        expect(true).assertTrue();
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testattachGroup002
     * @tc.name: testattachGroup002
     * @tc.desc: Test for attachGroup when failed by error type param
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testattachGroup002', 0, async (done: Function) => {
      try {
        await request.agent.attachGroup("gid", "tids" as Object as Array<string>);
        expect(false).assertTrue();
        done();
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testattachGroup003
     * @tc.name: testattachGroup003
     * @tc.desc: Test for attachGroup when failed by task not belong to this app
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testattachGroup003', 0, async (done: Function) => {
      let config: request.agent.GroupConfig = {
        gauge: true,
        notification: {},
      };
      try {
        let gid = await request.agent.createGroup(config);
        let tids = ['123123'];
        await request.agent.attachGroup(gid, tids);
        expect(false).assertTrue();
        done();
      } catch (err) {
        expect(err.code).assertEqual(21900006);
        done();
      }
    });
    /**
     * @tc.number: testattachGroup004
     * @tc.name: testattachGroup004
     * @tc.desc: Test for attachGroup when failed by
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testattachGroup004', 0, async (done: Function) => {
      let config: request.agent.GroupConfig = {
        gauge: true,
        notification: {},
      };
      try {
        let gid = await request.agent.createGroup(config);
        await request.agent.attachGroup(gid,undefined);
        expect(false).assertTrue();
        done();
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testattachGroup005
     * @tc.name: testattachGroup005
     * @tc.desc: Test for attachGroup when succeed with correct tid
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testattachGroup005', 0, async (done: Function) => {
      let config: request.agent.GroupConfig = {
        gauge: true,
        notification: {},
      };
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testGroupa05.txt',
        overwrite: true,
      };
      let task = await request.agent.create(context, conf);
      try {
        let gid = await request.agent.createGroup(config);
        await request.agent.attachGroup(gid, [task.tid]);
        expect(true).assertTrue();
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testattachGroup006
     * @tc.name: testattachGroup006
     * @tc.desc: Test for attachGroup when failed by error mode
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testattachGroup006', 0, async (done: Function) => {
      let config: request.agent.GroupConfig = {
        gauge: true,
        notification: {},
      };
      let conf1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testGroupa061.txt',
        overwrite: true,
        mode: request.agent.Mode.FOREGROUND
      };
      let conf2: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testGroupa061.txt',
        overwrite: true,
        mode: request.agent.Mode.BACKGROUND
      };
      let task1 = await request.agent.create(context, conf1);
      let tid1 = task1.tid;
      let task2 = await request.agent.create(context, conf2);
      let tid2 = task2.tid;
      try {
        let gid = await request.agent.createGroup(config);
        await request.agent.attachGroup(gid, [tid1, tid2]);
        expect(false).assertTrue();
        done();
      } catch (err) {
        expect(err.code).assertEqual(21900005);
        done();
      }
    });
    /**
     * @tc.number: testattachGroup007
     * @tc.name: testattachGroup007
     * @tc.desc: Test for attachGroup when failed by error state
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testattachGroup007', 0, async (done: Function) => {
      let config: request.agent.GroupConfig = {
        gauge: true,
        notification: {},
      };
      let conf1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testGroupa071.txt',
        overwrite: true,
        mode: request.agent.Mode.BACKGROUND
      };
      let conf2: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testGroupa071.txt',
        overwrite: true,
        mode: request.agent.Mode.BACKGROUND
      };
      let task1 = await request.agent.create(context, conf1);
      task1.start();
      let tid1 = task1.tid;
      let task2 = await request.agent.create(context, conf2);
      let tid2 = task2.tid;
      try {
        let gid = await request.agent.createGroup(config);
        await request.agent.attachGroup(gid, [tid1, tid2]);
        expect(false).assertTrue();
        done();
      } catch (err) {
        expect(err.code).assertEqual(21900007);
        await request.agent.remove(task1.tid);
        done();
      }
    });
    /**
     * @tc.number: testattachGroup008
     * @tc.name: testattachGroup008
     * @tc.desc: Test for attachGroup when failed by attach gid which has deleted
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testattachGroup008', 0, async (done: Function) => {
      let config: request.agent.GroupConfig = {
        gauge: true,
        notification: {},
      };
      try {
        let gid = await request.agent.createGroup(config);
        await request.agent.attachGroup(gid, []);
        await request.agent.deleteGroup(gid);
        await request.agent.attachGroup(gid, []);
        expect(false).assertTrue();
        done();
      } catch (err) {
        expect(err.code).assertEqual(21900008);
        done();
      }
    });
    /**
     * @tc.number: testattachGroup009
     * @tc.name: testattachGroup009
     * @tc.desc: Test for attachGroup when failed by attach gid which is not exist
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testattachGroup009', 0, async (done: Function) => {
      let gid = "123";
      try {
        await request.agent.attachGroup(gid, []);
        expect(false).assertTrue();
        done();
      } catch (err) {
        expect(err.code).assertEqual(21900008);
        done();
      }
    });
    /**
     * @tc.number: testattachGroup010
     * @tc.name: testattachGroup010
     * @tc.desc: Test for attachGroup when failed by attach tid which is not exist
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testattachGroup010', 0, async (done: Function) => {
      let config: request.agent.GroupConfig = {
        gauge: true,
        notification: {},
      };
      try {
        let gid = await request.agent.createGroup(config);
        await request.agent.attachGroup(gid, ["123"]);
        expect(false).assertTrue();
        done();
      } catch (err) {
        expect(err.code).assertEqual(21900006);
        done();
      }
    });
    /**
     * @tc.number: testattachGroup011
     * @tc.name: testattachGroup011
     * @tc.desc: Test for attachGroup with valid visibility in GroupConfig and different valid visibility in Config
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testattachGroup011', 0, async (done: Function) => {
      let groupConfig: request.agent.GroupConfig = {
        notification: {
          visibility: request.agent.VISIBILITY_COMPLETION,
        },
      };
      let taskConfig: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testGroupAttach004.txt',
        overwrite: true,
        notification: {
          visibility: request.agent.VISIBILITY_PROGRESS,
        },
      };
      try {
        let gid = await request.agent.createGroup(groupConfig);
        let task = await request.agent.create(context, taskConfig);
        await request.agent.attachGroup(gid, [task.tid]);
        expect(true).assertTrue();
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testattachGroup012
     * @tc.name: testattachGroup012
     * @tc.desc: Test for attachGroup with valid visibility in GroupConfig and invalid visibility in Config
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testattachGroup012', 0, async (done: Function) => {
      let groupConfig: request.agent.GroupConfig = {
        notification: {
          visibility: request.agent.VISIBILITY_COMPLETION,
        },
      };
      let taskConfig: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testGroupAttach005.txt',
        overwrite: true,
        notification: {
          visibility: 0b100, // Invalid value
        },
      };
      try {
        let gid = await request.agent.createGroup(groupConfig);
        let task = await request.agent.create(context, taskConfig);
        expect(false).assertTrue();
        done();
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testattachGroup013
     * @tc.name: testattachGroup013
     * @tc.desc: Test for attachGroup with invalid visibility in GroupConfig and valid visibility in Config
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testattachGroup013', 0, async (done: Function) => {
      let groupConfig: request.agent.GroupConfig = {
        notification: {
          visibility: 0b100, // Invalid value
        },
      };
      let taskConfig: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testGroupAttach006.txt',
        overwrite: true,
        notification: {
          visibility: request.agent.VISIBILITY_COMPLETION,
        },
      };
      try {
        let gid = await request.agent.createGroup(groupConfig);
        expect(false).assertTrue();
        done();
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testattachGroup014
     * @tc.name: testattachGroup014
     * @tc.desc: Test for attachGroup with invalid visibility in both GroupConfig and Config
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testattachGroup014', 0, async (done: Function) => {
      let groupConfig: request.agent.GroupConfig = {
        notification: {
          visibility: 0b100, // Invalid value
        },
      };
      let taskConfig: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testGroupAttach007.txt',
        overwrite: true,
        notification: {
          visibility: 'invalid' as Object as number, // Invalid type
        },
      };
      try {
        let gid = await request.agent.createGroup(groupConfig);
        expect(false).assertTrue();
        done();
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testdeleteGroup001
     * @tc.name: testdeleteGroup001
     * @tc.desc: Test for attachGroup when succeed
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testdeleteGroup001', 0, async (done: Function) => {
      let config: request.agent.GroupConfig = {
        gauge: true,
        notification: {},
      };
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testGroupd01.txt',
        overwrite: true,
      };
      let task = await request.agent.create(context, conf);
      let tid = task.tid;
      try {
        let gid = await request.agent.createGroup(config);
        let tids = [tid];
        await request.agent.attachGroup(gid, tids);
        await request.agent.deleteGroup(gid);
        expect(true).assertTrue();
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testdeleteGroup002
     * @tc.name: testdeleteGroup002
     * @tc.desc: Test for attachGroup when failed by git has benn deleted
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testdeleteGroup002', 0, async (done: Function) => {
      let config: request.agent.GroupConfig = {
        gauge: true,
        notification: {},
      };
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testGroupd02.txt',
        overwrite: true,
      };
      let task = await request.agent.create(context, conf);
      let tid = task.tid;
      try {
        let gid = await request.agent.createGroup(config);
        let tids = [tid];
        await request.agent.attachGroup(gid, tids);
        await request.agent.deleteGroup(gid);
        await request.agent.deleteGroup(gid);
        expect(false).assertTrue();
        done();
      } catch (err) {
        expect(err.code).assertEqual(21900008);
        done();
      }
    });
    /**
     * @tc.number: testdeleteGroup003
     * @tc.name: testdeleteGroup003
     * @tc.desc: Test for attachGroup when failed by git not exist
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testdeleteGroup003', 0, async (done: Function) => {
      let config: request.agent.GroupConfig = {
        gauge: true,
        notification: {},
      };
      try {
        let gid = "123";
        await request.agent.deleteGroup(gid);
        expect(false).assertTrue();
        done();
      } catch (err) {
        expect(err.code).assertEqual(21900008);
        done();
      }
    });
  })
}