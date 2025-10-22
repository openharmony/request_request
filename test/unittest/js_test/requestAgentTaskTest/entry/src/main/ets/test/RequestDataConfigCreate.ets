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
import wantAgent, { WantAgent } from '@ohos.app.ability.wantAgent';

export default function requestDataConfigCreateTest() {
  describe('requestDataConfigCreateTest', () => {
    beforeAll(() => {
      console.info('beforeAll called');
    });
    afterAll(() => {
      console.info('afterAll called');
    });
    beforeEach(() => {
      console.info('beforeEach called');
      if (fs.accessSync(cacheDir + '/test.txt')) {
        fs.unlinkSync(cacheDir + '/test.txt');
      }
      if (fs.accessSync(cacheDir + '/test1.txt')) {
        fs.unlinkSync(cacheDir + '/test1.txt');
      }
    });
    afterEach(async () => {
      console.info('afterEach called');
      if (fs.accessSync(cacheDir + '/test.txt')) {
        fs.unlinkSync(cacheDir + '/test.txt');
      }
      if (fs.accessSync(cacheDir + '/test1.txt')) {
        fs.unlinkSync(cacheDir + '/test1.txt');
      }
    });

    let context: common.UIAbilityContext = globalThis.abilityContext;
    let cacheDir: String = globalThis.cacheDir;
    let fileSpec: request.agent.FileSpec = {
      path: `./test.txt`
    };
    let formItem: request.agent.FormItem = {
      name: 'file',
      value: fileSpec
    };
    let fileSpec1: request.agent.FileSpec = {
      filename: 't1.txt',
      path: `./test.txt`
    };
    let fileSpec2: request.agent.FileSpec = {
      filename: 't2.txt',
      path: `./test.txt`
    };
    let fileSpec3: request.agent.FileSpec = {
      filename: 't3.txt',
      path: `./test.txt`
    };
    let formItem1: request.agent.FormItem = {
      name: 'file',
      value: [fileSpec1, fileSpec2, fileSpec3]
    };

    let wantAgentInfo: wantAgent.WantAgentInfo = {
      wants: [
        {
          deviceId: '',
          bundleName: 'com.example.request',
          abilityName: 'EntryAbility',
          action: '',
          entities: [],
          uri: '',
          parameters: {}
        }
      ],
      actionType: wantAgent.OperationType.START_ABILITY,
      requestCode: 0,
      wantAgentFlags:[wantAgent.WantAgentFlags.CONSTANT_FLAG]
    };

    /**
     * @tc.number: testTaskData001
     * @tc.name: testTaskData001
     * @tc.desc: Test create task when data lack name
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskData001', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: Record<string, string | number | Array<Record<string, Record<string, string>>>> = {
        "action": request.agent.Action.UPLOAD,
        "url": "http://127.0.0.1",
        "data": [{
          "value": {
            "path": "./test.txt"
          },
        }]
      };
      try {
        request.agent.create(context, conf as Object as request.agent.Config, (err, data) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testTaskData002
     * @tc.name: testTaskData002
     * @tc.desc: Test create task when data name is number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskData002', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: Record<string, string | number | Array<Record<string, number | Record<string, string>>>> = {
        "action": request.agent.Action.UPLOAD,
        "url": "http://127.0.0.1",
        "data": [{
          "name": 123,
          "value": {
            "path": "./test.txt"
          }
        }]
      };
      try {
        request.agent.create(context, conf as Object as request.agent.Config, (err, data) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testTaskData003
     * @tc.name: testTaskData003
     * @tc.desc: Test create task when data lack value
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskData003', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: Record<string, string | number | Array<Record<string, string>>> = {
        "action": request.agent.Action.UPLOAD,
        "url": "http://127.0.0.1",
        "data": [{
          "name": "test"
        }]
      };
      try {
        request.agent.create(context, conf as Object as request.agent.Config, (err, data) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testTaskData004
     * @tc.name: testTaskData004
     * @tc.desc: Test create task when data value is number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskData004', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: Record<string, string | number | Array<Record<string, string | number>>> = {
        "action": request.agent.Action.UPLOAD,
        "url": "http://127.0.0.1",
        "data": [{
          "name": "file",
          "value": 123
        }]
      };
      try {
        request.agent.create(context, conf as Object as request.agent.Config, (err, data) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testTaskData005
     * @tc.name: testTaskData005
     * @tc.desc: Test create task when data path is '', path is not exits
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskData005', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
          name: 'file',
          value: {
            path: ''
          }
        }]
      };
      try {
        request.agent.create(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testTaskData006
     * @tc.name: testTaskData006
     * @tc.desc: Test create task when data path is number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskData006', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: Record<string, string | number | Array<Record<string, string | Record<string, number>>>> = {
        "action": request.agent.Action.UPLOAD,
        "url": "http://127.0.0.1",
        "data": [{
          "name": "file",
          "value": {
            "path": 123
          }
        }]
      };
      try {
        request.agent.create(context, conf as Object as request.agent.Config, (err, data) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testTaskData007
     * @tc.name: testTaskData007
     * @tc.desc: Test create task when data path is not absolute path
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskData007', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
          name: 'file',
          value: {
            path: `${cacheDir}/test.txt`
          }
        }]
      };
      try {
        request.agent.create(context, conf, (err) => {
          expect(true).assertTrue();
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testTaskData008
     * @tc.name: testTaskData008
     * @tc.desc: Test create task when data filename is number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskData008', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: Record<string, string | number | Array<Record<string, string | Record<string, string | number>>>> = {
        "action": request.agent.Action.UPLOAD,
        "url": "http://127.0.0.1",
        "data": [{
          "name": "file",
          "value": {
            "path": "./test.txt",
            "filename": 123
          }
        }]
      };
      request.agent.create(context, conf as Object as request.agent.Config, async (err, data) => {
        expect(err === null).assertTrue();
        await request.agent.remove(data.tid);
        done();
      });
    });
    /**
     * @tc.number: testTaskData009
     * @tc.name: testTaskData009
     * @tc.desc: Test create task when data mimeType is number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskData009', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: Record<string, string | number | Array<Record<string, string | Record<string, number | string>>>> = {
        "action": request.agent.Action.UPLOAD,
        "url": "http://127.0.0.1",
        "data": [{
          "name": "file",
          "value": {
            "path": "./test.txt",
            "mimeType": 123
          }
        }]
      };
      request.agent.create(context, conf as Object as request.agent.Config, async (err, data) => {
        expect(err === null).assertTrue();
        await request.agent.remove(data.tid);
        done();
      });
    });
    /**
     * @tc.number: testTaskData010
     * @tc.name: testTaskData010
     * @tc.desc: Test create task when data path and filename is different
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskData010', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
          name: 'file',
          value: {
            path: `./test.txt`,
            filename: 'a.txt'
          }
        }]
      };
      request.agent.create(context, conf, async (err, data) => {
        expect(err === null).assertTrue();
        await request.agent.remove(data.tid);
        done();
      });
    });
    /**
     * @tc.number: testTaskData011
     * @tc.name: testTaskData011
     * @tc.desc: Test create task when data two files for upload
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskData011', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
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
      };
      request.agent.create(context, conf, async (err, data) => {
        expect(err === null).assertTrue();
        await request.agent.remove(data.tid);
        done();
      });
    });
    /**
     * @tc.number: testTaskData012
     * @tc.name: testTaskData012
     * @tc.desc: Test create task when data value is string
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskData012', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
          name: 'test',
          value: 'test'
        }]
      };
      try {
        request.agent.create(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testTaskData013
     * @tc.name: testTaskData013
     * @tc.desc: Test create task when data path and filename is same
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskData013', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
          name: 'file',
          value: {
            path: `./test.txt`,
            filename: 'test.txt'
          }
        }]
      };
      request.agent.create(context, conf, async (err, data) => {
        expect(err === null).assertTrue();
        await request.agent.remove(data.tid);
        done();
      });
    });
    /**
     * @tc.number: testTaskData014
     * @tc.name: testTaskData014
     * @tc.desc: Test create download task when data is string
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskData014', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskData014.txt',
        mode: request.agent.Mode.FOREGROUND,
        data: 'aaaaa',
      };
      request.agent.create(context, conf, async (err, data) => {
        expect(err === null).assertTrue();
        await request.agent.remove(data.tid);
        done();
      });
    });
    /**
     * @tc.number: testTaskData015
     * @tc.name: testTaskData015
     * @tc.desc: Test create upload task when data is string
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskData015', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: 'aaaaa',
      };
      try {
        request.agent.create(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testTaskData016
     * @tc.name: testTaskData016
     * @tc.desc: Test create task when data path is `zcszczvxdvvadZ`
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskData016', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
          name: 'file',
          value: {
            path: `zcszczvxdvvadZ`,
            filename: 'test.txt'
          }
        }]
      };
      try {
        request.agent.create(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(13400001);
        done();
      }
    });
    /**
     * @tc.number: testTaskData017
     * @tc.name: testTaskData017
     * @tc.desc: Test create download task when data is string
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskData017', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskData017.txt',
        mode: request.agent.Mode.FOREGROUND,
        data: [{
          name: 'file',
          value: {
            path: `./test.txt`,
            filename: 'test.txt'
          }
        }]
      };
      try {
        request.agent.create(context, conf, (err, data) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testTaskData018
     * @tc.name: testTaskData018
     * @tc.desc: Test create upload task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskData018', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let fileSpecD18: Array<Record<string, string>> = [];
      let formItemD18: Array<Record<string, string | Array<Record<string, string>>>> = [];
      for (let i = 0; i < 100; i++) {
        let file: Record<string, string> = {
          "path": "./test.txt",
          "filename": `td${i}.txt`
        };
        fileSpecD18.push(file);
      }
      formItemD18.push({
        "name": "testTaskData018", "value": fileSpecD18
      });
      let conf: Record<string, string | number | Array<Record<string, string | Array<Record<string, string>>>>> = {
        "action": request.agent.Action.UPLOAD,
        "url": "http://127.0.0.1",
        "data": formItemD18,
      };
      try {
        request.agent.create(context, conf as Object as request.agent.Config, (err, data) => {
          expect(true).assertTrue();
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testTaskData019
     * @tc.name: testTaskData019
     * @tc.desc: Test create upload task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskData019', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let fileSpecD19: Array<Record<string, string>> = [];
      let formItemD19: Array<Record<string, string | Array<Record<string, string>>>>= [];
      for (let i = 0; i < 101; i++) {
        let file: Record<string, string> = {
          "path": "./test.txt",
          "filename": `td${i}.txt`
        };
        fileSpecD19.push(file);
      }
      formItemD19.push({
        "name": "testTaskData019", "value": fileSpecD19
      });
      let conf: Record<string, string | number | Array<Record<string, string | Array<Record<string, string>>>>> = {
        "action": request.agent.Action.UPLOAD,
        "url": "http://127.0.0.1",
        "data": formItemD19,
      };
      try {
        request.agent.create(context, conf as Object as request.agent.Config, (err, data) => {
          expect(true).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testTaskData020
     * @tc.name: testTaskData020
     * @tc.desc: Test create task when data path is internal pattern
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskData020', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
          name: 'file',
          value: {
            path: `internal://cache/test.txt`
          }
        }]
      };
      try {
        request.agent.create(context, conf, (err) => {
          expect(true).assertTrue();
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testTaskData021
     * @tc.name: testTaskData021
     * @tc.desc: Test create task when data path is application pattern
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskData021', 0, async (done: Function) => {
      let context: common.UIAbilityContext = globalThis.abilityContext.getApplicationContext();
      let dir: String = context.cacheDir;
      let file = fs.openSync(dir + '/data_021', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
          name: 'file',
          value: {
            path: '/data/storage/../../data/storage/el1/../el2/base/cache/data_021'
          }
        }]
      };
      try {
        await request.agent.create(context, conf).then(async () => {
          if (fs.accessSync(dir + '/data_021')) {
            fs.unlinkSync(dir + '/data_021');
          }
          expect(true).assertTrue();
          done();
        }).catch((err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testTaskData022
     * @tc.name: testTaskData022
     * @tc.desc: Test create task when data path is file pattern
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskData022', 0, async (done: Function) => {
      let context: common.UIAbilityContext = globalThis.abilityContext.getApplicationContext();
      let dir: String = context.cacheDir;
      let file = fs.openSync(dir + '/data_022', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
          name: 'file',
          value: {
            path: 'file://com.acts.request/data/storage/el2/base/cache/data_022'
          }
        }]
      };
      try {
        await request.agent.create(context, conf).then(async () => {
          if (fs.accessSync(dir + '/data_022')) {
            fs.unlinkSync(dir + '/data_022');
          }
          expect(true).assertTrue();
          done();
        }).catch((err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testTaskData023
     * @tc.name: testTaskData023
     * @tc.desc: Test create task when data path is from public directory
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskData023', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        mode: request.agent.Mode.FOREGROUND,
        data: [{
          name: 'file',
          value: {
            path: 'file://media/Photo/1/test.txt'
          }
        }]
      };
      try {
        request.agent.create(context, conf, (err) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(13400001);
        done();
      }
    });
    /**
     * @tc.number: testTaskData024
     * @tc.name: testTaskData024
     * @tc.desc: Test create task when data contentType is string
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskData024', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
          name: 'file',
          value: {
            path: `${cacheDir}/test.txt`,
            contentType: 'test'
          }
        }]
      };
      try {
        request.agent.create(context, conf, (err) => {
          expect(true).assertTrue();
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
  })
}