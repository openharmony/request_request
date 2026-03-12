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
 * Header and Authentication combinations test
 * Tests Content-Type headers, Authorization headers, User-Agent, Range, Cookie, and custom headers
 */

export default function requestAgentComboHeaderAuthTest() {
  describe('requestAgentComboHeaderAuthTest', () => {
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
      const files = ['test.txt', 'header_test.txt', 'auth_test.txt', 'range_test.txt'];
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

    // ==================== Content-Type Headers ====================

    /**
     * @tc.number: testComboHeaderContentTypeJson001
     * @tc.name: testComboHeaderContentTypeJson001
     * @tc.desc: Content-Type application/json header for download test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, URL: http://127.0.0.1:18080/test.json, saveas: './header_test.txt', headers: {'Content-Type': 'application/json'}.
     *           2. Call request.agent.create() with the config to create a task.
     *           3. Assert task.config.headers equals {'Content-Type': 'application/json'}.
     *           4. Assert task.config.headers['Content-Type'] equals 'application/json'.
     *           5. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboHeaderContentTypeJson001', 0, async (done: Function) => {
      console.info("===> testComboHeaderContentTypeJson001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.json",
          saveas: './header_test.txt',
          headers: {
            'Content-Type': 'application/json'
          }
        };
        let task = await request.agent.create(context, config);
        expect(task.config.headers).assertDeepEquals({ 'Content-Type': 'application/json' });
        expect(task.config.headers['Content-Type']).assertEqual('application/json');
        await task.delete();
        console.info("===> testComboHeaderContentTypeJson001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testComboHeaderContentTypeForm002
     * @tc.name: testComboHeaderContentTypeForm002
     * @tc.desc: Content-Type application/x-www-form-urlencoded header test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create test file 'test.txt' in cache directory with content 'test content for form upload'.
     *           2. Create Config with action: UPLOAD, URL: http://127.0.0.1:18080/upload, headers: {'Content-Type': 'application/x-www-form-urlencoded'}, data: [{name: 'file', value: {path: './test.txt'}}].
     *           3. Call request.agent.create() with the config to create a task.
     *           4. Assert task.config.headers['Content-Type'] equals 'application/x-www-form-urlencoded'.
     *           5. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboHeaderContentTypeForm002', 0, async (done: Function) => {
      console.info("===> testComboHeaderContentTypeForm002 start");
      try {
        let testFile = cacheDir + '/test.txt';
        let file = fs.openSync(testFile, fs.OpenMode.CREATE | fs.OpenMode.READ_WRITE);
        fs.writeSync(file, 'test content for form upload');
        fs.closeSync(file);

        let config: request.agent.Config = {
          action: request.agent.Action.UPLOAD,
          url: "http://127.0.0.1:18080/upload",
          headers: {
            'Content-Type': 'application/x-www-form-urlencoded'
          },
          data: [{
            name: 'file',
            value: { path: './test.txt' }
          }]
        };
        let task = await request.agent.create(context, config);
        expect(task.config.headers['Content-Type']).assertEqual('application/x-www-form-urlencoded');
        await task.delete();
        console.info("===> testComboHeaderContentTypeForm002 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testComboHeaderContentTypeMultipart003
     * @tc.name: testComboHeaderContentTypeMultipart003
     * @tc.desc: Content-Type multipart/form-data header for upload test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create test file 'test.txt' in cache directory with content 'test content for multipart upload'.
     *           2. Create Config with action: UPLOAD, URL: http://127.0.0.1:18080/upload, headers: {'Content-Type': 'multipart/form-data'}, data: [{name: 'file', value: {path: './test.txt'}}].
     *           3. Call request.agent.create() with the config to create a task.
     *           4. Assert task.config.headers['Content-Type'] equals 'multipart/form-data'.
     *           5. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboHeaderContentTypeMultipart003', 0, async (done: Function) => {
      console.info("===> testComboHeaderContentTypeMultipart003 start");
      try {
        let testFile = cacheDir + '/test.txt';
        let file = fs.openSync(testFile, fs.OpenMode.CREATE | fs.OpenMode.READ_WRITE);
        fs.writeSync(file, 'test content for multipart upload');
        fs.closeSync(file);

        let config: request.agent.Config = {
          action: request.agent.Action.UPLOAD,
          url: "http://127.0.0.1:18080/upload",
          headers: {
            'Content-Type': 'multipart/form-data'
          },
          data: [{
            name: 'file',
            value: { path: './test.txt' }
          }]
        };
        let task = await request.agent.create(context, config);
        expect(task.config.headers['Content-Type']).assertEqual('multipart/form-data');
        await task.delete();
        console.info("===> testComboHeaderContentTypeMultipart003 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testComboHeaderContentTypePlain004
     * @tc.name: testComboHeaderContentTypePlain004
     * @tc.desc: Content-Type text/plain header test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, URL: http://127.0.0.1:18080/test.txt, headers: {'Content-Type': 'text/plain'}.
     *           2. Call request.agent.create() with the config to create a task.
     *           3. Assert task.config.headers['Content-Type'] equals 'text/plain'.
     *           4. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboHeaderContentTypePlain004', 0, async (done: Function) => {
      console.info("===> testComboHeaderContentTypePlain004 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          headers: {
            'Content-Type': 'text/plain'
          }
        };
        let task = await request.agent.create(context, config);
        expect(task.config.headers['Content-Type']).assertEqual('text/plain');
        await task.delete();
        console.info("===> testComboHeaderContentTypePlain004 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    // ==================== Authorization Headers ====================

    /**
     * @tc.number: testComboHeaderAuthBearer001
     * @tc.name: testComboHeaderAuthBearer001
     * @tc.desc: Authorization Bearer token header test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, URL: http://127.0.0.1:18080/secure/data, saveas: './auth_test.txt', headers: {'Authorization': 'Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.test_token'}.
     *           2. Call request.agent.create() with the config to create a task.
     *           3. Assert task.config.headers['Authorization'] equals the Bearer token.
     *           4. Assert task.config.headers['Authorization'] starts with 'Bearer '.
     *           5. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboHeaderAuthBearer001', 0, async (done: Function) => {
      console.info("===> testComboHeaderAuthBearer001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/secure/data",
          saveas: './auth_test.txt',
          headers: {
            'Authorization': 'Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.test_token'
          }
        };
        let task = await request.agent.create(context, config);
        expect(task.config.headers['Authorization']).assertEqual('Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.test_token');
        expect(task.config.headers['Authorization'].startsWith('Bearer ')).assertTrue();
        await task.delete();
        console.info("===> testComboHeaderAuthBearer001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testComboHeaderAuthBasic002
     * @tc.name: testComboHeaderAuthBasic002
     * @tc.desc: Authorization Basic auth header test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, URL: http://127.0.0.1:18080/secure/resource, headers: {'Authorization': 'Basic dXNlcjpwYXNzd29yZA=='}.
     *           2. Call request.agent.create() with the config to create a task.
     *           3. Assert task.config.headers['Authorization'] equals the Basic auth token.
     *           4. Assert task.config.headers['Authorization'] starts with 'Basic '.
     *           5. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboHeaderAuthBasic002', 0, async (done: Function) => {
      console.info("===> testComboHeaderAuthBasic002 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/secure/resource",
          headers: {
            'Authorization': 'Basic dXNlcjpwYXNzd29yZA=='
          }
        };
        let task = await request.agent.create(context, config);
        expect(task.config.headers['Authorization']).assertEqual('Basic dXNlcjpwYXNzd29yZA==');
        expect(task.config.headers['Authorization'].startsWith('Basic ')).assertTrue();
        await task.delete();
        console.info("===> testComboHeaderAuthBasic002 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testComboHeaderAuthApiKey003
     * @tc.name: testComboHeaderAuthApiKey003
     * @tc.desc: X-API-Key custom authentication header test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: UPLOAD, URL: http://127.0.0.1:18080/api/upload, headers: {'X-API-Key': 'api_key_123456789'}, data: [{name: 'file', value: {path: './test.txt'}}].
     *           2. Call request.agent.create() with the config to create a task.
     *           3. Assert task.config.headers['X-API-Key'] equals 'api_key_123456789'.
     *           4. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboHeaderAuthApiKey003', 0, async (done: Function) => {
      console.info("===> testComboHeaderAuthApiKey003 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.UPLOAD,
          url: "http://127.0.0.1:18080/api/upload",
          headers: {
            'X-API-Key': 'api_key_123456789'
          },
          data: [{
            name: 'file',
            value: { path: './test.txt' }
          }]
        };
        let task = await request.agent.create(context, config);
        expect(task.config.headers['X-API-Key']).assertEqual('api_key_123456789');
        await task.delete();
        console.info("===> testComboHeaderAuthApiKey003 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    // ==================== User-Agent Header ====================

    /**
     * @tc.number: testComboHeaderUserAgent001
     * @tc.name: testComboHeaderUserAgent001
     * @tc.desc: Custom User-Agent header test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, URL: http://127.0.0.1:18080/test.txt, headers: {'User-Agent': 'OpenHarmony-Request-Agent/1.0'}.
     *           2. Call request.agent.create() with the config to create a task.
     *           3. Assert task.config.headers['User-Agent'] equals 'OpenHarmony-Request-Agent/1.0'.
     *           4. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboHeaderUserAgent001', 0, async (done: Function) => {
      console.info("===> testComboHeaderUserAgent001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          headers: {
            'User-Agent': 'OpenHarmony-Request-Agent/1.0'
          }
        };
        let task = await request.agent.create(context, config);
        expect(task.config.headers['User-Agent']).assertEqual('OpenHarmony-Request-Agent/1.0');
        await task.delete();
        console.info("===> testComboHeaderUserAgent001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testComboHeaderUserAgent002
     * @tc.name: testComboHeaderUserAgent002
     * @tc.desc: User-Agent with device info header test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, URL: http://127.0.0.1:18080/test.txt, headers: {'User-Agent': 'Mozilla/5.0 (OpenHarmony; HarmonyOS) AppleWebKit/537.36'}.
     *           2. Call request.agent.create() with the config to create a task.
     *           3. Assert task.config.headers['User-Agent'] includes 'OpenHarmony'.
     *           4. Assert task.config.headers['User-Agent'] includes 'HarmonyOS'.
     *           5. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboHeaderUserAgent002', 0, async (done: Function) => {
      console.info("===> testComboHeaderUserAgent002 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          headers: {
            'User-Agent': 'Mozilla/5.0 (OpenHarmony; HarmonyOS) AppleWebKit/537.36'
          }
        };
        let task = await request.agent.create(context, config);
        expect(task.config.headers['User-Agent'].includes('OpenHarmony')).assertTrue();
        expect(task.config.headers['User-Agent'].includes('HarmonyOS')).assertTrue();
        await task.delete();
        console.info("===> testComboHeaderUserAgent002 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    // ==================== Range Request Header ====================

    /**
     * @tc.number: testComboHeaderRange001
     * @tc.name: testComboHeaderRange001
     * @tc.desc: Range header for partial content download test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, URL: http://127.0.0.1:18080/large-file.zip, saveas: './range_test.txt', headers: {'Range': 'bytes=0-1023'}.
     *           2. Call request.agent.create() with the config to create a task.
     *           3. Assert task.config.headers['Range'] equals 'bytes=0-1023'.
     *           4. Assert task.config.headers['Range'] starts with 'bytes='.
     *           5. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboHeaderRange001', 0, async (done: Function) => {
      console.info("===> testComboHeaderRange001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/large-file.zip",
          saveas: './range_test.txt',
          headers: {
            'Range': 'bytes=0-1023'
          }
        };
        let task = await request.agent.create(context, config);
        expect(task.config.headers['Range']).assertEqual('bytes=0-1023');
        expect(task.config.headers['Range'].startsWith('bytes=')).assertTrue();
        await task.delete();
        console.info("===> testComboHeaderRange001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testComboHeaderRange002
     * @tc.name: testComboHeaderRange002
     * @tc.desc: Range header with open-ended range test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, URL: http://127.0.0.1:18080/large-file.zip, headers: {'Range': 'bytes=1024-'}.
     *           2. Call request.agent.create() with the config to create a task.
     *           3. Assert task.config.headers['Range'] equals 'bytes=1024-'.
     *           4. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboHeaderRange002', 0, async (done: Function) => {
      console.info("===> testComboHeaderRange002 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/large-file.zip",
          headers: {
            'Range': 'bytes=1024-'
          }
        };
        let task = await request.agent.create(context, config);
        expect(task.config.headers['Range']).assertEqual('bytes=1024-');
        await task.delete();
        console.info("===> testComboHeaderRange002 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    // ==================== Cookie Header ====================

    /**
     * @tc.number: testComboHeaderCookie001
     * @tc.name: testComboHeaderCookie001
     * @tc.desc: Cookie header with single cookie test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, URL: http://127.0.0.1:18080/protected/resource, headers: {'Cookie': 'session_id=abc123xyz789'}.
     *           2. Call request.agent.create() with the config to create a task.
     *           3. Assert task.config.headers['Cookie'] equals 'session_id=abc123xyz789'.
     *           4. Assert task.config.headers['Cookie'] includes 'session_id'.
     *           5. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboHeaderCookie001', 0, async (done: Function) => {
      console.info("===> testComboHeaderCookie001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/protected/resource",
          headers: {
            'Cookie': 'session_id=abc123xyz789'
          }
        };
        let task = await request.agent.create(context, config);
        expect(task.config.headers['Cookie']).assertEqual('session_id=abc123xyz789');
        expect(task.config.headers['Cookie'].includes('session_id')).assertTrue();
        await task.delete();
        console.info("===> testComboHeaderCookie001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testComboHeaderCookie002
     * @tc.name: testComboHeaderCookie002
     * @tc.desc: Cookie header with multiple cookies test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, URL: http://127.0.0.1:18080/protected/resource, headers: {'Cookie': 'user_pref=dark_mode; auth_token=xyz789; lang=zh_CN'}.
     *           2. Call request.agent.create() with the config to create a task.
     *           3. Assert task.config.headers['Cookie'] equals the cookie string.
     *           4. Assert task.config.headers['Cookie'] includes 'auth_token'.
     *           5. Assert task.config.headers['Cookie'] includes 'lang=zh_CN'.
     *           6. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboHeaderCookie002', 0, async (done: Function) => {
      console.info("===> testComboHeaderCookie002 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/protected/resource",
          headers: {
            'Cookie': 'user_pref=dark_mode; auth_token=xyz789; lang=zh_CN'
          }
        };
        let task = await request.agent.create(context, config);
        expect(task.config.headers['Cookie']).assertEqual('user_pref=dark_mode; auth_token=xyz789; lang=zh_CN');
        expect(task.config.headers['Cookie'].includes('auth_token')).assertTrue();
        expect(task.config.headers['Cookie'].includes('lang=zh_CN')).assertTrue();
        await task.delete();
        console.info("===> testComboHeaderCookie002 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    // ==================== Custom Headers ====================

    /**
     * @tc.number: testComboHeaderCustom001
     * @tc.name: testComboHeaderCustom001
     * @tc.desc: Multiple custom headers combination test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, URL: http://127.0.0.1:18080/api/data, headers: {'X-Request-ID': 'req-12345-67890', 'X-Device-ID': 'device-abc-xyz', 'X-App-Version': '1.0.0'}.
     *           2. Call request.agent.create() with the config to create a task.
     *           3. Assert task.config.headers['X-Request-ID'] equals 'req-12345-67890'.
     *           4. Assert task.config.headers['X-Device-ID'] equals 'device-abc-xyz'.
     *           5. Assert task.config.headers['X-App-Version'] equals '1.0.0'.
     *           6. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboHeaderCustom001', 0, async (done: Function) => {
      console.info("===> testComboHeaderCustom001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/api/data",
          headers: {
            'X-Request-ID': 'req-12345-67890',
            'X-Device-ID': 'device-abc-xyz',
            'X-App-Version': '1.0.0'
          }
        };
        let task = await request.agent.create(context, config);
        expect(task.config.headers['X-Request-ID']).assertEqual('req-12345-67890');
        expect(task.config.headers['X-Device-ID']).assertEqual('device-abc-xyz');
        expect(task.config.headers['X-App-Version']).assertEqual('1.0.0');
        await task.delete();
        console.info("===> testComboHeaderCustom001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testComboHeaderCustom002
     * @tc.name: testComboHeaderCustom002
     * @tc.desc: Accept header with quality values test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, URL: http://127.0.0.1:18080/content, headers: {'Accept': 'application/json;q=0.9, text/html;q=0.8, */*;q=0.7'}.
     *           2. Call request.agent.create() with the config to create a task.
     *           3. Assert task.config.headers['Accept'] includes 'application/json'.
     *           4. Assert task.config.headers['Accept'] includes 'q=0.9'.
     *           5. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboHeaderCustom002', 0, async (done: Function) => {
      console.info("===> testComboHeaderCustom002 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/content",
          headers: {
            'Accept': 'application/json;q=0.9, text/html;q=0.8, */*;q=0.7'
          }
        };
        let task = await request.agent.create(context, config);
        expect(task.config.headers['Accept'].includes('application/json')).assertTrue();
        expect(task.config.headers['Accept'].includes('q=0.9')).assertTrue();
        await task.delete();
        console.info("===> testComboHeaderCustom002 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    // ==================== Header + Action Combinations ====================

    /**
     * @tc.number: testComboHeaderWithUploadAction001
     * @tc.name: testComboHeaderWithUploadAction001
     * @tc.desc: Complex headers with UPLOAD action test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create test file 'test.txt' in cache directory with content 'test content'.
     *           2. Create Config with action: UPLOAD, URL: http://127.0.0.1:18080/upload, method: POST, headers: {'Content-Type': 'multipart/form-data', 'Authorization': 'Bearer token123', 'X-Upload-Source': 'request_agent_test'}, data: [{name: 'file', value: {path: './test.txt'}}].
     *           3. Call request.agent.create() with the config to create a task.
     *           4. Assert task.config.action equals request.agent.Action.UPLOAD.
     *           5. Assert task.config.headers['Authorization'] equals 'Bearer token123'.
     *           6. Assert task.config.headers['X-Upload-Source'] equals 'request_agent_test'.
     *           7. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboHeaderWithUploadAction001', 0, async (done: Function) => {
      console.info("===> testComboHeaderWithUploadAction001 start");
      try {
        let testFile = cacheDir + '/test.txt';
        let file = fs.openSync(testFile, fs.OpenMode.CREATE | fs.OpenMode.READ_WRITE);
        fs.writeSync(file, 'test content');
        fs.closeSync(file);

        let config: request.agent.Config = {
          action: request.agent.Action.UPLOAD,
          url: "http://127.0.0.1:18080/upload",
          method: request.agent.Method.POST,
          headers: {
            'Content-Type': 'multipart/form-data',
            'Authorization': 'Bearer token123',
            'X-Upload-Source': 'request_agent_test'
          },
          data: [{
            name: 'file',
            value: { path: './test.txt' }
          }]
        };
        let task = await request.agent.create(context, config);
        expect(task.config.action).assertEqual(request.agent.Action.UPLOAD);
        expect(task.config.headers['Authorization']).assertEqual('Bearer token123');
        expect(task.config.headers['X-Upload-Source']).assertEqual('request_agent_test');
        await task.delete();
        console.info("===> testComboHeaderWithUploadAction001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testComboHeaderWithDownloadAction002
     * @tc.name: testComboHeaderWithDownloadAction002
     * @tc.desc: Complex headers with DOWNLOAD action test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, URL: http://127.0.0.1:18080/secure/download, saveas: './header_test.txt', headers: {'Authorization': 'Basic dXNlcjpwYXNz', 'Accept': 'application/octet-stream', 'If-Modified-Since': 'Wed, 21 Oct 2025 07:28:00 GMT'}.
     *           2. Call request.agent.create() with the config to create a task.
     *           3. Assert task.config.action equals request.agent.Action.DOWNLOAD.
     *           4. Assert task.config.headers['Authorization'] equals 'Basic dXNlcjpwYXNz'.
     *           5. Assert task.config.headers['Accept'] equals 'application/octet-stream'.
     *           6. Assert task.config.headers['If-Modified-Since'] equals 'Wed, 21 Oct 2025 07:28:00 GMT'.
     *           7. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboHeaderWithDownloadAction002', 0, async (done: Function) => {
      console.info("===> testComboHeaderWithDownloadAction002 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/secure/download",
          saveas: './header_test.txt',
          headers: {
            'Authorization': 'Basic dXNlcjpwYXNz',
            'Accept': 'application/octet-stream',
            'If-Modified-Since': 'Wed, 21 Oct 2025 07:28:00 GMT'
          }
        };
        let task = await request.agent.create(context, config);
        expect(task.config.action).assertEqual(request.agent.Action.DOWNLOAD);
        expect(task.config.headers['Authorization']).assertEqual('Basic dXNlcjpwYXNz');
        expect(task.config.headers['Accept']).assertEqual('application/octet-stream');
        expect(task.config.headers['If-Modified-Since']).assertEqual('Wed, 21 Oct 2025 07:28:00 GMT');
        await task.delete();
        console.info("===> testComboHeaderWithDownloadAction002 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    // ==================== Header Case Sensitivity ====================

    /**
     * @tc.number: testComboHeaderCaseSensitivity001
     * @tc.name: testComboHeaderCaseSensitivity001
     * @tc.desc: Header keys with different case sensitivity test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, URL: http://127.0.0.1:18080/test.txt, headers: {'content-type': 'application/json', 'CONTENT-TYPE': 'text/html'}.
     *           2. Call request.agent.create() with the config to create a task.
     *           3. Assert task.config.headers['content-type'] equals 'application/json'.
     *           4. Assert task.config.headers['CONTENT-TYPE'] equals 'text/html'.
     *           5. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboHeaderCaseSensitivity001', 0, async (done: Function) => {
      console.info("===> testComboHeaderCaseSensitivity001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          headers: {
            'content-type': 'application/json',
            'CONTENT-TYPE': 'text/html'
          }
        };
        let task = await request.agent.create(context, config);
        // Both headers should be preserved
        expect(task.config.headers['content-type']).assertEqual('application/json');
        expect(task.config.headers['CONTENT-TYPE']).assertEqual('text/html');
        await task.delete();
        console.info("===> testComboHeaderCaseSensitivity001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    // ==================== Header with Special Characters ====================

    /**
     * @tc.number: testComboHeaderSpecialChars001
     * @tc.name: testComboHeaderSpecialChars001
     * @tc.desc: Header values with special characters test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, URL: http://127.0.0.1:18080/test.txt, headers: {'X-Custom-Header': 'value_with-special.chars+123', 'X-Encoded-Value': 'Hello%20World%21'}.
     *           2. Call request.agent.create() with the config to create a task.
     *           3. Assert task.config.headers['X-Custom-Header'] equals 'value_with-special.chars+123'.
     *           4. Assert task.config.headers['X-Encoded-Value'] equals 'Hello%20World%21'.
     *           5. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboHeaderSpecialChars001', 0, async (done: Function) => {
      console.info("===> testComboHeaderSpecialChars001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          headers: {
            'X-Custom-Header': 'value_with-special.chars+123',
            'X-Encoded-Value': 'Hello%20World%21'
          }
        };
        let task = await request.agent.create(context, config);
        expect(task.config.headers['X-Custom-Header']).assertEqual('value_with-special.chars+123');
        expect(task.config.headers['X-Encoded-Value']).assertEqual('Hello%20World%21');
        await task.delete();
        console.info("===> testComboHeaderSpecialChars001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    // ==================== Empty and Whitespace Headers ====================

    /**
     * @tc.number: testComboHeaderEmpty001
     * @tc.name: testComboHeaderEmpty001
     * @tc.desc: Header with empty value test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create Config with action: DOWNLOAD, URL: http://127.0.0.1:18080/test.txt, headers: {'X-Empty-Header': ''}.
     *           2. Call request.agent.create() with the config to create a task.
     *           3. Assert task.config.headers['X-Empty-Header'] equals ''.
     *           4. Delete the task and finish test.
     * @tc.require: issues#1573
     */
    it('testComboHeaderEmpty001', 0, async (done: Function) => {
      console.info("===> testComboHeaderEmpty001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          headers: {
            'X-Empty-Header': ''
          }
        };
        let task = await request.agent.create(context, config);
        expect(task.config.headers['X-Empty-Header']).assertEqual('');
        await task.delete();
        console.info("===> testComboHeaderEmpty001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

  });
}
