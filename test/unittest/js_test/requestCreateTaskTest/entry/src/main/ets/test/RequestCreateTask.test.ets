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
    let formItem = {
      name: 'file',
      value: fileSpec
    }

    /**
     * @tc.name: testTaskAction001
     * @tc.desc: Test create task when lack action
     * @tc.type: FUNC
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
        expect(err != null && err.code === 401).assertTrue();
        done();
      }
    })

    /**
     * @tc.name: testTaskAction002
     * @tc.desc: Test create task when action is string
     * @tc.type: FUNC
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
        expect(err != null && err.code === 401).assertTrue();
        done();
      }
    })

    /**
     * @tc.name: testTaskAction003
     * @tc.desc: Test create task when action is 2
     * @tc.type: FUNC
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
        expect(err != null && err.code === 401).assertTrue();
        done();
      }
    })

    /**
     * @tc.name: testTaskAction004
     * @tc.desc: Test create task when action is UPLOAD
     * @tc.type: FUNC
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
        request.agent.remove(data.tid);
        done();
      })
    })

    /**
     * @tc.name: testTaskAction005
     * @tc.desc: Test create task when action is DOWNLOAD
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskAction005', 0, function (done)  {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `./`,
        gauge: true
      }
      request.agent.create(context, conf).then((data) => {
        if (fs.accessSync(`${cacheDir}/test.apk`)) {
          expect(true).assertTrue();
          request.agent.remove(data.tid);
          done();
        }
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testTaskUrl001
     * @tc.desc: Test create task when lack url
     * @tc.type: FUNC
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
        expect(err != null && err.code === 401).assertTrue();
        done();
      }
    })

    /**
     * @tc.name: testTaskUrl002
     * @tc.desc: Test create task when url is empty
     * @tc.type: FUNC
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
        expect(err != null && err.code === 401).assertTrue();
        done();
      }
    })

    /**
     * @tc.name: testTaskUrl003
     * @tc.desc: Test create task when url is not support download
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskUrl003', 0, function (done)  {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/openharmony/request_request',
      }
      request.agent.create(context, conf, (err, data) => {
        expect(err == null).assertTrue();
        request.agent.remove(data.tid);
        done();
      })
    })

    /**
     * @tc.name: testTaskUrl004
     * @tc.desc: Test create task when url is not support upload
     * @tc.type: FUNC
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
      request.agent.create(context, conf, (err, data) => {
        expect(err == null).assertTrue();
        request.agent.remove(data.tid);
        done();
      })
    })

    /**
     * @tc.name: testTaskTitle001
     * @tc.desc: Test create task when title is given
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskTitle001', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        title: 'download test.txt'
      }
      let task = await request.agent.create(context, conf);
      expect(task.conf.title).assertEqual('download test.txt');
      request.agent.remove(task.tid);
      done();
    })

    /**
     * @tc.name: testTaskTitle002
     * @tc.desc: Test create task when title is number
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskTitle002', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        title: 123
      }
      let task = await request.agent.create(context, conf);
      expect(task.conf.title).assertEqual(123);
      request.agent.remove(task.tid);
      done();
    })

    /**
     * @tc.name: testTaskDescription001
     * @tc.desc: Test create task when description is given
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskDescription001', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        description: 'test upload'
      }
      let task = await request.agent.create(context, conf);
      expect(task.conf.description).assertEqual('test upload')
      request.agent.remove(task.tid);
      done()
    })

    /**
     * @tc.name: testTaskDescription002
     * @tc.desc: Test create task when description is number
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskDescription002', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        description: 123
      }
      request.agent.create(context, conf, function (err, data) {
        expect(err == null && data.conf.description == 123).assertTrue();
        request.agent.remove(data.tid);
        done();
      })
    })

    /**
     * @tc.name: testTaskMode001
     * @tc.desc: Test create task when mode is FOREGROUND
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskMode001', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        mode: request.agent.Mode.FOREGROUND
      }
      request.agent.create(context, conf).then((data) => {
        expect(true).assertTrue();
        request.agent.remove(data.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testTaskMode002
     * @tc.desc: Test create task when mode is BACKGROUND
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskMode002', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        mode: request.agent.Mode.BACKGROUND
      }
      request.agent.create(context, conf).then((data) => {
        expect(true).assertTrue();
        request.agent.remove(data.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testTaskMode003
     * @tc.desc: Test create task when mode is string
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskMode003', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        mode: "BACKGROUND"
      }
      let task = await request.agent.create(context, conf);
      task.start().then(() => {
        expect(true).assertTrue();
        task.stop();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testTaskOverwrite001
     * @tc.desc: Test create task when overwrite is true and file exists
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskOverwrite001', 0, async function (done) {
      let file = fs.openSync(cacheDir + '/test.apk', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        overwrite: true
      }
      let task = await request.agent.create(context, conf);
      task.start().then(() => {
        expect(true).assertTrue();
        task.stop();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testTaskOverwrite002
     * @tc.desc: Test create task when overwrite is true and file not exists
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskOverwrite002', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        overwrite: true
      }
      let task = await request.agent.create(context, conf);
      task.start().then(() => {
        expect(true).assertTrue();
        task.stop();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testTaskOverwrite003
     * @tc.desc: Test create task when overwrite is false and file exists
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskOverwrite003', 0, async function (done) {
      let file = fs.openSync(cacheDir + '/test.apk', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        overwrite: false
      }
      try {
        request.agent.create(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(err != null && err.code === 13400001).assertTrue();
        done();
      }
    })

    /**
     * @tc.name: testTaskOverwrite004
     * @tc.desc: Test create task when overwrite is false and file not exists
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskOverwrite004', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        overwrite: false
      }
      let task = await request.agent.create(context, conf);
      task.start().then(() => {
        expect(true).assertTrue();
        task.stop();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testTaskOverwrite005
     * @tc.desc: Test create task when overwrite is string
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskOverwrite005', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        overwrite: "true"
      }
      let task = await request.agent.create(context, conf);
      task.start().then(() => {
        expect(task.conf.overwrite).assertEqual("true");
        task.stop();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testTaskMethod001
     * @tc.desc: Test create task when method is POST for upload
     * @tc.type: FUNC
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
        task.stop();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testTaskMethod002
     * @tc.desc: Test create task when method is POST for download
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskMethod002', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        method: 'POST'
      }
      let task = await request.agent.create(context, conf);
      task.start().then(() => {
        expect(true).assertTrue();
        task.stop();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testTaskMethod003
     * @tc.desc: Test create task when method is number
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskMethod003', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        method: 123
      }
      let task = await request.agent.create(context, conf);
      await task.start().then(() => {
        expect(task.conf.method).assertEqual(123);
        task.stop();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testTaskMethod004
     * @tc.desc: Test create task when method is empty
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskMethod004', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        method: ''
      }
      let task = await request.agent.create(context, conf);
      await task.start().then(() => {
        expect(task.conf.method).assertEqual('');
        task.stop();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testTaskMethod005
     * @tc.desc: Test create task when method is GET for upload
     * @tc.type: FUNC
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
      await task.start().then(() => {
        expect(true).assertTrue();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
      await task.stop();
    })

    /**
     * @tc.name: testTaskMethod006
     * @tc.desc: Test create task when method is PUT for download
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskMethod006', 0, async function (done) {
      await sleep(2000);
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        method: 'PUT'
      }
      let task = await request.agent.create(context, conf);
      await task.start().then(() => {
        expect(true).assertTrue();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
      await task.stop();
    })


    /**
     * @tc.name: testTaskSaveas001
     * @tc.desc: Test create task when lack saveas is number for download
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskSaveas001', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        mode: request.agent.Mode.FOREGROUND,
        saveas: 123
      }
      let task = await request.agent.create(context, conf);
      task.on('completed', function() {
        if (fs.accessSync(`${cacheDir}/test.apk`)) {
          expect(true).assertTrue();
          done();
        }
      })
      task.start();
    })

    /**
     * @tc.name: testTaskData001
     * @tc.desc: Test create task when data lack name
     * @tc.type: FUNC
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
        expect(err != null && err.code === 401).assertTrue();
        done();
      }
    })

    /**
     * @tc.name: testTaskData002
     * @tc.desc: Test create task when data name is number
     * @tc.type: FUNC
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
        expect(err != null && err.code === 401).assertTrue();
        done();
      }
    })

    /**
     * @tc.name: testTaskData003
     * @tc.desc: Test create task when data lack value
     * @tc.type: FUNC
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
        expect(err != null && err.code === 401).assertTrue();
        done();
      }
    })

    /**
     * @tc.name: testTaskData004
     * @tc.desc: Test create task when data value is number
     * @tc.type: FUNC
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
        expect(err != null && err.code === 401).assertTrue();
        done();
      }
    })

    /**
     * @tc.name: testTaskData005
     * @tc.desc: Test create task when data path is '', path is not exits
     * @tc.type: FUNC
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
        expect(err != null && err.code === 401).assertTrue();
        done();
      }
    })

    /**
     * @tc.name: testTaskData006
     * @tc.desc: Test create task when data path is number
     * @tc.type: FUNC
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
        expect(err != null && err.code === 401).assertTrue();
        done();
      }
    })

    /**
     * @tc.name: testTaskData007
     * @tc.desc: Test create task when data path is not absolute path
     * @tc.type: FUNC
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
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.name: testTaskData008
     * @tc.desc: Test create task when data filename is number
     * @tc.type: FUNC
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
      request.agent.create(context, conf, (err, data) => {
        expect(err == null).assertTrue();
        request.agent.remove(data.tid);
        done();
      })
    })

    /**
     * @tc.name: testTaskData009
     * @tc.desc: Test create task when data mimeType is number
     * @tc.type: FUNC
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
      request.agent.create(context, conf, (err, data) => {
        expect(err == null).assertTrue();
        request.agent.remove(data.tid);
        done();
      })
    })

    /**
     * @tc.name: testTaskData010
     * @tc.desc: Test create task when data path and filename is different
     * @tc.type: FUNC
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
      request.agent.create(context, conf, (err, data) => {
        expect(err == null).assertTrue();
        request.agent.remove(data.tid);
        done();
      })
    })

    /**
     * @tc.name: testTaskData011
     * @tc.desc: Test create task when data two files for upload
     * @tc.type: FUNC
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
      request.agent.create(context, conf, (err, data) => {
        expect(err == null).assertTrue();
        request.agent.remove(data.tid);
        done();
      })
    })

    /**
     * @tc.name: testTaskData012
     * @tc.desc: Test create task when data value is string
     * @tc.type: FUNC
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
        expect(err != null && err.code === 401).assertTrue();
        done();
      }

    })

    /**
     * @tc.name: testTaskData013
     * @tc.desc: Test create task when data path and filename is same
     * @tc.type: FUNC
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
      request.agent.create(context, conf, (err, data) => {
        expect(err == null).assertTrue();
        request.agent.remove(data.tid);
        done();
      })
    })

    /**
     * @tc.name: testTaskNetwork001
     * @tc.desc: Test create task when network is 3
     * @tc.type: FUNC
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
      request.agent.create(context, conf).then((data) => {
        expect(true).assertTrue();
        request.agent.remove(data.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testTaskNetwork002
     * @tc.desc: Test create task when network is string
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskNetwork002', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        network: "ANY"
      }
      request.agent.create(context, conf).then((data) => {
        expect(true).assertTrue();
        request.agent.remove(data.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testTaskNetwork003
     * @tc.desc: Test create task when network is WIFI for DOWNLOAD
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskNetwork003', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        network: request.agent.Network.WIFI
      }
      request.agent.create(context, conf).then((data) => {
        expect(true).assertTrue();
        request.agent.remove(data.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testTaskNetwork004
     * @tc.desc: Test create task when network is any for UPLOAD
     * @tc.type: FUNC
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
      let task = await request.agent.create(context,  conf);
      task.start().then(() => {
        expect(true).assertTrue();
        task.stop();
        done();
      }).catch((err) => {
        expect(err == null && err.code == 13400003).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testTaskHeaders001
     * @tc.desc: Test create task when headers content-type is application/json but data is file for upload
     * @tc.type: FUNC
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
      await task.start().then(() => {
        expect(true).assertTrue();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
      await task.stop();
    })

    /**
     * @tc.name: testTaskHeaders002
     * @tc.desc: Test create task when lack headers for upload
     * @tc.type: FUNC
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
      await task.start().then(() => {
        expect(typeof task.conf.headers).assertEqual('undefined');
        task.stop();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testTaskHeaders003
     * @tc.desc: Test create task when lack headers for download
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskHeaders003', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
      }
      let task = await request.agent.create(context, conf);
      await task.start().then(() => {
        expect(typeof task.conf.headers).assertEqual('undefined');
        task.stop();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testTaskRetry001
     * @tc.desc: Test create task when retry is true for FOREGROUND
     * @tc.type: FUNC
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
      task.start().then(() => {
        expect(true).assertTrue();
        task.stop();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testTaskRetry002
     * @tc.desc: Test create task when retry is true for background
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskRetry002', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        mode: request.agent.Mode.BACKGROUND,
        retry: true,
        gauge: true
      }
      let task = await request.agent.create(context, conf);
      task.start().then(() => {
        expect(true).assertTrue();
        task.stop();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testTaskRetry003
     * @tc.desc: Test create task when retry is string
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskRetry003', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'http://xxx',
        mode: request.agent.Mode.FOREGROUND,
        retry: 'true',
        overwrite: true
      }
      let task = await request.agent.create(context, conf);
      task.on('failed', function () {
        expect(true).assertTrue();
        done();
      })
      task.start();
    })

    /**
     * @tc.name: testTaskRetry004
     * @tc.desc: Test create task when retry is false for FOREGROUND
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskRetry004', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test1.apk',
        mode: request.agent.Mode.FOREGROUND,
        retry: false
      }
      let task = await request.agent.create(context, conf);
      task.on('failed', function () {
        expect(true).assertTrue();
        done();
      })
      task.start();
    })

    /**
     * @tc.name: testTaskRetry005
     * @tc.desc: Test create task when retry is false for background
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskRetry005', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test1.apk',
        mode: request.agent.Mode.FOREGROUND,
        retry: false,
        overwrite: true
      }
      let task = await request.agent.create(context, conf);
      task.on('failed', function () {
        expect(true).assertTrue();
        done();
      })
      task.start();
    })

    /**
     * @tc.name: testTaskIndex001
     * @tc.desc: Test create task when index is string
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskIndex001', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        index: '0',
        mode: request.agent.Mode.FOREGROUND
      }
      let task = await request.agent.create(context, conf);
      task.on('completed', function() {
        expect(true).assertTrue();
        done();
      })
      await task.start();
    })

    /**
     * @tc.name: testTaskIndex002
     * @tc.desc: Test create task when index is 0 and begins greater than ends
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskIndex002', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        index: 0,
        begins: 10,
        ends: 5
      }
      let task = await request.agent.create(context, conf);
      task.start().then(() => {
        expect(true).assertTrue();
        task.stop();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testTaskIndex003
     * @tc.desc: Test create task when index is 1 but only one file for upload
     * @tc.type: FUNC
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
          expect(err != null && err.code == 401).assertTrue();
          done();
        })
      } catch (err) {
        expect(err != null && err.code == 401).assertTrue();
        done();
      }
    })

    /**
     * @tc.name: testTaskIndex004
     * @tc.desc: Test create task when index is 0 and begins is 5 and ends is 10 for download
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskIndex004', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://sf3-cn.feishucdn.com/obj/ee-appcenter/6d6bc5/Feishu-win32_ia32-5.10.6-signed.exe',
        mode: request.agent.Mode.FOREGROUND,
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
     * @tc.name: testTaskIndex005
     * @tc.desc: Test create task when index is 0 and begins is 5 and ends is 10 for upload
     * @tc.type: FUNC
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
        task.stop();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testTaskIndex006
     * @tc.desc: Test create task when index is 0 and begins is 5 and ends is not exists for download
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskIndex006', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        mode: request.agent.Mode.FOREGROUND,
        begins: 5,
        gauge: true
      }
      let task = await request.agent.create(context, conf);
      await task.start()
      task.on('failed', function(progress) {
        expect(true).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testTaskIndex007
     * @tc.desc: Test create task when index is 0 and begins is 5 and ends is not exists for upload
     * @tc.type: FUNC
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
        task.stop();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testTaskIndex008
     * @tc.desc: Test create task when index is 0 and begins is not exists and ends is 10 for download
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskIndex008', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://sf3-cn.feishucdn.com/obj/ee-appcenter/6d6bc5/Feishu-win32_ia32-5.10.6-signed.exe',
        mode: request.agent.Mode.FOREGROUND,
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
     * @tc.name: testTaskIndex009
     * @tc.desc: Test create task when index is 0 and begins is 5 and ends is not exists for upload
     * @tc.type: FUNC
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
        task.stop();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testTaskIndex010
     * @tc.desc: Test create task when index is 1 but have two files for upload
     * @tc.type: FUNC
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
        task.stop();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testTaskToken001
     * @tc.desc: Test create task when token is 7 bytes
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskToken001', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        token: 'abcdef'
      }
      request.agent.create(context, conf).then((data) => {
        expect(data.conf.token).assertEqual('abcdef');
        request.agent.remove(data.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testTaskToken002
     * @tc.desc: Test create task when token is 2049 bytes
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskToken002', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        token: 'a'.padStart(2049, 'a')
      }
      request.agent.create(context, conf).then((data) => {
        expect(data.conf.token).assertEqual('a'.padStart(2049, 'a'));
        request.agent.remove(data.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testTaskToken003
     * @tc.desc: Test create task when token is number
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskToken003', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        token: 666
      }
      request.agent.create(context, conf).then((data) => {
        expect(data.conf.token).assertEqual(666);
        request.agent.remove(data.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testTaskToken004
     * @tc.desc: Test create task when token is 2048 bytes
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskToken004', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        token: 'a'.padStart(2048, 'a')
      }
      request.agent.create(context, conf).then((data) => {
        expect(true).assertTrue();
        request.agent.remove(data.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testCreate001
     * @tc.desc: Test create for conf is string type
     * @tc.type: FUNC
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
  })
}