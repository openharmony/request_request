/*
 * Copyright (C) 2022 Huawei Device Co., Ltd.
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
import common from '@ohos.app.ability.common';

export default function RequestUploadFileTest() {
  describe('RequestUploadFileTest', () => {
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
    });

    let sleep: (ms: number) => Promise<void> = async (ms: number): Promise<void> => {
      return new Promise((resolve) => {
        setTimeout(resolve, ms);
      });
    };

    let context: common.UIAbilityContext = globalThis.abilityContext;
    let cacheDir: String = globalThis.cacheDir;
    let RequestData: request.RequestData = {
      name: 'name',
      value: '123'
    };
    let File: request.File = {
      filename: 'test',
      name: 'test',
      uri: 'internal://cache/test.txt',
      type: 'txt'
    };
    let File1: request.File = {
      filename: 'test1',
      name: 'test',
      uri: 'internal://cache/test1.txt',
      type: 'txt'
    };
    let File2: request.File = {
      filename: 'test2',
      name: 'test',
      uri: 'internal://cache/test2.txt',
      type: 'txt'
    };
    let uploadConfig: request.UploadConfig = {
      url: 'http://127.0.0.1',
      header: {
        headers: 'http'
      },
      method: 'POST',
      files: [File],
      data: [RequestData]
    };
    /**
     * @tc.number: testUploadUrl001
     * @tc.name: testUploadUrl001
     * @tc.desc: Test upload when lack url
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadUrl001', 0, (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: Record<string, string | Record<string, string> | Array<request.File> | Array<request.RequestData>> = {
        "header": {
          "Accept": "*/*"
        },
        "method": "POST",
        "files": [File],
        "data": [RequestData]
      };
      try {
        request.uploadFile(context, conf as Object as request.UploadConfig, (err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testUploadUrl002
     * @tc.name: testUploadUrl002
     * @tc.desc: Test upload when url is undefined
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadUrl002', 0, (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: Record<string, undefined | string | Record<string, string> | Array<request.File> | Array<request.RequestData>> = {
        "url": undefined,
        "header": {
          "Accept": "*/*"
        },
        "method": "POST",
        "files": [File],
        "data": [RequestData]
      };
      try {
        request.uploadFile(context, conf as Object as request.UploadConfig, (err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testUploadUrl003
     * @tc.name: testUploadUrl003
     * @tc.desc: Test upload when url is not support upload
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadUrl003', 0, (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.UploadConfig = {
        url: 'https://gitee.com/openharmony/request_request',
        header: {
          'Accept': '*/*'
        },
        method: 'POST',
        files: [File],
        data: [RequestData]
      };
      request.uploadFile(context, conf).then(async (data) => {
        expect(true).assertTrue();
        let task = data;
        await task.delete();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testUploadUrl004
     * @tc.name: testUploadUrl004
     * @tc.desc: Test upload when url is not http/https scheme
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadUrl004', 0, (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.UploadConfig = {
        url: 'admin://127.0.0.1',
        header: {
          'Accept': '*/*'
        },
        method: 'POST',
        files: [File],
        data: [RequestData]
      };
      try {
        request.uploadFile(context, conf, (err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testUploadUrl005
     * @tc.name: testUploadUrl005
     * @tc.desc: Test upload when url length is 8193
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadUrl005', 0, (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.UploadConfig = {
        url: 'http://127.0.0.1/' + 'a'.repeat(8176),
        header: {
          'Accept': '*/*'
        },
        method: 'POST',
        files: [File],
        data: [RequestData]
      };
      try {
        request.uploadFile(context, conf, (err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testUploadUrl006
     * @tc.name: testUploadUrl006
     * @tc.desc: Test upload when url is empty
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadUrl006', 0, (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.UploadConfig = {
        url: '',
        header: {
          'Accept': '*/*'
        },
        method: 'POST',
        files: [File],
        data: [RequestData]
      };
      try {
        request.uploadFile(context, conf, (err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testUploadUrl007
     * @tc.name: testUploadUrl007
     * @tc.desc: Test upload when url length is 8192
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadUrl007', 0, (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.UploadConfig = {
        url: 'http://127.0.0.1/' + 'a'.repeat(8175),
        header: {
          'Accept': '*/*'
        },
        method: 'POST',
        files: [File],
        data: [RequestData]
      };
      request.uploadFile(context, conf).then(async (data) => {
        expect(true).assertTrue();
        let task = data;
        await task.delete();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testUploadHeader001
     * @tc.name: testUploadHeader001
     * @tc.desc: Test upload when lack header
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadHeader001', 0, (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: Record<string, string | Array<request.File> | Array<request.RequestData>> = {
        "url": "http://127.0.0.1/",
        "method": "POST",
        "files": [File],
        "data": [RequestData]
      };
      try {
        request.uploadFile(context, conf as Object as request.UploadConfig, async (err, data) => {
          expect(true).assertTrue();
          let task = data;
          await task.delete();
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testUploadHeader002
     * @tc.name: testUploadHeader002
     * @tc.desc: Test upload when header is empty
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadHeader002', 0, (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.UploadConfig = {
        url: 'http://127.0.0.1/',
        header: {},
        method: 'POST',
        files: [File],
        data: [RequestData]
      };
      try {
        request.uploadFile(context, conf, async (err, data) => {
          expect(true).assertTrue();
          let task = data;
          await task.delete();
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testUploadMethod001
     * @tc.name: testUploadMethod001
     * @tc.desc: Test upload when method is POST
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadMethod001', 0, (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.UploadConfig = {
        url: 'http://127.0.0.1/',
        header: {
          'Accept': '*/*'
        },
        method: 'POST',
        files: [File],
        data: [RequestData]
      };
      try {
        request.uploadFile(context, conf, async (err, data) => {
          expect(true).assertTrue();
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testUploadMethod002
     * @tc.name: testUploadMethod002
     * @tc.desc: Test upload when method is PUT
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadMethod002', 0, (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.UploadConfig = {
        url: 'http://127.0.0.1/',
        header: {
          'Accept': '*/*'
        },
        method: 'PUT',
        files: [File],
        data: [RequestData]
      };
      try {
        request.uploadFile(context, conf, async (err, data) => {
          expect(true).assertTrue();
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testUploadIndex001
     * @tc.name: testUploadIndex001
     * @tc.desc: Test upload when index exceeds the total number of files
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadIndex001', 0, (done: Function) => {
      let file1 = fs.openSync(cacheDir + '/test1.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      let file2 = fs.openSync(cacheDir + '/test2.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file1);
      fs.closeSync(file2);
      let conf: request.UploadConfig = {
        url: 'http://127.0.0.1',
        header: {
          'Accept': '*/*'
        },
        index: 100,
        method: 'POST',
        files: [File1, File2],
        data: [RequestData]
      };
      try {
        request.uploadFile(context, conf, (err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testUploadIndex002
     * @tc.name: testUploadIndex002
     * @tc.desc: Test upload when index is negative number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadIndex002', 0, (done: Function) => {
      let file1 = fs.openSync(cacheDir + '/test1.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      let file2 = fs.openSync(cacheDir + '/test2.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file1);
      fs.closeSync(file2);
      let conf: request.UploadConfig = {
        url: 'http://127.0.0.1',
        header: {
          'Accept': '*/*'
        },
        index: -1,
        method: 'POST',
        files: [File1, File2],
        data: [RequestData]
      };
      try {
        request.uploadFile(context, conf, (err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testUploadIndex003
     * @tc.name: testUploadIndex003
     * @tc.desc: Test upload when index is correct number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadIndex003', 0, (done: Function) => {
      let file1 = fs.openSync(cacheDir + '/test1.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      let file2 = fs.openSync(cacheDir + '/test2.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file1);
      fs.closeSync(file2);
      let conf: request.UploadConfig = {
        url: 'http://127.0.0.1',
        header: {
          'Accept': '*/*'
        },
        index: 1,
        method: 'POST',
        files: [File1, File2],
        data: [RequestData]
      };
      request.uploadFile(context, conf).then(async (data) => {
        expect(true).assertTrue();
        let task = data;
        await task.delete();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testUploadBegins001
     * @tc.name: testUploadBegins001
     * @tc.desc: Test upload when begins is correct number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadBegins001', 0, (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.UploadConfig = {
        url: 'http://127.0.0.1',
        header: {
          'Accept': '*/*'
        },
        begins: 50,
        method: 'POST',
        files: [File],
        data: [RequestData]
      };
      request.uploadFile(context, conf).then(async (data) => {
        expect(true).assertTrue();
        let task = data;
        await task.delete();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testUploadEnds001
     * @tc.name: testUploadEnds001
     * @tc.desc: Test upload when ends is correct number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadEnds001', 0, (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.UploadConfig = {
        url: 'http://127.0.0.1',
        header: {
          'Accept': '*/*'
        },
        ends: 100,
        method: 'POST',
        files: [File],
        data: [RequestData]
      };
      request.uploadFile(context, conf).then(async (data) => {
        expect(true).assertTrue();
        let task = data;
        await task.delete();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testUploadFiles001
     * @tc.name: testUploadFiles001
     * @tc.desc: Test upload when lack files
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadFiles001', 0, (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: Record<string, string | Record<string, string> | Array<request.File> | Array<request.RequestData>> = {
        "url": "http://127.0.0.1",
        "header": {
          "Accept": "*/*"
        },
        "method": "POST",
        "data": [RequestData]
      };
      try {
        request.uploadFile(context, conf as Object as request.UploadConfig, (err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testUploadFiles002
     * @tc.name: testUploadFiles002
     * @tc.desc: Test upload when files is empty
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadFiles002', 0, (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.UploadConfig = {
        url: 'http://127.0.0.1',
        header: {
          'Accept': '*/*'
        },
        method: 'POST',
        files: [],
        data: [RequestData]
      };
      try {
        request.uploadFile(context, conf, (err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testUploadFiles003
     * @tc.name: testUploadFiles003
     * @tc.desc: Test upload when url of file is not exist
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadFiles003', 0, (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.UploadConfig = {
        url: 'http://127.0.0.1',
        header: {
          'Accept': '*/*'
        },
        method: 'POST',
        files: [
          {
            filename: 'test',
            name: 'test',
            uri: 'internal://cache/not-exist-test.txt',
            type: 'txt'
          }
        ],
        data: [RequestData]
      };
      try {
        request.uploadFile(context, conf, (err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(13400002);
        done();
      }
    });
    /**
     * @tc.number: testUploadFiles004
     * @tc.name: testUploadFiles004
     * @tc.desc: Test upload when file url is error path
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadFiles004', 0, (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.UploadConfig = {
        url: 'http://127.0.0.1',
        header: {
          'Accept': '*/*'
        },
        method: 'POST',
        files: [
          {
            filename: 'test',
            name: 'test',
            uri: 'internal://cache',
            type: 'txt'
          }
        ],
        data: [RequestData]
      };
      try {
        request.uploadFile(context, conf, (err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testUploadFiles005
     * @tc.name: testUploadFiles005
     * @tc.desc: Test upload when target file is dir not file
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadFiles005', 0, async (done: Function) => {
      let path = cacheDir + '/test9upDir';
      let conf: request.UploadConfig = {
        url: 'http://127.0.0.1',
        header: {
          'Accept': '*/*'
        },
        method: 'POST',
        files: [
          {
            filename: 'test',
            name: 'test',
            uri: 'internal://cache/test9upDir',
            type: 'txt'
          }
        ],
        data: [RequestData]
      };
      try {
        fs.mkdirSync(path);
        await request.uploadFile(context, conf).then(async () => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(13400002);
        fs.rmdirSync(path);
        expect(true).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testUploadFiles006
     * @tc.name: testUploadFiles006
     * @tc.desc: Test upload when file name is empty
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadFiles006', 0, (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.UploadConfig = {
        url: 'http://127.0.0.1',
        header: {
          'Accept': '*/*'
        },
        method: 'POST',
        data: [RequestData],
        files: [
          {
            filename: '',
            name: 'test',
            uri: 'internal://cache/test.txt',
            type: 'txt'
          }
        ]
      };
      try {
        request.uploadFile(context, conf, (data) => {
          expect(true).assertTrue();
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testUploadFiles007
     * @tc.name: testUploadFiles007
     * @tc.desc: Test upload when file is lack of name
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadFiles007', 0, (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.UploadConfig = {
        url: 'http://127.0.0.1',
        header: {
          'Accept': '*/*'
        },
        method: 'POST',
        data: [RequestData],
        files: [
          {
            filename: 'test',
            name: '',
            uri: 'internal://cache/test.txt',
            type: 'txt'
          }
        ]
      };
      try {
        request.uploadFile(context, conf, (data) => {
          expect(true).assertTrue();
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testUploadFiles008
     * @tc.name: testUploadFiles008
     * @tc.desc: Test upload when name is empty
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadFiles008', 0, (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.UploadConfig = {
        url: 'http://127.0.0.1',
        header: {
          'Accept': '*/*'
        },
        method: 'POST',
        data: [RequestData],
        files: [
          {
            filename: 'test',
            name: '',
            uri: 'internal://cache/test.txt',
            type: 'txt'
          }
        ]
      };
      try {
        request.uploadFile(context, conf, (data) => {
          expect(true).assertTrue();
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testUploadFiles009
     * @tc.name: testUploadFiles009
     * @tc.desc: Test upload when uri is not in internal://cache/
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadFiles009', 0, (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.UploadConfig = {
        url: 'http://127.0.0.1',
        header: {
          'Accept': '*/*'
        },
        method: 'POST',
        data: [RequestData],
        files: [
          {
            filename: 'test',
            name: 'test',
            uri: 'internal://test/test.txt',
            type: 'txt'
          }
        ]
      };
      try {
        request.uploadFile(context, conf, (data) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(true).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testUploadData001
     * @tc.name: testUploadData001
     * @tc.desc: Test upload when lack data
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadData001', 0, (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: Record<string, string | Record<string, string> | Array<request.File>> = {
        "url": "http://127.0.0.1",
        "header": {
          "Accept": "*/*"
        },
        "method": "POST",
        "files": [File],
      };
      try {
        request.uploadFile(context, conf as Object as request.UploadConfig, (err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testUploadData002
     * @tc.name: testUploadData002
     * @tc.desc: Test upload when data is empty
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadData002', 0, (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.UploadConfig = {
        url: 'http://127.0.0.1',
        header: {
          'Accept': '*/*'
        },
        method: 'POST',
        files: [File],
        data: []
      };
      try {
        request.uploadFile(context, conf, (err: Error) => {
          expect(true).assertTrue();
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testUploadFileTask001
     * @tc.name: testUploadFileTask001
     * @tc.desc: Test upload when param conf error
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadFileTask001', 0, (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      try {
        request.uploadFile(context, "conf" as Object as request.UploadConfig, (err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testUploadFileTask002
     * @tc.name: testUploadFileTask002
     * @tc.desc: Test upload for callback
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadFileTask002', 0, (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.UploadConfig = {
        url: 'http://127.0.0.1',
        header: {
          'Accept': '*/*'
        },
        method: 'POST',
        files: [File],
        data: [RequestData]
      };
      try {
        request.uploadFile(context, conf, async (err, data) => {
          expect(true).assertTrue();
          let task = data;
          await task.delete();
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testUploadFileTask003
     * @tc.name: testUploadFileTask003
     * @tc.desc: Test upload for promise
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadFileTask003', 0, (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.UploadConfig = {
        url: 'http://127.0.0.1',
        header: {
          'Accept': '*/*'
        },
        method: 'POST',
        files: [File],
        data: [RequestData]
      };
      try {
        request.uploadFile(context, conf).then(async (data) => {
          expect(true).assertTrue();
          let task = data;
          await task.delete();
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testUploadFileTaskOn001
     * @tc.name: testUploadFileTaskOn001
     * @tc.desc: Test uploadFile on task for error type like 'test'
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadFileTaskOn001', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.UploadConfig = {
        url: 'http://127.0.0.1',
        header: {
          'Accept': '*/*'
        },
        method: 'POST',
        files: [File],
        data: [RequestData]
      };
      let task = await request.uploadFile(context, conf);
      try {
        task.on('test' as Object as 'progress', () => {
        });
      } catch (err) {
        expect(true).assertTrue();
        await task.delete();
        done();
      }
    });
    /**
     * @tc.number: testUploadFileTaskOn002
     * @tc.name: testUploadFileTaskOn002
     * @tc.desc: Test uploadFile on task for error type like number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadFileTaskOn002', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.UploadConfig = {
        url: 'http://127.0.0.1',
        header: {
          'Accept': '*/*'
        },
        method: 'POST',
        files: [File],
        data: [RequestData]
      };
      let task = await request.uploadFile(context, conf);
      try {
        task.on(123 as Object as 'progress', () => {
        });
      } catch (err) {
        expect(true).assertTrue();
        await task.delete();
        done();
      }
    });
    /**
     * @tc.number: testUploadFileTaskOn003
     * @tc.name: testUploadFileTaskOn003
     * @tc.desc: Test uploadFile on task for fail
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadFileTaskOn002', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.UploadConfig = {
        url: 'http://127.0.0.1',
        header: {
          'Accept': '*/*'
        },
        method: 'POST',
        files: [File],
        data: [RequestData]
      };
      let task = await request.uploadFile(context, conf);
      try {
        task.on('fail', () => {
          expect(true).assertTrue();
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testUploadFileTaskDelete001
     * @tc.name: testUploadFileTaskDelete001
     * @tc.desc: Test uploadTask delete for promise
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadFileTaskDelete001', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.UploadConfig = {
        url: 'http://127.0.0.1',
        header: {
          'Accept': '*/*'
        },
        method: 'POST',
        files: [File],
        data: [RequestData]
      };
      let task = await request.uploadFile(context, conf);
      task.delete().then(() => {
        expect(true).assertTrue();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testUploadFileTaskDelete002
     * @tc.name: testUploadFileTaskDelete002
     * @tc.desc: Test uploadTask delete for callback
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testUploadFileTaskDelete002', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.UploadConfig = {
        url: 'http://127.0.0.1',
        header: {
          'Accept': '*/*'
        },
        method: 'POST',
        files: [File],
        data: [RequestData]
      };
      let task = await request.uploadFile(context, conf);
      task.delete(async (err: Error) => {
        expect(true).assertTrue();
        done();
      });
    });
  })
}
