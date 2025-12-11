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

export default function requestCallbackTaskTest() {
  describe('requestCallbackTaskTest', () => {
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
     * @tc.number: testOnTask001
     * @tc.name: testOnTask001
     * @tc.desc: Test on task for 'test'
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, saveas, and mode: BACKGROUND.
     *           2. Call request.agent.create() to create a task instance.
     *           3. Try to bind 'test' event (force type conversion to 'progress') with an empty callback.
     *           4. Catch the error, assert err.code equals 401.
     *           5. Clean up with request.agent.remove(), then call done().
     * @tc.require: issues#1573
     */
    it('testOnTask001', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testOnTask001.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      try {
        task.on('test' as Object as 'progress', (progress) => {
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        await request.agent.remove(task.tid);
        done();
      }
    });

    /**
     * @tc.number: testOnTask002
     * @tc.name: testOnTask002
     * @tc.desc: Test on task for number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, saveas, and mode: BACKGROUND.
     *           2. Call request.agent.create() to create a task instance.
     *           3. Try to bind event with number 123 (force type conversion to 'progress') and an empty callback.
     *           4. Catch the error, assert err.code equals 401.
     *           5. Clean up with request.agent.remove(), then call done().
     * @tc.require: issues#1573
     */
    it('testOnTask002', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testOnTask002.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      try {
        task.on(123 as Object as 'progress', (progress) => {
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        await request.agent.remove(task.tid);
        done();
      }
    });

    /**
     * @tc.number: testOnTask003
     * @tc.name: testOnTask003
     * @tc.desc: Test on task for failed
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, invalid URL, saveas, and mode: BACKGROUND.
     *           2. Call request.agent.create() to create a task instance.
     *           3. Bind 'failed' event, assert progress.state equals request.agent.State.FAILED in the callback.
     *           4. Call task.start() to execute the task, wait for event trigger.
     *           5. Trigger done() after event assertion.
     * @tc.require: issues#1573
     */
    it('testOnTask003', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test1.txt',
        saveas: 'testOnTask003.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      task.on('failed', (progress) => {
        expect(progress.state).assertEqual(request.agent.State.FAILED);
        done();
      });
      await task.start();
    });

    /**
     * @tc.number: testOnTask004
     * @tc.name: testOnTask004
     * @tc.desc: Test on task for progress
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, saveas, and mode: BACKGROUND.
     *           2. Call request.agent.create() to create a task instance.
     *           3. Bind 'progress' event, assert progress.state equals request.agent.State.RUNNING.
     *           4. Unbind 'progress' event in the callback to avoid repeated triggers.
     *           5. Call task.start() to execute the task, trigger done() after assertion.
     * @tc.require: issues#1573
     */
    it('testOnTask004', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testOnTask004.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      task.on('progress', (progress) => {
        expect(progress.state).assertEqual(request.agent.State.RUNNING);
        task.off('progress');
        done();
      });
      await task.start();
    });

    /**
     * @tc.number: testOnTask005
     * @tc.name: testOnTask005
     * @tc.desc: Test on task background task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, saveas, and mode: BACKGROUND.
     *           2. Call request.agent.create() to create a task instance.
     *           3. Bind 'completed' event, assert progress.state equals request.agent.State.COMPLETED.
     *           4. Call task.start() to execute the task, trigger done() after event assertion.
     * @tc.require: issues#1573
     */
    it('testOnTask005', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testOnTask005.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      task.on('completed', (progress) => {
        expect(progress.state).assertEqual(request.agent.State.COMPLETED);
        done();
      });
      await task.start();
    });

    /**
     * @tc.number: testOnTask006
     * @tc.name: testOnTask006
     * @tc.desc: Test on task for progress state REMOVED
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, large file URL, saveas, and mode: BACKGROUND.
     *           2. Call request.agent.create() to create a task instance.
     *           3. Bind 'remove' event, assert progress.state equals request.agent.State.REMOVED.
     *           4. Call task.start() to execute the task, wait 2 seconds for task running.
     *           5. Call request.agent.remove() to delete the task, trigger done() after event assertion.
     * @tc.require: issues#1573
     */
    it('testOnTask006', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/large_test.zip',
        saveas: 'testOnTask006.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      task.on('remove', (progress) => {
        expect(progress.state).assertEqual(request.agent.State.REMOVED);
        done();
      });
      await task.start();
      await sleep(2000);
      await request.agent.remove(task.tid);
    });

    /**
     * @tc.number: testOnTask007
     * @tc.name: testOnTask007
     * @tc.desc: Test on task for progress twice
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, saveas, and mode: BACKGROUND.
     *           2. Call request.agent.create() to create a task instance.
     *           3. Bind 'progress' event twice with empty callbacks.
     *           4. Assert true if no error occurs, clean up with request.agent.remove().
     *           5. Call done() after cleanup; catch error and assert false if exception occurs.
     * @tc.require: issues#1573
     */
    it('testOnTask007', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testOnTask007.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      try {
        task.on('progress', (progress) => {
        });
        task.on('progress', (progress) => {
        });
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testOnTask008
     * @tc.name: testOnTask008
     * @tc.desc: Test on task for pause
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, large file URL, saveas, and mode: BACKGROUND.
     *           2. Call request.agent.create() to create a task instance.
     *           3. Call task.start() to execute the task, wait 1 second for task running.
     *           4. Call task.pause() to pause the task, then bind 'pause' event with empty callback.
     *           5. Assert true if no error occurs, clean up with request.agent.remove(), call done().
     * @tc.require: issues#1573
     */
    it('testOnTask008', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/large_test.zip',
        saveas: 'testOnTask008.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      await task.start();
      await sleep(1000);
      await task.pause();
      try {
        task.on('pause', (progress) => {
        });
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testOnTask009
     * @tc.name: testOnTask009
     * @tc.desc: Test on task for resume
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, large file URL, saveas, and mode: BACKGROUND.
     *           2. Call request.agent.create() to create a task instance.
     *           3. Call task.start() → task.pause() → task.resume() in sequence to trigger resume state.
     *           4. Bind 'resume' event with empty callback, assert true if no error occurs.
     *           5. Call done() directly; catch error and assert false if exception occurs.
     * @tc.require: issues#1573
     */
    it('testOnTask009', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/large_test.zip',
        saveas: 'testOnTask009.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      await task.start();
      await task.pause();
      await task.resume();
      try {
        task.on('resume', (progress) => {
        });
        expect(true).assertTrue();
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testOnTask010
     * @tc.name: testOnTask010
     * @tc.desc: Test on task for faults
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, invalid URL, saveas, and mode: BACKGROUND.
     *           2. Call request.agent.create() to create a task instance.
     *           3. Bind 'faultOccur' event, assert faults equals request.agent.Faults.PROTOCOL.
     *           4. Call task.start() to execute the task, trigger done() after event assertion.
     * @tc.require: issues#1573
     */
    it('testOnTask010', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test1.txt',
        saveas: 'testOnTask010.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      task.on('faultOccur', (faults) => {
        expect(faults).assertEqual(request.agent.Faults.PROTOCOL);
        done();
      });
      await task.start();
    });

    /**
     * @tc.number: testOnTask011
     * @tc.name: testOnTask011
     * @tc.desc: Test on task for faults
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, invalid domain URL, saveas, and mode: BACKGROUND.
     *           2. Call request.agent.create() to create a task instance.
     *           3. Bind 'faultOccur' event, assert faults equals request.agent.Faults.DNS.
     *           4. Call task.start() to execute the task, trigger done() after event assertion.
     * @tc.require: issues#1573
     */
    it('testOnTask011', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee111.com/chenzhixue/downloadTest/releases/download/v1.0/test1.txt',
        saveas: 'testOnTask011.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      task.on('faultOccur', (faults) => {
        expect(faults).assertEqual(request.agent.Faults.DNS);
        done();
      });
      await task.start();
    });

    /**
     * @tc.number: testOnTask012
     * @tc.name: testOnTask012
     * @tc.desc: Test on task for faults
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, invalid format URL (https:////), saveas, and mode: BACKGROUND.
     *           2. Call request.agent.create() to create a task instance.
     *           3. Bind 'faultOccur' event, assert faults equals request.agent.Faults.PARAM.
     *           4. Call task.start() to execute the task, trigger done() after event assertion.
     * @tc.require: issues#1573
     */
    it('testOnTask012', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https:////',
        saveas: 'testOnTask012.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      task.on('faultOccur', (faults) => {
        expect(faults).assertEqual(request.agent.Faults.PARAM);
        done();
      });
      await task.start();
    });

    /**
     * @tc.number: testOffTask001
     * @tc.name: testOffTask001
     * @tc.desc: Test off task for 'test'
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, saveas, and mode: BACKGROUND.
     *           2. Call request.agent.create() to create a task instance.
     *           3. Try to unbind 'test' event (force type conversion to 'progress') with empty callback.
     *           4. Catch the error, assert err.code equals 401.
     *           5. Clean up with request.agent.remove(), then call done().
     * @tc.require: issues#1573
     */
    it('testOffTask001', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testOffTask001.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      try {
        task.off('test' as Object as 'progress', () => {
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        await request.agent.remove(task.tid);
        done();
      }
    });

    /**
     * @tc.number: testOffTask002
     * @tc.name: testOffTask002
     * @tc.desc: Test off task for number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, saveas, and mode: BACKGROUND.
     *           2. Call request.agent.create() to create a task instance.
     *           3. Try to unbind event with number 123 (force type conversion to 'progress').
     *           4. Catch the error, assert err.code equals 401.
     *           5. Clean up with request.agent.remove(), then call done().
     * @tc.require: issues#1573
     */
    it('testOffTask002', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testOffTask002.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      try {
        task.off(123 as Object as 'progress');
      } catch (err) {
        expect(err.code).assertEqual(401);
        await request.agent.remove(task.tid);
        done();
      }
    });

    /**
     * @tc.number: testOffTask003
     * @tc.name: testOffTask003
     * @tc.desc: Test off task for failed
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, invalid URL, saveas, and mode: BACKGROUND.
     *           2. Call request.agent.create() to create a task instance.
     *           3. Bind 'failed' event (assert false if triggered), then unbind it with task.off('failed').
     *           4. Call task.start() to execute the task, wait 2 seconds for task completion.
     *           5. Assert true (event not triggered), call done().
     * @tc.require: issues#1573
     */
    it('testOffTask003', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test1.txt',
        saveas: 'testOffTask003.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      task.on('failed', () => {
        expect(false).assertTrue();
        done();
      });
      task.off('failed');
      await task.start();
      await sleep(2000);
      expect(true).assertTrue();
      done();
    });

    /**
     * @tc.number: testOffTask004
     * @tc.name: testOffTask004
     * @tc.desc: Test off task for completed
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, saveas, and mode: BACKGROUND.
     *           2. Call request.agent.create() to create a task instance.
     *           3. Bind 'completed' event (assert true if triggered) for normal verification.
     *           4. Define a CompletedCallback (assert false if triggered), bind it then unbind with task.off().
     *           5. Call task.start() to execute the task, trigger done() after normal event assertion.
     * @tc.require: issues#1573
     */
    it('testOffTask004', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testOffTask004.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      task.on('completed', (progress) => {
        expect(true).assertTrue();
        done();
      });

      let CompletedCallback: (progress: request.agent.Progress) => void = (progress: request.agent.Progress) : void => {
        expect(false).assertTrue();
        done();
      }

      task.on('completed', CompletedCallback);
      task.off('completed', CompletedCallback);
      await task.start();
    });

    /**
     * @tc.number: testOffTask005
     * @tc.name: testOffTask005
     * @tc.desc: Test off task for progress
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, saveas, and mode: BACKGROUND.
     *           2. Call request.agent.create() to create a task instance.
     *           3. Bind 'progress' and 'completed' events (assert true if triggered) for normal verification.
     *           4. Define a ProgressCallback (assert false if triggered), bind it then unbind with task.off().
     *           5. Call task.start() to execute the task, trigger done() after normal event assertion.
     * @tc.require: issues#1573
     */
    it('testOffTask005', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testOffTask005.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      task.on('progress', (progress) => {
        expect(true).assertTrue();
        done();
      });
      task.on('completed', (progress) => {
        expect(true).assertTrue();
        done();
      });

      let ProgressCallback: (progress: request.agent.Progress) => void = (progress: request.agent.Progress) : void => {
        expect(false).assertTrue();
        done();
      }

      task.on('progress', ProgressCallback);
      task.off('progress', ProgressCallback);
      await task.start();
    });

    /**
     * @tc.number: testOffTask006
     * @tc.name: testOffTask006
     * @tc.desc: Test off task twice
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, saveas, and mode: BACKGROUND.
     *           2. Call request.agent.create() to create a task instance.
     *           3. Unbind 'completed' event twice in sequence with task.off('completed').
     *           4. Assert true if no error occurs, clean up with request.agent.remove().
     *           5. Call done() after cleanup; catch error and assert false if exception occurs.
     * @tc.require: issues#1573
     */
    it('testOffTask006', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testOffTask006.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      try {
        task.off('completed');
        task.off('completed');
        expect(true).assertTrue();
        await request.agent.remove(task.tid);
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testOffTask007
     * @tc.name: testOffTask007
     * @tc.desc: Test off task for pause
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, saveas, and mode: BACKGROUND.
     *           2. Call request.agent.create() to create a task instance.
     *           3. Bind two 'pause' events (assert true if triggered) for normal verification.
     *           4. Define a PauseCallback (assert false if triggered), bind it then unbind with task.off().
     *           5. Call task.start() → task.pause() in sequence, trigger done() after normal event assertion.
     * @tc.require: issues#1573
     */
    it('testOffTask007', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testOffTask007.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      task.on('pause', (progress) => {
        expect(true).assertTrue();
        done();
      });
      task.on('pause', (progress) => {
        expect(true).assertTrue();
        done();
      });

      let PauseCallback: (progress: request.agent.Progress) => void = (progress: request.agent.Progress) : void => {
        expect(false).assertTrue();
        done();
      }

      task.on('pause', PauseCallback);
      task.off('pause', PauseCallback);
      await task.start();
      await task.pause();
    });

    /**
     * @tc.number: testOffTask008
     * @tc.name: testOffTask008
     * @tc.desc: Test off task for resume
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, saveas, and mode: BACKGROUND.
     *           2. Call request.agent.create() to create a task instance.
     *           3. Bind two 'resume' events (assert true if triggered) for normal verification.
     *           4. Define a ResumeCallback (assert false if triggered), bind it then unbind with task.off().
     *           5. Call task.start() → task.pause() → task.resume() in sequence,
     *              trigger done() after normal event assertion.
     * @tc.require: issues#1573
     */
    it('testOffTask008', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testOffTask008.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      task.on('resume', (progress) => {
        expect(true).assertTrue();
        done();
      });
      task.on('resume', (progress) => {
        expect(true).assertTrue();
        done();
      });

      let ResumeCallback: (progress: request.agent.Progress) => void = (progress: request.agent.Progress) : void => {
        expect(false).assertTrue();
        done();
      }

      task.on('resume', ResumeCallback);
      task.off('resume', ResumeCallback);
      await task.start();
      await task.pause();
      await task.resume();
    });

    /**
     * @tc.number: testOffTask009
     * @tc.name: testOffTask009
     * @tc.desc: Test off task for remove
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, saveas, and mode: BACKGROUND.
     *           2. Call request.agent.create() to create a task instance.
     *           3. Bind 'remove' event (assert true if triggered) for normal verification.
     *           4. Define a CompletedCallback (assert false if triggered), bind it then unbind with task.off().
     *           5. Call task.start() → request.agent.remove() in sequence,
     *              trigger done() after normal event assertion.
     * @tc.require: issues#1573
     */
    it('testOffTask009', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testOffTask009.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      task.on('remove', (progress) => {
        expect(true).assertTrue();
        done();
      });

      let CompletedCallback: (progress: request.agent.Progress) => void = (progress: request.agent.Progress) : void => {
        expect(false).assertTrue();
        done();
      }

      task.on('remove', CompletedCallback);
      task.off('remove', CompletedCallback);
      await task.start();
      await request.agent.remove(task.tid);
    });

    /**
     * @tc.number: testOnResponse001
     * @tc.name: testOnResponse001
     * @tc.desc: Test on response
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, saveas, mode: BACKGROUND, and overwrite: true.
     *           2. Call request.agent.create() to create a task instance.
     *           3. Bind 'response' event, assert response.statusCode=200, version="HTTP/1.1", reason="OK".
     *           4. Call task.start() to execute the task, trigger done() after event assertion.
     * @tc.require: issues#1573
     */
    it('testOnResponse001', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testOnResponse001.txt',
        mode: request.agent.Mode.BACKGROUND,
        overwrite: true,
      };
      let task = await request.agent.create(context, conf);
      task.on('response', (response: request.agent.HttpResponse) => {
        expect(response.statusCode).assertEqual(200);
        expect(response.version).assertEqual("HTTP/1.1");
        expect(response.reason).assertEqual("OK");
        done();
      });
      await task.start();
    });

    /**
     * @tc.number: testOnResponse002
     * @tc.name: testOnResponse002
     * @tc.desc: Test off response
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, saveas, mode: BACKGROUND, and overwrite: true.
     *           2. Define a callback (assert false if triggered), call request.agent.create() to create task.
     *           3. Bind the callback to 'response' event, then unbind it with task.off().
     *           4. Bind 'completed' event (assert progress.state=COMPLETED) for normal verification.
     *           5. Call task.start() to execute the task, trigger done() after normal event assertion.
     * @tc.require: issues#1573
     */
    it('testOnResponse002', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testOnResponse002.txt',
        mode: request.agent.Mode.BACKGROUND,
        overwrite: true,
      };
      let callback = (response: request.agent.HttpResponse) => {
        expect(false).assertTrue();
        done();
      };
      let task = await request.agent.create(context, conf);
      task.on('response', callback);
      task.off('response', callback);
      task.on('completed', (progress) => {
        expect(progress.state).assertEqual(request.agent.State.COMPLETED);
        done();
      });
      await task.start();
    });

    /**
     * @tc.number: testOnResponse003
     * @tc.name: testOnResponse003
     * @tc.desc: Test off all response
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, saveas, mode: BACKGROUND, and overwrite: true.
     *           2. Call request.agent.create() to create a task instance.
     *           3. Bind two 'response' callbacks (both assert false if triggered),
     *              then unbind all with task.off('response').
     *           4. Bind 'completed' event (assert progress.state=COMPLETED) for normal verification.
     *           5. Call task.start() to execute the task, trigger done() after normal event assertion.
     * @tc.require: issues#1573
     */
    it('testOnResponse003', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testOnResponse003.txt',
        mode: request.agent.Mode.BACKGROUND,
        overwrite: true,
      };
      let task = await request.agent.create(context, conf);
      task.on('response', (response: request.agent.HttpResponse) => {
        expect(false).assertTrue();
        done();
      });
      task.on('response', (response: request.agent.HttpResponse) => {
        expect(response.statusCode).assertEqual(200);
        expect(false).assertTrue();
        done();
      });
      task.off('response');
      task.on('completed', (progress) => {
        expect(progress.state).assertEqual(request.agent.State.COMPLETED);
        done();
      });
      await task.start();
    });

    /**
     * @tc.number: testOnResponse004
     * @tc.name: testOnResponse004
     * @tc.desc: Test off with no response
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, saveas, mode: BACKGROUND, and overwrite: true.
     *           2. Call request.agent.create() to create a task instance.
     *           3. Unbind 'response' event directly (no prior binding) with task.off('response').
     *           4. Bind 'completed' event (assert progress.state=COMPLETED) for normal verification.
     *           5. Call task.start() to execute the task, trigger done() after normal event assertion.
     * @tc.require: issues#1573
     */
    it('testOnResponse004', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testOnResponse004.txt',
        mode: request.agent.Mode.BACKGROUND,
        overwrite: true,
      };
      let task = await request.agent.create(context, conf);
      task.off('response');
      task.on('completed', (progress) => {
        expect(progress.state).assertEqual(request.agent.State.COMPLETED);
        done();
      });
      await task.start();
    });

    /**
     * @tc.number: testOnResponse005
     * @tc.name: testOnResponse005
     * @tc.desc: Test on response with a bad request
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, non-existent file URL,
     *              saveas, mode: BACKGROUND, and overwrite: true.
     *           2. Call request.agent.create() to create a task instance.
     *           3. Bind 'response' event, assert response.statusCode=404, version="HTTP/1.1", reason="Not Found".
     *           4. Unbind 'response' event in the callback to avoid repeated triggers.
     *           5. Call task.start() to execute the task, trigger done() after event assertion.
     * @tc.require: issues#1573
     */
    it('testOnResponse005', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.jpg',
        saveas: 'testOnResponse005.txt',
        mode: request.agent.Mode.BACKGROUND,
        overwrite: true,
      };
      let task = await request.agent.create(context, conf);
      task.on('response', (response: request.agent.HttpResponse) => {
        expect(response.statusCode).assertEqual(404);
        expect(response.version).assertEqual("HTTP/1.1");
        expect(response.reason).assertEqual("Not Found");
        task.off('response');
        done();
      });
      await task.start();
    });

    /**
     * @tc.name: testOnWait001
     * @tc.desc: test the task won't receive wait callback when callback is off.
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, saveas, mode: BACKGROUND, and overwrite: true.
     *           2. Call request.agent.create() to create a task instance.
     *           3. Bind 'wait' event with a callback (assert false if triggered),
     *              then unbind it with task.off('wait').
     *           4. Bind 'completed' event (assert true if triggered) for normal task completion verification.
     *           5. Try to call task.start() to execute the task; catch error and assert false if exception occurs.
     * @tc.expect: no wait callback is triggered.
     * @tc.type: Func
     * @tc.level: Level 1
     * @tc.require: issue#IC6PSO
     */
    it('testOnWait001', 0, async (done: Function) => {
      const conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: `testOnWait001.txt`,
        mode: request.agent.Mode.BACKGROUND,
        overwrite: true,
      };
      const task = await request.agent.create(context, conf);
      task.on('wait', async (reason) => {
        expect(false).assertTrue();
        done();
      });
      task.off('wait');
      task.on('completed', (progress) => {
        expect(true).assertTrue();
        done();
      });
      try {
        await task.start();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });
  })
}