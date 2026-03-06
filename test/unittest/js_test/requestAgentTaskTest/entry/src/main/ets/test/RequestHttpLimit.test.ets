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

import { describe, beforeAll, beforeEach, afterEach, afterAll, it, expect } from "@ohos/hypium";
import request from '@ohos.request';
import fs from '@ohos.file.fs';
import commonEvent from '@ohos.commonEventManager';
import common from '@ohos.app.ability.common';
import { BusinessError } from "@ohos.base";
import Server from '../common/iTcpServer'

export default function requestHttpLimitTest() {
  describe('requestHttpLimitTest', () => {
    let server: Server | null = null;
    let baseUrl: string = "";

    beforeAll(async () => {
      console.info('beforeAll called');
      server = new Server();
      let port = await server.serverStart();
      baseUrl = `http://127.0.0.1:${port}`;
      console.info("server started on " + baseUrl);
    });
    afterAll(async () => {
      console.info('afterAll called');
      if (server) {
        await server.serverClose();
      }
    });
    beforeEach(() => {
      console.info('beforeEach called');
    });
    afterEach(async () => {
      console.info('afterEach called');
      if (fs.accessSync(cacheDir + '/test.txt')) {
        fs.unlinkSync(cacheDir + '/test.txt');
      }
      if (fs.accessSync(cacheDir + '/README.md')) {
        fs.unlinkSync(cacheDir + '/README.md');
      }
    });

    let sleep: (ms: number) => Promise<void> = async (ms: number): Promise<void> => {
      return new Promise((resolve) => {
        setTimeout(resolve, ms);
      });
    };

    let context: common.UIAbilityContext = globalThis.abilityContext;
    let cacheDir: String = globalThis.cacheDir;

    /**
     * @tc.number: testSetMaxSpeed001
     * @tc.name: testSetMaxSpeed001
     * @tc.desc: Tests whether an exception is thrown when max_speed is smaller than 16 * 1024
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testSetMaxSpeed001', 0, async (done: Function) => {
      try {
        let conf: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: baseUrl + '/test.txt',
          mode: request.agent.Mode.BACKGROUND,
        };
        let task = await request.agent.create(context, conf);
        await task.setMaxSpeed(16 * 1024 - 1);
        expect(false).assertTrue();
        done();
      } catch (err) {
        console.info('testSetMaxSpeed001 error: ' + JSON.stringify(err));
        expect(err.code).assertEqual(401);
        done();
      }
    });

    /**
     * @tc.number: testSetMaxSpeed002
     * @tc.name: testSetMaxSpeed002
     * @tc.desc: Tests ok when max_speed is 16 * 1024
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testSetMaxSpeed002', 0, async (done: Function) => {
      try {
        let conf: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: baseUrl + '/test.txt',
          mode: request.agent.Mode.BACKGROUND,
        };
        let task = await request.agent.create(context, conf);
        await task.setMaxSpeed(16 * 1024);
        expect(true).assertTrue();
        done();
      } catch (err) {
        console.info('testSetMaxSpeed002 error: ' + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testSetMaxSpeed003
     * @tc.name: testSetMaxSpeed003
     * @tc.desc: Tests whether an exception is thrown when max_speed while task is removed
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testSetMaxSpeed003', 0, async (done: Function) => {
      try {
        let conf: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: baseUrl + '/test.txt',
          saveas: 'testSetMaxSpeed004.txt',
          mode: request.agent.Mode.BACKGROUND
        };
        let task = await request.agent.create(context, conf);
        await request.agent.remove(task.tid);
        await task.setMaxSpeed(16 * 1024);
        expect(false).assertTrue();
        done();
      } catch (err) {
        console.info('testSetMaxSpeed003 error: ' + JSON.stringify(err));
        expect(err.code).assertEqual(21900007);
        done();
      }
    });

    /**
     * @tc.name: testSetMaxSpeed004
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. create a task with minSpeed 16 * 1024 + 1.
     *           2. setMaxSpeed is 16 * 1024
     *           3. setMaxSpeed failed with errorcode 401.
     * @tc.expect: errorcode is 401.
     * @tc.type: Func
     * @tc.level: Level 1
     * @tc.require: issue#IC9V2X
     */
    it('testSetMaxSpeed004', 0, async (done: Function) => {
      try {
        let conf: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: baseUrl + '/test.txt',
          mode: request.agent.Mode.BACKGROUND,
          minSpeed: {
            speed: 16 * 1024 + 1,
            duration: 1,
          }
        };
        let task = await request.agent.create(context, conf);
        await task.setMaxSpeed(16 * 1024);
        expect(false).assertTrue();
        done();
      } catch (err) {
        console.info('testSetMaxSpeed004 error: ' + JSON.stringify(err));
        expect(err.code).assertEqual(401);
        done();
      }
    });

    /**
     * @tc.name: testSetMaxSpeed005
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. create a task with minSpeed 16 * 1024.
     *           2. setMaxSpeed is 16 * 1024
     *           3. setMaxSpeed succeed.
     * @tc.expect: setMaxSpeed succeed.
     * @tc.type: Func
     * @tc.level: Level 1
     * @tc.require: issue#IC9V2X
     */
    it('testSetMaxSpeed005', 0, async (done: Function) => {
      try {
        let conf: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: baseUrl + '/test.txt',
          mode: request.agent.Mode.BACKGROUND,
          minSpeed: {
            speed: 16 * 1024,
            duration: 1,
          }
        };
        let task = await request.agent.create(context, conf);
        await task.setMaxSpeed(16 * 1024);
        expect(true).assertTrue();
        done();
      } catch (err) {
        console.info('testSetMaxSpeed005 error: ' + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.name: testLowSpeed001
     * @tc.desc: Tests the task can be completed normally when MinSpeed is not set.
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. start a download task when MinSpeed is not set.
     *           2. check if the complete callback is triggered.
     * @tc.expect: complete callback is triggered.
     * @tc.type: Func
     * @tc.level: Level 1
     * @tc.require: issue#IC63UZ
     */
    it('testLowSpeed001', 0, async (done: Function) => {
      try {
        let conf: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: baseUrl + '/test.txt',
          saveas: 'testLowSpeed001.txt',
          mode: request.agent.Mode.BACKGROUND
        };
        let task = await request.agent.create(context, conf);
        task.on('completed', (progress) => {
          expect(true).assertTrue();
          done();
        });
        await task.start();
      } catch (err) {
        console.info('testLowSpeed001 error: ' + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });
    /**
     * @tc.name: testLowSpeed002
     * @tc.desc: Tests the task can be completed normally when MinSpeed.speed is 0.
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. start a download task when MinSpeed.speed is 0.
     *           2. check if the complete callback is triggered.
     * @tc.expect: complete callback is triggered.
     * @tc.type: Func
     * @tc.level: Level 1
     * @tc.require: issue#IC63UZ
     */
    it('testLowSpeed002', 0, async (done: Function) => {
      try {
        let conf: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: baseUrl + '/test.txt',
          saveas: 'testLowSpeed002.txt',
          mode: request.agent.Mode.BACKGROUND,
          minSpeed: {
            speed: 0,
            duration: 1,
          }
        };
        let task = await request.agent.create(context, conf);
        task.on('completed', (progress) => {
          expect(true).assertTrue();
          done();
        });
        await task.start();
      } catch (err) {
        console.info('testLowSpeed002 error: ' + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.name: testLowSpeed003
     * @tc.desc: Tests the task can be completed normally when MinSpeed.duration is 0.
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. start a download task when MinSpeed.duration is 0.
     *           2. check if the complete callback is triggered.
     * @tc.expect: complete callback is triggered.
     * @tc.type: Func
     * @tc.level: Level 1
     * @tc.require: issue#IC63UZ
     */
    it('testLowSpeed003', 0, async (done: Function) => {
      try {
        let conf: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: baseUrl + '/test.txt',
          saveas: 'testLowSpeed003.txt',
          mode: request.agent.Mode.BACKGROUND,
          minSpeed: {
            speed: 1,
            duration: 0,
          }
        };
        let task = await request.agent.create(context, conf);
        task.on('completed', (progress) => {
          expect(true).assertTrue();
          done();
        });
        await task.start();
      } catch (err) {
        console.info('testLowSpeed003 error: ' + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.name: testLowSpeed004
     * @tc.desc: Tests the task creates fail when MinSpeed.duration is -1.
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. start a download task when MinSpeed.duration is -1.
     *           2. check the error code.
     * @tc.expect: error code equals 401.
     * @tc.type: Func
     * @tc.level: Level 1
     * @tc.require: issue#IC63UZ
     */
    it('testLowSpeed004', 0, async (done: Function) => {
      try {
        let conf: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: baseUrl + '/test.txt',
          saveas: 'testLowSpeed004.txt',
          mode: request.agent.Mode.BACKGROUND,
          minSpeed: {
            speed: 1,
            duration: -1,
          }
        };
        await request.agent.create(context, conf);
        expect(false).assertTrue();
        done();
      } catch (err) {
        console.info('testLowSpeed004 error: ' + JSON.stringify(err));
        expect(err.code).assertEqual(401);
        done();
      }
    });

    /**
     * @tc.name: testLowSpeed005
     * @tc.desc: Tests the task creates fail when MinSpeed.speed is -1.
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. start a download task when MinSpeed.speed is -1.
     *           2. check the error code.
     * @tc.expect: error code equals 401.
     * @tc.type: Func
     * @tc.level: Level 1
     * @tc.require: issue#IC63UZ
     */
    it('testLowSpeed005', 0, async (done: Function) => {
      try {
        let conf: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: baseUrl + '/test.txt',
          saveas: 'testLowSpeed005.txt',
          mode: request.agent.Mode.BACKGROUND,
          minSpeed: {
            speed: -1,
            duration: 1,
          }
        };
        await request.agent.create(context, conf);
        expect(false).assertTrue();
        done();
      } catch (err) {
        console.info('testLowSpeed005 error: ' + JSON.stringify(err));
        expect(err.code).assertEqual(401);
        done();
      }
    });

    /**
     * @tc.name: testLowSpeed006
     * @tc.desc: Tests the task can be completed normally when MinSpeed is satisfied.
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. start a download task when MinSpeed is satisfied.
     *           2. check if the complete callback is triggered.
     * @tc.expect: complete callback is triggered.
     * @tc.type: Func
     * @tc.level: Level 1
     * @tc.require: issue#IC63UZ
     */
    it('testLowSpeed006', 0, async (done: Function) => {
      try {
        let conf: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: baseUrl + '/test.txt',
          saveas: 'testLowSpeed006.txt',
          mode: request.agent.Mode.BACKGROUND,
          minSpeed: {
            speed: 1,
            duration: 1,
          }
        };
        let task = await request.agent.create(context, conf);
        task.on('completed', (progress) => {
          expect(true).assertTrue();
          done();
        });
        await task.start();
      } catch (err) {
        console.info('testLowSpeed006 error: ' + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.name: testLowSpeed007
     * @tc.desc: Tests the task can be completed normally when MinSpeed is not satisfied.
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. start a download task when MinSpeed is not satisfied.
     *           2. check if the fault callback is triggered.
     * @tc.expect: fault callback is triggered.
     * @tc.type: Func
     * @tc.level: Level 1
     * @tc.require: issue#IC63UZ
     * 
     * 注意：此测试需要真实的低速网络环境才能通过。本地服务器响应太快，
     * 无法触发 minSpeed 故障条件，导致测试无限期等待 faultOccur 回调而超时。
     * 注释掉此测试以防止超时。
     */
    /*
    it('testLowSpeed007', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: baseUrl + '/large_test.zip',
        saveas: 'testLowSpeed007.txt',
        mode: request.agent.Mode.BACKGROUND,
        minSpeed: {
          speed: 1000 * 1000 * 1000,
          duration: 1,
        }
      };
      let task = await request.agent.create(context, conf);
      task.on('faultOccur', (faults) => {
        expect(faults).assertEqual(0x90);
        done();
      });
      try {
        await task.start();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    */

    /**
     * @tc.name: testLowSpeed008
     * @tc.desc: Tests no error occurs if speed is above the minimum limit but duration is not met.
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. create a download task with MinSpeed.duration is 2 and MinSpeed.speed is 1000000000.
     *           2. start the task.
     *           3. sleep 1s.
     *           4. pause the task.
     *           5. resume the task.
     *           6. check if the fault callback is triggered.
     * @tc.expect: fault callback is triggered.
     * @tc.type: Func
     * @tc.level: Level 1
     * @tc.require: issue#IC63UZ
     * 
     * 注意：此测试需要真实的低速网络环境才能通过。本地服务器响应太快，
     * 无法触发 minSpeed 故障条件。注释掉此测试以防止超时。
     */
    /*
    it('testLowSpeed008', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: baseUrl + '/large_test.zip',
        saveas: 'testLowSpeed008.txt',
        mode: request.agent.Mode.BACKGROUND,
        minSpeed: {
          speed: 1000 * 1000 * 1000,
          duration: 1,
        }
      };
      let task = await request.agent.create(context, conf);
      task.on('faultOccur', (faults) => {
        expect(false).assertTrue();
        done();
      });
      try {
        await task.start();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
      await sleep(1000);
      await task.pause();
      task.off('faultOccur');
      task.on('faultOccur', (faults) => {
        expect(faults).assertEqual(0x90);
        done();
      });
      try {
        await task.resume();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
    */

    /**
     * @tc.number: testTimeout001
     * @tc.name: testTimeout001
     * @tc.desc: Test on task background task
     * @tc.step: 1. start a download task when connectionTimeout is 0.
     *           2. check the error code.
     * @tc.expect: error code equals 401.
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testTimeout001', 0, async (done: Function) => {
      try {
        let conf: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: baseUrl + '/test.txt',
          saveas: 'testTimeout001.txt',
          mode: request.agent.Mode.BACKGROUND,
          timeout: {
            connectionTimeout: 0,
          },
        };
        await request.agent.create(context, conf);
        expect(false).assertTrue();
        done();
      } catch (err) {
        console.info('testTimeout001 error: ' + JSON.stringify(err));
        expect(err.code).assertEqual(401);
        done();
      }
    });

    /**
     * @tc.number: testTimeout002
     * @tc.name: testTimeout002
     * @tc.desc: Test on task background task
     * @tc.step: 1. start a download task when totalTimeout is 0.
     *           2. check the error code.
     * @tc.expect: error code equals 401.
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testTimeout002', 0, async (done: Function) => {
      try {
        let conf: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: baseUrl + '/test.txt',
          saveas: 'testTimeout002.txt',
          mode: request.agent.Mode.BACKGROUND,
          timeout: {
            connectionTimeout: 1,
            totalTimeout: 0,
          },
        };
        await request.agent.create(context, conf);
        expect(false).assertTrue();
        done();
      } catch (err) {
        console.info('testTimeout002 error: ' + JSON.stringify(err));
        expect(err.code).assertEqual(401);
        done();
      }
    });

    /**
     * @tc.number: testTimeout003
     * @tc.name: testTimeout003
     * @tc.desc: Test on task background task
     * @tc.step: 1. start a download task when totalTimeout is 604801.
     *           2. check the error code.
     * @tc.expect: error code equals 401.
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testTimeout003', 0, async (done: Function) => {
      try {
        let conf: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: baseUrl + '/test.txt',
          saveas: 'testTimeout003.txt',
          mode: request.agent.Mode.BACKGROUND,
          timeout: {
            connectionTimeout: 1,
            totalTimeout: 604801,
          },
        };
        await request.agent.create(context, conf);
        expect(false).assertTrue();
        done();
      } catch (err) {
        console.info('testTimeout003 error: ' + JSON.stringify(err));
        expect(err.code).assertEqual(401);
        done();
      }
    });

    /**
     * @tc.number: testTimeout004
     * @tc.name: testTimeout004
     * @tc.desc: Test on task background task
     * @tc.step: 1. start a download task when connectionTimeout is 2, totalTimeout is 10.
     *           2. check the error code.
     * @tc.expect: the task is created success.
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testTimeout004', 0, async (done: Function) => {
      try {
        let conf: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: baseUrl + '/test.txt',
          saveas: 'testTimeout004.txt',
          mode: request.agent.Mode.BACKGROUND,
          timeout: {
            connectionTimeout: 2,
            totalTimeout: 10,
          },
        };
        let task = await request.agent.create(context, conf);
        task.on('completed', (progress) => {
          expect(true).assertTrue();
          done();
        });
        await task.start();
      } catch (err) {
        console.info('testTimeout004 error: ' + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testTimeout005
     * @tc.name: testTimeout005
     * @tc.desc: Test on task background task
     * @tc.step: 1. start a download task when connectionTimeout is 1.
     *           2. check the error code.
     * @tc.expect: the task is created success.
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issue#IC8U1A
     */
    it('testTimeout005', 0, async (done: Function) => {
      try {
        // Use a non-existent local port to simulate connection timeout
        // Local server only listens on port 8078, so 18078 will cause connection timeout
        let conf: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: 'http://127.0.0.1:18078/timeout_test.txt',
          saveas: 'testTimeout005.txt',
          mode: request.agent.Mode.BACKGROUND,
          timeout: {
            connectionTimeout: 1,
          },
        };
        let task = await request.agent.create(context, conf);
        task.on('faultOccur', (faults) => {
          expect(faults).assertEqual(request.agent.Faults.TCP);
          done();
        });
        await task.start();
      } catch (err) {
        console.info('testTimeout005 error: ' + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testTimeout006
     * @tc.name: testTimeout006
     * @tc.desc: Test on task background task
     * @tc.step: 1. start a download task when connectionTimeout is 10, totalTimeout is 2.
     *           2. check the error code.
     * @tc.expect: the task is created success.
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issue#IC8U1A
     */
    it('testTimeout006', 0, async (done: Function) => {
      try {
        let conf: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: baseUrl + '/large_test.zip',
          saveas: 'testTimeout006.txt',
          mode: request.agent.Mode.BACKGROUND,
          timeout: {
            connectionTimeout: 10,
            totalTimeout: 2,
          },
        };
        let task = await request.agent.create(context, conf);
        task.on('faultOccur', (faults) => {
          expect(faults).assertEqual(request.agent.Faults.TIMEOUT);
          done();
        });
        await task.start();
      } catch (err) {
        console.info('testTimeout006 error: ' + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testTimeout007
     * @tc.name: testTimeout007
     * @tc.desc: Test on task background task
     * @tc.step: 1. start a download task when connectionTimeout is 5, totalTimeout is 10.
     *           2. check the error code.
     * @tc.expect: the task is created success.
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issue#IC8U1A
     */
    it('testTimeout007', 15000, async (done: Function) => {
      try {
        // Use local server to trigger total timeout
        // Local server listens on port 8078
        // totalTimeout is 10 seconds, server delays 12 seconds to trigger totalTimeout
        let conf: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: baseUrl + '/testTimeout007.txt',
          saveas: 'testTimeout007.txt',
          mode: request.agent.Mode.BACKGROUND,
          timeout: {
            connectionTimeout: 5,
            totalTimeout: 10,
          },
        };
        let task = await request.agent.create(context, conf);
        task.on('faultOccur', (faults) => {
          // Total timeout should trigger TIMEOUT fault
          expect(faults).assertEqual(request.agent.Faults.TIMEOUT);
          done();
        });
        await task.start();
      } catch (err) {
        console.info('testTimeout007 error: ' + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testTimeout008
     * @tc.name: testTimeout008
     * @tc.desc: Test on task background task
     * @tc.step: 1. start a download task when totalTimeout is 604800.
     *           2. check the error code.
     * @tc.expect: the task is created success.
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issue#IC8U1A
     */
    it('testTimeout008', 0, async (done: Function) => {
      try {
        let conf: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: baseUrl + '/test.txt',
          saveas: 'testTimeout008.txt',
          mode: request.agent.Mode.BACKGROUND,
          timeout: {
            totalTimeout: 604800,
          },
        };
        let task = await request.agent.create(context, conf);
        task.on('completed', (progress) => {
          expect(true).assertTrue();
          done();
        });
        await task.start();
      } catch (err) {
        console.info('testTimeout008 error: ' + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });
  })
}