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
      }
      try {
        request.downloadFile(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        })
      } catch (err) {
        expect(err.code).assertEqual(13400002);
        done();
      }
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
     * @tc.desc: Test downloadFile when url length exceeds 2048.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testDownloadUrl005', 0, function (done) {
      let conf = {
        url: 'http://127.0.0.1/' + 'a'.repeat(2048),
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


  })
}