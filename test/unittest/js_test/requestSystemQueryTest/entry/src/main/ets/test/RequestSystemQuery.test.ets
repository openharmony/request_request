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

export default function requestSystemQueryTest() {
  describe('requestSystemQueryTest', function () {
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
     * @tc.name: testSystemSearchTask001
     * @tc.desc: Test search task for bundle is "123"
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testSystemSearchTask001', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `testSystemSearchTask001.txt`,
      }
      let task = await request.agent.create(context, conf);
      let filter = {
        bundle: "123",
      }
      request.agent.search(filter).then(async (tids) => {
        expect(tids.length).assertEqual(0);
        await request.agent.remove(task.tid);
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.name: testSystemSearchTask002
     * @tc.desc: Test search task for filter is {}
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testSystemSearchTask002', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `testSystemSearchTask002.txt`,
      }
      let task = await request.agent.create(context, conf);
      request.agent.search({}, async (err, tids) => {
        expect(tids.length > 0).assertTrue();
        await request.agent.remove(task.tid);
        done();
      })
    })

    /**
     * @tc.name: testSystemSearchTask003
     * @tc.desc: Test search task for filter is 123
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testSystemSearchTask003', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `testSystemSearchTask003.txt`,
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
     * @tc.name: testSystemSearchTask004
     * @tc.desc: Test search task for bundle is "*"
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testSystemSearchTask004', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `testSystemSearchTask004.txt`,
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
     * @tc.name: testSystemQueryTask001
     * @tc.desc: Test query task for tid is empty
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testSystemQueryTask001', 0, async function (done) {
      try {
        await request.agent.query("");
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.name: testSystemQueryTask002
     * @tc.desc: Test query task for tid is number
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testSystemQueryTask002', 0, async function (done) {
      try {
        await request.agent.query(123);
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.name: testSystemQueryTask003
     * @tc.desc: Test query task for tid is "123"
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testSystemQueryTask003', 0, async function (done) {
      request.agent.query("123", (err, info) => {
        expect(err.code).assertEqual(21900006);
        done();
      })
    })

    /**
     * @tc.name: testSystemQueryTask004
     * @tc.desc: Test query task for tid is "123"
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testSystemQueryTask004', 0, async function (done) {
      await request.agent.query("123").then((info) => {
        expect(false).assertTrue();
        done();
      }).catch((err) => {
        expect(err.code).assertEqual(21900006);
        done();
      })
    })

    /**
     * @tc.name: testSystemQueryTask005
     * @tc.desc: Test query task
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testSystemQueryTask005', 0, async function (done) {
      try {
        await request.agent.query();
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.name: testSystemQueryTask006
     * @tc.desc: Test query task
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testSystemQueryTask006', 0, async function (done) {
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
        mode:request.agent.Mode.FOREGROUND
      }
      let task = await request.agent.create(context, conf);
      request.agent.query(task.tid).then(async (info) => {
        expect(info.uid.length > 0).assertTrue();
        expect(info.bundle).assertEqual("com.acts.request");
        expect(info.url).assertEqual("");
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
     * @tc.name: testSystemQueryTask007
     * @tc.desc: Test show task
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testSystemQueryTask007', 0, async function (done) {
      let conf = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        saveas: `testSystemQueryTask007.txt`,
        mode: request.agent.Mode.BACKGROUND
      }
      let task = await request.agent.create(context, conf);
      request.agent.query(task.tid, async (err, info) => {
        expect(info.uid.length > 0).assertTrue();
        expect(info.bundle).assertEqual("com.acts.request");
        expect(info.url).assertEqual("");
        expect(info.data).assertEqual("");
        expect(info.saveas).assertEqual("/data/storage/el2/base/haps/testModule/cache/testSystemQueryTask007.txt");
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
  })
}