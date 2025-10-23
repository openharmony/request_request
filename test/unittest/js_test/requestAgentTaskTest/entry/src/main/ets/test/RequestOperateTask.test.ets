/*
 * Copyright (C) 2023 Huawei Device Co., Ltd.
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

import { describe, beforeAll, beforeEach, afterEach, afterAll, it, expect } from "@ohos/hypium";
import request from '@ohos.request';
import fs from '@ohos.file.fs';
import commonEvent from '@ohos.commonEventManager';
import common from '@ohos.app.ability.common';
import { BusinessError } from "@ohos.base";

export default function requestOperateTaskTest() {
  describe('requestOperateTaskTest', () => {
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
      if (fs.accessSync(cacheDir + '/README.md')) {
        fs.unlinkSync(cacheDir + '/README.md');
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
     * @tc.number: testStartTask001
     * @tc.name: testStartTask001
     * @tc.desc: Test start FOREGROUND task for callback
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testStartTask001', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testStartTask001.txt',
        mode: request.agent.Mode.FOREGROUND
      };
      let task = await request.agent.create(context, conf);
      await task.start(async (err: Error) => {
        expect(err === null).assertTrue();
        await task.stop();
        done();
      });
    });
    /**
     * @tc.number: testStartTask002
     * @tc.name: testStartTask002
     * @tc.desc: Test start FOREGROUND task for promise
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testStartTask002', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testStartTask002.txt',
        mode: request.agent.Mode.FOREGROUND
      };
      let task = await request.agent.create(context, conf);
      await task.start().then(async () => {
        expect(true).assertTrue();
        await task.stop();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testStartTask003
     * @tc.name: testStartTask003
     * @tc.desc: Test start background task for callback
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testStartTask003', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testStartTask003.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      await task.start(async (err: Error) => {
        expect(err === null).assertTrue();
        await task.stop();
        done();
      });
    });
    /**
     * @tc.number: testStartTask004
     * @tc.name: testStartTask004
     * @tc.desc: Test start background task for promise
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testStartTask004', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testStartTask004.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      await task.start().then(async () => {
        expect(true).assertTrue();
        await task.stop();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testStartTask005
     * @tc.name: testStartTask005
     * @tc.desc: Test start pause start background task as Start a pause task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testStartTask005', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testStartTask005.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      try {
        await task.start();
        await task.pause();
        await task.start();
      } catch (err) {
        expect(err.code).assertEqual(21900007);
        await request.agent.remove(task.tid);
        done();
      }
    });
    /**
     * @tc.number: testStartTask006
     * @tc.name: testStartTask006
     * @tc.desc: Test start stop pause background task as Pause a stop task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testStartTask006', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testStartTask006.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      try {
        await task.start();
        await task.stop();
        await task.pause();
      } catch (err) {
        expect(err.code).assertEqual(21900007);
        done();
      }
    });
    /**
     * @tc.number: testStartTask007
     * @tc.name: testStartTask007
     * @tc.desc: Test start stop pause FOREGROUND as Pause a stop task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testStartTask007', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testStartTask007.txt',
        mode: request.agent.Mode.FOREGROUND
      };
      let task = await request.agent.create(context, conf);
      try {
        await task.start();
        await task.stop();
        await task.pause();
      } catch (err) {
        expect(err.code).assertEqual(21900007);
        done();
      }
    });
    /**
     * @tc.number: testStartTask008
     * @tc.name: testStartTask008
     * @tc.desc: Test start two FOREGROUND task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testStartTask008', 0, async (done: Function) => {
      try {
        let conf: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/large_test.zip',
          saveas: `test.txt`,
          mode: request.agent.Mode.FOREGROUND
        };
        let task = await request.agent.create(context, conf);
        await task.start();
        let conf1: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
          saveas: `README.md`,
          mode: request.agent.Mode.FOREGROUND
        };
        request.agent.create(context, conf1, async (err, task1) => {
          expect(true).assertTrue();
          await request.agent.remove(task1.tid);
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testStartTask009
     * @tc.name: testStartTask009
     * @tc.desc: Test start a removed FOREGROUND task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testStartTask009', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        mode: request.agent.Mode.BACKGROUND,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: `testStartTask009.txt`,
      };
      try {
        let task: request.agent.Task = await request.agent.create(context, conf);
        await request.agent.remove(task.tid);
        await task.start();
        expect(false).assertTrue();
        done();
      } catch (err) {
        expect(err.code).assertEqual(21900007);
        expect(true).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testStartTask0010
     * @tc.name: testStartTask010
     * @tc.desc: Test start a running BACKGROUND task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testStartTask0010', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testStartTask0010.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      try {
        await task.start();
        await task.start();
      } catch (err) {
        expect(err.code).assertEqual(21900007);
        done();
      }
    });
    /**
     * @tc.number: testStartTask011
     * @tc.name: testStartTask011
     * @tc.desc: Test start two BACKGROUND task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testStartTask011', 0, async (done: Function) => {
      try {
        let conf: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/large_test.zip',
          saveas: `test.txt`,
          mode: request.agent.Mode.BACKGROUND
        };
        let task = await request.agent.create(context, conf);
        await task.start();
        let conf1: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
          saveas: `README.md`,
          mode: request.agent.Mode.BACKGROUND
        };
        request.agent.create(context, conf1, async (err, task1) => {
          expect(true).assertTrue();
          await request.agent.remove(task1.tid);
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testPauseTask001
     * @tc.name: testPauseTask001
     * @tc.desc: Test pause FOREGROUND task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testPauseTask001', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testPauseTask001.txt',
        mode: request.agent.Mode.FOREGROUND
      };
      let task = await request.agent.create(context, conf);
      await task.start();
      await task.pause(async (err: Error) => {
        expect(err === null).assertTrue();
        await request.agent.remove(task.tid);
        done();
      });
    });
    /**
     * @tc.number: testPauseTask002
     * @tc.name: testPauseTask002
     * @tc.desc: Test pause stop background task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testPauseTask002', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testPauseTask002.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      try {
        await task.start();
        await task.pause();
        await task.stop();
      } catch (err) {
        expect(err.code).assertEqual(21900007);
        await request.agent.remove(task.tid);
        done();
      }
    });
    /**
     * @tc.number: testPauseTask003
     * @tc.name: testPauseTask003
     * @tc.desc: Test pause background task for callback
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testPauseTask003', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testPauseTask003.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      await task.start();
      await task.pause(async (err: Error) => {
        expect(err === null).assertTrue();
        await request.agent.remove(task.tid);
        done();
      });
    });
    /**
     * @tc.number: testPauseTask004
     * @tc.name: testPauseTask004
     * @tc.desc: Test pause background task for promise
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testPauseTask004', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testPauseTask004.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      await task.start();
      await task.pause().then(async () => {
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testPauseTask005
     * @tc.name: testPauseTask005
     * @tc.desc: Test pause resume pause background task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testPauseTask005', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/large_test.zip',
        saveas: 'testPauseTask005.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      await task.start();
      await task.pause();
      await task.resume().then(() => {
        expect(true).assertTrue();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
      await task.pause().then(async () => {
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testPauseTask006
     * @tc.name: testPauseTask006
     * @tc.desc: Test pause a created background task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testPauseTask006', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testPauseTask006.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      try {
        await task.pause();
      } catch (err) {
        expect(err.code).assertEqual(21900007);
        await request.agent.remove(task.tid);
        done();
      }
    });
    /**
     * @tc.number: testPauseTask007
     * @tc.name: testPauseTask007
     * @tc.desc: Test pause a paused FOREGROUND task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testPauseTask007', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testPauseTask007.txt',
        mode: request.agent.Mode.FOREGROUND
      };
      let task = await request.agent.create(context, conf);
      try {
        await task.start();
        await task.pause();
        await task.pause();
      } catch (err) {
        expect(err.code).assertEqual(21900007);
        await request.agent.remove(task.tid);
        done();
      }
    });
    /**
     * @tc.number: testPauseTask008
     * @tc.name: testPauseTask008
     * @tc.desc: Test pause a removed FOREGROUND task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testPauseTask008', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testPauseTask008.txt',
        mode: request.agent.Mode.FOREGROUND
      };
      let task = await request.agent.create(context, conf);
      try {
        await task.start();
        await request.agent.remove(task.tid);
        await task.pause();
      } catch (err) {
        expect(err.code).assertEqual(21900007);
        done();
      }
    });
    /**
     * @tc.number: testResumeTask001
     * @tc.name: testResumeTask001
     * @tc.desc: Test start resume FOREGROUND task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testResumeTask001', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testResumeTask001.txt',
        mode: request.agent.Mode.FOREGROUND
      };
      let task = await request.agent.create(context, conf);
      try {
        await task.resume();
      } catch (err) {
        expect(err.code).assertEqual(21900007);
        await request.agent.remove(task.tid);
        done();
      }
    });
    /**
     * @tc.number: testResumeTask002
     * @tc.name: testResumeTask002
     * @tc.desc: Test pause resume background task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testResumeTask002', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testResumeTask002.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      await task.start();
      await task.pause();
      await task.resume(async (err: Error) => {
        expect(err === null).assertTrue();
        await request.agent.remove(task.tid);
        done();
      });
    });
    /**
     * @tc.number: testResumeTask003
     * @tc.name: testResumeTask003
     * @tc.desc: Test stop resume background task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testResumeTask003', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testResumeTask003.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      try {
        await task.start();
        await task.stop().then(() => {
          expect(true).assertTrue();
          done();
        }).catch((err: Error) => {
          expect(false).assertTrue();
          done();
        });
        await task.resume();
      } catch (err) {
        expect(err.code).assertEqual(21900007);
        done();
      }
    });
    /**
     * @tc.number: testResumeTask004
     * @tc.name: testResumeTask004
     * @tc.desc: Test start resume background task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testResumeTask004', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testResumeTask004.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      try {
        await task.start();
        await task.resume();
      } catch (err) {
        expect(err.code).assertEqual(21900007);
        await request.agent.remove(task.tid);
        done();
      }
    });
    /**
     * @tc.number: testResumeTask005
     * @tc.name: testResumeTask005
     * @tc.desc: Test pause resume FOREGROUND task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testResumeTask005', 0, async (done: Function) => {
      try {
        let conf: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/large_test.zip',
          saveas: 'testResumeTask005.txt',
          mode: request.agent.Mode.FOREGROUND
        };
        let task = await request.agent.create(context, conf);
        await task.start();
        await task.pause();
        await task.resume(async (err: Error) => {
          expect(err === null).assertTrue();
          await request.agent.remove(task.tid);
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testResumeTask006
     * @tc.name: testResumeTask006
     * @tc.desc: Test stop resume BACKGROUND task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testResumeTask006', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/large_test.zip',
        saveas: 'testResumeTask006.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      try {
        await task.start();
        await task.stop().then(() => {
          expect(true).assertTrue();
        }).catch((err: Error) => {
          expect(false).assertTrue();
          done();
        });
        await task.resume();
      } catch (err) {
        expect(err.code).assertEqual(21900007);
        done();
      }
    });
    /**
     * @tc.number: testResumeTask007
     * @tc.name: testResumeTask007
     * @tc.desc: Test resume BACKGROUND task for promise
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testResumeTask007', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/large_test.zip',
        saveas: 'testResumeTask007.txt',
        mode: request.agent.Mode.FOREGROUND
      };
      let task = await request.agent.create(context, conf);
      await task.start();
      await task.pause();
      await task.resume().then(() => {
        expect(true).assertTrue();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testResumeTask008
     * @tc.name: testResumeTask008
     * @tc.desc: Test resume a removed BACKGROUND task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testResumeTask008', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/large_test.zip',
        saveas: 'testResumeTask008.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      try {
        let task: request.agent.Task = await request.agent.create(context, conf);
        await request.agent.remove(task.tid);
        await task.resume();
        expect(false).assertTrue();
        done();
      } catch (err) {
        expect(err.code).assertEqual(21900007);
        expect(true).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testResumeTask009
     * @tc.name: testResumeTask009
     * @tc.desc: Test resume a created BACKGROUND task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testResumeTask009', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/large_test.zip',
        saveas: 'testResumeTask009.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      try {
        await task.resume();
      } catch (err) {
        expect(err.code).assertEqual(21900007);
        done();
      }
    });
    /**
     * @tc.number: testStopTask001
     * @tc.name: testStopTask001
     * @tc.desc: Test stop FOREGROUND task for callback
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testStopTask001', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testStopTask001.txt',
        mode: request.agent.Mode.FOREGROUND
      };
      let task = await request.agent.create(context, conf);
      await task.start();
      await task.stop(async (err: Error) => {
        expect(err === null).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testStopTask002
     * @tc.name: testStopTask002
     * @tc.desc: Test stop FOREGROUND task for promise
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testStopTask002', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testStopTask002.txt',
        mode: request.agent.Mode.FOREGROUND
      };
      let task = await request.agent.create(context, conf);
      await task.start();
      await task.stop().then(() => {
        expect(true).assertTrue();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testStopTask003
     * @tc.name: testStopTask003
     * @tc.desc: Test stop background task for callback
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testStopTask003', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testStopTask003.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      await task.start();
      await task.stop(async (err: Error) => {
        expect(err === null).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testStopTask004
     * @tc.name: testStopTask004
     * @tc.desc: Test stop background task for promise
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testStopTask004', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testStopTask004.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      await task.start();
      await task.stop().then(() => {
        expect(true).assertTrue();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testStopTask005
     * @tc.name: testStopTask005
     * @tc.desc: Test stop pause background task for promise
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testStopTask005', 0, async (done: Function) => {
      await sleep(2000);
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testStopTask005.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      try {
        let task = await request.agent.create(context, conf);
        await task.start();
        await task.stop();
        await task.pause();
      } catch (err) {
        expect(err.code).assertEqual(21900007);
        done();
      }
    });
    /**
     * @tc.number: testStopTask006
     * @tc.name: testStopTask006
     * @tc.desc: Test stop pause background task for promise
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testStopTask006', 0, async (done: Function) => {
      await sleep(2000);
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/large_test.zip',
        saveas: 'testStopTask006.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      try {
        let task = await request.agent.create(context, conf);
        await task.start();
        await task.stop();
        await task.pause();
      } catch (err) {
        expect(err.code).assertEqual(21900007);
        done();
      }
    });
    /**
     * @tc.number: testStopTask007
     * @tc.name: testStopTask007
     * @tc.desc: Test stop a created FOREGROUND task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testStopTask007', 0, async (done: Function) => {
      await sleep(2000);
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/large_test.zip',
        saveas: 'testStopTask007.txt',
        mode: request.agent.Mode.FOREGROUND
      };
      try {
        let task = await request.agent.create(context, conf);
        await task.stop();
      } catch (err) {
        expect(err.code).assertEqual(21900007);
        done();
      }
    });
    /**
     * @tc.number: testStopTask008
     * @tc.name: testStopTask008
     * @tc.desc: Test stop a removed FOREGROUND task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testStopTask008', 0, async (done: Function) => {
      await sleep(2000);
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/large_test.zip',
        saveas: 'testStopTask008.txt',
        mode: request.agent.Mode.FOREGROUND
      };
      try {
        let task: request.agent.Task = await request.agent.create(context, conf);
        await request.agent.remove(task.tid);
        await task.stop();
        expect(false).assertTrue();
        done();
      } catch (err) {
        expect(err.code).assertEqual(21900007);
        expect(true).assertTrue();
        done();
      }
    });
    
    /**
     * @tc.number: testRemove001
     * @tc.name: testRemove001
     * @tc.desc: Test remove for parameter error
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testRemove001', 0, async (done: Function) => {
      try {
        await request.agent.remove(true as Object as 'true');
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testRemove002
     * @tc.name: testRemove002
     * @tc.desc: Test remove for parameter error
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testRemove002', 0, async (done: Function) => {
      try {
        await request.agent.remove("123");
      } catch (err) {
        expect(err.code).assertEqual(21900006);
        done();
      }
    });
    /**
     * @tc.number: testRemove003
     * @tc.name: testRemove003
     * @tc.desc: Test remove for promise
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testRemove003', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testRemove003.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      request.agent.remove(task.tid).then(() => {
        expect(true).assertTrue();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testRemove004
     * @tc.name: testRemove004
     * @tc.desc: Test remove for callback
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testRemove004', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testRemove004.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      request.agent.remove(task.tid, async (err: Error) => {
        expect(err === null).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testRemove005
     * @tc.name: testRemove005
     * @tc.desc: Test remove for callback
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testRemove005', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testRemove005.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      try {
        await request.agent.remove(task.tid);
        await request.agent.remove(task.tid);
      } catch (err) {
        expect(err.code).assertEqual(21900006);
        done();
      }
    });
    /**
     * @tc.number: testRemove006
     * @tc.name: testRemove006
     * @tc.desc: Test remove for promise FOREGROUND
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testRemove006', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testRemove006.txt',
        mode: request.agent.Mode.FOREGROUND
      };
      let task = await request.agent.create(context, conf);
      request.agent.remove(task.tid).then(() => {
        expect(true).assertTrue();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testRemove007
     * @tc.name: testRemove007
     * @tc.desc: Test remove a running task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testRemove007', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testRemove007.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      try {
        await task.start();
        await request.agent.remove(task.tid);
        expect(true).assertTrue();
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testRemove008
     * @tc.name: testRemove008
     * @tc.desc: Test remove a paused task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testRemove008', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testRemove008.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      try {
        await task.start();
        await task.pause();
        await request.agent.remove(task.tid);
        expect(true).assertTrue();
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testRemove009
     * @tc.name: testRemove009
     * @tc.desc: Test remove a stopped task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testRemove009', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testRemove009.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      try {
        await task.start();
        await task.stop();
        await request.agent.remove(task.tid);
        expect(true).assertTrue();
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testRedirect001
     * @tc.name: testRedirect001
     * @tc.desc: Test redirect enable
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testRedirect001', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testRedirect001.txt',
        mode: request.agent.Mode.BACKGROUND,
        redirect: true
      };
      let task = await request.agent.create(context, conf);
      task.on("completed", (progress) => {
        expect(true).assertTrue();
        done();
      });
      await task.start();
    });
    /**
     * @tc.number: testRedirect002
     * @tc.name: testRedirect002
     * @tc.desc: Test redirect disable
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testRedirect002', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testRedirect002.txt',
        mode: request.agent.Mode.BACKGROUND,
        redirect: false
      };
      let task = await request.agent.create(context, conf);
      task.on("failed", (progress) => {
        expect(true).assertTrue();
        done();
      });
      await task.start();
    });

    /**
     * @tc.number: testDownloadTaskUserFile001
     * @tc.name: testDownloadTaskUserFile001
     * @tc.desc: Test when download user file with wrong `Mode`
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadTaskUserFile001', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.BACKGROUND,
        saveas: 'file://media/Photo/1/test.txt'
      };
      try {
        await request.agent.create(context, conf).then(async () => {
          expect(false).assertTrue();
          done();
        }).catch((err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(true).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testDownloadTaskUserFile002
     * @tc.name: testDownloadTaskUserFile002
     * @tc.desc: Test when download user file without `overrite`
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadTaskUserFile002', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.FOREGROUND,
        saveas: 'file://media/Photo/1/test.txt',
        overwrite: false,
      };
      try {
        await request.agent.create(context, conf).then(async () => {
          expect(false).assertTrue();
          done();
        }).catch((err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(true).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testDownloadTaskUserFile003
     * @tc.name: testDownloadTaskUserFile003
     * @tc.desc: Test when download user file without `overrite`
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadTaskUserFile003', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.FOREGROUND,
        saveas: 'file://media/Photo/1/test.txt',
        overwrite: true,
      };
      try {
        await request.agent.create(context, conf).then(async () => {
          expect(false).assertTrue();
          done();
        }).catch((err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(true).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testUploadTaskUserFile001
     * @tc.name: testUploadTaskUserFile001
     * @tc.desc: Test when upload user file with wrong `Mode`
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadTaskUserFile001', 0, async (done: Function) => {
      let fileSpec: request.agent.FileSpec = {
        path: `file://media/Photo/1/test.img`
      };
      let formItem: request.agent.FormItem = {
        name: 'file',
        value: fileSpec
      };
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        mode: request.agent.Mode.BACKGROUND,
        data: [formItem],
      };
      try {
        await request.agent.create(context, conf).then(async () => {
          expect(false).assertTrue();
          done();
        }).catch((err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(true).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testGetTaskId001
     * @tc.name: testGetTaskId001
     * @tc.desc: Tests whether an exception is thrown when task_id is illegal
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testGetTaskId001', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: `test_get_task_id_001`,
      };
      try {
        await request.agent.getTask(context, "3424657889323469022694654311244554");
      } catch (err) {
        expect(err.code).assertEqual(21900006);
        done();
      }
    });
    /**
     * @tc.number: testGetTaskId002
     * @tc.name: testGetTaskId002
     * @tc.desc: Tests whether an exception is thrown when task_id is illegal
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testGetTaskId002', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: `test_get_task_id_002`,
      };
      try {
        await request.agent.getTask(context, "avbdfdf");
      } catch (err) {
        expect(err.code).assertEqual(21900006);
        done();
      }
    });
    /**
     * @tc.number: testGetTaskId003
     * @tc.name: testGetTaskId003
     * @tc.desc: Tests fot correct gettask
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testGetTaskId003', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: `test_get_task_id_003`,
      };
      let task = await request.agent.create(context, conf);
      try {
        await request.agent.getTask(context, task.tid);
        expect(true).assertTrue();
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testGetTaskId004
     * @tc.name: testGetTaskId004
     * @tc.desc: Tests whether an exception is thrown when lack task_id
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testGetTaskId004', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: `test_get_task_id_003`,
      };
      try {
        await request.agent.getTask(context, "");
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testGetTaskId005
     * @tc.name: testGetTaskId005
     * @tc.desc: Tests whether an exception is thrown when task_id is fake
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testGetTaskId005', 0, async (done: Function) => {
      let tid = '1'.repeat(33);
      try {
        await request.agent.getTask(undefined, tid);
      } catch (err) {
        expect(err.code).assertEqual(21900006);
        done();
      }
    });
    /**
     * @tc.number: testGetTaskId006
     * @tc.name: testGetTaskId006
     * @tc.desc: Tests whether an exception is thrown when task_id is fake and undefined
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testGetTaskId006', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: `test_get_task_id_004`,
      };
      try {
        await request.agent.getTask(undefined, "123123");
      } catch (err) {
        expect(err.code).assertEqual(21900006);
        done();
      }
    });
    /**
     * @tc.number: testGetTaskId007
     * @tc.name: testGetTaskId007
     * @tc.desc: Tests whether an exception is thrown when task_id is fake and undefined
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testGetTaskId007', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: `test_get_task_id_007`,
      };
      let task = await request.agent.create(context, conf);
      try {
        await request.agent.getTask(undefined, task.tid);
        expect(true).assertTrue();
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testTaskBroadcastEvent001
     * @tc.name: testTaskBroadcastEvent001
     * @tc.desc: Test downloadtTask for broadcast event when success
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskBroadcastEvent001', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskBroadcastEvent001.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      try {
        await task.start();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
      let subscriberCallback = (err: BusinessError, data: commonEvent.CommonEventData) => {
        try {
          commonEvent.unsubscribe(subscriber, unSubscriberCallback);
          expect(data.code).assertEqual(64);
          done();
        } catch (err) {
          expect(false).assertTrue();
          done();
        }
      };
      let unSubscriberCallback = (err: BusinessError) => {
        if (err) {
          expect(false).assertTrue();
          done();
        }
      };
      let commonEventSubscribeInfo: commonEvent.CommonEventSubscribeInfo = {
        events: [request.agent.BroadcastEvent.COMPLETE]
      };
      let subscriber: commonEvent.CommonEventSubscriber = await commonEvent.createSubscriber(commonEventSubscribeInfo);
      await commonEvent.subscribe(subscriber, subscriberCallback);
    });
    /**
     * @tc.number: testTaskBroadcastEvent002
     * @tc.name: testTaskBroadcastEvent002
     * @tc.desc: Test downloadtTask for broadcast event when failed
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskBroadcastEvent002', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.mp3',
        saveas: 'testTaskBroadcastEvent002.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      try {
        await task.start();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
      let subscriberCallback = (err: BusinessError, data: commonEvent.CommonEventData) => {
        try {
          commonEvent.unsubscribe(subscriber, unSubscriberCallback);
          expect(data.code).assertEqual(65);
          done();
        } catch (err) {
          expect(false).assertTrue();
          done();
        }
      };
      let unSubscriberCallback = (err: BusinessError) => {
        if (err) {
          expect(false).assertTrue();
          done();
        }
      };
      let commonEventSubscribeInfo: commonEvent.CommonEventSubscribeInfo = {
        events: [request.agent.BroadcastEvent.COMPLETE]
      };
      let subscriber: commonEvent.CommonEventSubscriber = await commonEvent.createSubscriber(commonEventSubscribeInfo);
      await commonEvent.subscribe(subscriber, subscriberCallback);
    });
  })
}