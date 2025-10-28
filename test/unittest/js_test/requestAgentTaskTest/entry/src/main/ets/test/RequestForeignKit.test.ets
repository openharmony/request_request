/*
 * Copyright (C) 2025 Huawei Device Co., Ltd.
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

export default function requestForeignKitTest() {
  describe('requestForeignKitTest', () => {
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

    let context: common.UIAbilityContext = globalThis.abilityContext;
    let cacheDir: String = globalThis.cacheDir;

    /**
     * @tc.number: testDownloadTaskUserFile001
     * @tc.name: testDownloadTaskUserFile001
     * @tc.desc: Test when download user file with wrong `Mode`
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
    it('testDownloadTaskUserFile001', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.BACKGROUND,
        saveas: 'file://media/Photo/1/test.txt'
      };
      try {
        await request.agent.create(context, conf).then(async () => {
          expect(false).assertTrue();
          done();
        }).catch((err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(true).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testDownloadTaskUserFile002
     * @tc.name: testDownloadTaskUserFile002
     * @tc.desc: Test when download user file without `overrite`
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
    it('testDownloadTaskUserFile002', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.FOREGROUND,
        saveas: 'file://media/Photo/1/test.txt',
        overwrite: false,
      };
      try {
        await request.agent.create(context, conf).then(async () => {
          expect(false).assertTrue();
          done();
        }).catch((err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(true).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testDownloadTaskUserFile003
     * @tc.name: testDownloadTaskUserFile003
     * @tc.desc: Test when download user file without `overrite`
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
    it('testDownloadTaskUserFile003', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.FOREGROUND,
        saveas: 'file://media/Photo/1/test.txt',
        overwrite: true,
      };
      try {
        await request.agent.create(context, conf).then(async () => {
          expect(false).assertTrue();
          done();
        }).catch((err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(true).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testUploadTaskUserFile001
     * @tc.name: testUploadTaskUserFile001
     * @tc.desc: Test when upload user file with wrong `Mode`
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
    it('testUploadTaskUserFile001', 0, async (done: Function) => {
      let fileSpec: request.agent.FileSpec = {
        path: `file://media/Photo/1/test.img`
      };
      let formItem: request.agent.FormItem = {
        name: 'file',
        value: fileSpec
      };
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        mode: request.agent.Mode.BACKGROUND,
        data: [formItem],
      };
      try {
        await request.agent.create(context, conf).then(async () => {
          expect(false).assertTrue();
          done();
        }).catch((err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(true).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testTaskBroadcastEvent001
     * @tc.name: testTaskBroadcastEvent001
     * @tc.desc: Test downloadtTask for broadcast event when success
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
    it('testTaskBroadcastEvent001', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testTaskBroadcastEvent001.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      try {
        await task.start();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
      let subscriberCallback = (err: BusinessError, data: commonEvent.CommonEventData) => {
        try {
          commonEvent.unsubscribe(subscriber, unSubscriberCallback);
          expect(data.code).assertEqual(64);
          done();
        } catch (err) {
          expect(false).assertTrue();
          done();
        }
      };
      let unSubscriberCallback = (err: BusinessError) => {
        if (err) {
          expect(false).assertTrue();
          done();
        }
      };
      let commonEventSubscribeInfo: commonEvent.CommonEventSubscribeInfo = {
        events: [request.agent.BroadcastEvent.COMPLETE]
      };
      let subscriber: commonEvent.CommonEventSubscriber = await commonEvent.createSubscriber(commonEventSubscribeInfo);
      await commonEvent.subscribe(subscriber, subscriberCallback);
    });
    
    /**
     * @tc.number: testTaskBroadcastEvent002
     * @tc.name: testTaskBroadcastEvent002
     * @tc.desc: Test downloadtTask for broadcast event when failed
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskBroadcastEvent002', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.mp3',
        saveas: 'testTaskBroadcastEvent002.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      let task = await request.agent.create(context, conf);
      try {
        await task.start();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
      let subscriberCallback = (err: BusinessError, data: commonEvent.CommonEventData) => {
        try {
          commonEvent.unsubscribe(subscriber, unSubscriberCallback);
          expect(data.code).assertEqual(65);
          done();
        } catch (err) {
          expect(false).assertTrue();
          done();
        }
      };
      let unSubscriberCallback = (err: BusinessError) => {
        if (err) {
          expect(false).assertTrue();
          done();
        }
      };
      let commonEventSubscribeInfo: commonEvent.CommonEventSubscribeInfo = {
        events: [request.agent.BroadcastEvent.COMPLETE]
      };
      let subscriber: commonEvent.CommonEventSubscriber = await commonEvent.createSubscriber(commonEventSubscribeInfo);
      await commonEvent.subscribe(subscriber, subscriberCallback);
    });
  })
}