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
      if (fs.accessSync(cacheDir + '/test.apk')) {
        fs.unlinkSync(cacheDir + '/test.apk');
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
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testStartTask001', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
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
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testStartTask002', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `test.apk`,
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
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testStartTask003', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `test.apk`,
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
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testStartTask004', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `test.apk`,
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
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testStartTask005', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `test.apk`,
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
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testStartTask006', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `test.apk`,
        mode: request.agent.Mode.BACKGROUND
      }
      let task = await request.agent.create(context, conf);
      try {
        await task.start();
        await task.stop();
        await task.start();
      } catch (err) {
        expect(err.code).assertEqual(21900007);
        done();
      }
    })

    /**
     * @tc.number: testStartTask007
     * @tc.name: testStartTask007
     * @tc.desc: Test start stop start FOREGROUND
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testStartTask007', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `test.apk`,
        mode: request.agent.Mode.FOREGROUND
      }
      let task = await request.agent.create(context, conf);
      try {
        await task.start();
        await task.stop();
        await task.start();
      } catch (err) {
        expect(err.code).assertEqual(21900007);
        done();
      }
    })

    /**
     * @tc.number: testStartTask008
     * @tc.name: testStartTask008
     * @tc.desc: Test start two FOREGROUND task
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testStartTask008', 0, async function (done) {
      let conf1 = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `test2.apk`,
        mode: request.agent.Mode.FOREGROUND
      }

      try {
        request.agent.create(context, conf1, async (err, task1) => {
          expect(true).assertTrue();
          await request.agent.remove(task1.tid);
          done();
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
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testPauseTask001', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
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
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testPauseTask002', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `test.apk`,
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
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testPauseTask003', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `test.apk`,
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
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testPauseTask004', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `test.apk`,
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
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testPauseTask005', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://sf3-cn.feishucdn.com/obj/ee-appcenter/6d6bc5/Feishu-win32_ia32-5.10.6-signed.exe',
        saveas: `test.apk`,
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
     * @tc.desc: Test resume FOREGROUND task
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testResumeTask001', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `test.apk`,
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
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testResumeTask002', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `test.apk`,
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
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testResumeTask003', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `test.apk`,
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
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testResumeTask004', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `test.apk`,
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
     * @tc.number: testStopTask001
     * @tc.name: testStopTask001
     * @tc.desc: Test stop FOREGROUND task for callback
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testStopTask001', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `test.apk`,
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
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testStopTask002', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `test.apk`,
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
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testStopTask003', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `test.apk`,
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
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testStopTask004', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `test.apk`,
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
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testStopTask005', 0, async function (done) {
      await sleep(2000);
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `test.apk`,
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
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testOnTask001', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `test.apk`,
        mode: request.agent.Mode.FOREGROUND
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
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testOnTask002', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `test.apk`,
        mode: request.agent.Mode.FOREGROUND
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
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testOnTask003', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test1.apk',
        saveas: `test.apk`,
        mode: request.agent.Mode.FOREGROUND
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
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testOnTask004', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `test.apk`,
        mode: request.agent.Mode.FOREGROUND
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
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testOnTask005', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `test.apk`,
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
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testOnTask006', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://sf3-cn.feishucdn.com/obj/ee-appcenter/6d6bc5/Feishu-win32_ia32-5.10.6-signed.exe',
        saveas: `test.apk`,
        mode: request.agent.Mode.FOREGROUND
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
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testOnTask007', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `test.apk`,
        mode: request.agent.Mode.FOREGROUND
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
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testOffTask001', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `test.apk`,
        mode: request.agent.Mode.FOREGROUND
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
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testOffTask002', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `test.apk`,
        mode: request.agent.Mode.FOREGROUND
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
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testOffTask003', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test1.apk',
        saveas: `test.apk`,
        mode: request.agent.Mode.FOREGROUND
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
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testOffTask004', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `test.apk`,
        mode: request.agent.Mode.FOREGROUND
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
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testOffTask005', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `test.apk`,
        mode: request.agent.Mode.FOREGROUND
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
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testOffTask006', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `test.apk`,
        mode: request.agent.Mode.FOREGROUND
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
     * @tc.type: FUNC
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
     * @tc.type: FUNC
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
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testRemove003', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        mode: request.agent.Mode.FOREGROUND
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
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testRemove004', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        mode: request.agent.Mode.FOREGROUND
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
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testRemove005', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        mode: request.agent.Mode.FOREGROUND
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
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testRedirect001', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `test.apk`,
        mode: request.agent.Mode.FOREGROUND,
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
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testRedirect002', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `test.apk`,
        mode: request.agent.Mode.FOREGROUND,
        redirect: false
      }
      let task = await request.agent.create(context, conf);
      task.on("failed", (progress) => {
        expect(true).assertTrue();
        done();
      })
      await task.start();
    })
  })
}