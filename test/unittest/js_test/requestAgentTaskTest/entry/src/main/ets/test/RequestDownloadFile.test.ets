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

export default function requestDownloadFileTest() {
  describe('requestDownloadFileTest', function () {
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

    let globalDownloadConf: request.DownloadConfig = {
      url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      header: {
        headers: 'http'
      },
      enableMetered: false,
      enableRoaming: false,
      description: 'downloadV9 test!',
      networkType: request.NETWORK_WIFI,
      filePath: '/data/storage/el2/base/haps/entry/files/test' + new Date().getTime() + '.txt',
      title: 'downloadV9 test!',
      background: false
    };

    /**
     * @tc.number: testDownloadUrl001
     * @tc.name: testDownloadUrl001
     * @tc.desc: Test downloadFile when lack url
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadUrl001', 0, function (done) {
      let conf = {}
      try {
        request.downloadFile(context, conf, (err) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.number: testDownloadUrl002
     * @tc.name: testDownloadUrl002
     * @tc.desc: Test downloadFile when url is undefined
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadUrl002', 0, function (done) {
      let conf = {
        url: undefined,
      }
      try {
        request.downloadFile(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.number: testDownloadUrl003
     * @tc.name: testDownloadUrl003
     * @tc.desc: Test downloadFile when url is not support download
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadUrl003', 0, function (done) {
      let conf = {
        url: 'https://gitee.com/openharmony/request_request',
        filePath: 'test.txt',
      }
      request.downloadFile(context, conf).then(async (data) => {
        expect(true).assertTrue();
        let task = data;
        await task.delete();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testDownloadUrl004
     * @tc.name: testDownloadUrl004
     * @tc.desc: Test downloadFile when url is not http/https scheme.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadUrl004', 0, function (done) {
      let conf = {
        url: 'admin://127.0.0.1',
        filePath: 'test.txt',
      }
      try {
        request.downloadFile(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.number: testDownloadUrl005
     * @tc.name: testDownloadUrl005
     * @tc.desc: Test downloadFile when url length exceeds 8192.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadUrl005', 0, function (done) {
      let conf = {
        url: 'http://127.0.0.1/' + 'a'.repeat(8192),
      }
      try {
        request.downloadFile(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.number: testDownloadUrl006
     * @tc.name: testDownloadUrl006
     * @tc.desc: Test downloadFile when url is empty
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadUrl006', 0, function (done) {
      let conf = {
        url: '',
      }
      try {
        request.downloadFile(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.number: testDownloadFile001
     * @tc.name: testDownloadFile001
     * @tc.desc: Test create for conf is string type
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFile001', 0, async function (done) {
      try {
        await request.downloadFile(context, "conf");
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.number: testDownloadFile002
     * @tc.name: testDownloadFile002
     * @tc.desc: Test create for download again failed
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFile002', 0, async function (done) {
      let conf = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        filePath: 'testV9_file02.txt'
      }
      try {
        await request.downloadFile(context, conf);
        await sleep(2000);
        await request.downloadFile(context, conf);
        done();
      } catch (err) {
        expect(err.code).assertEqual(13400002);
        done();
      }
    })

    /**
     * @tc.number: testDownloadFileOn001
     * @tc.name: testDownloadFileOn001
     * @tc.desc: Test downloadFile task on for error type like 'test'
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFileOn001', 0, async function (done) {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      }
      let task = await request.downloadFile(context, conf);
      try {
        task.on('test', function () {
        });
      } catch (err) {
        expect(true).assertTrue();
        await task.delete();
        done();
      }
    })

    /**
     * @tc.number: testDownloadFileOn002
     * @tc.name: testDownloadFileOn002
     * @tc.desc: Test downloadFile task on for error type like number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFileOn002', 0, async function (done) {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      }
      let task = await request.downloadFile(context, conf);
      try {
        task.on(123, function () {
        });
      } catch (err) {
        expect(true).assertTrue();
        await task.delete();
        done();
      }
    })

    /**
     * @tc.number: testDownloadFileOn003
     * @tc.name: testDownloadFileOn003
     * @tc.desc: Test downloadFile task on for fail
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFileOn003', 0, async function (done) {
      let conf = {
        url: 'https://www.aaaaaaa.com/test.txt',
      }
      let task = await request.downloadFile(context, conf);
      try {
        task.on('fail', function () {
          expect(true).assertTrue();
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    })

    /**
     * @tc.number: testDownloadFileOn004
     * @tc.name: testDownloadFileOn004
     * @tc.desc: Test downloadFile task on for progress
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFileOn004', 0, async function (done) {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      }
      let task = await request.downloadFile(context, conf);
      try {
        task.on('progress', function (size, total) {
          expect(true).assertTrue();
          expect(total).assertEqual(1042003);
          task.off('progress');
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    })

    /**
     * @tc.number: testDownloadFileOn005
     * @tc.name: testDownloadFileOn005
     * @tc.desc: Test downloadFile task on for progress twice
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFileOn005', 0, async function (done) {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      }
      let task = await request.downloadFile(context, conf);
      try {
        task.on('progress', function (size, total) {
        });
        task.on('progress', function (size, total) {
        });
        expect(true).assertTrue();
        await task.delete();
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    })

    /**
     * @tc.number: testDownloadFileOn006
     * @tc.name: testDownloadFileOn006
     * @tc.desc: Test downloadFile task on for complete
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFileOn006', 0, async function (done) {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      }
      let task = await request.downloadFile(context, conf);
      try {
        task.on('complete', function () {
          expect(true).assertTrue();
          task.off('complete');
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    })

    /**
     * @tc.number: testDownloadFileOn007
     * @tc.name: testDownloadFileOn007
     * @tc.desc: Test downloadFile task on for pause
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFileOn007', 0, async function (done) {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      }
      let task = await request.downloadFile(context, conf);
      try {
        task.on('pause', function () {
          expect(true).assertTrue();
        });
        await sleep(1000);
        await task.suspend();
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    })

    /**
     * @tc.number: testDownloadFileOn008
     * @tc.name: testDownloadFileOn008
     * @tc.desc: Test downloadFile task on for remove
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFileOn008', 0, async function (done) {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      }
      let task = await request.downloadFile(context, conf);
      try {
        task.on('remove', function () {
          expect(true).assertTrue();
          done();
        });
        await task.delete();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    })

    /**
     * @tc.number: testDownloadFileOff001
     * @tc.name: testDownloadFileOff001
     * @tc.desc: Test downloadFile task off for error type like 'test'
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFileOff001', 0, async function (done) {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      }
      let task = await request.downloadFile(context, conf);
      try {
        task.off('test', function () {
        });
      } catch (err) {
        expect(true).assertTrue();
        await task.delete();
        done();
      }
    })

    /**
     * @tc.number: testDownloadFileOff002
     * @tc.name: testDownloadFileOff002
     * @tc.desc: Test downloadFile task off for error type like number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFileOff002', 0, async function (done) {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      }
      let task = await request.downloadFile(context, conf);
      try {
        task.off(123, function () {
        });
        done();
      } catch (err) {
        expect(true).assertTrue();
        await task.delete();
        done();
      }
    })

    /**
     * @tc.number: testDownloadFileOff003
     * @tc.name: testDownloadFileOff003
     * @tc.desc: Test downloadFile task off for complete
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFileOff003', 0, async function (done) {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      }
      let task = await request.downloadFile(context, conf);
      task.on('complete', function (progress) {
        expect(true).assertTrue();
        done();
      })

      function CompletedCallback() {
        expect(false).assertTrue();
        done();
      }

      task.on('complete', CompletedCallback);
      task.off('complete', CompletedCallback);
    })

    /**
     * @tc.number: testDownloadFileOff004
     * @tc.name: testDownloadFileOff004
     * @tc.desc: Test downloadFile task off task twice
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFileOff004', 0, async function (done) {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      }
      let task = await request.downloadFile(context, conf);
      try {
        task.off('complete');
        task.off('complete');
        expect(true).assertTrue();
        await task.delete();
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    })

    /**
     * @tc.number: testDownloadFileOff005
     * @tc.name: testDownloadFileOff005
     * @tc.desc: Test downloadFile task off for progress
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadFileOff005', 0, async function (done) {
      let filePath = cacheDir + '/test.txt';
      if (fs.accessSync(filePath)) {
        fs.unlinkSync(filePath);
      }
      let conf = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      }
      let task = await request.downloadFile(context, conf);
      try {
        task.on('progress', function (progress) {
          expect(true).assertTrue();
          done();
        })
        task.on('complete', function (progress) {
          expect(true).assertTrue();
          done();
        })

        function ProgressCallback(progress) {
          expect(false).assertTrue();
          done();
        }

        task.on('progress', ProgressCallback);
        task.off('progress', ProgressCallback);
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    })

    /**
     * @tc.number: testDownloadTaskDelete001
     * @tc.name: testDownloadTaskDelete001
     * @tc.desc: Test downloadTask delete for parameter error
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadTaskDelete001', 0, async function (done) {
      try {
        task.delete();
      } catch (err) {
        expect(true).assertTrue();
        done();
      }
    })

    /**
     * @tc.number: testDownloadTaskDelete002
     * @tc.name: testDownloadTaskDelete002
     * @tc.desc: Test downloadTask delete for promise
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadTaskDelete002', 0, async function (done) {
      let conf = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      }
      let task = await request.downloadFile(context, conf);
      task.delete().then(() => {
        expect(true).assertTrue();
        done();
      }).catch((err) => {
        expect(false).assertTrue();
        done();
      })
    })

    /**
     * @tc.number: testDownloadTaskDelete003
     * @tc.name: testDownloadTaskDelete003
     * @tc.desc: Test downloadTask delete for callback
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadTaskDelete003', 0, async function (done) {
      let conf = {
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
      }
      let task = await request.downloadFile(context, conf);
      task.delete(async (err) => {
        expect(true).assertTrue();
        done();
      })
    })

    
  })
}