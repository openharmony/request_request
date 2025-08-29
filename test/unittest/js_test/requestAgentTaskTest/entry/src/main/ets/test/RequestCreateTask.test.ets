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
import { describe, beforeAll, beforeEach, afterEach, afterAll, it, expect } from "@ohos/hypium"
import request from '@ohos.request';
import fs from '@ohos.file.fs';
import common from '@ohos.app.ability.common';

export default function requestCreateTaskTest() {
  describe('requestCreateTaskTest', () => {
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
    /**
     * @tc.number: testTaskAction001
     * @tc.name: testTaskAction001
     * @tc.desc: Test create task when lack action
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskAction001', 0, (done: Function) => {
      let conf: Record<string, string> = {
        "url": "http://127.0.0.1"
      };
      try {
        request.agent.create(context, conf as Object as request.agent.Config, (err) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testTaskAction002
     * @tc.name: testTaskAction002
     * @tc.desc: Test create task when action is string
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskAction002', 0, async (done: Function) => {
      let conf: Record<string, string> = {
        "action": "UPLOAD",
        "url": "http://127.0.0.1"
      };
      try {
        request.agent.create(context, conf as Object as request.agent.Config, (err) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.message.toLowerCase().includes('action')).assertTrue();
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testTaskAction003
     * @tc.name: testTaskAction003
     * @tc.desc: Test create task when action is 2
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskAction003', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: 2,
        url: 'http://127.0.0.1'
      };
      try {
        request.agent.create(context, conf, (err) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testTaskAction004
     * @tc.name: testTaskAction004
     * @tc.desc: Test create task when action is UPLOAD
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskAction004', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [formItem]
      };
      request.agent.create(context, conf, async (err, data) => {
        expect(err === null).assertTrue();
        await request.agent.remove(data.tid);
        done();
      });
    });
    /**
     * @tc.number: testTaskAction005
     * @tc.name: testTaskAction005
     * @tc.desc: Test create task when action is DOWNLOAD
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskAction005', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: `./`,
        gauge: true
      };
      request.agent.create(context, conf).then(async (data) => {
        if (fs.accessSync(`${cacheDir}/test.txt`)) {
          expect(true).assertTrue();
          await request.agent.remove(data.tid);
          done();
        }
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskAction006
     * @tc.name: testTaskAction006
     * @tc.desc: Test create task when action is DOWNLOAD
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskAction006', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: 0,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: `./`,
        gauge: true
      };
      request.agent.create(context, conf).then(async (data) => {
        if (fs.accessSync(`${cacheDir}/test.txt`)) {
          expect(true).assertTrue();
          await request.agent.remove(data.tid);
          done();
        }
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskAction007
     * @tc.name: testTaskAction007
     * @tc.desc: Test create task when action is 1
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskAction007', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: 1,
        url: 'http://127.0.0.1',
        data: [formItem]
      };
      request.agent.create(context, conf, async (err, data) => {
        expect(err === null).assertTrue();
        await request.agent.remove(data.tid);
        done();
      });
    });
    /**
     * @tc.number: testTaskUrl001
     * @tc.name: testTaskUrl001
     * @tc.desc: Test create task when lack url
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskUrl001', 0, async (done: Function) => {
      let conf: Record<string, string | number> = {
        "action": request.agent.Action.DOWNLOAD,
      };
      try {
        request.agent.create(context, conf as Object as request.agent.Config, (err) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testTaskUrl002
     * @tc.name: testTaskUrl002
     * @tc.desc: Test create task when url is empty
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskUrl002', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: '',
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
     * @tc.number: testTaskUrl003
     * @tc.name: testTaskUrl003
     * @tc.desc: Test create task when url is not support download
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskUrl003', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/openharmony/request_request',
      };
      request.agent.create(context, conf, async (err, data) => {
        expect(err === null).assertTrue();
        await request.agent.remove(data.tid);
        done();
      });
    });
    /**
     * @tc.number: testTaskUrl004
     * @tc.name: testTaskUrl004
     * @tc.desc: Test create task when url is not support upload
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskUrl004', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'https://gitee.com/openharmony/request_request',
        data: [formItem]
      };
      request.agent.create(context, conf, async (err, data) => {
        expect(err === null).assertTrue();
        await request.agent.remove(data.tid);
        done();
      });
    });
    /**
     * @tc.number: testTaskUrl005
     * @tc.name: testTaskUrl005
     * @tc.desc: Test create task when url is not http/https scheme.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskUrl005', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'admin://127.0.0.1',
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
     * @tc.number: testTaskUrl006
     * @tc.name: testTaskUrl006
     * @tc.desc: Test create task when url length is 8193.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskUrl006', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'http://127.0.0.1/' + 'a'.repeat(8176),
        saveas: 'testTaskUrl006'
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
     * @tc.number: testTaskUrl007
     * @tc.name: testTaskUrl007
     * @tc.desc: Test create task when url length is 8192.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskUrl007', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'http://127.0.0.1/' + 'a'.repeat(8175),
        saveas: 'testTaskUrl007'
      };
      try {
        request.agent.create(context, conf, async (err, data) => {
          expect(true).assertTrue();
          await request.agent.remove(data.tid);
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testTaskUrl008
     * @tc.name: testTaskUrl008
     * @tc.desc: Test upload when url length is 8192
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskUrl008', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1/' + 'a'.repeat(8175),
        data: [formItem],
        saveas: 'testTaskUrl008'
      };
      try {
        request.agent.create(context, conf, (err, data) => {
          expect(true).assertTrue();
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testTaskUrl009
     * @tc.name: testTaskUrl009
     * @tc.desc: Test upload when url length is 8193
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskUrl009', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1/' + 'a'.repeat(8176),
        data: [formItem],
        saveas: 'testTaskUrl009'
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
     * @tc.number: testTaskTitle001
     * @tc.name: testTaskTitle001
     * @tc.desc: Test create task when title is given
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskTitle001', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        title: 'download test.txt',
        saveas: 'testTaskTitle001.txt',
      };
      let task = await request.agent.create(context, conf);
      expect(task.config.title).assertEqual('download test.txt');
      await request.agent.remove(task.tid);
      done();
    });
    /**
     * @tc.number: testTaskTitle002
     * @tc.name: testTaskTitle002
     * @tc.desc: Test create task when title is number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskTitle002', 0, async (done: Function) => {
      let conf: Record<string, string | number> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "title": 123,
        "saveas": "testTaskTitle002.txt",
      };
      let task = await request.agent.create(context, conf as Object as request.agent.Config);
      expect(task.config.title).assertEqual(123);
      await request.agent.remove(task.tid);
      done();
    });
    /**
     * @tc.number: testTaskTitle003
     * @tc.name: testTaskTitle003
     * @tc.desc: Test create task when title length exceeds 256
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskTitle003', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        title: 'a'.repeat(257),
        saveas: 'testTaskTitle003.txt',
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
     * @tc.number: testTaskTitle004
     * @tc.name: testTaskTitle004
     * @tc.desc: Test create task when title length is 256
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskTitle004', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        title: 'a'.repeat(256),
        saveas: 'testTaskTitle004.txt',
      };
      try {
        let task = await request.agent.create(context, conf);
        done();
      } catch (err) {
        expect(true).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testTaskDescription001
     * @tc.name: testTaskDescription001
     * @tc.desc: Test create task when description is given
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskDescription001', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        description: 'test upload',
        saveas: 'testTaskDescription001.txt',
      };
      let task = await request.agent.create(context, conf);
      expect(task.config.description).assertEqual('test upload');
      await request.agent.remove(task.tid);
      done();
    });
    /**
     * @tc.number: testTaskDescription002
     * @tc.name: testTaskDescription002
     * @tc.desc: Test create task when description is number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskDescription002', 0, async (done: Function) => {
      let conf: Record<string, string | number> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "description": 123,
        "saveas": "testTaskDescription002.txt",
      };
      request.agent.create(context, conf as Object as request.agent.Config, async (err, data) => {
        expect(data.config.description).assertEqual(123);
        await request.agent.remove(data.tid);
        done();
      });
    });
    /**
     * @tc.number: testTaskDescription003
     * @tc.name: testTaskDescription003
     * @tc.desc: Test create task when description length exceeds 1024
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskDescription003', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskDescription003.txt',
        description: 'a'.repeat(1025)
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
     * @tc.number: testTaskDescription004
     * @tc.name: testTaskDescription004
     * @tc.desc: Test create task when description length is 1024
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskDescription004', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        description: 'a'.repeat(1024),
        saveas: 'testTaskDescription004.txt',
      };
      request.agent.create(context, conf, async (err, data) => {
        expect(err === null).assertTrue();
        await request.agent.remove(data.tid);
        done();
      });
    });
    /**
     * @tc.number: testTaskMode001
     * @tc.name: testTaskMode001
     * @tc.desc: Test create task when mode is FOREGROUND
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskMode001', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskMode001.txt',
        mode: request.agent.Mode.FOREGROUND
      };
      request.agent.create(context, conf).then(async (data) => {
        expect(true).assertTrue();
        await request.agent.remove(data.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskMode002
     * @tc.name: testTaskMode002
     * @tc.desc: Test create task when mode is BACKGROUND
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskMode002', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskMode002.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      request.agent.create(context, conf).then(async (data) => {
        expect(true).assertTrue();
        await request.agent.remove(data.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskMode003
     * @tc.name: testTaskMode003
     * @tc.desc: Test create task when mode is string
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskMode003', 0, async (done: Function) => {
      let conf: Record<string, string | number> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "saveas": "testTaskMode003.txt",
        "mode": "BACKGROUND"
      };
      let task = await request.agent.create(context, conf as Object as request.agent.Config);
      task.start().then(async () => {
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskMode004
     * @tc.name: testTaskMode004
     * @tc.desc: Test create task when mode is 0
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskMode004', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskMode004.txt',
        mode: 0
      };
      request.agent.create(context, conf).then(async (data) => {
        expect(true).assertTrue();
        await request.agent.remove(data.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskMode005
     * @tc.name: testTaskMode005
     * @tc.desc: Test create task when mode is 1
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskMode005', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskMode005.txt',
        mode: 1
      };
      request.agent.create(context, conf).then(async (data) => {
        expect(true).assertTrue();
        await request.agent.remove(data.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskMode006
     * @tc.name: testTaskMode006
     * @tc.desc: Test create task when mode is 3
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskMode006', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskMode006.txt',
        mode: 3
      };
      let task = await request.agent.create(context, conf);
      task.start().then(async () => {
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskMode007
     * @tc.name: testTaskMode007
     * @tc.desc: Test create task when mode is other type
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskMode007', 0, async (done: Function) => {
      let conf: Record<string, string | number> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        "saveas": 'testTaskMode007.txt',
        "mode": request.agent.Action.DOWNLOAD
      };
      let task = await request.agent.create(context, conf as Object as request.agent.Config);
      task.start().then(async () => {
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskMode008
     * @tc.name: testTaskMode008
     * @tc.desc: Test create task when mode is empty
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskMode008', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskMode008.txt'
      };
      request.agent.create(context, conf).then(async (data) => {
        expect(typeof data.config.mode).assertEqual('undefined');
        expect(true).assertTrue();
        await request.agent.remove(data.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskOverwrite001
     * @tc.name: testTaskOverwrite001
     * @tc.desc: Test create task when overwrite is true and file exists
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskOverwrite001', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        overwrite: true
      };
      let task = await request.agent.create(context, conf);
      task.start().then(async () => {
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskOverwrite002
     * @tc.name: testTaskOverwrite002
     * @tc.desc: Test create task when overwrite is true and file not exists
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskOverwrite002', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        overwrite: true
      };
      let task = await request.agent.create(context, conf);
      task.start().then(async () => {
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskOverwrite003
     * @tc.name: testTaskOverwrite003
     * @tc.desc: Test create task when overwrite is false and file exists
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskOverwrite003', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        overwrite: false
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
     * @tc.number: testTaskOverwrite004
     * @tc.name: testTaskOverwrite004
     * @tc.desc: Test create task when overwrite is false and file not exists
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskOverwrite004', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        overwrite: false
      };
      let task = await request.agent.create(context, conf);
      task.start().then(async () => {
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskOverwrite005
     * @tc.name: testTaskOverwrite005
     * @tc.desc: Test create task when overwrite is string and file not exists
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskOverwrite005', 0, async (done: Function) => {
      let conf: Record<string, string | number> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "overwrite": "true"
      };
      let task = await request.agent.create(context, conf as Object as request.agent.Config);
      task.start().then(async () => {
        expect(task.config.overwrite).assertEqual("true");
        await request.agent.remove(task.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskOverwrite006
     * @tc.name: testTaskOverwrite006
     * @tc.desc: Test create task when overwrite is string and file exists
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskOverwrite006', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: Record<string, string | number> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "overwrite": "true"
      };
      try {
        request.agent.create(context, conf as Object as request.agent.Config, (err, data) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(13400001);
        done();
      }
    });
    /**
     * @tc.number: testTaskOverwrite007
     * @tc.name: testTaskOverwrite007
     * @tc.desc: Test create task when overwrite is not given and file not exists
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskOverwrite007', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt'
      };
      try {
        let task = await request.agent.create(context, conf);
        task.start().then(async () => {
          expect(true).assertTrue();
          await request.agent.remove(task.tid);
          done();
        }).catch((err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(13400001);
        done();
      }
    });
    /**
     * @tc.number: testTaskOverwrite008
     * @tc.name: testTaskOverwrite008
     * @tc.desc: Test create task when overwrite is not given and file exists
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskOverwrite008', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt'
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
     * @tc.number: testTaskOverwrite009
     * @tc.name: testTaskOverwrite009
     * @tc.desc: Test create task when overwrite is true and saveas is dir not file
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskOverwrite009', 0, async (done: Function) => {
      let dirPath = cacheDir + '/testTaskOverwrite009';
      fs.mkdirSync(dirPath);
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        overwrite: true,
        saveas: './testTaskOverwrite009'
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
     * @tc.number: testTaskMethod001
     * @tc.name: testTaskMethod001
     * @tc.desc: Test create task when method is POST for upload
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskMethod001', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
          name: 'file',
          value: {
            path: `./test.txt`
          },
        }],
        method: 'POST'
      };
      let task = await request.agent.create(context, conf);
      task.start().then(() => {
        expect(true).assertTrue();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskMethod002
     * @tc.name: testTaskMethod002
     * @tc.desc: Test create task when method is POST for download
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskMethod002', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskMethod002.txt',
        method: 'POST'
      };
      let task = await request.agent.create(context, conf);
      task.start().then(async () => {
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskMethod003
     * @tc.name: testTaskMethod003
     * @tc.desc: Test create task when method is number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskMethod003', 0, async (done: Function) => {
      let conf: Record<string, string | number> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "saveas": "testTaskMethod003.txt",
        "method": 123
      };
      let task = await request.agent.create(context, conf as Object as request.agent.Config);
      task.start().then(async () => {
        expect(task.config.method).assertEqual(123);
        await request.agent.remove(task.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskMethod004
     * @tc.name: testTaskMethod004
     * @tc.desc: Test create task when method is empty
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskMethod004', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskMethod004.txt',
        method: ''
      };
      let task = await request.agent.create(context, conf);
      task.start().then(async () => {
        expect(task.config.method).assertEqual('');
        await request.agent.remove(task.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskMethod005
     * @tc.name: testTaskMethod005
     * @tc.desc: Test create task when method is GET for upload
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskMethod005', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
          name: 'file',
          value: {
            path: `./test.txt`
          },
        }],
        method: 'GET'
      };
      let task = await request.agent.create(context, conf);
      task.start().then(async () => {
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskMethod006
     * @tc.name: testTaskMethod006
     * @tc.desc: Test create task when method is PUT for download
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskMethod006', 0, async (done: Function) => {
      await sleep(2000);
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskMethod006.txt',
        method: 'PUT'
      };
      let task = await request.agent.create(context, conf);
      task.start().then(async () => {
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskSaveas001
     * @tc.name: testTaskSaveas001
     * @tc.desc: Test create task when saveas is number for download
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskSaveas001', 0, async (done: Function) => {
      let conf: Record<string, string | number> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "mode": request.agent.Mode.BACKGROUND,
        "saveas": 123
      };
      let task = await request.agent.create(context, conf as Object as request.agent.Config);
      task.on('completed', () => {
        if (fs.accessSync(`${cacheDir}/test.txt`)) {
          expect(true).assertTrue();
          done();
        }
      });
      await task.start();
    });
    /**
     * @tc.number: testTaskSaveas002
     * @tc.name: testTaskSaveas002
     * @tc.desc: Test create task when lack saveas for download
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskSaveas002', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      try {
        let task = await request.agent.create(context, conf);
        task.on('completed', () => {
          if (fs.accessSync(`${cacheDir}/test.txt`)) {
            expect(true).assertTrue();
            done();
          } else {
            expect(false).assertTrue();
            done();
          }
        });
        await task.start();
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
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
     * @tc.number: testTaskNetwork001
     * @tc.name: testTaskNetwork001
     * @tc.desc: Test create task when network is 3
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskNetwork001', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
          name: 'file',
          value: {
            path: `./test.txt`,
          }
        }],
        network: 3
      };
      request.agent.create(context, conf).then(async (data) => {
        expect(true).assertTrue();
        await request.agent.remove(data.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskNetwork002
     * @tc.name: testTaskNetwork002
     * @tc.desc: Test create task when network is string
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskNetwork002', 0, async (done: Function) => {
      let conf: Record<string, string | number> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "saveas": "testTaskNetwork002.txt",
        "network": "ANY"
      };
      request.agent.create(context, conf as Object as request.agent.Config).then(async (data) => {
        expect(true).assertTrue();
        await request.agent.remove(data.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskNetwork003
     * @tc.name: testTaskNetwork003
     * @tc.desc: Test create task when network is WIFI for DOWNLOAD
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskNetwork003', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskNetwork003.txt',
        network: request.agent.Network.WIFI
      };
      request.agent.create(context, conf).then(async (data) => {
        expect(true).assertTrue();
        await request.agent.remove(data.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskNetwork004
     * @tc.name: testTaskNetwork004
     * @tc.desc: Test create task when network is WIFI for UPLOAD
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskNetwork004', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
          name: 'file',
          value: {
            path: `./test.txt`,
          }
        }],
        network: request.agent.Network.WIFI
      };
      let task = await request.agent.create(context, conf);
      task.start().then(async () => {
        expect(true).assertTrue();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskNetwork005
     * @tc.name: testTaskNetwork005
     * @tc.desc: Test create task when network is ANY for DOWNLOAD
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskNetwork005', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskNetwork005.txt',
        network: request.agent.Network.ANY
      };
      request.agent.create(context, conf).then(async (data) => {
        expect(true).assertTrue();
        await request.agent.remove(data.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskNetwork006
     * @tc.name: testTaskNetwork006
     * @tc.desc: Test create task when network is 0
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskNetwork006', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskNetwork006.txt',
        network: 0
      };
      request.agent.create(context, conf).then(async (data) => {
        expect(true).assertTrue();
        await request.agent.remove(data.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskNetwork007
     * @tc.name: testTaskNetwork007
     * @tc.desc: Test create task when network is 1
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskNetwork007', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskNetwork007.txt',
        network: 1
      };
      request.agent.create(context, conf).then(async (data) => {
        expect(true).assertTrue();
        await request.agent.remove(data.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskNetwork008
     * @tc.name: testTaskNetwork008
     * @tc.desc: Test create task when network is CELLULAR for DOWNLOAD
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskNetwork008', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskNetwork008.txt',
        network: request.agent.Network.CELLULAR
      };
      request.agent.create(context, conf).then(async (data) => {
        expect(true).assertTrue();
        await request.agent.remove(data.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskNetwork009
     * @tc.name: testTaskNetwork009
     * @tc.desc: Test create task when network is 2
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskNetwork009', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskNetwork009.txt',
        network: 2
      };
      request.agent.create(context, conf).then(async (data) => {
        expect(true).assertTrue();
        await request.agent.remove(data.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskNetwork010
     * @tc.name: testTaskNetwork010
     * @tc.desc: Test create task when network is empty
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskNetwork010', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskNetwork010.txt'
      };
      request.agent.create(context, conf).then(async (data) => {
        expect(true).assertTrue();
        await request.agent.remove(data.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskHeaders001
     * @tc.name: testTaskHeaders001
     * @tc.desc: Test create task when headers content-type is application/json but data is file for upload
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskHeaders001', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
          name: 'file',
          value: {
            path: `./test.txt`
          },
        }],
        headers: {
          'content-type': 'application/json'
        },
      };
      let task = await request.agent.create(context, conf);
      task.start().then(() => {
        expect(true).assertTrue();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskHeaders002
     * @tc.name: testTaskHeaders002
     * @tc.desc: Test create task when lack headers for upload
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskHeaders002', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
          name: 'file',
          value: {
            path: `./test.txt`
          },
        }],
      };
      let task = await request.agent.create(context, conf);
      task.start().then(() => {
        expect(typeof task.config.headers).assertEqual('undefined');
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskHeaders003
     * @tc.name: testTaskHeaders003
     * @tc.desc: Test create task when lack headers for download
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskHeaders003', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskHeaders003.txt',
      };
      let task = await request.agent.create(context, conf);
      task.start().then(async () => {
        expect(typeof task.config.headers).assertEqual('undefined');
        await request.agent.remove(task.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskHeaders004
     * @tc.name: testTaskHeaders004
     * @tc.desc: Test create task when headers content-type is multipart/form-data but data is file for download
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskHeaders004', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: `README.md`,
        mode: request.agent.Mode.BACKGROUND,
        headers: {
          'content-type': 'multipart/form-data'
        },
      };
      let task = await request.agent.create(context, conf);
      task.start().then(() => {
        expect(true).assertTrue();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskRetry001
     * @tc.name: testTaskRetry001
     * @tc.desc: Test create task when retry is true for FOREGROUND
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskRetry001', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'http://xxx',
        mode: request.agent.Mode.FOREGROUND,
        retry: true
      };
      let task = await request.agent.create(context, conf);
      task.start().then(async () => {
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskRetry002
     * @tc.name: testTaskRetry002
     * @tc.desc: Test create task when retry is true for background
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskRetry002', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.BACKGROUND,
        saveas: 'testTaskRetry002.txt',
        retry: true,
        gauge: true
      };
      let task = await request.agent.create(context, conf);
      task.start().then(async () => {
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskRetry003
     * @tc.name: testTaskRetry003
     * @tc.desc: Test create task when retry is string
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskRetry003', 0, async (done: Function) => {
      let conf: Record<string, string | number | boolean> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "http://xxx",
        "mode": request.agent.Mode.BACKGROUND,
        "retry": "true",
        "overwrite": true
      };
      let task = await request.agent.create(context, conf as Object as request.agent.Config);
      task.on('failed', () => {
        expect(true).assertTrue();
        done();
      });
      await task.start();
    });
    /**
     * @tc.number: testTaskRetry004
     * @tc.name: testTaskRetry004
     * @tc.desc: Test create task when retry is false for FOREGROUND
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskRetry004', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test1.txt',
        mode: request.agent.Mode.FOREGROUND,
        retry: false
      };
      let task = await request.agent.create(context, conf);
      task.on('failed', () => {
        expect(true).assertTrue();
        done();
      });
      await task.start();
    });
    /**
     * @tc.number: testTaskRetry005
     * @tc.name: testTaskRetry005
     * @tc.desc: Test create task when retry is false for background
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskRetry005', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test1.txt',
        mode: request.agent.Mode.BACKGROUND,
        retry: false,
        overwrite: true
      };
      let task = await request.agent.create(context, conf);
      task.on('failed', () => {
        expect(true).assertTrue();
        done();
      });
      await task.start();
    });
    /**
     * @tc.number: testTaskIndex001
     * @tc.name: testTaskIndex001
     * @tc.desc: Test create task when index is string
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskIndex001', 0, async (done: Function) => {
      let conf: Record<string, string | number> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "saveas": "testTaskIndex001.txt",
        "index": "0",
        "mode": request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf as Object as request.agent.Config);
      task.on('completed', () => {
        expect(true).assertTrue();
        done();
      });
      await task.start();
    });
    /**
     * @tc.number: testTaskIndex002
     * @tc.name: testTaskIndex002
     * @tc.desc: Test create task when index is 0 and begins greater than ends
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskIndex002', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskIndex002.txt',
        index: 0,
        begins: 10,
        ends: 5
      };
      let task = await request.agent.create(context, conf);
      task.start().then(async () => {
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskIndex003
     * @tc.name: testTaskIndex003
     * @tc.desc: Test create task when index is 1 but only one file for upload
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskIndex003', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        index: 1
      };
      try {
        let task = await request.agent.create(context, conf);
        task.start().then(() => {
          expect(false).assertTrue();
          done();
        }).catch((err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testTaskIndex005
     * @tc.name: testTaskIndex005
     * @tc.desc: Test create task when index is 0 and begins is 5 and ends is 10 for upload
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskIndex005', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
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
      };
      let task = await request.agent.create(context, conf);
      task.start().then(() => {
        expect(true).assertTrue();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskIndex006
     * @tc.name: testTaskIndex006
     * @tc.desc: Test create task when unsupport range request
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskIndex006', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.BACKGROUND,
        begins: 5,
        gauge: true
      };
      let task = await request.agent.create(context, conf);
      await task.start();
      task.on('failed', (progress) => {
        expect(true).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskIndex007
     * @tc.name: testTaskIndex007
     * @tc.desc: Test create task when index is 0 and begins is 5 and ends is not exists for upload
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskIndex007', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
          name: 'file',
          value: {
            path: `./test.txt`,
          }
        }],
        begins: 5,
      };
      let task = await request.agent.create(context, conf);
      task.start().then(() => {
        expect(true).assertTrue();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskIndex009
     * @tc.name: testTaskIndex009
     * @tc.desc: Test create task when index is 0 and begins is 5 and ends is not exists for upload
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskIndex009', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
          name: 'file',
          value: {
            path: `./test.txt`,
          }
        }],
        ends: 10
      };
      let task = await request.agent.create(context, conf);
      task.start().then(() => {
        expect(true).assertTrue();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskIndex010
     * @tc.name: testTaskIndex010
     * @tc.desc: Test create task when index is 1 but have two files for upload
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskIndex010', 0, async (done: Function) => {
      let test1 = fs.openSync(cacheDir + '/test1.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(test1);
      let test2 = fs.openSync(cacheDir + '/test2.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(test2);
      let conf: request.agent.Config = {
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
              }
            ]
          }
        ],
        index: 1,
      };
      let task = await request.agent.create(context, conf);
      task.start().then(() => {
        expect(true).assertTrue();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskToken001
     * @tc.name: testTaskToken001
     * @tc.desc: Test create task when token is 7 bytes
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskToken001', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskToken001.txt',
        token: 'abcdef'
      };
      try {
        request.agent.create(context, conf).then((data) => {
          expect(false).assertTrue();
          done();
        }).catch((err: Error) => {
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testTaskToken002
     * @tc.name: testTaskToken002
     * @tc.desc: Test create task when token is 2049 bytes
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskToken002', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskToken002.txt',
        token: 'a'.padStart(2049, 'a')
      };
      try {
        request.agent.create(context, conf).then((data) => {
          expect(false).assertTrue();
          done();
        }).catch((err: Error) => {
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testTaskToken003
     * @tc.name: testTaskToken003
     * @tc.desc: Test create task when token is number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskToken003', 0, async (done: Function) => {
      let conf: Record<string, string | number> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "saveas": "testTaskToken003.txt",
        "token": 666
      };
      request.agent.create(context, conf as Object as request.agent.Config).then(async (data) => {
        expect(data.config.token).assertEqual(666);
        await request.agent.remove(data.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskToken004
     * @tc.name: testTaskToken004
     * @tc.desc: Test create task when token is 2048 bytes
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskToken004', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskToken004.txt',
        token: 'a'.padStart(2048, 'a')
      };
      request.agent.create(context, conf).then(async (data) => {
        expect(true).assertTrue();
        await request.agent.remove(data.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testTaskToken005
     * @tc.name: testTaskToken005
     * @tc.desc: Test create task when token is 8 bytes
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskToken005', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskToken005.txt',
        token: '11111111'
      };
      request.agent.create(context, conf).then(async (data) => {
        expect(true).assertTrue();
        await request.agent.remove(data.tid);
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testCreate001
     * @tc.name: testCreate001
     * @tc.desc: Test create for conf is string type
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testCreate001', 0, async (done: Function) => {
      try {
        await request.agent.create(context, "conf" as Object as request.agent.Config);
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });
    /**
     * @tc.number: testCreate002
     * @tc.name: testCreate002
     * @tc.desc: Test create
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testCreate002', 0, async (done: Function) => {
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
        expect(request.agent.WaitingReason.TASK_QUEUE_FULL).assertEqual(0x00);
        expect(request.agent.WaitingReason.NETWORK_NOT_MATCH).assertEqual(0x01);
        expect(request.agent.WaitingReason.APP_BACKGROUND).assertEqual(0x02);
        expect(request.agent.WaitingReason.USER_INACTIVATED).assertEqual(0x03);
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testTaskProxy001
     * @tc.name: testTaskProxy001
     * @tc.desc: Test create task when proxy format error
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskProxy001', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        proxy: 'https://127.0.0.1:8080'
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
     * @tc.number: testTaskProxy002
     * @tc.name: testTaskProxy002
     * @tc.desc: Test create task when proxy format error
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskProxy002', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        proxy: 'http://127.0.0.1:8080/proxy'
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
     * @tc.number: testTaskProxy003
     * @tc.name: testTaskProxy003
     * @tc.desc: Test create task when proxy format error
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskProxy003', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        proxy: 'http://127.0.0.1:111111'
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
     * @tc.number: testTaskMultipart001
     * @tc.name: testTaskMultipart001
     * @tc.desc: Test create task when multipart error
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskMultipart001', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        multipart: true,
        data: [formItem1]
      };
      try {
        request.agent.create(context, conf, (err, data) => {
          expect(true).assertTrue();
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testTaskMultipart002
     * @tc.name: testTaskMultipart002
     * @tc.desc: Test create task when multipart error
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */

    it('testTaskMultipart002', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        multipart: false,
        data: [formItem1]
      };
      try {
        request.agent.create(context, conf, (err, data) => {
          expect(true).assertTrue();
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testTaskVisibility001
     * @tc.name: testTaskVisibility001
     * @tc.desc: Test create task with valid visibility COMPLETION using enum
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskVisibility001', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskVisibility001.txt',
        notification: {
          visibility: request.agent.VISIBILITY_COMPLETION,
        },
      };
      try {
        let task = await request.agent.create(context, conf);
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testTaskVisibility002
     * @tc.name: testTaskVisibility002
     * @tc.desc: Test create task with valid visibility PROGRESS using enum
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskVisibility002', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskVisibility002.txt',
        notification: {
          visibility: request.agent.VISIBILITY_PROGRESS,
        },
      };
      try {
        let task = await request.agent.create(context, conf);
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testTaskVisibility003
     * @tc.name: testTaskVisibility003
     * @tc.desc: Test create task with valid visibility ANY (COMPLETION | PROGRESS)
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskVisibility003', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskVisibility003.txt',
        notification: {
          visibility: request.agent.VISIBILITY_COMPLETION | request.agent.VISIBILITY_PROGRESS,
        },
      };
      try {
        let task = await request.agent.create(context, conf);
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testTaskVisibility004
     * @tc.name: testTaskVisibility004
     * @tc.desc: Test create task when fail by invalid visibility value 0b00
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskVisibility004', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskVisibility004.txt',
        notification: {
          visibility: 0b00, // Invalid value
        },
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
     * @tc.number: testTaskVisibility005
     * @tc.name: testTaskVisibility005
     * @tc.desc: Test create task when fail by invalid visibility value
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskVisibility005', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskVisibility005.txt',
        notification: {
          visibility: 0b100, // Invalid value
        },
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
     * @tc.number: testTaskVisibility006
     * @tc.name: testTaskVisibility006
     * @tc.desc: Test create task when fail by invalid visibility type
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskVisibility006', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskVisibility006.txt',
        notification: {
          visibility: 'invalid' as Object as number, // Invalid type
        },
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
     * @tc.number: testTaskVisibility007
     * @tc.name: testTaskVisibility007
     * @tc.desc: Test create task with valid visibility PROGRESS using enum and gauge recover larger
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskVisibility007', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskVisibility007.txt',
        gauge: true,
        notification: {
          visibility: request.agent.VISIBILITY_PROGRESS,
        },
      };
      try {
        let task = await request.agent.create(context, conf);
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.number: testTaskVisibility008
     * @tc.name: testTaskVisibility008
     * @tc.desc: Test create task with valid visibility PROGRESS using enum and gauge recover smaller
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskVisibility008', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskVisibility008.txt',
        gauge: false,
        notification: {
          visibility: request.agent.VISIBILITY_COMPLETION | request.agent.VISIBILITY_PROGRESS,
        },
      };
      try {
        let task = await request.agent.create(context, conf);
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
  })
}