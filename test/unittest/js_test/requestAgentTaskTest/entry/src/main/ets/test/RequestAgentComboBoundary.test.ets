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
 * 边界值测试
 * 测试文件大小边界、字符串长度边界、特殊字符、数值边界、空值处理
 */

export default function requestAgentComboBoundaryTest() {
  describe('requestAgentComboBoundaryTest', () => {
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
      const files = ['test.txt', 'empty.txt', 'boundary.txt', 'unicode.txt', 'emoji.txt'];
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

    // ==================== 文件大小边界测试 ====================

    /**
     * @tc.number: testBoundaryFileSizeZero001
     * @tc.name: testBoundaryFileSizeZero001
     * @tc.desc: Download with 0 byte file size boundary test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a 0-byte empty file in the cache directory
     *           2. Configure an upload task with the empty file
     *           3. Call request.agent.create() to create the task
     *           4. Verify the task action is UPLOAD
     *           5. Delete the task after verification
     * @tc.require: issues#1573
     */
    it('testBoundaryFileSizeZero001', 0, async (done: Function) => {
      console.info("===> testBoundaryFileSizeZero001 start");
      try {
        // Create 0 byte file
        let emptyFile = cacheDir + '/empty.txt';
        let file = fs.openSync(emptyFile, fs.OpenMode.CREATE | fs.OpenMode.READ_WRITE);
        fs.closeSync(file);

        let config: request.agent.Config = {
          action: request.agent.Action.UPLOAD,
          url: "http://127.0.0.1:18080/upload",
          data: [{
            name: 'file',
            value: { path: './empty.txt' }
          }]
        };
        let task = await request.agent.create(context, config);
        expect(task.config.action).assertEqual(request.agent.Action.UPLOAD);
        await task.delete();
        console.info("===> testBoundaryFileSizeZero001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testBoundaryFileSizeOneByte001
     * @tc.name: testBoundaryFileSizeOneByte001
     * @tc.desc: Download with 1 byte file size boundary test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a 1-byte file with content 'X' in the cache directory
     *           2. Configure an upload task with the 1-byte file
     *           3. Call request.agent.create() to create the task
     *           4. Verify the task action is UPLOAD
     *           5. Delete the task after verification
     * @tc.require: issues#1573
     */
    it('testBoundaryFileSizeOneByte001', 0, async (done: Function) => {
      console.info("===> testBoundaryFileSizeOneByte001 start");
      try {
        // Create 1 byte file
        let oneByteFile = cacheDir + '/onebyte.txt';
        let file = fs.openSync(oneByteFile, fs.OpenMode.CREATE | fs.OpenMode.READ_WRITE);
        fs.writeSync(file, 'X');
        fs.closeSync(file);

        let config: request.agent.Config = {
          action: request.agent.Action.UPLOAD,
          url: "http://127.0.0.1:18080/upload",
          data: [{
            name: 'file',
            value: { path: './onebyte.txt' }
          }]
        };
        let task = await request.agent.create(context, config);
        expect(task.config.action).assertEqual(request.agent.Action.UPLOAD);
        await task.delete();
        console.info("===> testBoundaryFileSizeOneByte001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testBoundaryBeginsZero001
     * @tc.name: testBoundaryBeginsZero001
     * @tc.desc: Range download with begins=0 boundary test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Configure a download task with begins parameter set to 0
     *           2. Set the saveas path to './boundary.txt'
     *           3. Call request.agent.create() to create the task
     *           4. Verify the task config begins value equals 0
     *           5. Delete the task after verification
     * @tc.require: issues#1573
     */
    it('testBoundaryBeginsZero001', 0, async (done: Function) => {
      console.info("===> testBoundaryBeginsZero001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          begins: 0,
          saveas: './boundary.txt',
        };
        let task = await request.agent.create(context, config);
        expect(task.config.begins).assertEqual(0);
        await task.delete();
        console.info("===> testBoundaryBeginsZero001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testBoundaryBeginsEndsEqual001
     * @tc.name: testBoundaryBeginsEndsEqual001
     * @tc.desc: Range download with begins=ends boundary test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Configure a download task with begins and ends both set to 100
     *           2. Set the saveas path to './boundary.txt'
     *           3. Call request.agent.create() to create the task
     *           4. Verify the task config begins value equals 100
     *           5. Verify the task config ends value equals 100
     *           6. Delete the task after verification
     * @tc.require: issues#1573
     */
    it('testBoundaryBeginsEndsEqual001', 0, async (done: Function) => {
      console.info("===> testBoundaryBeginsEndsEqual001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          begins: 100,
          ends: 100,
          saveas: './boundary.txt',
        };
        let task = await request.agent.create(context, config);
        expect(task.config.begins).assertEqual(100);
        expect(task.config.ends).assertEqual(100);
        await task.delete();
        console.info("===> testBoundaryBeginsEndsEqual001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    // ==================== 字符串长度边界测试 ====================

    /**
     * @tc.number: testBoundaryUrlMaxLength001
     * @tc.name: testBoundaryUrlMaxLength001
     * @tc.desc: URL with 2048 characters boundary test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a URL with 2048 characters (base URL + 2000 character path)
     *           2. Configure a download task with the long URL
     *           3. Set the saveas path to './boundary.txt'
     *           4. Call request.agent.create() to create the task
     *           5. Verify the task config URL length equals the expected length
     *           6. Delete the task after verification
     * @tc.require: issues#1573
     */
    it('testBoundaryUrlMaxLength001', 0, async (done: Function) => {
      console.info("===> testBoundaryUrlMaxLength001 start");
      try {
        // Create URL with 2048 characters
        let longPath = 'a'.repeat(2000);
        let url = "http://127.0.0.1:18080/" + longPath;

        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: url,
          saveas: './boundary.txt',
        };
        let task = await request.agent.create(context, config);
        expect(task.config.url.length).assertEqual(url.length);
        await task.delete();
        console.info("===> testBoundaryUrlMaxLength001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testBoundaryFilenameMaxLength001
     * @tc.name: testBoundaryFilenameMaxLength001
     * @tc.desc: Filename with 255 characters boundary test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a test file in the cache directory
     *           2. Create a filename with 255 characters (251 'a' characters + '.txt')
     *           3. Configure an upload task with the long filename
     *           4. Call request.agent.create() to create the task
     *           5. Verify the task config data filename equals the long filename
     *           6. Delete the task after verification
     * @tc.require: issues#1573
     */
    it('testBoundaryFilenameMaxLength001', 0, async (done: Function) => {
      console.info("===> testBoundaryFilenameMaxLength001 start");
      try {
        let testFile = cacheDir + '/test.txt';
        let file = fs.openSync(testFile, fs.OpenMode.CREATE | fs.OpenMode.READ_WRITE);
        fs.writeSync(file, 'test content');
        fs.closeSync(file);

        // Create filename with 255 characters
        let longFilename = 'a'.repeat(251) + '.txt';

        let config: request.agent.Config = {
          action: request.agent.Action.UPLOAD,
          url: "http://127.0.0.1:18080/upload",
          data: [{
            name: 'file',
            value: { path: './test.txt', filename: longFilename }
          }]
        };
        let task = await request.agent.create(context, config);
        expect(task.config.data![0].value.filename).assertEqual(longFilename);
        await task.delete();
        console.info("===> testBoundaryFilenameMaxLength001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testBoundaryTitleMaxLength001
     * @tc.name: testBoundaryTitleMaxLength001
     * @tc.desc: Title with maximum characters boundary test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a title string with 256 characters
     *           2. Configure a download task with the long title
     *           3. Set the saveas path to './boundary.txt'
     *           4. Call request.agent.create() to create the task
     *           5. Verify the task config title equals the long title
     *           6. Delete the task after verification
     * @tc.require: issues#1573
     */
    it('testBoundaryTitleMaxLength001', 0, async (done: Function) => {
      console.info("===> testBoundaryTitleMaxLength001 start");
      try {
        let longTitle = 'T'.repeat(256);

        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          title: longTitle,
          saveas: './boundary.txt',
        };
        let task = await request.agent.create(context, config);
        expect(task.config.title).assertEqual(longTitle);
        await task.delete();
        console.info("===> testBoundaryTitleMaxLength001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testBoundaryDescriptionMaxLength001
     * @tc.name: testBoundaryDescriptionMaxLength001
     * @tc.desc: Description with maximum characters boundary test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a description string with 512 characters
     *           2. Configure a download task with the long description
     *           3. Set the saveas path to './boundary.txt'
     *           4. Call request.agent.create() to create the task
     *           5. Verify the task config description equals the long description
     *           6. Delete the task after verification
     * @tc.require: issues#1573
     */
    it('testBoundaryDescriptionMaxLength001', 0, async (done: Function) => {
      console.info("===> testBoundaryDescriptionMaxLength001 start");
      try {
        let longDescription = 'D'.repeat(512);

        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          description: longDescription,
          saveas: './boundary.txt',
        };
        let task = await request.agent.create(context, config);
        expect(task.config.description).assertEqual(longDescription);
        await task.delete();
        console.info("===> testBoundaryDescriptionMaxLength001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    // ==================== 特殊字符测试 ====================

    /**
     * @tc.number: testBoundaryChineseCharacters001
     * @tc.name: testBoundaryChineseCharacters001
     * @tc.desc: Filename with Chinese characters boundary test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a test file in the cache directory
     *           2. Create a filename containing Chinese characters '中文文件名测试.txt'
     *           3. Configure an upload task with the Chinese filename
     *           4. Call request.agent.create() to create the task
     *           5. Verify the task config data filename equals the Chinese filename
     *           6. Delete the task after verification
     * @tc.require: issues#1573
     */
    it('testBoundaryChineseCharacters001', 0, async (done: Function) => {
      console.info("===> testBoundaryChineseCharacters001 start");
      try {
        let testFile = cacheDir + '/test.txt';
        let file = fs.openSync(testFile, fs.OpenMode.CREATE | fs.OpenMode.READ_WRITE);
        fs.writeSync(file, 'test content');
        fs.closeSync(file);

        let chineseFilename = '中文文件名测试.txt';

        let config: request.agent.Config = {
          action: request.agent.Action.UPLOAD,
          url: "http://127.0.0.1:18080/upload",
          data: [{
            name: 'file',
            value: { path: './test.txt', filename: chineseFilename }
          }]
        };
        let task = await request.agent.create(context, config);
        expect(task.config.data![0].value.filename).assertEqual(chineseFilename);
        await task.delete();
        console.info("===> testBoundaryChineseCharacters001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testBoundaryEmojiCharacters001
     * @tc.name: testBoundaryEmojiCharacters001
     * @tc.desc: Title with Emoji characters boundary test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a title string containing emoji characters '🎉🚀💯📱测试标题🎯'
     *           2. Configure a download task with the emoji title
     *           3. Set the saveas path to './emoji.txt'
     *           4. Call request.agent.create() to create the task
     *           5. Verify the task config title equals the emoji title
     *           6. Delete the task after verification
     * @tc.require: issues#1573
     */
    it('testBoundaryEmojiCharacters001', 0, async (done: Function) => {
      console.info("===> testBoundaryEmojiCharacters001 start");
      try {
        let emojiTitle = '🎉🚀💯📱测试标题🎯';

        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          title: emojiTitle,
          saveas: './emoji.txt',
        };
        let task = await request.agent.create(context, config);
        expect(task.config.title).assertEqual(emojiTitle);
        await task.delete();
        console.info("===> testBoundaryEmojiCharacters001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testBoundaryUnicodeCharacters001
     * @tc.name: testBoundaryUnicodeCharacters001
     * @tc.desc: Description with Unicode characters boundary test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a description string containing multiple Unicode characters
     *              (Japanese, Korean, Chinese, Arabic, and emoji)
     *           2. Configure a download task with the Unicode description
     *           3. Set the saveas path to './unicode.txt'
     *           4. Call request.agent.create() to create the task
     *           5. Verify the task config description equals the Unicode description
     *           6. Delete the task after verification
     * @tc.require: issues#1573
     */
    it('testBoundaryUnicodeCharacters001', 0, async (done: Function) => {
      console.info("===> testBoundaryUnicodeCharacters001 start");
      try {
        let unicodeDesc = '日本語テスト한국어测试مرحبا🌍🌎🌏';

        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          description: unicodeDesc,
          saveas: './unicode.txt',
        };
        let task = await request.agent.create(context, config);
        expect(task.config.description).assertEqual(unicodeDesc);
        await task.delete();
        console.info("===> testBoundaryUnicodeCharacters001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testBoundarySpecialSymbols001
     * @tc.name: testBoundarySpecialSymbols001
     * @tc.desc: Filename with special symbols boundary test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a test file in the cache directory
     *           2. Create a filename containing special symbols 'file-test_123.456(1)[2]{3}.txt'
     *           3. Configure an upload task with the special filename
     *           4. Call request.agent.create() to create the task
     *           5. Verify the task config data filename equals the special filename
     *           6. Delete the task after verification
     * @tc.require: issues#1573
     */
    it('testBoundarySpecialSymbols001', 0, async (done: Function) => {
      console.info("===> testBoundarySpecialSymbols001 start");
      try {
        let testFile = cacheDir + '/test.txt';
        let file = fs.openSync(testFile, fs.OpenMode.CREATE | fs.OpenMode.READ_WRITE);
        fs.writeSync(file, 'test content');
        fs.closeSync(file);

        let specialFilename = 'file-test_123.456(1)[2]{3}.txt';

        let config: request.agent.Config = {
          action: request.agent.Action.UPLOAD,
          url: "http://127.0.0.1:18080/upload",
          data: [{
            name: 'file',
            value: { path: './test.txt', filename: specialFilename }
          }]
        };
        let task = await request.agent.create(context, config);
        expect(task.config.data![0].value.filename).assertEqual(specialFilename);
        await task.delete();
        console.info("===> testBoundarySpecialSymbols001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    // ==================== 数值边界测试 ====================

    /**
     * @tc.number: testBoundaryPriorityNegative001
     * @tc.name: testBoundaryPriorityNegative001
     * @tc.desc: Priority with negative value boundary test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Configure a download task with priority set to -1
     *           2. Set the saveas path to './boundary.txt'
     *           3. Call request.agent.create() to create the task
     *           4. Verify the task config priority equals -1
     *           5. Delete the task after verification
     * @tc.require: issues#1573
     */
    it('testBoundaryPriorityNegative001', 0, async (done: Function) => {
      console.info("===> testBoundaryPriorityNegative001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          priority: -1,
          saveas: './boundary.txt',
        };
        let task = await request.agent.create(context, config);
        expect(task.config.priority).assertEqual(-1);
        await task.delete();
        console.info("===> testBoundaryPriorityNegative001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testBoundaryPriorityLargeValue001
     * @tc.name: testBoundaryPriorityLargeValue001
     * @tc.desc: Priority with large value 65535 boundary test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Configure a download task with priority set to 65535
     *           2. Set the saveas path to './boundary.txt'
     *           3. Call request.agent.create() to create the task
     *           4. Verify the task config priority equals 65535
     *           5. Delete the task after verification
     * @tc.require: issues#1573
     */
    it('testBoundaryPriorityLargeValue001', 0, async (done: Function) => {
      console.info("===> testBoundaryPriorityLargeValue001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          priority: 65535,
          saveas: './boundary.txt',
        };
        let task = await request.agent.create(context, config);
        expect(task.config.priority).assertEqual(65535);
        await task.delete();
        console.info("===> testBoundaryPriorityLargeValue001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testBoundaryIndexNegative001
     * @tc.name: testBoundaryIndexNegative001
     * @tc.desc: Index with negative value boundary test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Configure a download task with index set to -1
     *           2. Set the saveas path to './boundary.txt'
     *           3. Call request.agent.create() to create the task
     *           4. Verify the task config index equals -1
     *           5. Delete the task after verification
     * @tc.require: issues#1573
     */
    it('testBoundaryIndexNegative001', 0, async (done: Function) => {
      console.info("===> testBoundaryIndexNegative001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          index: -1,
          saveas: './boundary.txt',
        };
        let task = await request.agent.create(context, config);
        expect(task.config.index).assertEqual(-1);
        await task.delete();
        console.info("===> testBoundaryIndexNegative001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testBoundaryIndexLargeValue001
     * @tc.name: testBoundaryIndexLargeValue001
     * @tc.desc: Index with large value boundary test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Configure a download task with index set to 999999
     *           2. Set the saveas path to './boundary.txt'
     *           3. Call request.agent.create() to create the task
     *           4. Verify the task config index equals 999999
     *           5. Delete the task after verification
     * @tc.require: issues#1573
     */
    it('testBoundaryIndexLargeValue001', 0, async (done: Function) => {
      console.info("===> testBoundaryIndexLargeValue001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          index: 999999,
          saveas: './boundary.txt',
        };
        let task = await request.agent.create(context, config);
        expect(task.config.index).assertEqual(999999);
        await task.delete();
        console.info("===> testBoundaryIndexLargeValue001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    // ==================== 空字符串和特殊值测试 ====================

    /**
     * @tc.number: testBoundaryEmptyTitle001
     * @tc.name: testBoundaryEmptyTitle001
     * @tc.desc: Empty title string boundary test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Configure a download task with an empty title string ''
     *           2. Set the saveas path to './boundary.txt'
     *           3. Call request.agent.create() to create the task
     *           4. Verify the task config title equals empty string
     *           5. Delete the task after verification
     * @tc.require: issues#1573
     */
    it('testBoundaryEmptyTitle001', 0, async (done: Function) => {
      console.info("===> testBoundaryEmptyTitle001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          title: '',
          saveas: './boundary.txt',
        };
        let task = await request.agent.create(context, config);
        expect(task.config.title).assertEqual('');
        await task.delete();
        console.info("===> testBoundaryEmptyTitle001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testBoundaryEmptyDescription001
     * @tc.name: testBoundaryEmptyDescription001
     * @tc.desc: Empty description string boundary test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Configure a download task with an empty description string ''
     *           2. Set the saveas path to './boundary.txt'
     *           3. Call request.agent.create() to create the task
     *           4. Verify the task config description equals empty string
     *           5. Delete the task after verification
     * @tc.require: issues#1573
     */
    it('testBoundaryEmptyDescription001', 0, async (done: Function) => {
      console.info("===> testBoundaryEmptyDescription001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          description: '',
          saveas: './boundary.txt',
        };
        let task = await request.agent.create(context, config);
        expect(task.config.description).assertEqual('');
        await task.delete();
        console.info("===> testBoundaryEmptyDescription001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testBoundaryWhitespaceOnly001
     * @tc.name: testBoundaryWhitespaceOnly001
     * @tc.desc: Whitespace only title and description boundary test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a title string containing only whitespace characters (spaces, tabs, newlines)
     *           2. Configure a download task with the whitespace-only title
     *           3. Set the saveas path to './boundary.txt'
     *           4. Call request.agent.create() to create the task
     *           5. Verify the task config title equals the whitespace string
     *           6. Delete the task after verification
     * @tc.require: issues#1573
     */
    it('testBoundaryWhitespaceOnly001', 0, async (done: Function) => {
      console.info("===> testBoundaryWhitespaceOnly001 start");
      try {
        let whitespaceTitle = '   \t\n  ';

        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          title: whitespaceTitle,
          saveas: './boundary.txt',
        };
        let task = await request.agent.create(context, config);
        expect(task.config.title).assertEqual(whitespaceTitle);
        await task.delete();
        console.info("===> testBoundaryWhitespaceOnly001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testBoundaryIndexZero001
     * @tc.name: testBoundaryIndexZero001
     * @tc.desc: Index with zero value boundary test
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Configure a download task with index set to 0
     *           2. Set the saveas path to './boundary.txt'
     *           3. Call request.agent.create() to create the task
     *           4. Verify the task config index equals 0
     *           5. Delete the task after verification
     * @tc.require: issues#1573
     */
    it('testBoundaryIndexZero001', 0, async (done: Function) => {
      console.info("===> testBoundaryIndexZero001 start");
      try {
        let config: request.agent.Config = {
          action: request.agent.Action.DOWNLOAD,
          url: "http://127.0.0.1:18080/test.txt",
          index: 0,
          saveas: './boundary.txt',
        };
        let task = await request.agent.create(context, config);
        expect(task.config.index).assertEqual(0);
        await task.delete();
        console.info("===> testBoundaryIndexZero001 end");
        done();
      } catch (err) {
        console.error("===> error:" + JSON.stringify(err));
        expect(false).assertTrue();
        done();
      }
    });

  });
}
