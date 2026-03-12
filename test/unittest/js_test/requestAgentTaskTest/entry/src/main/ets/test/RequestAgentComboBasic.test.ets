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
 * 基础组合场景测试
 * 测试 Action、Mode、Method、Overwrite 等基础字段的组合
 */

export default function requestAgentComboBasicTest() {
  describe('requestAgentComboBasicTest', () => {
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
      const files = ['test.txt', 'test1.txt', 'test2.txt', 'combo.txt'];
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

    // ==================== Action + Mode 组合 ====================

    /**
     * @tc.number: testComboActionUploadModeForeground001
     * @tc.name: testComboActionUploadModeForeground001
     * @tc.desc: UPLOAD + FOREGROUND combination test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a test file with content 'test content for upload'.
     *           2. Create Config with action: UPLOAD, valid URL, mode: FOREGROUND, data with file path.
     *           3. Call request.agent.create() with the config to create task.
     *           4. Assert task.config.action equals UPLOAD.
     *           5. Assert task.config.mode equals FOREGROUND.
     *           6. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboActionUploadModeForeground001', 0, async (done: Function) => {
      console.info("===> testComboActionUploadModeForeground001 start");
      try {
        let testFile = cacheDir + '/test.txt';
        let file = fs.openSync(testFile, fs.OpenMode.CREATE | fs.OpenMode.READ_WRITE);
        fs.writeSync(file, 'test content for upload');
        fs.closeSync(file);

        let config: request.agent.Config = {
          action: request.agent.Action.UPLOAD,
          url: "http://127.0.0.1:18080/upload",
          mode: request.agent.Mode.FOREGROUND,
          data: [{
            name: 'file',
            value: { path: './test.txt' }
          }]
        };
        let task = await request.agent.create(context, config);
        expect(task.config.action).assertEqual(request.agent.Action.UPLOAD);
        expect(task.config.mode).assertEqual(request.agent.Mode.FOREGROUND);
        await task.delete();
        console.info("===> testComboActionUploadModeForeground001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testComboActionUploadModeBackground001
     * @tc.name: testComboActionUploadModeBackground001
     * @tc.desc: UPLOAD + BACKGROUND combination test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a test file with content 'test content for upload'.
     *           2. Create Config with action: UPLOAD, valid URL, mode: BACKGROUND, data with file path.
     *           3. Call request.agent.create() with the config to create task.
     *           4. Assert task.config.action equals UPLOAD.
     *           5. Assert task.config.mode equals BACKGROUND.
     *           6. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboActionUploadModeBackground001', 0, async (done: Function) => {
      console.info("===> testComboActionUploadModeBackground001 start");
      try {
        let testFile = cacheDir + '/test.txt';
        let file = fs.openSync(testFile, fs.OpenMode.CREATE | fs.OpenMode.READ_WRITE);
        fs.writeSync(file, 'test content for upload');
        fs.closeSync(file);

        let config: request.agent.Config = {
          action: request.agent.Action.UPLOAD,
          url: "http://127.0.0.1:18080/upload",
          mode: request.agent.Mode.BACKGROUND,
          data: [{
            name: 'file',
            value: { path: './test.txt' }
          }]
        };
        let task = await request.agent.create(context, config);
        expect(task.config.action).assertEqual(request.agent.Action.UPLOAD);
        expect(task.config.mode).assertEqual(request.agent.Mode.BACKGROUND);
        await task.delete();
        console.info("===> testComboActionUploadModeBackground001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testComboActionDownloadModeForeground001
     * @tc.name: testComboActionDownloadModeForeground001
     * @tc.desc: DOWNLOAD + FOREGROUND combination test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, mode: FOREGROUND.
     *           2. Call request.agent.create() with the config to create task.
     *           3. Assert task.config.action equals DOWNLOAD.
     *           4. Assert task.config.mode equals FOREGROUND.
     *           5. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboActionDownloadModeForeground001', 0, async (done: Function) => {
      console.info("===> testComboActionDownloadModeForeground001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          mode: request.agent.Mode.FOREGROUND,
        };
        let task = await request.agent.create(context, config);
        expect(task.config.action).assertEqual(request.agent.Action.DOWNLOAD);
        expect(task.config.mode).assertEqual(request.agent.Mode.FOREGROUND);
        await task.delete();
        console.info("===> testComboActionDownloadModeForeground001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testComboActionDownloadModeBackground001
     * @tc.name: testComboActionDownloadModeBackground001
     * @tc.desc: DOWNLOAD + BACKGROUND combination test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, mode: BACKGROUND.
     *           2. Call request.agent.create() with the config to create task.
     *           3. Assert task.config.action equals DOWNLOAD.
     *           4. Assert task.config.mode equals BACKGROUND.
     *           5. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboActionDownloadModeBackground001', 0, async (done: Function) => {
      console.info("===> testComboActionDownloadModeBackground001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          mode: request.agent.Mode.BACKGROUND,
        };
        let task = await request.agent.create(context, config);
        expect(task.config.action).assertEqual(request.agent.Action.DOWNLOAD);
        expect(task.config.mode).assertEqual(request.agent.Mode.BACKGROUND);
        await task.delete();
        console.info("===> testComboActionDownloadModeBackground001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    // ==================== Method + Action 组合 ====================

    /**
     * @tc.number: testComboUploadPost001
     * @tc.name: testComboUploadPost001
     * @tc.desc: UPLOAD + POST method combination test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a test file with content 'test content'.
     *           2. Create Config with action: UPLOAD, valid URL, method: POST, data with file path.
     *           3. Call request.agent.create() with the config to create task.
     *           4. Assert task.config.action equals UPLOAD.
     *           5. Assert task.config.method equals POST.
     *           6. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboUploadPost001', 0, async (done: Function) => {
      console.info("===> testComboUploadPost001 start");
      try {
        let testFile = cacheDir + '/test.txt';
        let file = fs.openSync(testFile, fs.OpenMode.CREATE | fs.OpenMode.READ_WRITE);
        fs.writeSync(file, 'test content');
        fs.closeSync(file);

        let config: request.agent.Config = {
          action: request.agent.Action.UPLOAD,
          url: "http://127.0.0.1:18080/upload",
          method: request.agent.Method.POST,
          data: [{
            name: 'file',
            value: { path: './test.txt' }
          }]
        };
        let task = await request.agent.create(context, config);
        expect(task.config.action).assertEqual(request.agent.Action.UPLOAD);
        expect(task.config.method).assertEqual(request.agent.Method.POST);
        await task.delete();
        console.info("===> testComboUploadPost001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testComboUploadPut001
     * @tc.name: testComboUploadPut001
     * @tc.desc: UPLOAD + PUT method combination test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a test file with content 'test content'.
     *           2. Create Config with action: UPLOAD, valid URL, method: PUT, data with file path.
     *           3. Call request.agent.create() with the config to create task.
     *           4. Assert task.config.action equals UPLOAD.
     *           5. Assert task.config.method equals PUT.
     *           6. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboUploadPut001', 0, async (done: Function) => {
      console.info("===> testComboUploadPut001 start");
      try {
        let testFile = cacheDir + '/test.txt';
        let file = fs.openSync(testFile, fs.OpenMode.CREATE | fs.OpenMode.READ_WRITE);
        fs.writeSync(file, 'test content');
        fs.closeSync(file);

        let config: request.agent.Config = {
          action: request.agent.Action.UPLOAD,
          url: "http://127.0.0.1:18080/upload",
          method: request.agent.Method.PUT,
          data: [{
            name: 'file',
            value: { path: './test.txt' }
          }]
        };
        let task = await request.agent.create(context, config);
        expect(task.config.action).assertEqual(request.agent.Action.UPLOAD);
        expect(task.config.method).assertEqual(request.agent.Method.PUT);
        await task.delete();
        console.info("===> testComboUploadPut001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testComboDownloadGet001
     * @tc.name: testComboDownloadGet001
     * @tc.desc: DOWNLOAD + GET method combination test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, method: GET.
     *           2. Call request.agent.create() with the config to create task.
     *           3. Assert task.config.action equals DOWNLOAD.
     *           4. Assert task.config.method equals GET.
     *           5. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboDownloadGet001', 0, async (done: Function) => {
      console.info("===> testComboDownloadGet001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          method: request.agent.Method.GET,
        };
        let task = await request.agent.create(context, config);
        expect(task.config.action).assertEqual(request.agent.Action.DOWNLOAD);
        expect(task.config.method).assertEqual(request.agent.Method.GET);
        await task.delete();
        console.info("===> testComboDownloadGet001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testComboDownloadPost001
     * @tc.name: testComboDownloadPost001
     * @tc.desc: DOWNLOAD + POST method combination test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, method: POST.
     *           2. Call request.agent.create() with the config to create task.
     *           3. Assert task.config.action equals DOWNLOAD.
     *           4. Assert task.config.method equals POST.
     *           5. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboDownloadPost001', 0, async (done: Function) => {
      console.info("===> testComboDownloadPost001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/download",
          method: request.agent.Method.POST,
        };
        let task = await request.agent.create(context, config);
        expect(task.config.action).assertEqual(request.agent.Action.DOWNLOAD);
        expect(task.config.method).assertEqual(request.agent.Method.POST);
        await task.delete();
        console.info("===> testComboDownloadPost001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    // ==================== Overwrite + Saveas 组合 ====================

    /**
     * @tc.number: testComboOverwriteTrueNewFile001
     * @tc.name: testComboOverwriteTrueNewFile001
     * @tc.desc: overwrite=true + new file combination test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, overwrite: true, saveas: './combo.txt'.
     *           2. Call request.agent.create() with the config to create task.
     *           3. Assert task.config.overwrite is true.
     *           4. Assert task.config.saveas equals './combo.txt'.
     *           5. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboOverwriteTrueNewFile001', 0, async (done: Function) => {
      console.info("===> testComboOverwriteTrueNewFile001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          overwrite: true,
          saveas: './combo.txt',
        };
        let task = await request.agent.create(context, config);
        expect(task.config.overwrite).assertTrue();
        expect(task.config.saveas).assertEqual('./combo.txt');
        await task.delete();
        console.info("===> testComboOverwriteTrueNewFile001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testComboOverwriteFalseNewFile001
     * @tc.name: testComboOverwriteFalseNewFile001
     * @tc.desc: overwrite=false + new file combination test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, overwrite: false, saveas: './combo.txt'.
     *           2. Call request.agent.create() with the config to create task.
     *           3. Assert task.config.overwrite is false.
     *           4. Assert task.config.saveas equals './combo.txt'.
     *           5. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboOverwriteFalseNewFile001', 0, async (done: Function) => {
      console.info("===> testComboOverwriteFalseNewFile001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          overwrite: false,
          saveas: './combo.txt',
        };
        let task = await request.agent.create(context, config);
        expect(task.config.overwrite).assertFalse();
        expect(task.config.saveas).assertEqual('./combo.txt');
        await task.delete();
        console.info("===> testComboOverwriteFalseNewFile001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testComboOverwriteTrueFileExists001
     * @tc.name: testComboOverwriteTrueFileExists001
     * @tc.desc: overwrite=true + existing file combination test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create an existing file at cacheDir + '/combo.txt' with content 'existing content'.
     *           2. Create Config with action: DOWNLOAD, valid URL, overwrite: true, saveas: './combo.txt'.
     *           3. Call request.agent.create() with the config to create task.
     *           4. Assert task.config.overwrite is true.
     *           5. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboOverwriteTrueFileExists001', 0, async (done: Function) => {
      console.info("===> testComboOverwriteTrueFileExists001 start");
      try {
        // 先创建文件
        let existingFile = cacheDir + '/combo.txt';
        let file = fs.openSync(existingFile, fs.OpenMode.CREATE | fs.OpenMode.READ_WRITE);
        fs.writeSync(file, 'existing content');
        fs.closeSync(file);

        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          overwrite: true,
          saveas: './combo.txt',
        };
        let task = await request.agent.create(context, config);
        expect(task.config.overwrite).assertTrue();
        await task.delete();
        console.info("===> testComboOverwriteTrueFileExists001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    // ==================== Network + Metered + Roaming 组合 ====================

    /**
     * @tc.number: testComboNetworkWifiNotMetered001
     * @tc.name: testComboNetworkWifiNotMetered001
     * @tc.desc: WIFI + metered=false combination test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, network: WIFI, metered: false.
     *           2. Call request.agent.create() with the config to create task.
     *           3. Assert task.config.network equals WIFI.
     *           4. Assert task.config.metered is false.
     *           5. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboNetworkWifiNotMetered001', 0, async (done: Function) => {
      console.info("===> testComboNetworkWifiNotMetered001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          network: request.agent.Network.WIFI,
          metered: false,
        };
        let task = await request.agent.create(context, config);
        expect(task.config.network).assertEqual(request.agent.Network.WIFI);
        expect(task.config.metered).assertFalse();
        await task.delete();
        console.info("===> testComboNetworkWifiNotMetered001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testComboNetworkCellularMetered001
     * @tc.name: testComboNetworkCellularMetered001
     * @tc.desc: CELLULAR + metered=true combination test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, network: CELLULAR, metered: true.
     *           2. Call request.agent.create() with the config to create task.
     *           3. Assert task.config.network equals CELLULAR.
     *           4. Assert task.config.metered is true.
     *           5. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboNetworkCellularMetered001', 0, async (done: Function) => {
      console.info("===> testComboNetworkCellularMetered001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          network: request.agent.Network.CELLULAR,
          metered: true,
        };
        let task = await request.agent.create(context, config);
        expect(task.config.network).assertEqual(request.agent.Network.CELLULAR);
        expect(task.config.metered).assertTrue();
        await task.delete();
        console.info("===> testComboNetworkCellularMetered001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testComboNetworkCellularRoamingFalse001
     * @tc.name: testComboNetworkCellularRoamingFalse001
     * @tc.desc: CELLULAR + roaming=false combination test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, network: CELLULAR, roaming: false.
     *           2. Call request.agent.create() with the config to create task.
     *           3. Assert task.config.network equals CELLULAR.
     *           4. Assert task.config.roaming is false.
     *           5. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboNetworkCellularRoamingFalse001', 0, async (done: Function) => {
      console.info("===> testComboNetworkCellularRoamingFalse001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          network: request.agent.Network.CELLULAR,
          roaming: false,
        };
        let task = await request.agent.create(context, config);
        expect(task.config.network).assertEqual(request.agent.Network.CELLULAR);
        expect(task.config.roaming).assertFalse();
        await task.delete();
        console.info("===> testComboNetworkCellularRoamingFalse001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testComboNetworkCellularRoamingTrue001
     * @tc.name: testComboNetworkCellularRoamingTrue001
     * @tc.desc: CELLULAR + roaming=true combination test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, network: CELLULAR, roaming: true.
     *           2. Call request.agent.create() with the config to create task.
     *           3. Assert task.config.network equals CELLULAR.
     *           4. Assert task.config.roaming is true.
     *           5. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboNetworkCellularRoamingTrue001', 0, async (done: Function) => {
      console.info("===> testComboNetworkCellularRoamingTrue001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          network: request.agent.Network.CELLULAR,
          roaming: true,
        };
        let task = await request.agent.create(context, config);
        expect(task.config.network).assertEqual(request.agent.Network.CELLULAR);
        expect(task.config.roaming).assertTrue();
        await task.delete();
        console.info("===> testComboNetworkCellularRoamingTrue001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    // ==================== Retry + Redirect 组合 ====================

    /**
     * @tc.number: testComboRetryTrueRedirectTrue001
     * @tc.name: testComboRetryTrueRedirectTrue001
     * @tc.desc: retry=true + redirect=true combination test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, retry: true, redirect: true.
     *           2. Call request.agent.create() with the config to create task.
     *           3. Assert task.config.retry is true.
     *           4. Assert task.config.redirect is true.
     *           5. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboRetryTrueRedirectTrue001', 0, async (done: Function) => {
      console.info("===> testComboRetryTrueRedirectTrue001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          retry: true,
          redirect: true,
        };
        let task = await request.agent.create(context, config);
        expect(task.config.retry).assertTrue();
        expect(task.config.redirect).assertTrue();
        await task.delete();
        console.info("===> testComboRetryTrueRedirectTrue001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testComboRetryFalseRedirectFalse001
     * @tc.name: testComboRetryFalseRedirectFalse001
     * @tc.desc: retry=false + redirect=false combination test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, retry: false, redirect: false.
     *           2. Call request.agent.create() with the config to create task.
     *           3. Assert task.config.retry is false.
     *           4. Assert task.config.redirect is false.
     *           5. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboRetryFalseRedirectFalse001', 0, async (done: Function) => {
      console.info("===> testComboRetryFalseRedirectFalse001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          retry: false,
          redirect: false,
        };
        let task = await request.agent.create(context, config);
        expect(task.config.retry).assertFalse();
        expect(task.config.redirect).assertFalse();
        await task.delete();
        console.info("===> testComboRetryFalseRedirectFalse001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    // ==================== Gauge + Mode 组合 ====================

    /**
     * @tc.number: testComboGaugeTrueModeBackground001
     * @tc.name: testComboGaugeTrueModeBackground001
     * @tc.desc: gauge=true + BACKGROUND mode combination test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, gauge: true, mode: BACKGROUND.
     *           2. Call request.agent.create() with the config to create task.
     *           3. Assert task.config.gauge is true.
     *           4. Assert task.config.mode equals BACKGROUND.
     *           5. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboGaugeTrueModeBackground001', 0, async (done: Function) => {
      console.info("===> testComboGaugeTrueModeBackground001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          gauge: true,
          mode: request.agent.Mode.BACKGROUND,
        };
        let task = await request.agent.create(context, config);
        expect(task.config.gauge).assertTrue();
        expect(task.config.mode).assertEqual(request.agent.Mode.BACKGROUND);
        await task.delete();
        console.info("===> testComboGaugeTrueModeBackground001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testComboGaugeFalseModeForeground001
     * @tc.name: testComboGaugeFalseModeForeground001
     * @tc.desc: gauge=false + FOREGROUND mode combination test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, gauge: false, mode: FOREGROUND.
     *           2. Call request.agent.create() with the config to create task.
     *           3. Assert task.config.gauge is false.
     *           4. Assert task.config.mode equals FOREGROUND.
     *           5. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboGaugeFalseModeForeground001', 0, async (done: Function) => {
      console.info("===> testComboGaugeFalseModeForeground001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          gauge: false,
          mode: request.agent.Mode.FOREGROUND,
        };
        let task = await request.agent.create(context, config);
        expect(task.config.gauge).assertFalse();
        expect(task.config.mode).assertEqual(request.agent.Mode.FOREGROUND);
        await task.delete();
        console.info("===> testComboGaugeFalseModeForeground001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    // ==================== Priority + Mode 组合 ====================

    /**
     * @tc.number: testComboPriorityHighModeBackground001
     * @tc.name: testComboPriorityHighModeBackground001
     * @tc.desc: High priority + BACKGROUND mode combination test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, priority: 100, mode: BACKGROUND.
     *           2. Call request.agent.create() with the config to create task.
     *           3. Assert task.config.priority equals 100.
     *           4. Assert task.config.mode equals BACKGROUND.
     *           5. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboPriorityHighModeBackground001', 0, async (done: Function) => {
      console.info("===> testComboPriorityHighModeBackground001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          priority: 100,
          mode: request.agent.Mode.BACKGROUND,
        };
        let task = await request.agent.create(context, config);
        expect(task.config.priority).assertEqual(100);
        expect(task.config.mode).assertEqual(request.agent.Mode.BACKGROUND);
        await task.delete();
        console.info("===> testComboPriorityHighModeBackground001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testComboPriorityLowModeForeground001
     * @tc.name: testComboPriorityLowModeForeground001
     * @tc.desc: Low priority + FOREGROUND mode combination test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, priority: -50, mode: FOREGROUND.
     *           2. Call request.agent.create() with the config to create task.
     *           3. Assert task.config.priority equals -50.
     *           4. Assert task.config.mode equals FOREGROUND.
     *           5. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboPriorityLowModeForeground001', 0, async (done: Function) => {
      console.info("===> testComboPriorityLowModeForeground001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          priority: -50,
          mode: request.agent.Mode.FOREGROUND,
        };
        let task = await request.agent.create(context, config);
        expect(task.config.priority).assertEqual(-50);
        expect(task.config.mode).assertEqual(request.agent.Mode.FOREGROUND);
        await task.delete();
        console.info("===> testComboPriorityLowModeForeground001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    // ==================== Version + Action 组合 ====================

    /**
     * @tc.number: testComboVersionApi9Download001
     * @tc.name: testComboVersionApi9Download001
     * @tc.desc: Version.API9 + DOWNLOAD combination test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, valid URL, version: API9.
     *           2. Call request.agent.create() with the config to create task.
     *           3. Assert task.config.version equals API9.
     *           4. Assert task.config.action equals DOWNLOAD.
     *           5. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboVersionApi9Download001', 0, async (done: Function) => {
      console.info("===> testComboVersionApi9Download001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          version: request.agent.Version.API9,
        };
        let task = await request.agent.create(context, config);
        expect(task.config.version).assertEqual(request.agent.Version.API9);
        expect(task.config.action).assertEqual(request.agent.Action.DOWNLOAD);
        await task.delete();
        console.info("===> testComboVersionApi9Download001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testComboVersionApi10Upload001
     * @tc.name: testComboVersionApi10Upload001
     * @tc.desc: Version.API10 + UPLOAD combination test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a test file with content 'test content'.
     *           2. Create Config with action: UPLOAD, valid URL, version: API10, data with file path.
     *           3. Call request.agent.create() with the config to create task.
     *           4. Assert task.config.version equals API10.
     *           5. Assert task.config.action equals UPLOAD.
     *           6. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboVersionApi10Upload001', 0, async (done: Function) => {
      console.info("===> testComboVersionApi10Upload001 start");
      try {
        let testFile = cacheDir + '/test.txt';
        let file = fs.openSync(testFile, fs.OpenMode.CREATE | fs.OpenMode.READ_WRITE);
        fs.writeSync(file, 'test content');
        fs.closeSync(file);

        let config: request.agent.Config = {
          action: request.agent.Action.UPLOAD,
          url: "http://127.0.0.1:18080/upload",
          version: request.agent.Version.API10,
          data: [{
            name: 'file',
            value: { path: './test.txt' }
          }]
        };
        let task = await request.agent.create(context, config);
        expect(task.config.version).assertEqual(request.agent.Version.API10);
        expect(task.config.action).assertEqual(request.agent.Action.UPLOAD);
        await task.delete();
        console.info("===> testComboVersionApi10Upload001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    // ==================== 复杂组合测试 ====================

    /**
     * @tc.number: testComboComplexDownload001
     * @tc.name: testComboComplexDownload001
     * @tc.desc: Complex download configuration combination test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with complex download settings: action: DOWNLOAD, valid URL,
     *              mode: BACKGROUND, network: ANY, priority: 50, retry: true, redirect: true,
     *              gauge: true, overwrite: true, metered: true, roaming: true, method: GET,
     *              headers with User-Agent and Accept, saveas: './combo_complex.txt',
     *              title: 'Complex Download Test', description: 'Testing multiple config combinations'.
     *           2. Call request.agent.create() with the config to create task.
     *           3. Assert task.config.action equals DOWNLOAD.
     *           4. Assert task.config.mode equals BACKGROUND.
     *           5. Assert task.config.network equals ANY.
     *           6. Assert task.config.priority equals 50.
     *           7. Assert task.config.retry is true.
     *           8. Assert task.config.redirect is true.
     *           9. Assert task.config.gauge is true.
     *           10. Assert task.config.overwrite is true.
     *           11. Assert task.config.metered is true.
     *           12. Assert task.config.roaming is true.
     *           13. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboComplexDownload001', 0, async (done: Function) => {
      console.info("===> testComboComplexDownload001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          mode: request.agent.Mode.BACKGROUND,
          network: request.agent.Network.ANY,
          priority: 50,
          retry: true,
          redirect: true,
          gauge: true,
          overwrite: true,
          metered: true,
          roaming: true,
          method: request.agent.Method.GET,
          headers: {
            'User-Agent': 'TestAgent/1.0',
            'Accept': '*/*'
          },
          saveas: './combo_complex.txt',
          title: 'Complex Download Test',
          description: 'Testing multiple config combinations',
        };
        let task = await request.agent.create(context, config);
        expect(task.config.action).assertEqual(request.agent.Action.DOWNLOAD);
        expect(task.config.mode).assertEqual(request.agent.Mode.BACKGROUND);
        expect(task.config.network).assertEqual(request.agent.Network.ANY);
        expect(task.config.priority).assertEqual(50);
        expect(task.config.retry).assertTrue();
        expect(task.config.redirect).assertTrue();
        expect(task.config.gauge).assertTrue();
        expect(task.config.overwrite).assertTrue();
        expect(task.config.metered).assertTrue();
        expect(task.config.roaming).assertTrue();
        await task.delete();
        console.info("===> testComboComplexDownload001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testComboComplexUpload001
     * @tc.name: testComboComplexUpload001
     * @tc.desc: Complex upload configuration combination test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a test file with content 'test content for complex upload'.
     *           2. Create Config with complex upload settings: action: UPLOAD, valid URL,
     *              mode: FOREGROUND, network: WIFI, priority: 75, retry: false, redirect: false,
     *              gauge: false, overwrite: false, metered: false, method: POST,
     *              headers with Content-Type and X-Custom-Header, data with file path and filename,
     *              title: 'Complex Upload Test', description: 'Testing upload config combinations'.
     *           3. Call request.agent.create() with the config to create task.
     *           4. Assert task.config.action equals UPLOAD.
     *           5. Assert task.config.mode equals FOREGROUND.
     *           6. Assert task.config.network equals WIFI.
     *           7. Assert task.config.priority equals 75.
     *           8. Assert task.config.retry is false.
     *           9. Assert task.config.redirect is false.
     *           10. Assert task.config.gauge is false.
     *           11. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboComplexUpload001', 0, async (done: Function) => {
      console.info("===> testComboComplexUpload001 start");
      try {
        let testFile = cacheDir + '/test.txt';
        let file = fs.openSync(testFile, fs.OpenMode.CREATE | fs.OpenMode.READ_WRITE);
        fs.writeSync(file, 'test content for complex upload');
        fs.closeSync(file);

        let config: request.agent.Config = {
          action: request.agent.Action.UPLOAD,
          url: "http://127.0.0.1:18080/upload",
          mode: request.agent.Mode.FOREGROUND,
          network: request.agent.Network.WIFI,
          priority: 75,
          retry: false,
          redirect: false,
          gauge: false,
          overwrite: false,
          metered: false,
          method: request.agent.Method.POST,
          headers: {
            'Content-Type': 'multipart/form-data',
            'X-Custom-Header': 'custom-value'
          },
          data: [{
            name: 'file',
            value: { path: './test.txt', filename: 'upload.txt' }
          }],
          title: 'Complex Upload Test',
          description: 'Testing upload config combinations',
        };
        let task = await request.agent.create(context, config);
        expect(task.config.action).assertEqual(request.agent.Action.UPLOAD);
        expect(task.config.mode).assertEqual(request.agent.Mode.FOREGROUND);
        expect(task.config.network).assertEqual(request.agent.Network.WIFI);
        expect(task.config.priority).assertEqual(75);
        expect(task.config.retry).assertFalse();
        expect(task.config.redirect).assertFalse();
        expect(task.config.gauge).assertFalse();
        await task.delete();
        console.info("===> testComboComplexUpload001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

  });
}
