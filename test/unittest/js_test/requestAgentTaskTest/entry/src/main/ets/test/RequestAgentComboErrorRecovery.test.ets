/*
 * Copyright (c) 2025 Huawei Device Co., Ltd.
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

import { describe, beforeAll, beforeEach, afterEach, afterAll, it, expect } from "@ohos/hypium"
import request from '@ohos.request';
import fs from '@ohos.file.fs';
import common from '@ohos.app.ability.common';
import Server from '../common/iTcpServer'

/**
 * 错误恢复组合场景测试
 * 测试重试机制、错误处理等恢复相关组合
 */

export default function requestAgentComboErrorRecoveryTest() {
  describe('requestAgentComboErrorRecoveryTest', () => {
    let server: Server | null = null;
    let context: common.UIAbilityContext = globalThis.abilityContext;
    let cacheDir: string = globalThis.cacheDir;

    beforeAll(async () => {
      console.info('beforeAll called');
      server = new Server();
      await server.serverStart();
    });

    afterAll(async () => {
      console.info('afterAll called');
      if (server) {
        await server.serverClose();
      }
    });

    beforeEach(() => {
      console.info('beforeEach called');
      const files = ['test.txt', 'recovery.txt'];
      for (const file of files) {
        const path = cacheDir + '/' + file;
        if (fs.accessSync(path)) {
          fs.unlinkSync(path);
        }
      }
    });

    afterEach(() => {
      console.info('afterEach called');
    });

    // ==================== 重试机制组合 ====================

    /**
     * @tc.number: testComboRetryTrueWithBackground001
     * @tc.name: testComboRetryTrueWithBackground001
     * @tc.desc: retry=true with BACKGROUND mode
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, retry: true, mode: BACKGROUND.
     *           2. Call request.agent.create() to create the task.
     *           3. Assert task.config.retry is true and task.config.mode is BACKGROUND.
     *           4. Call task.getTaskInfo() and assert info.retry is true.
     *           5. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboRetryTrueWithBackground001', 0, async (done: Function) => {
      console.info("===> testComboRetryTrueWithBackground001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          retry: true,
          mode: request.agent.Mode.BACKGROUND,
        };
        let task = await request.agent.create(context, config);
        expect(task.config.retry).assertTrue();
        expect(task.config.mode).assertEqual(request.agent.Mode.BACKGROUND);
        
        let info = await task.getTaskInfo();
        expect(info.retry).assertTrue();
        
        await task.delete();
        console.info("===> testComboRetryTrueWithBackground001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testComboRetryFalseWithForeground001
     * @tc.name: testComboRetryFalseWithForeground001
     * @tc.desc: retry=false with FOREGROUND mode
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, retry: false, mode: FOREGROUND.
     *           2. Call request.agent.create() to create the task.
     *           3. Assert task.config.retry is false and task.config.mode is FOREGROUND.
     *           4. Call task.getTaskInfo() and assert info.retry is false.
     *           5. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboRetryFalseWithForeground001', 0, async (done: Function) => {
      console.info("===> testComboRetryFalseWithForeground001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          retry: false,
          mode: request.agent.Mode.FOREGROUND,
        };
        let task = await request.agent.create(context, config);
        expect(task.config.retry).assertFalse();
        expect(task.config.mode).assertEqual(request.agent.Mode.FOREGROUND);
        
        let info = await task.getTaskInfo();
        expect(info.retry).assertFalse();
        
        await task.delete();
        console.info("===> testComboRetryFalseWithForeground001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testComboRetryWithNetworkCellular001
     * @tc.name: testComboRetryWithNetworkCellular001
     * @tc.desc: retry with CELLULAR network
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to cellular network
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, retry: true, network: CELLULAR, metered: true.
     *           2. Call request.agent.create() to create the task.
     *           3. Assert task.config.retry is true and task.config.network is CELLULAR.
     *           4. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboRetryWithNetworkCellular001', 0, async (done: Function) => {
      console.info("===> testComboRetryWithNetworkCellular001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          retry: true,
          network: request.agent.Network.CELLULAR,
          metered: true,
        };
        let task = await request.agent.create(context, config);
        expect(task.config.retry).assertTrue();
        expect(task.config.network).assertEqual(request.agent.Network.CELLULAR);
        
        await task.delete();
        console.info("===> testComboRetryWithNetworkCellular001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testComboRetryWithRangeDownload001
     * @tc.name: testComboRetryWithRangeDownload001
     * @tc.desc: retry with range download (断点续传)
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, retry: true, mode: BACKGROUND, begins: 1024.
     *           2. Call request.agent.create() to create the task.
     *           3. Assert task.config.retry is true and task.config.begins equals 1024.
     *           4. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboRetryWithRangeDownload001', 0, async (done: Function) => {
      console.info("===> testComboRetryWithRangeDownload001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          retry: true,
          begins: 1024,
          mode: request.agent.Mode.BACKGROUND,
        };
        let task = await request.agent.create(context, config);
        expect(task.config.retry).assertTrue();
        expect(task.config.begins).assertEqual(1024);
        
        await task.delete();
        console.info("===> testComboRetryWithRangeDownload001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    // ==================== 错误处理组合 ====================

    /**
     * @tc.number: testComboErrorHandlingWithEvent001
     * @tc.name: testComboErrorHandlingWithEvent001
     * @tc.desc: Error handling with failed event
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, retry: false.
     *           2. Call request.agent.create() to create the task.
     *           3. Subscribe to 'failed' event using task.on('failed', callback).
     *           4. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboErrorHandlingWithEvent001', 0, async (done: Function) => {
      console.info("===> testComboErrorHandlingWithEvent001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          retry: false,
        };
        let task = await request.agent.create(context, config);
        
        // 订阅失败事件
        task.on('failed', (error) => {
          console.info("===> failed event:" + JSON.stringify(error));
        });
        
        await task.delete();
        console.info("===> testComboErrorHandlingWithEvent001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testComboRecoveryWithQuery001
     * @tc.name: testComboRecoveryWithQuery001
     * @tc.desc: Task recovery with query
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, retry: true, mode: BACKGROUND.
     *           2. Call request.agent.create() to create the task and get tid.
     *           3. Call request.agent.query(tid) to query task info.
     *           4. Assert the queried info.tid equals task tid and info.retry is true.
     *           5. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboRecoveryWithQuery001', 0, async (done: Function) => {
      console.info("===> testComboRecoveryWithQuery001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          retry: true,
          mode: request.agent.Mode.BACKGROUND,
        };
        let task = await request.agent.create(context, config);
        let tid = task.tid;
        
        // 查询任务状态（模拟恢复）
        let info = await request.agent.query(tid);
        expect(info.tid).assertEqual(tid);
        expect(info.retry).assertTrue();
        
        await task.delete();
        console.info("===> testComboRecoveryWithQuery001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testComboRecoveryWithSearch001
     * @tc.name: testComboRecoveryWithSearch001
     * @tc.desc: Task recovery with search
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, retry: true.
     *           2. Call request.agent.create() to create the task and get tid.
     *           3. Create Filter with action: DOWNLOAD and call request.agent.search(filter).
     *           4. Assert the search result tids includes the task tid.
     *           5. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboRecoveryWithSearch001', 0, async (done: Function) => {
      console.info("===> testComboRecoveryWithSearch001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          retry: true,
        };
        let task = await request.agent.create(context, config);
        let tid = task.tid;
        
        // 搜索任务（模拟恢复）
        let filter: request.agent.Filter = {
          action: request.agent.Action.DOWNLOAD,
        };
        let tids = await request.agent.search(filter);
        expect(tids.includes(tid)).assertTrue();
        
        await task.delete();
        console.info("===> testComboRecoveryWithSearch001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    // ==================== 复杂恢复场景 ====================

    /**
     * @tc.number: testComboComplexErrorRecovery001
     * @tc.name: testComboComplexErrorRecovery001
     * @tc.desc: Complex error recovery scenario
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, retry: true, mode: BACKGROUND,
     *              network: ANY, begins: 0, saveas: './recovery.txt'.
     *           2. Call request.agent.create() to create the task.
     *           3. Subscribe to 'failed' event using task.on('failed', callback).
     *           4. Call task.getTaskInfo() and assert info.retry is true and state is INITIALIZED.
     *           5. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboComplexErrorRecovery001', 0, async (done: Function) => {
      console.info("===> testComboComplexErrorRecovery001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          retry: true,
          mode: request.agent.Mode.BACKGROUND,
          network: request.agent.Network.ANY,
          begins: 0,
          saveas: './recovery.txt',
        };
        let task = await request.agent.create(context, config);
        
        // 订阅失败事件
        task.on('failed', (error) => {
          console.info("===> failed, will retry:" + JSON.stringify(error));
        });
        
        let info = await task.getTaskInfo();
        expect(info.retry).assertTrue();
        expect(info.progress.state).assertEqual(request.agent.State.INITIALIZED);
        
        await task.delete();
        console.info("===> testComboComplexErrorRecovery001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testComboRetryWithUpload001
     * @tc.name: testComboRetryWithUpload001
     * @tc.desc: retry with UPLOAD action
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create test file 'test.txt' with content 'test content'.
     *           2. Create Config with action: UPLOAD, valid URL, retry: true, data with file path.
     *           3. Call request.agent.create() to create the upload task.
     *           4. Assert task.config.retry is true and task.config.action is UPLOAD.
     *           5. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboRetryWithUpload001', 0, async (done: Function) => {
      console.info("===> testComboRetryWithUpload001 start");
      try {
        let testFile = cacheDir + '/test.txt';
        let file = fs.openSync(testFile, fs.OpenMode.CREATE | fs.OpenMode.READ_WRITE);
        fs.writeSync(file, 'test content');
        fs.closeSync(file);

        let config: request.agent.Config = {
          action: request.agent.Action.UPLOAD,
          url: "http://127.0.0.1:18080/upload",
          retry: true,
          data: [{
            name: 'file',
            value: { path: './test.txt' }
          }],
        };
        let task = await request.agent.create(context, config);
        expect(task.config.retry).assertTrue();
        expect(task.config.action).assertEqual(request.agent.Action.UPLOAD);
        
        await task.delete();
        console.info("===> testComboRetryWithUpload001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testComboRetryWithHighPriority001
     * @tc.name: testComboRetryWithHighPriority001
     * @tc.desc: retry with high priority
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, retry: true, priority: 1000, mode: BACKGROUND.
     *           2. Call request.agent.create() to create the task.
     *           3. Assert task.config.retry is true and task.config.priority equals 1000.
     *           4. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboRetryWithHighPriority001', 0, async (done: Function) => {
      console.info("===> testComboRetryWithHighPriority001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          retry: true,
          priority: 1000,
          mode: request.agent.Mode.BACKGROUND,
        };
        let task = await request.agent.create(context, config);
        expect(task.config.retry).assertTrue();
        expect(task.config.priority).assertEqual(1000);
        
        await task.delete();
        console.info("===> testComboRetryWithHighPriority001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

  });
}
