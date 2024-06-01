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

// @ts-nocheck
import { describe, beforeAll, beforeEach, afterEach, afterAll, it, expect } from "@ohos/hypium";
import request from '@ohos.request';
import fs from '@ohos.file.fs';

export default function requestOperateTaskTest() {
  describe('requestOperateTaskTest', function () {
    beforeAll(function () {
      console.info('beforeAll called');
    })

    afterAll(function () {
      console.info('afterAll called');
    })

    beforeEach(function () {
      console.info('beforeEach called');
    })

    afterEach(async function () {
      console.info('afterEach called');
      if (fs.accessSync(cacheDir + '/test.txt')) {
        fs.unlinkSync(cacheDir + '/test.txt');
      }
      if (fs.accessSync(cacheDir + '/README.md')) {
        fs.unlinkSync(cacheDir + '/README.md');
      }
    })

    function sleep(ms) {
      return new Promise(resolve => setTimeout(resolve, ms));
    }

    let context = globalThis.abilityContext;
    let cacheDir = globalThis.cacheDir;
    let fileSpec = {
      path: `./test.txt`
    }
    let formItem = [{
      name: 'file',
      value: [ fileSpec ]
    }]

    /**
     * @tc.number: testStartTask001
     * @tc.name: testStartTask001
     * @tc.desc: Test start FOREGROUND task for callback
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testStartTask001', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testStartTask001.txt',
        mode: request.agent.Mode.FOREGROUND
      }
      let task = await request.agent.create(context, conf);
      await task.start(async (err) => {
        expect(err === null).assertTrue();
        await task.stop();
        done()
      })
    })

    /**
     * @tc.number: testStartTask002
     * @tc.name: testStartTask002
     * @tc.desc: Test start FOREGROUND task for promise
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testStartTask002', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testStartTask002.txt',
        mode: request.agent.Mode.FOREGROUND
      }
      let task = await request.agent.create(context, conf);
      await task.start().then(async () => {
        expect(true).assertTrue();
        await task.stop();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testStartTask003
     * @tc.name: testStartTask003
     * @tc.desc: Test start background task for callback
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testStartTask003', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testStartTask003.txt',
        mode: request.agent.Mode.BACKGROUND
      }
      let task = await request.agent.create(context, conf);
      await task.start(async (err) => {
        expect(err === null).assertTrue();
        await task.stop();
        done();
      })
    })

    /**
     * @tc.number: testStartTask004
     * @tc.name: testStartTask004
     * @tc.desc: Test start background task for promise
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testStartTask004', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testStartTask004.txt',
        mode: request.agent.Mode.BACKGROUND
      }
      let task = await request.agent.create(context, conf)
      await task.start().then(async () => {
        expect(true).assertTrue();
        await task.stop();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testStartTask005
     * @tc.name: testStartTask005
     * @tc.desc: Test start pause start background task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testStartTask005', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testStartTask005.txt',
        mode: request.agent.Mode.BACKGROUND
      }
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
    })

    /**
     * @tc.number: testStartTask006
     * @tc.name: testStartTask006
     * @tc.desc: Test start stop start background task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testStartTask006', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testStartTask006.txt',
        mode: request.agent.Mode.BACKGROUND
      }
      let task = await request.agent.create(context, conf);
      try {
        await task.start();
        await task.stop();
        await task.pause();
      } catch (err) {
        expect(err.code).assertEqual(21900007);
        done();
      }
    })

    /**
     * @tc.number: testStartTask007
     * @tc.name: testStartTask007
     * @tc.desc: Test start stop start FOREGROUND
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testStartTask007', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testStartTask007.txt',
        mode: request.agent.Mode.FOREGROUND
      }
      let task = await request.agent.create(context, conf);
      try {
        await task.start();
        await task.stop();
        await task.pause();
      } catch (err) {
        expect(err.code).assertEqual(21900007);
        done();
      }
    })

    /**
     * @tc.number: testStartTask008
     * @tc.name: testStartTask008
     * @tc.desc: Test start two FOREGROUND task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testStartTask008', 0, async function (done) {
      try {
        let conf = {
          action: request.agent.Action.DOWNLOAD,
          url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/large_test.zip',
          saveas: `test.txt`,
          mode: request.agent.Mode.FOREGROUND
        }
        let task = await request.agent.create(context, conf);
        await task.start();
        await sleep(2000);

        let conf1 = {
          action: request.agent.Action.DOWNLOAD,
          url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
          saveas: `README.md`,
          mode: request.agent.Mode.FOREGROUND
        }
        request.agent.create(context, conf1, async (err, task1) => {
          expect(true).assertTrue();
          await request.agent.remove(task1.tid);
        })
        await sleep(1000);
        task.on('progress', async function(progress) {
          if (progress.state === request.agent.State.RUNNING) {
            expect(true).assertTrue();
            task.off('progress');
            await request.agent.remove(task.tid);
            done();
          } else {
            expect(false).assertTrue();
            done();
          }
        })

      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    })

    /**
     * @tc.number: testPauseTask001
     * @tc.name: testPauseTask001
     * @tc.desc: Test pause FOREGROUND task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testPauseTask001', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testPauseTask001.txt',
        mode: request.agent.Mode.FOREGROUND
      }
      let task = await request.agent.create(context, conf);
      await task.start();
      await task.pause(async (err) => {
        expect(err === null).assertTrue();
        await request.agent.remove(task.tid);
        done();
      })
    })

    /**
     * @tc.number: testPauseTask002
     * @tc.name: testPauseTask002
     * @tc.desc: Test pause stop background task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testPauseTask002', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testPauseTask002.txt',
        mode: request.agent.Mode.BACKGROUND
      }
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
    })

    /**
     * @tc.number: testPauseTask003
     * @tc.name: testPauseTask003
     * @tc.desc: Test pause background task for callback
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testPauseTask003', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testPauseTask003.txt',
        mode: request.agent.Mode.BACKGROUND
      }
      let task = await request.agent.create(context, conf);
      await task.start();
      await task.pause(async (err) => {
        expect(err === null).assertTrue()
        await request.agent.remove(task.tid);
        done()
      })
    })

    /**
     * @tc.number: testPauseTask004
     * @tc.name: testPauseTask004
     * @tc.desc: Test pause background task for promise
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testPauseTask004', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testPauseTask004.txt',
        mode: request.agent.Mode.BACKGROUND
      }
      let task = await request.agent.create(context, conf);
      await task.start();
      await task.pause().then(async () => {
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testPauseTask005
     * @tc.name: testPauseTask005
     * @tc.desc: Test pause resume pause background task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testPauseTask005', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/large_test.zip',
        saveas: 'testPauseTask005.txt',
        mode: request.agent.Mode.BACKGROUND
      }
      let task = await request.agent.create(context, conf);
      await task.start();
      await task.pause();
      await task.resume().then(() => {
        expect(true).assertTrue();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
      await task.pause().then(async () => {
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testResumeTask001
     * @tc.name: testResumeTask001
     * @tc.desc: Test start resume FOREGROUND task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testResumeTask001', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testResumeTask001.txt',
        mode: request.agent.Mode.FOREGROUND
      }
      let task = await request.agent.create(context, conf);
      try {
        await task.resume();
      } catch (err) {
        expect(err.code).assertEqual(21900007);
        await request.agent.remove(task.tid);
        done();
      }
    })

    /**
     * @tc.number: testResumeTask002
     * @tc.name: testResumeTask002
     * @tc.desc: Test pause resume background task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testResumeTask002', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testResumeTask002.txt',
        mode: request.agent.Mode.BACKGROUND
      }

      let task = await request.agent.create(context, conf);
      await task.start();
      await task.pause();
      await task.resume(async (err) => {
        expect(err === null).assertTrue();
        await request.agent.remove(task.tid);
        done();
      })
    })

    /**
     * @tc.number: testResumeTask003
     * @tc.name: testResumeTask003
     * @tc.desc: Test stop resume background task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testResumeTask003', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testResumeTask003.txt',
        mode: request.agent.Mode.BACKGROUND
      }
      let task = await request.agent.create(context, conf);
      try {
        await task.start();
        await task.stop().then(() => {
          expect(true).assertTrue();
          done();
        }).catch((err) => {
          expect(false).assertTrue();
          done();
        })
        await task.resume();
      } catch (err) {
        expect(err.code).assertEqual(21900007);
        done();
      }
    })

    /**
     * @tc.number: testResumeTask004
     * @tc.name: testResumeTask004
     * @tc.desc: Test start resume background task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testResumeTask004', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testResumeTask004.txt',
        mode: request.agent.Mode.BACKGROUND
      }
      let task = await request.agent.create(context, conf);
      try {
        await task.start();
        await task.resume();
      } catch (err) {
        expect(err.code).assertEqual(21900007);
        await request.agent.remove(task.tid);
        done();
      }
    })

    /**
     * @tc.number: testResumeTask005
     * @tc.name: testResumeTask005
     * @tc.desc: Test pause resume FOREGROUND task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testResumeTask005', 0, async function (done) {
      try {
        let conf = {
          action: request.agent.Action.DOWNLOAD,
          url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/large_test.zip',
          saveas: 'testResumeTask005.txt',
          mode: request.agent.Mode.FOREGROUND
        }
        let task = await request.agent.create(context, conf);
        await task.start();
        await task.pause();
        await task.resume(async (err) => {
          expect(err === null).assertTrue();
          await request.agent.remove(task.tid);
          done();
        })
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    })

    /**
     * @tc.number: testResumeTask006
     * @tc.name: testResumeTask006
     * @tc.desc: Test stop resume BACKGROUND task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testResumeTask006', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/large_test.zip',
        saveas: 'testResumeTask006.txt',
        mode: request.agent.Mode.BACKGROUND
      }
      let task = await request.agent.create(context, conf);
      try {
        await task.start();
        await task.stop().then(() => {
          expect(true).assertTrue();
        }).catch((err) => {
          expect(false).assertTrue();
          done();
        })
        await task.resume();
      } catch (err) {
        expect(err.code).assertEqual(21900007);
        done();
      }
    })

    /**
     * @tc.number: testStopTask001
     * @tc.name: testStopTask001
     * @tc.desc: Test stop FOREGROUND task for callback
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testStopTask001', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testStopTask001.txt',
        mode: request.agent.Mode.FOREGROUND
      }
      let task = await request.agent.create(context, conf);
      await task.start();
      await task.stop((err) => {
        expect(err === null).assertTrue()
        done()
      })
    })

    /**
     * @tc.number: testStopTask002
     * @tc.name: testStopTask002
     * @tc.desc: Test stop FOREGROUND task for promise
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testStopTask002', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testStopTask002.txt',
        mode: request.agent.Mode.FOREGROUND
      }
      let task = await request.agent.create(context, conf);
      await task.start();
      await task.stop().then(() => {
        expect(true).assertTrue();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testStopTask003
     * @tc.name: testStopTask003
     * @tc.desc: Test stop background task for callback
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testStopTask003', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testStopTask003.txt',
        mode: request.agent.Mode.BACKGROUND
      }
      let task = await request.agent.create(context, conf);
      await task.start();
      await task.stop((err) => {
        expect(err === null).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testStopTask004
     * @tc.name: testStopTask004
     * @tc.desc: Test stop background task for promise
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testStopTask004', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testStopTask004.txt',
        mode: request.agent.Mode.BACKGROUND
      }
      let task = await request.agent.create(context, conf);
      await task.start();
      await task.stop().then(() => {
        expect(true).assertTrue();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testStopTask005
     * @tc.name: testStopTask005
     * @tc.desc: Test stop pause background task for promise
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testStopTask005', 0, async function (done) {
      await sleep(2000);
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testStopTask005.txt',
        mode: request.agent.Mode.BACKGROUND
      }
      try{
        let task = await request.agent.create(context, conf);
        await task.start();
        await task.stop();
        await task.pause();
      } catch (err) {
        expect(err.code).assertEqual(21900007);
        done();
      }
    })

    /**
     * @tc.number: testStopTask006
     * @tc.name: testStopTask006
     * @tc.desc: Test stop pause background task for promise
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testStopTask006', 0, async function (done) {
      await sleep(2000);
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/large_test.zip',
        saveas: 'testStopTask006.txt',
        mode: request.agent.Mode.BACKGROUND
      }
      try{
        let task = await request.agent.create(context, conf);
        await task.start();
        await task.stop();
        await task.pause();
      } catch (err) {
        expect(err.code).assertEqual(21900007);
        done();
      }
    })

    /**
     * @tc.number: testOnTask001
     * @tc.name: testOnTask001
     * @tc.desc: Test on task for 'test'
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testOnTask001', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testOnTask001.txt',
        mode: request.agent.Mode.BACKGROUND
      }
      let task = await request.agent.create(context, conf);
      try {
        task.on('test', function (progress) {})
      } catch (err) {
        expect(err.code).assertEqual(401);
        await request.agent.remove(task.tid);
        done();
      }
    })

    /**
     * @tc.number: testOnTask002
     * @tc.name: testOnTask002
     * @tc.desc: Test on task for number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testOnTask002', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testOnTask002.txt',
        mode: request.agent.Mode.BACKGROUND
      }
      let task = await request.agent.create(context, conf);
      try {
        task.on(123, function (progress) {})
      } catch (err) {
        expect(err.code).assertEqual(401);
        await request.agent.remove(task.tid);
        done();
      }
    })

    /**
     * @tc.number: testOnTask003
     * @tc.name: testOnTask003
     * @tc.desc: Test on task for failed
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testOnTask003', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test1.txt',
        saveas: 'testOnTask003.txt',
        mode: request.agent.Mode.BACKGROUND
      }
      let task = await request.agent.create(context, conf);
      task.on('failed', function (progress) {
        expect(progress.state).assertEqual(request.agent.State.FAILED);
        done();
      })
      await task.start();
    })

    /**
     * @tc.number: testOnTask004
     * @tc.name: testOnTask004
     * @tc.desc: Test on task for completed
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testOnTask004', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testOnTask004.txt',
        mode: request.agent.Mode.BACKGROUND
      }
      let task = await request.agent.create(context, conf);
      task.on('progress', function (progress) {
        expect(progress.state).assertEqual(request.agent.State.RUNNING);
        task.off('progress');
        done();
      })
      task.on('completed', function (progress) {
        expect(progress.state).assertEqual(request.agent.State.COMPLETED);
        done();
      })
      await task.start();
    })

    /**
     * @tc.number: testOnTask005
     * @tc.name: testOnTask005
     * @tc.desc: Test on task background task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testOnTask005', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testOnTask005.txt',
        mode: request.agent.Mode.BACKGROUND
      }
      let task = await request.agent.create(context, conf);
      task.on('progress', function (progress) {
        expect(progress.state).assertEqual(request.agent.State.RUNNING);
        task.off('progress');
        done();
      })
      task.on('completed', function (progress) {
        expect(progress.state).assertEqual(request.agent.State.COMPLETED);
        done();
      })
      await task.start();
    })

    /**
     * @tc.number: testOnTask006
     * @tc.name: testOnTask006
     * @tc.desc: Test on task for progress state REMOVED
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testOnTask006', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/large_test.zip',
        saveas: 'testOnTask006.txt',
        mode: request.agent.Mode.BACKGROUND
      }
      let task = await request.agent.create(context, conf);
      task.on('remove', function (progress) {
        expect(progress.state).assertEqual(request.agent.State.REMOVED);
        done();
      })
      await task.start();
      await sleep(2000);
      await request.agent.remove(task.tid);
    })

    /**
     * @tc.number: testOnTask007
     * @tc.name: testOnTask007
     * @tc.desc: Test on task for progress twice
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testOnTask007', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testOnTask007.txt',
        mode: request.agent.Mode.BACKGROUND
      }
      let task = await request.agent.create(context, conf);
      try {
        task.on('progress', function (progress) {});
        task.on('progress', function (progress) {});
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    })

    /**
     * @tc.number: testOffTask001
     * @tc.name: testOffTask001
     * @tc.desc: Test off task for 'test'
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testOffTask001', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testOffTask001.txt',
        mode: request.agent.Mode.BACKGROUND
      }
      let task = await request.agent.create(context, conf);
      try {
        task.off('test', function() {});
      } catch (err) {
        expect(err.code).assertEqual(401);
        await request.agent.remove(task.tid);
        done();
      }
    })

    /**
     * @tc.number: testOffTask002
     * @tc.name: testOffTask002
     * @tc.desc: Test off task for number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testOffTask002', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testOffTask002.txt',
        mode: request.agent.Mode.BACKGROUND
      }
      let task = await request.agent.create(context, conf);
      try {
        task.off(123);
      } catch (err) {
        expect(err.code).assertEqual(401);
        await request.agent.remove(task.tid);
        done();
      }
    })

    /**
     * @tc.number: testOffTask003
     * @tc.name: testOffTask003
     * @tc.desc: Test off task for failed
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testOffTask003', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test1.txt',
        saveas: 'testOffTask003.txt',
        mode: request.agent.Mode.BACKGROUND
      }
      let task = await request.agent.create(context, conf);
      task.on('failed', function() {
        expect(false).assertTrue();
        done();
      })
      task.off('failed');
      await task.start();
      await sleep(2000);
      expect(true).assertTrue();
      done();
    })

    /**
     * @tc.number: testOffTask004
     * @tc.name: testOffTask004
     * @tc.desc: Test off task for completed
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testOffTask004', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testOffTask004.txt',
        mode: request.agent.Mode.BACKGROUND
      }
      let task = await request.agent.create(context, conf);
      task.on('completed', function (progress) {
        expect(true).assertTrue();
        done();
      })
      function CompletedCallback(progress) {
        expect(false).assertTrue();
        done();
      }
      task.on('completed', CompletedCallback);
      task.off('completed', CompletedCallback);
      await task.start();
    })

    /**
     * @tc.number: testOffTask005
     * @tc.name: testOffTask005
     * @tc.desc: Test off task for progress
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testOffTask005', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testOffTask005.txt',
        mode: request.agent.Mode.BACKGROUND
      }
      let task = await request.agent.create(context, conf);
      task.on('progress', function (progress) {
        expect(true).assertTrue();
        done();
      })
      task.on('completed', function (progress) {
        expect(true).assertTrue();
        done();
      })
      function ProgressCallback(progress) {
        expect(false).assertTrue();
        done();
      }
      task.on('progress', ProgressCallback);
      task.off('progress', ProgressCallback);
      await task.start();
    })

    /**
     * @tc.number: testOffTask006
     * @tc.name: testOffTask006
     * @tc.desc: Test off task twice
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testOffTask006', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testOffTask006.txt',
        mode: request.agent.Mode.BACKGROUND
      }
      let task = await request.agent.create(context, conf);
      try {
        task.off('completed');
        task.off('completed');
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    })

    /**
     * @tc.number: testRemove001
     * @tc.name: testRemove001
     * @tc.desc: Test remove for parameter error
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testRemove001', 0, async function (done) {
      try {
        await request.agent.remove(true);
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.number: testRemove002
     * @tc.name: testRemove002
     * @tc.desc: Test remove for parameter error
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testRemove002', 0, async function (done) {
      try {
        await request.agent.remove("123");
      } catch (err) {
        expect(err.code).assertEqual(21900006);
        done();
      }
    })

    /**
     * @tc.number: testRemove003
     * @tc.name: testRemove003
     * @tc.desc: Test remove for promise
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testRemove003', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testRemove003.txt',
        mode: request.agent.Mode.BACKGROUND
      }
      let task = await request.agent.create(context, conf);
      request.agent.remove(task.tid).then(() => {
        expect(true).assertTrue();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testRemove004
     * @tc.name: testRemove004
     * @tc.desc: Test remove for callback
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testRemove004', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testRemove004.txt',
        mode: request.agent.Mode.BACKGROUND
      }
      let task = await request.agent.create(context, conf);
      request.agent.remove(task.tid, (err) => {
        expect(err === null).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testRemove005
     * @tc.name: testRemove005
     * @tc.desc: Test remove for callback
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testRemove005', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testRemove005.txt',
        mode: request.agent.Mode.BACKGROUND
      }
      let task = await request.agent.create(context, conf);
      try {
        await request.agent.remove(task.tid);
        await request.agent.remove(task.tid);
      } catch (err) {
        expect(err.code).assertEqual(21900006);
        done();
      }
    })

    /**
     * @tc.number: testRedirect001
     * @tc.name: testRedirect001
     * @tc.desc: Test redirect enable
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testRedirect001', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testRedirect001.txt',
        mode: request.agent.Mode.BACKGROUND,
        redirect: true
      }
      let task = await request.agent.create(context, conf);
      task.on("completed", (progress) => {
        expect(true).assertTrue();
        done();
      })
      await task.start();
    })

    /**
     * @tc.number: testRedirect002
     * @tc.name: testRedirect002
     * @tc.desc: Test redirect disable
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testRedirect002', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testRedirect002.txt',
        mode: request.agent.Mode.BACKGROUND,
        redirect: false
      }
      let task = await request.agent.create(context, conf);
      task.on("failed", (progress) => {
        expect(true).assertTrue();
        done();
      })
      await task.start();
    })

    /**
     * @tc.number: testOnResponse001
     * @tc.name: testOnResponse001
     * @tc.desc: Test on response
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testOnResponse001', 0, async function (done) {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testOnResponse001.txt',
        mode: request.agent.Mode.BACKGROUND,
        overwrite: true,
      }
      let task = await request.agent.create(context, conf);
      task.on('response', (response: request.agent.HttpResponse) => {
        expect(response.statusCode).assertEqual(200);
		    expect(response.version).assertEqual("HTTP/1.1");
		    expect(response.reason).assertEqual("OK");
        done();
      })
      await task.start();
    })

    /**
     * @tc.number: testOnResponse002
     * @tc.name: testOnResponse002
     * @tc.desc: Test off response
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testOnResponse002', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testOnResponse002.txt',
        mode: request.agent.Mode.BACKGROUND,
        overwrite: true,
      }
	    let callback = (response: request.agent.HttpResponse) => {
        expect(false).assertTrue();
        done();
      };
      let task = await request.agent.create(context, conf);
      task.on('response', callback)
	    task.off('response', callback);
      task.on('completed', function (progress) {
        expect(progress.state).assertEqual(request.agent.State.COMPLETED);
        done();
      })
      await task.start();
    })

    /**
     * @tc.number: testOnResponse003
     * @tc.name: testOnResponse003
     * @tc.desc: Test off all response
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testOnResponse003', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testOnResponse003.txt',
        mode: request.agent.Mode.BACKGROUND,
        overwrite: true,
      }
      let task = await request.agent.create(context, conf);
      task.on('response', function (response: request.agent.HttpResponse) {
        expect(false).assertTrue();
        done();
      })
	    task.on('response', function (response: request.agent.HttpResponse) {
		    expect(response.statusCode).assertEqual(200);
        expect(false).assertTrue();
        done();
      })
	    task.off('response');
      task.on('completed', function (progress) {
        expect(progress.state).assertEqual(request.agent.State.COMPLETED);
        done();
      })
      await task.start();
    })

    /**
     * @tc.number: testOnResponse004
     * @tc.name: testOnResponse004
     * @tc.desc: Test off with no response
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testOnResponse004', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testOnResponse004.txt',
        mode: request.agent.Mode.BACKGROUND,
        overwrite: true,
      }
      let task = await request.agent.create(context, conf);
	    task.off('response');
      task.on('completed', function (progress) {
        expect(progress.state).assertEqual(request.agent.State.COMPLETED);
        done();
      })
      await task.start();
    })

    /**
     * @tc.number: testOnResponse005
     * @tc.name: testOnResponse005
     * @tc.desc: Test on response with a bad request
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testOnResponse005', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.jpg',
        saveas: 'testOnResponse005.txt',
        mode: request.agent.Mode.BACKGROUND,
        overwrite: true,
      }
      let task = await request.agent.create(context, conf);
      task.on('response', function (response: request.agent.HttpResponse) {
        expect(response.statusCode).assertEqual(404);
		    expect(response.version).assertEqual("HTTP/1.1");
		    expect(response.reason).assertEqual("Not Found");
        task.off('response');
        done();
      })
      await task.start();
    })

    /**
     * @tc.number: testDownloadTaskSaveas001
     * @tc.name: testDownloadTaskSaveas001
     * @tc.desc: Test when saveas is undefined
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadTaskSaveas001', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testDownloadTaskSaveas001.txt',
        mode: request.agent.Mode.BACKGROUND
      }
      await request.agent.create(context, conf).then(async () => {
        expect(true).assertTrue();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testDownloadTaskSaveas002
     * @tc.name: testDownloadTaskSaveas002
     * @tc.desc: Test when saveas is ./
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadTaskSaveas002', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.BACKGROUND,
        saveas: './'
      }
      await request.agent.create(context, conf).then(async () => {
        expect(true).assertTrue();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testDownloadTaskSaveas003
     * @tc.name: testDownloadTaskSaveas003
     * @tc.desc: Test when saveas is ./filename
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadTaskSaveas003', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.BACKGROUND,
        saveas: './test_saveas_003'
      }
      await request.agent.create(context, conf).then(async () => {
        expect(true).assertTrue();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testDownloadTaskSaveas004
     * @tc.name: testDownloadTaskSaveas004
     * @tc.desc: Test when saveas is filename
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadTaskSaveas004', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.BACKGROUND,
        saveas: 'test_saveas_004'
      }
      await request.agent.create(context, conf).then(async () => {
        expect(true).assertTrue();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testDownloadTaskSaveas005
     * @tc.name: testDownloadTaskSaveas005
     * @tc.desc: Test when saveas is pathname/filename
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadTaskSaveas005', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.BACKGROUND,
        saveas: 'a/b/test_saveas_005'
      }
      await request.agent.create(context, conf).then(async () => {
        expect(true).assertTrue();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testDownloadTaskSaveas006
     * @tc.name: testDownloadTaskSaveas006
     * @tc.desc: Test when saveas is internal://cache/filename
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadTaskSaveas006', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.BACKGROUND,
        saveas: 'internal://cache/test_saveas_006'
      }
      await request.agent.create(context, conf).then(async () => {
        expect(true).assertTrue();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testDownloadTaskSaveas007
     * @tc.name: testDownloadTaskSaveas007
     * @tc.desc: Test when saveas is whole path in base
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadTaskSaveas007', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.BACKGROUND,
        saveas: '/data/storage/el1/base/test_saveas_007'
      }
      await request.agent.create(context, conf).then(async () => {
        expect(true).assertTrue();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testDownloadTaskSaveas008
     * @tc.name: testDownloadTaskSaveas008
     * @tc.desc: Test when saveas is whole path but not in base
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadTaskSaveas008', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.BACKGROUND,
        saveas: '/data/storage/test_saveas_008'
      }
      try {
        await request.agent.create(context, conf).then(async () => {
          expect(false).assertTrue();
          done();
        }).catch((err) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(true).assertTrue();
        done();
      }
    })

    /**
     * @tc.number: testDownloadTaskSaveas009
     * @tc.name: testDownloadTaskSaveas009
     * @tc.desc: Test when saveas is whole path which contains ..
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadTaskSaveas009', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.BACKGROUND,
        saveas: '/data/storage/../../data/storage/el1/../el2/base/test_saveas_009'
      }
      await request.agent.create(context, conf).then(async () => {
        expect(true).assertTrue();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testDownloadTaskSaveas010
     * @tc.name: testDownloadTaskSaveas010
     * @tc.desc: Test when saveas is whole path which contains error ..
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadTaskSaveas010', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.BACKGROUND,
        saveas: '/data/storage/../../el2/base/test_saveas_010'
      }
      try {
        await request.agent.create(context, conf).then(async () => {
          expect(false).assertTrue();
          done();
        }).catch((err) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(true).assertTrue();
        done();
      }
    })

    /**
     * @tc.number: testDownloadTaskSaveas011
     * @tc.name: testDownloadTaskSaveas011
     * @tc.desc: Test when saveas is file:// with correct package name
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadTaskSaveas011', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.BACKGROUND,
        saveas: 'file://com.acts.request/data/storage/el2/base/test_saveas_011'
      }
      await request.agent.create(context, conf).then(async () => {
        expect(true).assertTrue();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testDownloadTaskSaveas012
     * @tc.name: testDownloadTaskSaveas012
     * @tc.desc: Test when saveas is file:// with error package name
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadTaskSaveas012', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.BACKGROUND,
        saveas: 'file://com.acts.request1111/data/storage/el2/base/test_saveas_012'
      }
      try {
        await request.agent.create(context, conf).then(async () => {
          expect(false).assertTrue();
          done();
        }).catch((err) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(true).assertTrue();
        done();
      }
    })

    /**
     * @tc.number: testDownloadTaskSaveas013
     * @tc.name: testDownloadTaskSaveas013
     * @tc.desc: Test when saveas is internal://files/filename
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadTaskSaveas013', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.BACKGROUND,
        saveas: 'internal://files/path_save_as_013/test_saveas_013'
      }
      await request.agent.create(context, conf).then(async () => {
        expect(true).assertTrue();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testUploadTaskUserFile001
     * @tc.name: testUploadTaskUserFile001
     * @tc.desc: Test when upload user file with wrong `Mode`
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadTaskUserFile001', 0, async function (done) {
      let fileSpec = {
        path: `file://media/Photo/1/test.img`
      }
      let formItem = {
        name: 'file',
        value: fileSpec
      }
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        mode: request.agent.Mode.BACKGROUND,
        data: [formItem],
      }
      try {
        await request.agent.create(context, conf).then(async () => {
          expect(false).assertTrue();
          done();
        }).catch((err) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(true).assertTrue();
        done();
      }
    })

    /**
     * @tc.number: testUploadTaskPath001
     * @tc.name: testUploadTaskPath001
     * @tc.desc: Test when path is ./filename
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadTaskPath001', 0, async function (done) {
      let context = globalThis.abilityContext;
      let dir = context.cacheDir;
      let file = fs.openSync(dir + '/upload_001', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
                 name: 'file',
                 value: {
                   path: `./upload_001`
                 }
               }]
      }
      try {
        await request.agent.create(context, conf).then(async () => {
          if (fs.accessSync(dir + '/upload_001')) {
            fs.unlinkSync(dir + '/upload_001');
          }
          expect(true).assertTrue();
          done();
        }).catch((err) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    })

    /**
     * @tc.number: testUploadTaskPath002
     * @tc.name: testUploadTaskPath002
     * @tc.desc: Test when path is filename
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadTaskPath002', 0, async function (done) {
      let context = globalThis.abilityContext;
      let dir = context.cacheDir;
      let file = fs.openSync(dir + '/upload_002', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
                 name: 'file',
                 value: {
                   path: `./upload_002`
                 }
               }]
      }
      try {
        await request.agent.create(context, conf).then(async () => {
          if (fs.accessSync(dir + '/upload_002')) {
            fs.unlinkSync(dir + '/upload_002');
          }
          expect(true).assertTrue();
          done();
        }).catch((err) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    })

    /**
     * @tc.number: testUploadTaskPath003
     * @tc.name: testUploadTaskPath003
     * @tc.desc: Test when path is internal://cache/filename
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadTaskPath003', 0, async function (done) {
      let context = globalThis.abilityContext;
      let dir = context.cacheDir;
      let file = fs.openSync(dir + '/upload_003', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
                 name: 'file',
                 value: {
                   path: `internal://cache/upload_003`
                 }
               }]
      }
      try {
        await request.agent.create(context, conf).then(async () => {
          if (fs.accessSync(dir + '/upload_003')) {
            fs.unlinkSync(dir + '/upload_003');
          }
          expect(true).assertTrue();
          done();
        }).catch((err) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    })

    /**
     * @tc.number: testUploadTaskPath004
     * @tc.name: testUploadTaskPath004
     * @tc.desc: Test when path is internal://files/filename
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadTaskPath004', 0, async function (done) {
      let context = globalThis.abilityContext;
      let dir = context.filesDir;
      let file = fs.openSync(dir + '/upload_004', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
                 name: 'file',
                 value: {
                   path: `internal://files/upload_004`
                 }
               }]
      }
      try {
        await request.agent.create(context, conf).then(async () => {
          if (fs.accessSync(dir + '/upload_004')) {
            fs.unlinkSync(dir + '/upload_004');
          }
          expect(true).assertTrue();
          done();
        }).catch((err) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    })

    /**
     * @tc.number: testUploadTaskPath005
     * @tc.name: testUploadTaskPath005
     * @tc.desc: Test when path is whole path in base
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadTaskPath005', 0, async function (done) {
      let context = globalThis.abilityContext;
      let dir = context.cacheDir;
      let file = fs.openSync(dir + '/upload_005', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
                 name: 'file',
                 value: {
                   path: dir + `/upload_005`
                 }
               }]
      }
      try {
        await request.agent.create(context, conf).then(async () => {
          if (fs.accessSync(dir + '/upload_005')) {
            fs.unlinkSync(dir + '/upload_005');
          }
          expect(true).assertTrue();
          done();
        }).catch((err) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    })

    /**
     * @tc.number: testUploadTaskPath006
     * @tc.name: testUploadTaskPath006
     * @tc.desc: Test when path is whole path but not in base
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadTaskPath006', 0, async function (done) {
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
                 name: 'file',
                 value: {
                   path: `/data/storage/upload_006`
                 }
               }]
      }
      try {
        await request.agent.create(context, conf).then(async () => {
          expect(false).assertTrue();
          done();
        }).catch((err) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(true).assertTrue();
        done();
      }
    })

    /**
     * @tc.number: testUploadTaskPath007
     * @tc.name: testUploadTaskPath007
     * @tc.desc: Test when path is whole path which contains ..
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadTaskPath007', 0, async function (done) {
      let context = globalThis.abilityContext.getApplicationContext();
      let dir = context.cacheDir;
      let file = fs.openSync(dir + '/upload_007', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
                 name: 'file',
                 value: {
                   path: '/data/storage/../../data/storage/el1/../el2/base/cache/upload_007'
                 }
               }]
      }
      try {
        await request.agent.create(context, conf).then(async () => {
          if (fs.accessSync(dir + '/upload_007')) {
            fs.unlinkSync(dir + '/upload_007');
          }
          expect(true).assertTrue();
          done();
        }).catch((err) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    })

    /**
     * @tc.number: testUploadTaskPath008
     * @tc.name: testUploadTaskPath008
     * @tc.desc: Test when path is whole path which contains error ..
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadTaskPath008', 0, async function (done) {
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
                 name: 'file',
                 value: {
                   path: '/data/storage/../../el2/base/upload_008'
                 }
               }]
      }
      try {
        await request.agent.create(context, conf).then(async () => {
          expect(false).assertTrue();
          done();
        }).catch((err) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(true).assertTrue();
        done();
      }
    })

    /**
     * @tc.number: testUploadTaskPath009
     * @tc.name: testUploadTaskPath009
     * @tc.desc: Test when path is file:// with correct package name
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadTaskPath009', 0, async function (done) {
      let context = globalThis.abilityContext.getApplicationContext();
      let dir = context.cacheDir;
      let file = fs.openSync(dir + '/upload_009', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
                 name: 'file',
                 value: {
                   path: 'file://com.acts.request/data/storage/el2/base/cache/upload_009'
                 }
               }]
      }
      try {
        await request.agent.create(context, conf).then(async () => {
          if (fs.accessSync(dir + '/upload_009')) {
            fs.unlinkSync(dir + '/upload_009');
          }
          expect(true).assertTrue();
          done();
        }).catch((err) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    })

    /**
     * @tc.number: testUploadTaskPath010
     * @tc.name: testUploadTaskPath010
     * @tc.desc: Test when path is file:// with error package name
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadTaskPath010', 0, async function (done) {
      let context = globalThis.abilityContext.getApplicationContext();
      let dir = context.cacheDir;
      let file = fs.openSync(dir + '/upload_010', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
                 name: 'file',
                 value: {
                   path: 'file://com.acts.request1111/data/storage/el2/base/cache/upload_010'
                 }
               }]
      }
      try {
        await request.agent.create(context, conf).then(async () => {
          if (fs.accessSync(dir + '/upload_010')) {
            fs.unlinkSync(dir + '/upload_010');
          }
          expect(false).assertTrue();
          done();
        }).catch((err) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(true).assertTrue();
        done();
      }
    })

    /**
     * @tc.number: testRemoveTaskStart001
     * @tc.name: testRemoveTaskStart001
     * @tc.desc: Test remove task and then start
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testRemoveTaskStart001', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        mode: request.agent.Mode.BACKGROUND,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: `test_remove_task_start_001`,
      }
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
    })
  })
}