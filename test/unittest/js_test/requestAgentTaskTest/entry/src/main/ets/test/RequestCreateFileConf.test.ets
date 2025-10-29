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

export default function requestCreateFileConfTest() {
  describe('requestCreateFileConfTest', () => {
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
     * @tc.number: testTaskOverwrite001
     * @tc.name: testTaskOverwrite001
     * @tc.desc: Test create task when overwrite is true and file exists
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. In the cache directory, create the target file (testTaskOverwrite001.txt) with 
     *             read-write permissions and close it to simulate the "file exists" scenario.
     *           2. Create a Config object with action: DOWNLOAD, valid URL, saveas set to the 
     *             created file name, and overwrite: true.
     *           3. Call request.agent.create() to create a download task instance.
     *           4. Call task.start() to execute the task; in the success callback, assert true 
     *             (task created and started successfully).
     *           5. Clean up by calling request.agent.remove() to delete the task, then call done().
     *           6. In the catch callback, assert false (task execution failed) and call done().
     * @tc.require: issues#1573
     */
    it('testTaskOverwrite001', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/testTaskOverwrite001.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskOverwrite001.txt',
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a Config object with action: DOWNLOAD, valid URL, 
     *             saveas: testTaskOverwrite002.txt (file not exists), and overwrite: true.
     *           2. Call request.agent.create() to create a download task instance.
     *           3. Call task.start() to execute the task; in the success callback, assert true 
     *             (task created and started successfully).
     *           4. Clean up by calling request.agent.remove() to delete the task, then call done().
     *           5. In the catch callback, assert false (task execution failed) and call done().
     * @tc.require: issues#1573
     */
    it('testTaskOverwrite002', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskOverwrite002.txt',
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. In the cache directory, create the target file (testTaskOverwrite003.txt) with 
     *             read-write permissions and close it to simulate the "file exists" scenario.
     *           2. Create a Config object with action: DOWNLOAD, valid URL, saveas set to the 
     *             created file name, and overwrite: false.
     *           3. Try to call request.agent.create() to create a task; in the success callback 
     *             (unexpected), assert false and call done().
     *           4. Catch the thrown exception, assert err.code equals 13400001 (file exists error), 
     *             then call done().
     * @tc.require: issues#1573
     */
    it('testTaskOverwrite003', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/testTaskOverwrite003.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskOverwrite003.txt',
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a Config object with action: DOWNLOAD, valid URL, 
     *             saveas: testTaskOverwrite004.txt (file not exists), and overwrite: false.
     *           2. Call request.agent.create() to create a download task instance.
     *           3. Call task.start() to execute the task; in the success callback, assert true 
     *             (task created and started successfully).
     *           4. Clean up by calling request.agent.remove() to delete the task, then call done().
     *           5. In the catch callback, assert false (task execution failed) and call done().
     * @tc.require: issues#1573
     */
    it('testTaskOverwrite004', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskOverwrite004.txt',
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a Record object (for type conversion) with action: DOWNLOAD, valid URL, 
     *             saveas: testTaskOverwrite005, and overwrite: "true" (string type).
     *           2. Call request.agent.create() with type-cast config to create a download task instance.
     *           3. Call task.start() to execute the task; in the success callback, assert 
     *             task.config.overwrite equals "true".
     *           4. Clean up by calling request.agent.remove() to delete the task, then call done().
     *           5. In the catch callback, assert false (task execution failed) and call done().
     * @tc.require: issues#1573
     */
    it('testTaskOverwrite005', 0, async (done: Function) => {
      let conf: Record<string, string | number> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "saveas": "testTaskOverwrite005",
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. In the cache directory, create the target file (testTaskOverwrite006.txt) with 
     *             read-write permissions and close it to simulate the "file exists" scenario.
     *           2. Create a Record object (for type conversion) with action: DOWNLOAD, valid URL, 
     *             saveas set to the created file name, and overwrite: "true" (string type).
     *           3. Try to call request.agent.create() with type-cast config; in the success callback 
     *             (unexpected), assert false and call done().
     *           4. Catch the thrown exception, assert err.code equals 13400001 (file exists error), 
     *             then call done().
     * @tc.require: issues#1573
     */
    it('testTaskOverwrite006', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/testTaskOverwrite006.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: Record<string, string | number> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "saveas": "testTaskOverwrite006.txt",
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a Config object with action: DOWNLOAD, valid URL, 
     *             saveas: testTaskOverwrite007.txt (file not exists), and no overwrite parameter.
     *           2. Try to call request.agent.create() to create a download task instance.
     *           3. Call task.start() to execute the task; in the success callback, assert true 
     *             (task created and started successfully).
     *           4. Clean up by calling request.agent.remove() to delete the task, then call done().
     *           5. In the task catch callback, assert false (task execution failed); in the outer 
     *             catch, assert err.code equals 13400001 and call done().
     * @tc.require: issues#1573
     */
    it('testTaskOverwrite007', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskOverwrite007.txt'
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. In the cache directory, create the target file (testTaskOverwrite008.txt) with 
     *             read-write permissions and close it to simulate the "file exists" scenario.
     *           2. Create a Config object with action: DOWNLOAD, valid URL, saveas set to the 
     *             created file name, and no overwrite parameter.
     *           3. Try to call request.agent.create() to create a task; in the success callback 
     *             (unexpected), assert false and call done().
     *           4. Catch the thrown exception, assert err.code equals 13400001 (file exists error), 
     *             then call done().
     * @tc.require: issues#1573
     */
    it('testTaskOverwrite008', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/testTaskOverwrite008.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskOverwrite008.txt'
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. In the cache directory, create a target directory (testTaskOverwrite009) to 
     *             simulate "saveas is a directory" scenario.
     *           2. Create a Config object with action: DOWNLOAD, valid URL, overwrite: true, and 
     *             saveas set to the created directory path.
     *           3. Try to call request.agent.create() to create a task; in the success callback 
     *             (unexpected), assert false and call done().
     *           4. Catch the thrown exception, assert err.code equals 13400001 (saveas is directory 
     *             error), then call done().
     * @tc.require: issues#1573
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
     * @tc.number: testTaskSaveas001
     * @tc.name: testTaskSaveas001
     * @tc.desc: Test create task when saveas is number for download
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a Record object (for type conversion) with action: DOWNLOAD, valid URL, 
     *             mode: BACKGROUND, and saveas: 123 (number type).
     *           2. Call request.agent.create() with type-cast config to create a download task instance.
     *           3. Bind 'completed' event to the task; in the callback, check if the default file 
     *             (test.txt) exists in the cache directory.
     *           4. If the file exists, assert true and call done(); otherwise, the assertion will 
     *             fail implicitly.
     *           5. Call task.start() to execute the download task.
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a Config object with action: DOWNLOAD, valid URL, mode: BACKGROUND, 
     *             and no saveas parameter.
     *           2. Try to call request.agent.create() to create a download task instance.
     *           3. If task is created, bind 'completed' event: check if the default file (test.txt) 
     *             exists in the cache directory, assert true if exists, else assert false.
     *           4. Call task.start() to execute the task; in the outer catch, assert err.code equals 
     *             401 (missing saveas error) and call done().
     * @tc.require: issues#1573
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
     * @tc.number: testTaskIndex001
     * @tc.name: testTaskIndex001
     * @tc.desc: Test create task when index is string
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a Record object (for type conversion) with action: DOWNLOAD, valid URL, 
     *             saveas: testTaskIndex001.txt, index: "0" (string type), and mode: BACKGROUND.
     *           2. Call request.agent.create() with type-cast config to create a download task instance.
     *           3. Bind 'completed' event to the task; in the callback, assert true (task completed 
     *             successfully) and call done().
     *           4. Call task.start() to execute the download task.
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a Config object with action: DOWNLOAD, valid URL, 
     *             saveas: testTaskIndex002.txt, index: 0, begins: 10 (greater than ends: 5).
     *           2. Call request.agent.create() to create a download task instance.
     *           3. Call task.start() to execute the task; in the success callback, assert true 
     *             (task executed successfully despite invalid range) and call done().
     *           4. Clean up by calling request.agent.remove() to delete the task.
     *           5. In the catch callback, assert false (task execution failed) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. In the cache directory, create an upload file (testTaskIndex003.txt) with 
     *             read-write permissions and close it (only one file).
     *           2. Create a Config object with action: UPLOAD, url: http://127.0.0.1, and index: 1 
     *             (exceeds number of files: 1).
     *           3. Try to call request.agent.create() to create an upload task; in the success 
     *             callback, call task.start() and assert false (unexpected success).
     *           4. In the task start catch callback, assert false; in the outer catch, assert 
     *             err.code equals 401 (invalid index error) and call done().
     * @tc.require: issues#1573
     */
    it('testTaskIndex003', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/testTaskIndex003.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. In the cache directory, create an upload file (testTaskIndex005.txt) with 
     *             read-write permissions and close it.
     *           2. Create a Config object with action: UPLOAD, url: http://127.0.0.1, data 
     *             (contains the upload file path), begins: 5, and ends: 10.
     *           3. Call request.agent.create() to create an upload task instance.
     *           4. Call task.start() to execute the task; in the success callback, assert true 
     *             (task executed successfully) and call done().
     *           5. In the catch callback, assert false (task execution failed) and call done().
     * @tc.require: issues#1573
     */
    it('testTaskIndex005', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/testTaskIndex005.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
          name: 'file',
          value: {
            path: `./testTaskIndex005.txt`,
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a Config object with action: DOWNLOAD, valid URL, mode: BACKGROUND, 
     *             saveas: testTaskIndex006, begins: 5, and gauge: true (unsupported range).
     *           2. Call request.agent.create() to create a download task instance.
     *           3. Call task.start() to execute the task.
     *           4. Bind 'failed' event to the task; in the callback, assert true (task failed as 
     *             expected) and call done().
     * @tc.require: issues#1573
     */
    it('testTaskIndex006', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.BACKGROUND,
        saveas: 'testTaskIndex006',
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. In the cache directory, create an upload file (testTaskIndex007.txt) with 
     *             read-write permissions and close it.
     *           2. Create a Config object with action: UPLOAD, url: http://127.0.0.1, data 
     *             (contains the upload file path), and begins: 5 (ends not specified).
     *           3. Call request.agent.create() to create an upload task instance.
     *           4. Call task.start() to execute the task; in the success callback, assert true 
     *             (task executed successfully) and call done().
     *           5. In the catch callback, assert false (task execution failed) and call done().
     * @tc.require: issues#1573
     */
    it('testTaskIndex007', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/testTaskIndex007.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
          name: 'file',
          value: {
            path: `./testTaskIndex007.txt`,
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. In the cache directory, create an upload file (testTaskIndex009.txt) with 
     *             read-write permissions and close it.
     *           2. Create a Config object with action: UPLOAD, url: http://127.0.0.1, data 
     *             (contains the upload file path), and ends: 10 (begins not specified).
     *           3. Call request.agent.create() to create an upload task instance.
     *           4. Call task.start() to execute the task; in the success callback, assert true 
     *             (task executed successfully) and call done().
     *           5. In the catch callback, assert false (task execution failed) and call done().
     * @tc.require: issues#1573
     */
    it('testTaskIndex009', 0, async (done: Function) => {
      let file = fs.openSync(cacheDir + '/testTaskIndex009.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
          name: 'file',
          value: {
            path: `./testTaskIndex009.txt`,
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. In the cache directory, create two upload files (test1.txt, test2.txt) with 
     *             read-write permissions and close them.
     *           2. Create a Config object with action: UPLOAD, url: http://127.0.0.1, data 
     *             (contains both upload file paths), and index: 1 (valid, matches number of files).
     *           3. Call request.agent.create() to create an upload task instance.
     *           4. Call task.start() to execute the task; in the success callback, assert true 
     *             (task executed successfully) and call done().
     *           5. In the catch callback, assert false (task execution failed) and call done().
     * @tc.require: issues#1573
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
     * @tc.number: testTaskMultipart001
     * @tc.name: testTaskMultipart001
     * @tc.desc: Test create task when multipart error
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. In the cache directory, create a test file (test.txt) with read-write permissions 
     *             and close it.
     *           2. Create a Config object with action: UPLOAD, url: http://127.0.0.1, multipart: true, 
     *             and data: [formItem1] (invalid form item).
     *           3. Try to call request.agent.create() to create an upload task; in the success callback 
     *             (expected despite multipart error), assert true and call done().
     *           4. In the outer catch, assert false (unexpected exception) and call done().
     * @tc.require: issues#1573
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
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. In the cache directory, create a test file (test.txt) with read-write permissions 
     *             and close it.
     *           2. Create a Config object with action: UPLOAD, url: http://127.0.0.1, multipart: false, 
     *             and data: [formItem1] (invalid form item).
     *           3. Try to call request.agent.create() to create an upload task; in the success callback 
     *             (expected despite multipart error), assert true and call done().
     *           4. In the outer catch, assert false (unexpected exception) and call done().
     * @tc.require: issues#1573
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
  })
}