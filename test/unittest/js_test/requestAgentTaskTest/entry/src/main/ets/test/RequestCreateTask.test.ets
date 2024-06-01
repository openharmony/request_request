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

// @ts-nocheck
import { describe, beforeAll, beforeEach, afterEach, afterAll, it, expect } from "@ohos/hypium"
import request from '@ohos.request';
import fs from '@ohos.file.fs';

export default function requestCreateTaskTest() {
  describe('requestCreateTaskTest', function () {
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
    })

    function sleep(ms) {
      return new Promise(resolve => setTimeout(resolve, ms));
    }

    let context = globalThis.abilityContext;
    let cacheDir = globalThis.cacheDir;
    let fileSpec = {
      path: `./test.txt`
    }
    let formItem = {
      name: 'file',
      value: fileSpec
    }

    /**
     * @tc.number: testTaskAction001
     * @tc.name: testTaskAction001
     * @tc.desc: Test create task when lack action
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskAction001', 0, function (done)  {
      let conf = {
        url: 'http://127.0.0.1',
      }
      try {
        request.agent.create(context, conf, (err) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.number: testTaskAction002
     * @tc.name: testTaskAction002
     * @tc.desc: Test create task when action is string
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskAction002', 0, async function (done)  {
      let conf = {
        action: 'UPLOAD',
        url: 'http://127.0.0.1'
      }
      try {
        request.agent.create(context, conf, (err) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.number: testTaskAction003
     * @tc.name: testTaskAction003
     * @tc.desc: Test create task when action is 2
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskAction003', 0, function (done)  {
      let conf = {
        action: 2,
        url: 'http://127.0.0.1'
      }
      try {
        request.agent.create(context, conf, (err) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.number: testTaskAction004
     * @tc.name: testTaskAction004
     * @tc.desc: Test create task when action is UPLOAD
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskAction004', 0, function (done)  {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [formItem]
      }
      request.agent.create(context, conf, async (err, data) => {
        expect(err === null).assertTrue();
        await request.agent.remove(data.tid);
        done();
      })
    })

    /**
     * @tc.number: testTaskAction005
     * @tc.name: testTaskAction005
     * @tc.desc: Test create task when action is DOWNLOAD
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskAction005', 0, function (done)  {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: `./`,
        gauge: true
      }
      request.agent.create(context, conf).then(async (data) => {
        if (fs.accessSync(`${cacheDir}/test.txt`)) {
          expect(true).assertTrue();
          await request.agent.remove(data.tid);
          done();
        }
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testTaskUrl001
     * @tc.name: testTaskUrl001
     * @tc.desc: Test create task when lack url
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskUrl001', 0, function (done)  {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
      }
      try {
        request.agent.create(context, conf, (err) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.number: testTaskUrl002
     * @tc.name: testTaskUrl002
     * @tc.desc: Test create task when url is empty
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskUrl002', 0, function (done)  {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: '',
      }
      try {
        request.agent.create(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.number: testTaskUrl003
     * @tc.name: testTaskUrl003
     * @tc.desc: Test create task when url is not support download
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskUrl003', 0, function (done)  {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/openharmony/request_request',
      }
      request.agent.create(context, conf, async (err, data) => {
        expect(err === null).assertTrue();
        await request.agent.remove(data.tid);
        done();
      })
    })

    /**
     * @tc.number: testTaskUrl004
     * @tc.name: testTaskUrl004
     * @tc.desc: Test create task when url is not support upload
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskUrl004', 0, function (done)  {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'https://gitee.com/openharmony/request_request',
        data: [formItem]
      }
      request.agent.create(context, conf, async (err, data) => {
        expect(err === null).assertTrue();
        await request.agent.remove(data.tid);
        done();
      })
    })

    /**
     * @tc.number: testTaskUrl005
     * @tc.name: testTaskUrl005
     * @tc.desc: Test create task when url is not http/https scheme.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskUrl005', 0, function (done)  {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'admin://127.0.0.1',
      }
      try {
        request.agent.create(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.number: testTaskUrl006
     * @tc.name: testTaskUrl006
     * @tc.desc: Test create task when url length exceeds 2048.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskUrl006', 0, function (done)  {

      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'http://127.0.0.1/' + 'a'.repeat(2048),
      }
      try {
        request.agent.create(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.number: testTaskTitle001
     * @tc.name: testTaskTitle001
     * @tc.desc: Test create task when title is given
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskTitle001', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        title: 'download test.txt',
        saveas: 'testTaskTitle001.txt',
      }
      let task = await request.agent.create(context, conf);
      expect(task.config.title).assertEqual('download test.txt');
      await request.agent.remove(task.tid);
      done();
    })

    /**
     * @tc.number: testTaskTitle002
     * @tc.name: testTaskTitle002
     * @tc.desc: Test create task when title is number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskTitle002', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        title: 123,
        saveas: 'testTaskTitle002.txt',
      }
      let task = await request.agent.create(context, conf);
      expect(task.config.title).assertEqual(123);
      await request.agent.remove(task.tid);
      done();
    })

    /**
     * @tc.number: testTaskTitle003
     * @tc.name: testTaskTitle003
     * @tc.desc: Test create task when title length exceeds 256
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskTitle003', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        title: 'a'.repeat(257),
        saveas: 'testTaskTitle003.txt',
      }
      try {
        request.agent.create(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.number: testTaskDescription001
     * @tc.name: testTaskDescription001
     * @tc.desc: Test create task when description is given
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskDescription001', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        description: 'test upload',
        saveas: 'testTaskDescription001.txt',
      }
      let task = await request.agent.create(context, conf);
      expect(task.config.description).assertEqual('test upload')
      await request.agent.remove(task.tid);
      done()
    })

    /**
     * @tc.number: testTaskDescription002
     * @tc.name: testTaskDescription002
     * @tc.desc: Test create task when description is number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskDescription002', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        description: 123,
        saveas: 'testTaskDescription002.txt',
      }
      request.agent.create(context, conf, async (err, data) => {
        expect(data.config.description).assertEqual(123);
        await request.agent.remove(data.tid);
        done();
      })
    })

    /**
     * @tc.number: testTaskDescription003
     * @tc.name: testTaskDescription003
     * @tc.desc: Test create task when description length exceeds 1024
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskDescription003', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskDescription003.txt',
        description: 'a'.repeat(1025)
      }
      try {
        request.agent.create(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.number: testTaskMode001
     * @tc.name: testTaskMode001
     * @tc.desc: Test create task when mode is FOREGROUND
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskMode001', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskMode001.txt',
        mode: request.agent.Mode.FOREGROUND
      }
      request.agent.create(context, conf).then(async (data) => {
        expect(true).assertTrue();
        await request.agent.remove(data.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testTaskMode002
     * @tc.name: testTaskMode002
     * @tc.desc: Test create task when mode is BACKGROUND
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskMode002', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskMode002.txt',
        mode: request.agent.Mode.BACKGROUND
      }
      request.agent.create(context, conf).then(async (data) => {
        expect(true).assertTrue();
        await request.agent.remove(data.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testTaskMode002
     * @tc.name: testTaskMode003
     * @tc.desc: Test create task when mode is string
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskMode003', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskMode003.txt',
        mode: "BACKGROUND"
      }
      let task = await request.agent.create(context, conf);
      task.start().then(async () => {
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testTaskOverwrite001
     * @tc.name: testTaskOverwrite001
     * @tc.desc: Test create task when overwrite is true and file exists
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskOverwrite001', 0, async function (done) {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        overwrite: true
      }
      let task = await request.agent.create(context, conf);
      task.start().then(async () => {
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testTaskOverwrite002
     * @tc.name: testTaskOverwrite002
     * @tc.desc: Test create task when overwrite is true and file not exists
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskOverwrite002', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        overwrite: true
      }
      let task = await request.agent.create(context, conf);
      task.start().then(async () => {
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testTaskOverwrite003
     * @tc.name: testTaskOverwrite003
     * @tc.desc: Test create task when overwrite is false and file exists
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskOverwrite003', 0, async function (done) {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        overwrite: false
      }
      try {
        request.agent.create(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(err.code).assertEqual(13400001)
        done();
      }
    })

    /**
     * @tc.number: testTaskOverwrite004
     * @tc.name: testTaskOverwrite004
     * @tc.desc: Test create task when overwrite is false and file not exists
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskOverwrite004', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        overwrite: false
      }
      let task = await request.agent.create(context, conf);
      task.start().then(async () => {
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testTaskOverwrite005
     * @tc.name: testTaskOverwrite005
     * @tc.desc: Test create task when overwrite is string and file not exists
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskOverwrite005', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        overwrite: "true"
      }
      let task = await request.agent.create(context, conf);
      task.start().then(async () => {
        expect(task.config.overwrite).assertEqual("true");
        await request.agent.remove(task.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testTaskOverwrite006
     * @tc.name: testTaskOverwrite006
     * @tc.desc: Test create task when overwrite is string and file exists
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskOverwrite006', 0, async function (done) {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        overwrite: "true"
      }
      try {
        request.agent.create(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(err.code).assertEqual(13400001)
        done();
      }
    })

    /**
     * @tc.number: testTaskOverwrite007
     * @tc.name: testTaskOverwrite007
     * @tc.desc: Test create task when overwrite is not given and file not exists
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskOverwrite007', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt'
      }
      try {
        let task = await request.agent.create(context, conf);
        task.start().then(async () => {
          expect(true).assertTrue();
          await request.agent.remove(task.tid);
          done();
        }).catch((err) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(err.code).assertEqual(13400001)
        done();
      }
    })

    /**
     * @tc.number: testTaskOverwrite008
     * @tc.name: testTaskOverwrite008
     * @tc.desc: Test create task when overwrite is not given and file exists
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskOverwrite008', 0, async function (done) {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt'
      }
      try {
        request.agent.create(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(err.code).assertEqual(13400001)
        done();
      }
    })

    /**
     * @tc.number: testTaskMethod001
     * @tc.name: testTaskMethod001
     * @tc.desc: Test create task when method is POST for upload
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskMethod001', 0, async function (done) {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
                 name: 'file',
                 value: {
                   path: `./test.txt`
                 },
               }],
        method: 'POST'
      }
      let task = await request.agent.create(context, conf);
      task.start().then(() => {
        expect(true).assertTrue();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testTaskMethod002
     * @tc.name: testTaskMethod002
     * @tc.desc: Test create task when method is POST for download
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskMethod002', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskMethod002.txt',
        method: 'POST'
      }
      let task = await request.agent.create(context, conf);
      task.start().then(async () => {
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testTaskMethod003
     * @tc.name: testTaskMethod003
     * @tc.desc: Test create task when method is number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskMethod003', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskMethod003.txt',
        method: 123
      }
      let task = await request.agent.create(context, conf);
      task.start().then(async () => {
        expect(task.config.method).assertEqual(123);
        await request.agent.remove(task.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testTaskMethod004
     * @tc.name: testTaskMethod004
     * @tc.desc: Test create task when method is empty
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskMethod004', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskMethod004.txt',
        method: ''
      }
      let task = await request.agent.create(context, conf);
      task.start().then(async () => {
        expect(task.config.method).assertEqual('');
        await request.agent.remove(task.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testTaskMethod005
     * @tc.name: testTaskMethod005
     * @tc.desc: Test create task when method is GET for upload
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskMethod005', 0, async function (done) {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
                 name: 'file',
                 value: {
                   path: `./test.txt`
                 },
               }],
        method: 'GET'
      }
      let task = await request.agent.create(context, conf);
      task.start().then(() => {
        expect(true).assertTrue();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testTaskMethod006
     * @tc.name: testTaskMethod006
     * @tc.desc: Test create task when method is PUT for download
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskMethod006', 0, async function (done) {
      await sleep(2000);
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskMethod006.txt',
        method: 'PUT'
      }
      let task = await request.agent.create(context, conf);
      task.start().then(async () => {
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testTaskSaveas001
     * @tc.name: testTaskSaveas001
     * @tc.desc: Test create task when saveas is number for download
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskSaveas001', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.BACKGROUND,
        saveas: 123
      }
      let task = await request.agent.create(context, conf);
      task.on('completed', function() {
        if (fs.accessSync(`${cacheDir}/test.txt`)) {
          expect(true).assertTrue();
          done();
        }
      })
      await task.start();
    })

    /**
     * @tc.number: testTaskSaveas002
     * @tc.name: testTaskSaveas002
     * @tc.desc: Test create task when lack saveas for download
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskSaveas002', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.BACKGROUND
      }
      try {
        let task = await request.agent.create(context, conf);
        task.on('completed', function() {
          if (fs.accessSync(`${cacheDir}/test.txt`)) {
            expect(true).assertTrue();
            done();
          } else {
            expect(false).assertTrue();
            done();
          }
        })
        await task.start();
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.number: testTaskData001
     * @tc.name: testTaskData001
     * @tc.desc: Test create task when data lack name
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskData001', 0, function (done)  {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
                 value: {
                   path: `./test.txt`
                 },
               }]
      }
      try {
        request.agent.create(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.number: testTaskData002
     * @tc.name: testTaskData002
     * @tc.desc: Test create task when data name is number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskData002', 0, function (done)  {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
                 name: 123,
                 value: {
                   path: `./test.txt`
                 }
               }]
      }
      try {
        request.agent.create(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.number: testTaskData003
     * @tc.name: testTaskData003
     * @tc.desc: Test create task when data lack value
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskData003', 0, function (done)  {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
                 name: 'test'
               }]
      }
      try {
        request.agent.create(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.number: testTaskData004
     * @tc.name: testTaskData004
     * @tc.desc: Test create task when data value is number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskData004', 0, function (done)  {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
                 name: 'file',
                 value: 123
               }]
      }
      try {
        request.agent.create(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.number: testTaskData005
     * @tc.name: testTaskData005
     * @tc.desc: Test create task when data path is '', path is not exits
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskData005', 0, function (done)  {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
                 name: 'file',
                 value: {
                   path: ''
                 }
               }]
      }
      try {
        request.agent.create(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.number: testTaskData006
     * @tc.name: testTaskData006
     * @tc.desc: Test create task when data path is number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskData006', 0, function (done)  {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
                 name: 'file',
                 value: {
                   path: 123
                 }
               }]
      }
      try {
        request.agent.create(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.number: testTaskData007
     * @tc.name: testTaskData007
     * @tc.desc: Test create task when data path is not absolute path
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskData007', 0, function (done)  {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
                 name: 'file',
                 value: {
                   path: `${cacheDir}/test.txt`
                 }
               }]
      }
      try {
        request.agent.create(context, conf, (err) => {
          expect(true).assertTrue();
          done();
        })
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    })

    /**
     * @tc.number: testTaskData008
     * @tc.name: testTaskData008
     * @tc.desc: Test create task when data filename is number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskData008', 0, function (done)  {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
                 name: 'file',
                 value: {
                   path: `./test.txt`,
                   filename: 123
                 }
               }]
      }
      request.agent.create(context, conf, async (err, data) => {
        expect(err === null).assertTrue();
        await request.agent.remove(data.tid);
        done();
      })
    })

    /**
     * @tc.number: testTaskData009
     * @tc.name: testTaskData009
     * @tc.desc: Test create task when data mimeType is number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskData009', 0, function (done)  {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
                 name: 'file',
                 value: {
                   path: `./test.txt`,
                   mimeType: 123
                 }
               }]
      }
      request.agent.create(context, conf, async (err, data) => {
        expect(err === null).assertTrue();
        await request.agent.remove(data.tid);
        done();
      })
    })

    /**
     * @tc.number: testTaskData010
     * @tc.name: testTaskData010
     * @tc.desc: Test create task when data path and filename is different
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskData010', 0, function (done)  {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
                 name: 'file',
                 value: {
                   path: `./test.txt`,
                   filename: 'a.txt'
                 }
               }]
      }
      request.agent.create(context, conf, async (err, data) => {
        expect(err === null).assertTrue();
        await request.agent.remove(data.tid);
        done();
      })
    })

    /**
     * @tc.number: testTaskData011
     * @tc.name: testTaskData011
     * @tc.desc: Test create task when data two files for upload
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskData011', 0, function (done)  {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
         name: 'file',
         value: [
           {
             path: `./test.txt`,
           },
           {
             path: `./test.txt`,
           }
         ]
       }]
      }
      request.agent.create(context, conf, async (err, data) => {
        expect(err === null).assertTrue();
        await request.agent.remove(data.tid);
        done();
      })
    })

    /**
     * @tc.number: testTaskData012
     * @tc.name: testTaskData012
     * @tc.desc: Test create task when data value is string
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskData012', 0, function (done)  {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
                 name: 'test',
                 value: 'test'
               }]
      }
      try {
        request.agent.create(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }

    })

    /**
     * @tc.number: testTaskData013
     * @tc.name: testTaskData013
     * @tc.desc: Test create task when data path and filename is same
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskData013', 0, function (done)  {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
                 name: 'file',
                 value: {
                   path: `./test.txt`,
                   filename: 'test.txt'
                 }
               }]
      }
      request.agent.create(context, conf, async (err, data) => {
        expect(err === null).assertTrue();
        await request.agent.remove(data.tid);
        done();
      })
    })

    /**
     * @tc.number: testTaskNetwork001
     * @tc.name: testTaskNetwork001
     * @tc.desc: Test create task when network is 3
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskNetwork001', 0, async function (done) {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
                 name: 'file',
                 value: {
                   path: `./test.txt`,
                 }
               }],
        network: 3
      }
      request.agent.create(context, conf).then(async (data) => {
        expect(true).assertTrue();
        await request.agent.remove(data.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testTaskNetwork002
     * @tc.name: testTaskNetwork002
     * @tc.desc: Test create task when network is string
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskNetwork002', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskNetwork002.txt',
        network: "ANY"
      }
      request.agent.create(context, conf).then(async (data) => {
        expect(true).assertTrue();
        await request.agent.remove(data.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testTaskNetwork003
     * @tc.name: testTaskNetwork003
     * @tc.desc: Test create task when network is WIFI for DOWNLOAD
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskNetwork003', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskNetwork003.txt',
        network: request.agent.Network.WIFI
      }
      request.agent.create(context, conf).then(async (data) => {
        expect(true).assertTrue();
        await request.agent.remove(data.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testTaskNetwork004
     * @tc.name: testTaskNetwork004
     * @tc.desc: Test create task when network is any for UPLOAD
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskNetwork004', 0, async function (done) {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
                 name: 'file',
                 value: {
                   path: `./test.txt`,
                 }
               }],
        network: request.agent.Network.WIFI
      }
      let task = await request.agent.create(context, conf);
      task.start().then(async () => {
        expect(true).assertTrue();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testTaskHeaders001
     * @tc.name: testTaskHeaders001
     * @tc.desc: Test create task when headers content-type is application/json but data is file for upload
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskHeaders001', 0, async function (done) {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
                 name: 'file',
                 value: {
                   path: `./test.txt`
                 },
               }],
        headers: {'content-type': 'application/json'},
      }
      let task = await request.agent.create(context, conf);
      task.start().then(() => {
        expect(true).assertTrue();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testTaskHeaders002
     * @tc.name: testTaskHeaders002
     * @tc.desc: Test create task when lack headers for upload
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskHeaders002', 0, async function (done) {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
                 name: 'file',
                 value: {
                   path: `./test.txt`
                 },
               }],
      }
      let task = await request.agent.create(context, conf);
      task.start().then(() => {
        expect(typeof task.config.headers).assertEqual('undefined');
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testTaskHeaders003
     * @tc.name: testTaskHeaders003
     * @tc.desc: Test create task when lack headers for download
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskHeaders003', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskHeaders003.txt',
      }
      let task = await request.agent.create(context, conf);
      task.start().then(async () => {
        expect(typeof task.config.headers).assertEqual('undefined');
        await request.agent.remove(task.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testTaskRetry001
     * @tc.name: testTaskRetry001
     * @tc.desc: Test create task when retry is true for FOREGROUND
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskRetry001', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'http://xxx',
        mode: request.agent.Mode.FOREGROUND,
        retry: true
      }
      let task = await request.agent.create(context, conf);
      task.start().then(async () => {
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testTaskRetry002
     * @tc.name: testTaskRetry002
     * @tc.desc: Test create task when retry is true for background
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskRetry002', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.BACKGROUND,
        saveas: 'testTaskRetry002.txt',
        retry: true,
        gauge: true
      }
      let task = await request.agent.create(context, conf);
      task.start().then(async () => {
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testTaskRetry003
     * @tc.name: testTaskRetry003
     * @tc.desc: Test create task when retry is string
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskRetry003', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'http://xxx',
        mode: request.agent.Mode.BACKGROUND,
        retry: 'true',
        overwrite: true
      }
      let task = await request.agent.create(context, conf);
      task.on('failed', function () {
        expect(true).assertTrue();
        done();
      })
      await task.start();
    })

    /**
     * @tc.number: testTaskRetry004
     * @tc.name: testTaskRetry004
     * @tc.desc: Test create task when retry is false for FOREGROUND
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskRetry004', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test1.txt',
        mode: request.agent.Mode.FOREGROUND,
        retry: false
      }
      let task = await request.agent.create(context, conf);
      task.on('failed', function () {
        expect(true).assertTrue();
        done();
      })
      await task.start();
    })

    /**
     * @tc.number: testTaskRetry005
     * @tc.name: testTaskRetry005
     * @tc.desc: Test create task when retry is false for background
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskRetry005', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test1.txt',
        mode: request.agent.Mode.BACKGROUND,
        retry: false,
        overwrite: true
      }
      let task = await request.agent.create(context, conf);
      task.on('failed', function () {
        expect(true).assertTrue();
        done();
      })
      await task.start();
    })

    /**
     * @tc.number: testTaskIndex001
     * @tc.name: testTaskIndex001
     * @tc.desc: Test create task when index is string
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskIndex001', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskIndex001.txt',
        index: '0',
        mode: request.agent.Mode.BACKGROUND
      }
      let task = await request.agent.create(context, conf);
      task.on('completed', function() {
        expect(true).assertTrue();
        done();
      })
      await task.start();
    })

    /**
     * @tc.number: testTaskIndex002
     * @tc.name: testTaskIndex002
     * @tc.desc: Test create task when index is 0 and begins greater than ends
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskIndex002', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskIndex002.txt',
        index: 0,
        begins: 10,
        ends: 5
      }
      let task = await request.agent.create(context, conf);
      task.start().then(async () => {
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testTaskIndex003
     * @tc.name: testTaskIndex003
     * @tc.desc: Test create task when index is 1 but only one file for upload
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskIndex003', 0, async function (done) {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        index: 1
      }
      try {
        let task = await request.agent.create(context, conf);
        task.start().then(() => {
          expect(false).assertTrue();
          done();
        }).catch((err) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.number: testTaskIndex004
     * @tc.name: testTaskIndex004
     * @tc.desc: Test create task when index is 0 and begins is 5 and ends is 10 for download
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskIndex004', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://sf3-cn.feishucdn.com/obj/ee-appcenter/47273f95/Feishu-win32_ia32-7.9.7-signed.exe',
        saveas: 'testTaskIndex004.exe',
        mode: request.agent.Mode.BACKGROUND,
        begins: 5,
        ends: 10,
        gauge: true
      }
      let task = await request.agent.create(context, conf);
      task.on('completed', function(progress) {
        expect(progress.state).assertEqual(0x40);
        expect(progress.index).assertEqual(0);
        expect(progress.processed).assertEqual(6);
        expect(progress.sizes[0]).assertEqual(6);
        done();
      })
      await task.start();
    })

    /**
     * @tc.number: testTaskIndex005
     * @tc.name: testTaskIndex005
     * @tc.desc: Test create task when index is 0 and begins is 5 and ends is 10 for upload
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskIndex005', 0, async function (done) {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
                 name: 'file',
                 value: {
                   path: `./test.txt`,
                 }
               }],
        begins: 5,
        ends: 10
      }
      let task = await request.agent.create(context, conf);
      task.start().then(() => {
        expect(true).assertTrue();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testTaskIndex006
     * @tc.name: testTaskIndex006
     * @tc.desc: Test create task when unsupport range request
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskIndex006', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.BACKGROUND,
        begins: 5,
        gauge: true
      }
      let task = await request.agent.create(context, conf);
      await task.start();
      task.on('failed', function(progress) {
        expect(true).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testTaskIndex007
     * @tc.name: testTaskIndex007
     * @tc.desc: Test create task when index is 0 and begins is 5 and ends is not exists for upload
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskIndex007', 0, async function (done) {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
                 name: 'file',
                 value: {
                   path: `./test.txt`,
                 }
               }],
        begins: 5,
      }
      let task = await request.agent.create(context, conf);
      task.start().then(() => {
        expect(true).assertTrue();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testTaskIndex008
     * @tc.name: testTaskIndex008
     * @tc.desc: Test create task when index is 0 and begins is not exists and ends is 10 for download
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskIndex008', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://sf3-cn.feishucdn.com/obj/ee-appcenter/47273f95/Feishu-win32_ia32-7.9.7-signed.exe',
        saveas: 'testTaskIndex008.exe',
        mode: request.agent.Mode.BACKGROUND,
        ends: 10,
        gauge: true,
        overwrite: true
      }
      let task = await request.agent.create(context, conf);
      await task.start();
      task.on('completed', function(progress) {
        expect(progress.sizes[0]).assertEqual(11);
        done();
      })
    })

    /**
     * @tc.number: testTaskIndex009
     * @tc.name: testTaskIndex009
     * @tc.desc: Test create task when index is 0 and begins is 5 and ends is not exists for upload
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskIndex009', 0, async function (done) {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
                 name: 'file',
                 value: {
                   path: `./test.txt`,
                 }
               }],
        ends: 10
      }
      let task = await request.agent.create(context, conf);
      task.start().then(() => {
        expect(true).assertTrue();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testTaskIndex010
     * @tc.name: testTaskIndex010
     * @tc.desc: Test create task when index is 1 but have two files for upload
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskIndex010', 0, async function (done) {
      let test1 = fs.openSync(cacheDir + '/test1.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(test1);
      let test2 = fs.openSync(cacheDir + '/test2.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(test2);
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [
        {
          name: 'file',
          value: [
          {
            path: `./test1.txt`,
            filename: 'test1.txt'
          },
          {
            path: `./test2.txt`,
            filename: 'test2.txt'
          }]
        }],
        index: 1,
      }
      let task = await request.agent.create(context, conf);
      task.start().then(() => {
        expect(true).assertTrue();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testTaskToken001
     * @tc.name: testTaskToken001
     * @tc.desc: Test create task when token is 7 bytes
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskToken001', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskToken001.txt',
        token: 'abcdef'
      }
      try {
        request.agent.create(context, conf).then((data) => {
          expect(false).assertTrue();
          done();
        }).catch((err) => {})
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.number: testTaskToken002
     * @tc.name: testTaskToken002
     * @tc.desc: Test create task when token is 2049 bytes
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskToken002', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskToken002.txt',
        token: 'a'.padStart(2049, 'a')
      }
      try {
        request.agent.create(context, conf).then((data) => {
          expect(false).assertTrue();
          done();
        }).catch((err) => {})
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.number: testTaskToken003
     * @tc.name: testTaskToken003
     * @tc.desc: Test create task when token is number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskToken003', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskToken003.txt',
        token: 666
      }
      request.agent.create(context, conf).then(async (data) => {
        expect(data.config.token).assertEqual(666);
        await request.agent.remove(data.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testTaskToken004
     * @tc.name: testTaskToken004
     * @tc.desc: Test create task when token is 2048 bytes
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskToken004', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskToken004.txt',
        token: 'a'.padStart(2048, 'a')
      }
      request.agent.create(context, conf).then(async (data) => {
        expect(true).assertTrue();
        await request.agent.remove(data.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testCreate001
     * @tc.name: testCreate001
     * @tc.desc: Test create for conf is string type
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testCreate001', 0, async function (done) {
      try {
        await request.agent.create(context, "conf");
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.number: testCreate002
     * @tc.name: testCreate002
     * @tc.desc: Test create
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testCreate002', 0, async function (done) {
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
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    })

    /**
     * @tc.number: testTaskProxy001
     * @tc.name: testTaskProxy001
     * @tc.desc: Test create task when proxy format error
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskProxy001', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        proxy: 'https://127.0.0.1:8080'
      }
      try {
        request.agent.create(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.number: testTaskProxy002
     * @tc.name: testTaskProxy002
     * @tc.desc: Test create task when proxy format error
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskProxy002', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        proxy: 'http://127.0.0.1:8080/proxy'
      }
      try {
        request.agent.create(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.number: testTaskProxy003
     * @tc.name: testTaskProxy003
     * @tc.desc: Test create task when proxy format error
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskProxy003', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        proxy: 'http://127.0.0.1:111111'
      }
      try {
        request.agent.create(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
  })
}