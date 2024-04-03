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

export default function requestQueryTest() {
  describe('requestQueryTest', function () {
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
     * @tc.name: testShowTask001
     * @tc.desc: Test show task for tid is empty
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testShowTask001', 0, async function (done) {
      try {
        await request.agent.show("");
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.name: testShowTask002
     * @tc.desc: Test show task for tid is number
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testShowTask002', 0, async function (done) {
      try {
        await request.agent.show(123);
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.name: testShowTask003
     * @tc.desc: Test show task for tid is "123"
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testShowTask003', 0, async function (done) {
      request.agent.show("123", (err, info) => {
        expect(err.code).assertEqual(21900006);
        done();
      })
    })

    /**
     * @tc.name: testShowTask004
     * @tc.desc: Test show task for tid is "123"
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testShowTask004', 0, async function (done) {
      await request.agent.show("123").then((info) => {
        expect(false).assertTrue();
        done();
      }).catch((err) => {
        expect(err.code).assertEqual(21900006);
        done();
      })
    })

    /**
     * @tc.name: testShowTask005
     * @tc.desc: Test show task
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testShowTask005', 0, async function (done) {
      try {
        await request.agent.show();
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.name: testShowTask006
     * @tc.desc: Test show task
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testShowTask006', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        mode: request.agent.Mode.BACKGROUND,
        description: 'download task',
        saveas: `testShowTask006.txt`,
        data: 'download task'
      }
      let task = await request.agent.create(context, conf);
      request.agent.show(task.tid).then(async (info) => {
        expect(info.uid).assertEqual(undefined);
        expect(info.bundle).assertEqual(undefined);
        expect(info.url).assertEqual('https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk');
        expect(info.data).assertEqual('download task');
        expect(info.saveas).assertEqual("/data/storage/el2/base/haps/testModule/cache/testShowTask006.txt");
        expect(info.tid).assertEqual(task.tid);
        expect(info.title).assertEqual("download");
        expect(info.description).assertEqual('download task');
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
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testShowTask007
     * @tc.desc: Test show task
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testShowTask007', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `testShowTask007.txt`,
        mode: request.agent.Mode.BACKGROUND
      }
      let task = await request.agent.create(context, conf);
      request.agent.show(task.tid, async (err, info) => {
        expect(info.uid).assertEqual(undefined);
        expect(info.bundle).assertEqual(undefined);
        expect(info.url).assertEqual('https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk');
        expect(info.data).assertEqual("");
        expect(info.saveas).assertEqual("/data/storage/el2/base/haps/testModule/cache/testShowTask007.txt");
        expect(info.tid).assertEqual(task.tid);
        expect(info.title).assertEqual("download");
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
        done();
      })
    })

    /**
     * @tc.name: testShowTask008
     * @tc.desc: Test show task
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testShowTask008', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        mode: request.agent.Mode.BACKGROUND,
        saveas: `testShowTask008.txt`,
        token: "12345678"
      }
      let task = await request.agent.create(context, conf);
      request.agent.show(task.tid).then((info) => {
        expect(false).assertTrue();
        done();
      }).catch(async (err) => {
        expect(err.code).assertEqual(21900006);
        await request.agent.remove(task.tid);
        done();
      })
    })

    /**
     * @tc.name: testTouchTask001
     * @tc.desc: Test touch task for tid is empty
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTouchTask001', 0, async function (done) {
      try {
        await request.agent.touch("");
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.name: testTouchTask002
     * @tc.desc: Test touch task for tid is number
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTouchTask002', 0, async function (done) {
      try {
        await request.agent.touch(123, "12345678");
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.name: testTouchTask003
     * @tc.desc: Test touch task for token is number
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTouchTask003', 0, async function (done) {
      try {
        await request.agent.touch("123", 123);
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.name: testTouchTask004
     * @tc.desc: Test touch task for token length less than 8
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTouchTask004', 0, async function (done) {
      try {
        await request.agent.touch("123", "1234567");
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.name: testTouchTask005
     * @tc.desc: Test touch task for token length greater than 2048
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTouchTask005', 0, async function (done) {
      let token = 'a'.padStart(2049, 'a');
      try {
        await request.agent.touch("123", token);
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.name: testTouchTask006
     * @tc.desc: Test touch task for tid is "123"
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTouchTask006', 0, async function (done) {
      request.agent.touch("123", "12345678", (err, info) => {
        expect(err.code).assertEqual(21900006);
        done();
      })
    })

    /**
     * @tc.name: testTouchTask007
     * @tc.desc: Test touch task for token is "123456789"
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTouchTask007', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        mode: request.agent.Mode.BACKGROUND,
        saveas: `testTouchTask007.txt`,
        token: "12345678"
      }
      let task = await request.agent.create(context, conf);
      request.agent.touch(task.tid, "123456789").then((info) => {
        expect(false).assertTrue();
        done();
      }).catch(async (err) => {
        expect(err.code).assertEqual(21900006);
        await request.agent.remove(task.tid);
        done();
      })
    })

    /**
     * @tc.name: testTouchTask008
     * @tc.desc: Test touch task
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTouchTask008', 0, async function (done) {
      let file1 = fs.openSync(cacheDir + '/test1.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      let file2 = fs.openSync(cacheDir + '/test2.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file1);
      fs.closeSync(file2);
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [
                {name: 'file1', value: {path: `./test1.txt`, filename: 'test1.txt'}},
                {name: 'file2', value: {path: `./test2.txt`, filename: 'test2.txt'}},
                {name : 'key1', value: 'value1'}, {name : 'key2', value: 'value2'}
        ],
        mode: request.agent.Mode.FOREGROUND,
        token: "12345678"
      }
      let task = await request.agent.create(context, conf);
      request.agent.touch(task.tid, "12345678").then(async (info) => {
        expect(info.uid).assertEqual(undefined);
        expect(info.bundle).assertEqual(undefined);
        expect(info.url).assertEqual('http://127.0.0.1');
        expect(info.data.length).assertEqual(4);
        expect(info.saveas).assertEqual("");
        expect(info.tid).assertEqual(task.tid);
        expect(info.title).assertEqual("upload");
        expect(info.description).assertEqual("");
        expect(info.action).assertEqual(request.agent.Action.UPLOAD);
        expect(info.mode).assertEqual(request.agent.Mode.FOREGROUND);
        expect(info.mimeType).assertEqual("multipart/form-data");
        expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
        expect(info.progress.index).assertEqual(0);
        expect(info.progress.sizes.length).assertEqual(2);
        expect(info.faults).assertEqual(null);
        expect(info.gauge).assertEqual(false);
        expect(typeof info.ctime).assertEqual("number");
        expect(typeof info.mtime).assertEqual("number");
        expect(info.retry).assertEqual(false);
        expect(info.tries).assertEqual(0);
        expect(typeof info.reason).assertEqual("string");
        await request.agent.remove(task.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testTouchTask009
     * @tc.desc: Test touch task
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTouchTask009', 0, async function (done) {
      let file1 = fs.openSync(cacheDir + '/test1.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file1);
      let conf = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{name: 'file1', value: {path: `./test1.txt`, filename: 'test1.txt'}}],
        token: "12345678"
      }

      let task = await request.agent.create(context, conf);
      request.agent.touch(task.tid, "12345678", async (err, info) => {
        expect(info.uid).assertEqual(undefined);
        expect(info.bundle).assertEqual(undefined);
        expect(info.url).assertEqual('http://127.0.0.1');
        expect(info.data.length).assertEqual(1);
        expect(info.saveas).assertEqual("");
        expect(info.tid).assertEqual(task.tid);
        expect(info.title).assertEqual("upload");
        expect(info.description).assertEqual("");
        expect(info.action).assertEqual(request.agent.Action.UPLOAD);
        expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
        expect(info.mimeType).assertEqual("multipart/form-data");
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
        done();
      })
    })

    /**
     * @tc.name: testSearchTask001
     * @tc.desc: Test search task for tid is empty
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testSearchTask001', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `testSearchTask001.txt`,
      }
      let task = await request.agent.create(context, conf);
      request.agent.search(async (err, tids) => {
        expect(tids.length > 0).assertTrue();
        await request.agent.remove(task.tid);
        done();
      })
    })

    /**
     * @tc.name: testSearchTask002
     * @tc.desc: Test search task for tid is number
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testSearchTask002', 0, async function (done) {
      try {
        await request.agent.search(123);
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })


    /**
     * @tc.name: testSearchTask003
     * @tc.desc: Test search task for bundle is "123"
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testSearchTask003', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `testSearchTask003.txt`,
      }
      let task = await request.agent.create(context, conf);
      let filter = {
        bundle: "123",
      }
      request.agent.search(filter).then(async (tids) => {
        expect(tids.length > 0).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testSearchTask004
     * @tc.desc: Test search task for filter is {}
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testSearchTask004', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `testSearchTask004.txt`,
      }
      let task = await request.agent.create(context, conf);
      request.agent.search({}, async (err, tids) => {
        expect(tids.length > 0).assertTrue();
        await request.agent.remove(task.tid);
        done();
      })
    })

    /**
     * @tc.name: testSearchTask005
     * @tc.desc: Test search task for filter is 123
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testSearchTask005', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `testSearchTask005.txt`,
      }
      let task = await request.agent.create(context, conf);
      let filter = {
        bundle: 123,
      }
      request.agent.search(filter).then(async (tids) => {
        expect(tids.length > 0).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testSearchTask006
     * @tc.desc: Test search task
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testSearchTask006', 0, async function (done) {
      let filter = {
        before: Date.parse("2023-7-9 00:00:00"),
        after: Date.parse("2023-7-9 23:00:00"),
      }
      try {
        await request.agent.search(filter);
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.name: testSearchTask007
     * @tc.desc: Test search task
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testSearchTask007', 0, async function (done) {
      let filter = {
        before: Date.parse("2023-7-9 23:00:00"),
        after: Date.parse("2023-7-9 00:00:00"),
      }
      request.agent.search(filter).then((tids) => {
        expect(tids.length).assertEqual(0);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testSearchTask008
     * @tc.desc: Test search task
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testSearchTask008', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `testSearchTask008.txt`,
      }
      let task = await request.agent.create(context, conf);
      let filter = {
        before: Date.parse(new Date()),
      }
      request.agent.search(filter).then(async (tids) => {
        expect(tids.length > 0).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testSearchTask009
     * @tc.desc: Test search task for filter is 123
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testSearchTask009', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `testSearchTask009.txt`,
      }
      let task = await request.agent.create(context, conf);
      let filter = {
        after: Date.parse("2023-7-9 00:00:00"),
      }
      request.agent.search(filter).then(async (tids) => {
        expect(tids.length > 0).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testSearchTask010
     * @tc.desc: Test search task for before and after type is string
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testSearchTask010', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `testSearchTask010.txt`,
      }
      let task = await request.agent.create(context, conf);
      let filter = {
        before:"2023-7-9 23:00:00",
        after: "2023-7-9 00:00:00",
      }
      request.agent.search(filter).then(async (tids) => {
        expect(tids.length > 0).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testSearchTask011
     * @tc.desc: Test search task for state is "0"
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testSearchTask011', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `testSearchTask011.txt`,
      }
      let task = await request.agent.create(context, conf);
      let filter = {
        state: "0",
        action: "1",
        mode: "2"
      }
      request.agent.search(filter).then(async (tids) => {
        expect(tids.length > 0).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testSearchTask012
     * @tc.desc: Test search task for state is INITIALIZED
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testSearchTask012', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `testSearchTask012.txt`,
      }
      let task = await request.agent.create(context, conf);
      let filter = {
        state: request.agent.State.INITIALIZED
      }
      request.agent.search(filter).then(async (tids) => {
        expect(tids.length > 0).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testSearchTask013
     * @tc.desc: Test search task for Action is DOWNLOAD
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testSearchTask013', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `testSearchTask013.txt`,
      }
      let task = await request.agent.create(context, conf);
      let filter = {
        action: request.agent.Action.DOWNLOAD
      }
      request.agent.search(filter).then(async (tids) => {
        expect(tids.length > 0).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testSearchTask014
     * @tc.desc: Test search task for mode is BACKGROUND
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testSearchTask014', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `testSearchTask014.txt`,
      }
      let task = await request.agent.create(context, conf);
      let filter = {
        mode: request.agent.Mode.BACKGROUND
      }
      request.agent.search(filter).then(async (tids) => {
        expect(tids.length > 0).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testSearchTask015
     * @tc.desc: Test search task for bundle is "*"
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testSearchTask015', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `testSearchTask015.txt`,
      }
      let task = await request.agent.create(context, conf);
      let filter = {
        bundle: "*"
      }
      request.agent.search(filter).then(async (tids) => {
        expect(tids.length > 0).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testSearchTask016
     * @tc.desc: Test search task for filter is empty
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testSearchTask016', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `testSearchTask016.txt`,
      }
      let task = await request.agent.create(context, conf);
      request.agent.search().then(async (tids) => {
        expect(tids.length > 0).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testSearchTask017
     * @tc.desc: Test search task for filter is undefined
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testSearchTask017', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `testSearchTask017.txt`,
      }
      let task = await request.agent.create(context, conf);
      request.agent.search(undefined).then(async (tids) => {
        expect(tids.length > 0).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testSearchTask018
     * @tc.desc: Test search task for filter is null
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testSearchTask018', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `testSearchTask018.txt`,
      }
      let task = await request.agent.create(context, conf);
      request.agent.search(null, async (err, tids) => {
        expect(tids.length > 0).assertTrue();
        await request.agent.remove(task.tid);
        done();
      })
    })

    /**
     * @tc.name: testQueryTask001
     * @tc.desc: Test query task for tid is "123"
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testQueryTask001', 0, async function (done) {
      request.agent.query("123", (err, info) => {
        expect(err.code).assertEqual(202);
        done();
      })
    })
  })
}