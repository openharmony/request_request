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

export default function requestBasicConfigTest() {
  describe('requestBasicConfigTest', () => {
    beforeAll(() => {
      console.info('beforeAll called');
    });
    afterAll(() => {
      console.info('afterAll called');
    });
    beforeEach(() => {
      console.info('beforeEach called');
      if (fs.accessSync(cacheDir + '/test.txt')) {
        fs.unlinkSync(cacheDir + '/test.txt');
      }
      if (fs.accessSync(cacheDir + '/test1.txt')) {
        fs.unlinkSync(cacheDir + '/test1.txt');
      }
    });
    afterEach(async () => {
      console.info('afterEach called');
      if (fs.accessSync(cacheDir + '/test.txt')) {
        fs.unlinkSync(cacheDir + '/test.txt');
      }
      if (fs.accessSync(cacheDir + '/test1.txt')) {
        fs.unlinkSync(cacheDir + '/test1.txt');
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
     * @tc.number: testTaskNormalConfig001
     * @tc.name: testTaskNormalConfig001
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, and other parameters to undefined.
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
    it('testTaskNormalConfig001', 0, (done: Function) => {
        console.info("===> testTaskNormalConfig001 test start!");

        let config: request.agent.Config = {
            action: request.agent.Action.DOWNLOAD,
            url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
            title: undefined,
            description: undefined,
            mode: undefined,
            overwrite: true,
            method: undefined,
            headers: undefined,
            // data: undefined,
            saveas: undefined,
            network: undefined,
            metered: undefined,
            roaming: undefined,
            retry: undefined,
            redirect: undefined,
            proxy: undefined,
            index: undefined,
            begins: undefined,
            ends: undefined,
            gauge: undefined,
            precise: undefined,
            token: undefined,
            priority: undefined,
            extras: undefined,
            multipart: undefined,
            notification: undefined,
            minSpeed: undefined,
            timeout: undefined,
        }
        try {
            let task = await request.agent.create(context, config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual(undefined);
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(info.headers).assertEqual(undefined);
                expect(info.index).assertEqual(undefined);
                expect(info.metered).assertEqual(undefined);
                expect(info.method).assertEqual(undefined);
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(undefined);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(undefined);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(undefined);
                expect(info.redirect).assertEqual(undefined);
                expect(info.retry).assertEqual(undefined);
                expect(info.roaming).assertEqual(undefined);
                expect(info.saveas).assertEqual(undefined);
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual(undefined);
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskNormalConfig001 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskNormalConfig001 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskNormalConfig001 test fail!");
        }
    });

    /**
     * @tc.number: testTaskConfigTitle001
     * @tc.name: testTaskConfigTitle001
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, title to "title",
     and other parameters to undefined.
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
    it('testDownloadConfigTitle001', 0, (done: Function) => {
      console.info("===> testDownloadConfigTitle001 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: "title",
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
       try {
            let task = await request.agent.create(context, config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual(undefined);
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(info.headers).assertEqual(undefined);
                expect(info.index).assertEqual(undefined);
                expect(info.metered).assertEqual(undefined);
                expect(info.method).assertEqual(undefined);
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(undefined);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(undefined);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(undefined);
                expect(info.redirect).assertEqual(undefined);
                expect(info.retry).assertEqual(undefined);
                expect(info.roaming).assertEqual(undefined);
                expect(info.saveas).assertEqual(undefined);
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual("title");
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskConfigTitle001 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskConfigTitle001 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskConfigTitle001 test fail!");
        }
    });

    /**
     * @tc.number: testTaskConfigTitle002
     * @tc.name: testTaskConfigTitle002
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, title to "",
     and other parameters to undefined.
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
    it('testDownloadConfigTitle002', 0, (done: Function) => {
      console.info("===> testDownloadConfigTitle002 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: "",
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
       try {
            let task = await request.agent.create(context, config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual(undefined);
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(info.headers).assertEqual(undefined);
                expect(info.index).assertEqual(undefined);
                expect(info.metered).assertEqual(undefined);
                expect(info.method).assertEqual(undefined);
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(undefined);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(undefined);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(undefined);
                expect(info.redirect).assertEqual(undefined);
                expect(info.retry).assertEqual(undefined);
                expect(info.roaming).assertEqual(undefined);
                expect(info.saveas).assertEqual(undefined);
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual("");
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskConfigTitle002 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskConfigTitle002 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskConfigTitle002 test fail!");
        }
    });

    /**
     * @tc.number: testTaskConfigTitle003
     * @tc.name: testTaskConfigTitle003
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, title to 'a'.repeat(256),
     and other parameters to undefined.
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
    it('testTaskConfigTitle003', 0, (done: Function) => {
      console.info("===> testTaskConfigTitle003 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: 'a'.repeat(256),
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
       try {
            let task = await request.agent.create(context, config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual(undefined);
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(info.headers).assertEqual(undefined);
                expect(info.index).assertEqual(undefined);
                expect(info.metered).assertEqual(undefined);
                expect(info.method).assertEqual(undefined);
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(undefined);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(undefined);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(undefined);
                expect(info.redirect).assertEqual(undefined);
                expect(info.retry).assertEqual(undefined);
                expect(info.roaming).assertEqual(undefined);
                expect(info.saveas).assertEqual(undefined);
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual("a".repeat(256));
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskConfigTitle003 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskConfigTitle003 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskConfigTitle003 test fail!");
        }
    });

    /**
     * @tc.number: testTaskConfigTitle004
     * @tc.name: testTaskConfigTitle004
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, title to 'a'.repeat(257),
     and other parameters to undefined.
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
    it('testTaskConfigTitle004', 0, (done: Function) => {
      console.info("===> testTaskConfigTitle004 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: 'a'.repeat(257),
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }

      try {
        request.agent.create(context, config, async (err, data) => {
          console.info("===> testTaskConfigTitle004 test fail!");
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        console.info("===> testTaskConfigTitle004 test success!");
        done();
      }
    });

    /**
     * @tc.number: testTaskConfigTitle005
     * @tc.name: testTaskConfigTitle005
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, title to int,
     and other parameters to undefined.
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
    it('testTaskConfigTitle005', 0, (done: Function) => {
      console.info("===> testTaskConfigTitle005 test start!");

      let config: Record<string, string | number | undefined> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "title": 1,
        "description": undefined,
        "mode": undefined,
        "overwrite": undefined,
        "method": undefined,
        "headers": undefined,
        //data: undefined,
        "saveas": undefined,
        "network": undefined,
        "metered": undefined,
        "roaming": undefined,
        "retry": undefined,
        "redirect": undefined,
        "proxy": undefined,
        "index": undefined,
        "begins": undefined,
        "ends": undefined,
        "gauge": undefined,
        "precise": undefined,
        "token": undefined,
        "priority": undefined,
        "extras": undefined,
        "multipart": undefined,
        "notification": undefined,
        "minSpeed": undefined,
        "timeout": undefined,
      }

       try {
            let task = await request.agent.create(context, conf as Object as request.agent.Config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual(undefined);
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(info.headers).assertEqual(undefined);
                expect(info.index).assertEqual(undefined);
                expect(info.metered).assertEqual(undefined);
                expect(info.method).assertEqual(undefined);
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(undefined);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(undefined);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(undefined);
                expect(info.redirect).assertEqual(undefined);
                expect(info.retry).assertEqual(undefined);
                expect(info.roaming).assertEqual(undefined);
                expect(info.saveas).assertEqual(undefined);
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual(1);
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskConfigTitle005 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskConfigTitle005 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskConfigTitle005 test fail!");
        }
    });

    /**
     * @tc.number: testTaskConfigDescription001
     * @tc.name: testTaskConfigDescription001
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, description to "desc",
     and other parameters to undefined.
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
    it('testTaskConfigDescription001', 0, (done: Function) => {
      console.info("===> testTaskConfigDescription001 test start!");

        let config: request.agent.Config = {
            action: request.agent.Action.DOWNLOAD,
            url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
            title: undefined,
            description: "desc",
            mode: undefined,
            overwrite: undefined,
            method: undefined,
            headers: undefined,
            // data: undefined,
            saveas: undefined,
            network: undefined,
            metered: undefined,
            roaming: undefined,
            retry: undefined,
            redirect: undefined,
            proxy: undefined,
            index: undefined,
            begins: undefined,
            ends: undefined,
            gauge: undefined,
            precise: undefined,
            token: undefined,
            priority: undefined,
            extras: undefined,
            multipart: undefined,
            notification: undefined,
            minSpeed: undefined,
            timeout: undefined,
        }
        try {
            let task = await request.agent.create(context, conf as Object as request.agent.Config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual("desc");
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(info.headers).assertEqual(undefined);
                expect(info.index).assertEqual(undefined);
                expect(info.metered).assertEqual(undefined);
                expect(info.method).assertEqual(undefined);
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(undefined);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(undefined);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(undefined);
                expect(info.redirect).assertEqual(undefined);
                expect(info.retry).assertEqual(undefined);
                expect(info.roaming).assertEqual(undefined);
                expect(info.saveas).assertEqual(undefined);
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual(undefined);
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskConfigDescription001 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskConfigDescription001 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskConfigDescription001 test fail!");
        }
    });

    /**
     * @tc.number: testTaskConfigDescription002
     * @tc.name: testTaskConfigDescription002
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, description to "",
     and other parameters to undefined.
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
    it('testDownloadConfigDescription002', 0, (done: Function) => {
      console.info("===> testDownloadConfigDescription002 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: "",
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
        try {
            let task = await request.agent.create(context, conf as Object as request.agent.Config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual("");
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(info.headers).assertEqual(undefined);
                expect(info.index).assertEqual(undefined);
                expect(info.metered).assertEqual(undefined);
                expect(info.method).assertEqual(undefined);
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(undefined);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(undefined);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(undefined);
                expect(info.redirect).assertEqual(undefined);
                expect(info.retry).assertEqual(undefined);
                expect(info.roaming).assertEqual(undefined);
                expect(info.saveas).assertEqual(undefined);
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual(undefined);
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskConfigDescription002 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskConfigDescription002 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskConfigDescription002 test fail!");
        }
    });

    /**
     * @tc.number: testTaskConfigDescription003
     * @tc.name: testTaskConfigDescription003
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, description to 'a'.repeat(1024),
     and other parameters to undefined.
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
    it('testTaskConfigDescription003', 0, (done: Function) => {
      console.info("===> testTaskConfigDescription003 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: 'a'.repeat(1024),
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
        try {
            let task = await request.agent.create(context, conf as Object as request.agent.Config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual('a'.repeat(1024));
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(info.headers).assertEqual(undefined);
                expect(info.index).assertEqual(undefined);
                expect(info.metered).assertEqual(undefined);
                expect(info.method).assertEqual(undefined);
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(undefined);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(undefined);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(undefined);
                expect(info.redirect).assertEqual(undefined);
                expect(info.retry).assertEqual(undefined);
                expect(info.roaming).assertEqual(undefined);
                expect(info.saveas).assertEqual(undefined);
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual(undefined);
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskConfigDescription003 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskConfigDescription003 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskConfigDescription003 test fail!");
        }
    });

    /**
     * @tc.number: testTaskConfigDescription004
     * @tc.name: testTaskConfigDescription004
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, description to 'a'.repeat(1025),
     and other parameters to undefined.
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
    it('testTaskConfigDescription004', 0, (done: Function) => {
      console.info("===> testTaskConfigDescription004 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: 'a'.repeat(1025),
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }

      try {
        request.agent.create(context, config, async (err, data) => {
          console.info("===> testTaskConfigDescription004 test fail!");
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        console.info("===> testTaskConfigDescription004 test success!");
        done();
      }
    });

    /**
     * @tc.number: testTaskConfigDescription005
     * @tc.name: testTaskConfigDescription005
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, description to int,
     and other parameters to undefined.
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
    it('testTaskConfigDescription005', 0, (done: Function) => {
      console.info("===> testTaskConfigDescription005 test start!");

      let config: Record<string, string | number | undefined> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "title": undefined,
        "description": 1,
        "mode": undefined,
        "overwrite": undefined,
        "method": undefined,
        "headers": undefined,
        //data: undefined,
        "saveas": undefined,
        "network": undefined,
        "metered": undefined,
        "roaming": undefined,
        "retry": undefined,
        "redirect": undefined,
        "proxy": undefined,
        "index": undefined,
        "begins": undefined,
        "ends": undefined,
        "gauge": undefined,
        "precise": undefined,
        "token": undefined,
        "priority": undefined,
        "extras": undefined,
        "multipart": undefined,
        "notification": undefined,
        "minSpeed": undefined,
        "timeout": undefined,
      }

        try {
            let task = await request.agent.create(context, conf as Object as request.agent.Config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual(1);
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(info.headers).assertEqual(undefined);
                expect(info.index).assertEqual(undefined);
                expect(info.metered).assertEqual(undefined);
                expect(info.method).assertEqual(undefined);
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(undefined);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(undefined);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(undefined);
                expect(info.redirect).assertEqual(undefined);
                expect(info.retry).assertEqual(undefined);
                expect(info.roaming).assertEqual(undefined);
                expect(info.saveas).assertEqual(undefined);
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual(undefined);
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskConfigDescription005 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskConfigDescription005 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskConfigDescription005 test fail!");
        }
    });

    /**
     * @tc.number: testTaskConfigMode001
     * @tc.name: testTaskConfigMode001
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, mode to BACKGROUND,
     and other parameters to undefined.
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
    it('testTaskConfigMode001', 0, (done: Function) => {
      console.info("===> testTaskConfigMode001 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: request.agent.Mode.BACKGROUND,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
        try {
            let task = await request.agent.create(context, conf as Object as request.agent.Config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual(undefined);
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(info.headers).assertEqual(undefined);
                expect(info.index).assertEqual(undefined);
                expect(info.metered).assertEqual(undefined);
                expect(info.method).assertEqual(undefined);
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(request.agent.Mode.BACKGROUND);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(undefined);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(undefined);
                expect(info.redirect).assertEqual(undefined);
                expect(info.retry).assertEqual(undefined);
                expect(info.roaming).assertEqual(undefined);
                expect(info.saveas).assertEqual(undefined);
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual(undefined);
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskConfigMode001 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskConfigMode001 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskConfigMode001 test fail!");
        }
    });

    /**
     * @tc.number: testTaskConfigMode002
     * @tc.name: testTaskConfigMode002
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, mode to FOREGROUND,
     and other parameters to undefined.
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
    it('testTaskConfigMode002', 0, (done: Function) => {
      console.info("===> testTaskConfigMode002 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: request.agent.Mode.FOREGROUND,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
        try {
            let task = await request.agent.create(context, conf as Object as request.agent.Config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual(undefined);
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(info.headers).assertEqual(undefined);
                expect(info.index).assertEqual(undefined);
                expect(info.metered).assertEqual(undefined);
                expect(info.method).assertEqual(undefined);
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(request.agent.Mode.FOREGROUND);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(undefined);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(undefined);
                expect(info.redirect).assertEqual(undefined);
                expect(info.retry).assertEqual(undefined);
                expect(info.roaming).assertEqual(undefined);
                expect(info.saveas).assertEqual(undefined);
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual(undefined);
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskConfigMode002 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskConfigMode002 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskConfigMode002 test fail!");
        }
    });

    /**
     * @tc.number: testTaskConfigMode003
     * @tc.name: testTaskConfigMode003
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, mode to int,
     and other parameters to undefined.
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
    it('testTaskConfigMode003', 0, (done: Function) => {
      console.info("===> testTaskConfigMode003 test start!");

      let config: Record<string, string | number | undefined> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "title": undefined,
        "description": undefined,
        "mode": 1,
        "overwrite": undefined,
        "method": undefined,
        "headers": undefined,
        //data: undefined,
        "saveas": undefined,
        "network": undefined,
        "metered": undefined,
        "roaming": undefined,
        "retry": undefined,
        "redirect": undefined,
        "proxy": undefined,
        "index": undefined,
        "begins": undefined,
        "ends": undefined,
        "gauge": undefined,
        "precise": undefined,
        "token": undefined,
        "priority": undefined,
        "extras": undefined,
        "multipart": undefined,
        "notification": undefined,
        "minSpeed": undefined,
        "timeout": undefined,
      }

        try {
            let task = await request.agent.create(context, conf as Object as request.agent.Config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual(undefined);
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(info.headers).assertEqual(undefined);
                expect(info.index).assertEqual(undefined);
                expect(info.metered).assertEqual(undefined);
                expect(info.method).assertEqual(undefined);
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(1);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(undefined);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(undefined);
                expect(info.redirect).assertEqual(undefined);
                expect(info.retry).assertEqual(undefined);
                expect(info.roaming).assertEqual(undefined);
                expect(info.saveas).assertEqual(undefined);
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual(undefined);
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskConfigMode003 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskConfigMode003 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskConfigMode003 test fail!");
        }
    });

    /**
     * @tc.number: testTaskConfigOverwrite001
     * @tc.name: testTaskConfigOverwrite001
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, overwrite to true,
     and other parameters to undefined.
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
    it('testTaskConfigOverwrite001', 0, (done: Function) => {
      console.info("===> testTaskConfigOverwrite001 test start!");
      
      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: true,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
        try {
            let task = await request.agent.create(context, conf as Object as request.agent.Config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual(undefined);
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(info.headers).assertEqual(undefined);
                expect(info.index).assertEqual(undefined);
                expect(info.metered).assertEqual(undefined);
                expect(info.method).assertEqual(undefined);
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(undefined);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(true);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(undefined);
                expect(info.redirect).assertEqual(undefined);
                expect(info.retry).assertEqual(undefined);
                expect(info.roaming).assertEqual(undefined);
                expect(info.saveas).assertEqual(undefined);
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual(undefined);
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskConfigOverwrite001 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskConfigOverwrite001 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskConfigOverwrite001 test fail!");
        }
    });

    /**
     * @tc.number: testTaskConfigOverwrite002
     * @tc.name: testTaskConfigOverwrite002
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, overwrite to false,
     and other parameters to undefined.
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
    it('testTaskConfigOverwrite002', 0, (done: Function) => {
      console.info("===> testTaskConfigOverwrite002 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: false,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
        try {
            let task = await request.agent.create(context, conf as Object as request.agent.Config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual(undefined);
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(info.headers).assertEqual(undefined);
                expect(info.index).assertEqual(undefined);
                expect(info.metered).assertEqual(undefined);
                expect(info.method).assertEqual(undefined);
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(undefined);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(false);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(undefined);
                expect(info.redirect).assertEqual(undefined);
                expect(info.retry).assertEqual(undefined);
                expect(info.roaming).assertEqual(undefined);
                expect(info.saveas).assertEqual(undefined);
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual(undefined);
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskConfigOverwrite002 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskConfigOverwrite002 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskConfigOverwrite002 test fail!");
        }
    });

    /**
     * @tc.number: testTaskConfigMethod001
     * @tc.name: testTaskConfigMethod001
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, method to GET,
     and other parameters to undefined.
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
    it('testTaskConfigMethod001', 0, (done: Function) => {
      console.info("===> testTaskConfigMethod001 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: 'GET',
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
        try {
            let task = await request.agent.create(context, conf as Object as request.agent.Config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual(undefined);
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(info.headers).assertEqual(undefined);
                expect(info.index).assertEqual(undefined);
                expect(info.metered).assertEqual(undefined);
                expect(info.method).assertEqual('GET');
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(undefined);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(undefined);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(undefined);
                expect(info.redirect).assertEqual(undefined);
                expect(info.retry).assertEqual(undefined);
                expect(info.roaming).assertEqual(undefined);
                expect(info.saveas).assertEqual(undefined);
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual(undefined);
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskConfigMethod001 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskConfigMethod001 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskConfigMethod001 test fail!");
        }
    });

    /**
     * @tc.number: testTaskConfigMethod002
     * @tc.name: testTaskConfigMethod002
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, method to POST,
     and other parameters to undefined.
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
    it('testTaskConfigMethod002', 0, (done: Function) => {
      console.info("===> testTaskConfigMethod002 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: 'POST',
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
        try {
            let task = await request.agent.create(context, conf as Object as request.agent.Config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual(undefined);
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(info.headers).assertEqual(undefined);
                expect(info.index).assertEqual(undefined);
                expect(info.metered).assertEqual(undefined);
                expect(info.method).assertEqual('POST');
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(undefined);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(undefined);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(undefined);
                expect(info.redirect).assertEqual(undefined);
                expect(info.retry).assertEqual(undefined);
                expect(info.roaming).assertEqual(undefined);
                expect(info.saveas).assertEqual(undefined);
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual(undefined);
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskConfigMethod002 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskConfigMethod002 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskConfigMethod002 test fail!");
        }
    });

    /**
     * @tc.number: testTaskConfigMethod003
     * @tc.name: testTaskConfigMethod003
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url, method to PUT,
     and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require: issues#1573
     */
    it('testTaskConfigMethod003', 0, (done: Function) => {
      console.info("===> testTaskConfigMethod003 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: 'PUT',
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
        try {
            let task = await request.agent.create(context, conf as Object as request.agent.Config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual(undefined);
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(info.headers).assertEqual(undefined);
                expect(info.index).assertEqual(undefined);
                expect(info.metered).assertEqual(undefined);
                expect(info.method).assertEqual('PUT');
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(undefined);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(undefined);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(undefined);
                expect(info.redirect).assertEqual(undefined);
                expect(info.retry).assertEqual(undefined);
                expect(info.roaming).assertEqual(undefined);
                expect(info.saveas).assertEqual(undefined);
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual(undefined);
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskConfigMethod003 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskConfigMethod003 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskConfigMethod003 test fail!");
        }
    });

    /**
     * @tc.number: testTaskConfigMethod004
     * @tc.name: testTaskConfigMethod004
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     method to 1, and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskConfigMethod004', 0, (done: Function) => {
      console.info("===> testTaskConfigMethod004 test start!");

      let config: Record<string, string | number | undefined> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "title": undefined,
        "description": undefined,
        "mode": undefined,
        "overwrite": undefined,
        "method": 1,
        "headers": undefined,
        //data: undefined,
        "saveas": undefined,
        "network": undefined,
        "metered": undefined,
        "roaming": undefined,
        "retry": undefined,
        "redirect": undefined,
        "proxy": undefined,
        "index": undefined,
        "begins": undefined,
        "ends": undefined,
        "gauge": undefined,
        "precise": undefined,
        "token": undefined,
        "priority": undefined,
        "extras": undefined,
        "multipart": undefined,
        "notification": undefined,
        "minSpeed": undefined,
        "timeout": undefined,
      }

        try {
            let task = await request.agent.create(context, conf as Object as request.agent.Config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual(undefined);
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(info.headers).assertEqual(undefined);
                expect(info.index).assertEqual(undefined);
                expect(info.metered).assertEqual(undefined);
                expect(info.method).assertEqual(1);
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(undefined);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(undefined);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(undefined);
                expect(info.redirect).assertEqual(undefined);
                expect(info.retry).assertEqual(undefined);
                expect(info.roaming).assertEqual(undefined);
                expect(info.saveas).assertEqual(undefined);
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual(undefined);
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskConfigMethod003 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskConfigMethod003 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskConfigMethod003 test fail!");
        }
    });

    /**
     * @tc.number: testTaskConfigHeaders001
     * @tc.name: testTaskConfigHeaders001
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     headers to 'application/json' and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskConfigHeaders001', 0, (done: Function) => {
      console.info("===> testTaskConfigHeaders001 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: {
          'content-type': 'application/json'
        },
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
        try {
            let task = await request.agent.create(context, conf as Object as request.agent.Config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual(undefined);
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(typeof info.headers).assertEqual('object');
                expect(info.index).assertEqual(undefined);
                expect(info.metered).assertEqual(undefined);
                expect(info.method).assertEqual(undefined);
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(undefined);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(undefined);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(undefined);
                expect(info.redirect).assertEqual(undefined);
                expect(info.retry).assertEqual(undefined);
                expect(info.roaming).assertEqual(undefined);
                expect(info.saveas).assertEqual(undefined);
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual(undefined);
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskConfigMethod003 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskConfigMethod003 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskConfigMethod003 test fail!");
        }
    });
    
    /**
     * @tc.number: testTaskConfigHeaders002
     * @tc.name: testTaskConfigHeaders002
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     headers to 'multipart/form-data' and other parameters to undefined.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskConfigHeaders002', 0, (done: Function) => {
      console.info("===> testTaskConfigHeaders002 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: {
          'content-type': 'multipart/form-data'
        },
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
        try {
            let task = await request.agent.create(context, conf as Object as request.agent.Config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual(undefined);
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(typeof info.headers).assertEqual('object');
                expect(info.index).assertEqual(undefined);
                expect(info.metered).assertEqual(undefined);
                expect(info.method).assertEqual(undefined);
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(undefined);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(undefined);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(undefined);
                expect(info.redirect).assertEqual(undefined);
                expect(info.retry).assertEqual(undefined);
                expect(info.roaming).assertEqual(undefined);
                expect(info.saveas).assertEqual(undefined);
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual(undefined);
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskConfigHeaders002 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskConfigHeaders002 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskConfigHeaders002 test fail!");
        }
    });

    /**
     * @tc.number: testTaskConfigHeaders003
     * @tc.name: testTaskConfigHeaders003
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     headers to 1, and other parameters to undefined.
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
    it('testTaskConfigHeaders003', 0, (done: Function) => {
      console.info("===> testTaskConfigHeaders003 test start!");

      let config: Record<string, string | number | undefined> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "title": undefined,
        "description": undefined,
        "mode": undefined,
        "overwrite": undefined,
        "method": undefined,
        "headers": 1,
        //data: undefined,
        "saveas": undefined,
        "network": undefined,
        "metered": undefined,
        "roaming": undefined,
        "retry": undefined,
        "redirect": undefined,
        "proxy": undefined,
        "index": undefined,
        "begins": undefined,
        "ends": undefined,
        "gauge": undefined,
        "precise": undefined,
        "token": undefined,
        "priority": undefined,
        "extras": undefined,
        "multipart": undefined,
        "notification": undefined,
        "minSpeed": undefined,
        "timeout": undefined,
      }

      try {
        request.agent.create(context, config as Object as request.agent.Config, async (err, data) => {
          expect(false).assertTrue();
          console.info("===> testTaskConfigHeaders003 test failed!");
          done();
        });
      } catch (err) {
        expect(true).assertTrue();
        console.info("===> testTaskConfigHeaders003 test success!");
        done();
      }
    });

    /**
     * @tc.number: testTaskConfigSaveas001
     * @tc.name: testTaskConfigSaveas001
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     saveas to 'test.txt' and other parameters to undefined.
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
    it('testTaskConfigSaveas001', 0, (done: Function) => {
      console.info("===> testTaskConfigSaveas001 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: 'test.txt',
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
        try {
            let task = await request.agent.create(context, conf as Object as request.agent.Config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual(undefined);
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(typeof info.headers).assertEqual(undefined);
                expect(info.index).assertEqual(undefined);
                expect(info.metered).assertEqual(undefined);
                expect(info.method).assertEqual(undefined);
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(undefined);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(undefined);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(undefined);
                expect(info.redirect).assertEqual(undefined);
                expect(info.retry).assertEqual(undefined);
                expect(info.roaming).assertEqual(undefined);
                expect(info.saveas).assertEqual('test.txt');
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual(undefined);
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskConfigSaveas001 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskConfigSaveas001 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskConfigSaveas001 test fail!");
        }
    });

    /**
     * @tc.number: testTaskConfigSaveas002
     * @tc.name: testTaskConfigSaveas002
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     saveas to 1, and other parameters to undefined.
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
    it('testTaskConfigSaveas002', 0, (done: Function) => {
      console.info("===> testTaskConfigSaveas002 test start!");

      let config: Record<string, string | number | undefined> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "title": undefined,
        "description": undefined,
        "mode": undefined,
        "overwrite": undefined,
        "method": undefined,
        "headers": undefined,
        //data: undefined,
        "saveas": 1,
        "network": undefined,
        "metered": undefined,
        "roaming": undefined,
        "retry": undefined,
        "redirect": undefined,
        "proxy": undefined,
        "index": undefined,
        "begins": undefined,
        "ends": undefined,
        "gauge": undefined,
        "precise": undefined,
        "token": undefined,
        "priority": undefined,
        "extras": undefined,
        "multipart": undefined,
        "notification": undefined,
        "minSpeed": undefined,
        "timeout": undefined,
      }

        try {
            let task = await request.agent.create(context, conf as Object as request.agent.Config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual(undefined);
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(typeof info.headers).assertEqual(undefined);
                expect(info.index).assertEqual(undefined);
                expect(info.metered).assertEqual(undefined);
                expect(info.method).assertEqual(undefined);
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(undefined);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(undefined);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(undefined);
                expect(info.redirect).assertEqual(undefined);
                expect(info.retry).assertEqual(undefined);
                expect(info.roaming).assertEqual(undefined);
                expect(info.saveas).assertEqual(1);
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual(undefined);
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskConfigSaveas002 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskConfigSaveas002 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskConfigSaveas002 test fail!");
        }
    });

    /**
     * @tc.number: testTaskConfigSaveas003
     * @tc.name: testTaskConfigSaveas003
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     saveas to './' and other parameters to undefined.
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
    it('testTaskConfigSaveas003', 0, (done: Function) => {
      console.info("===> testTaskConfigSaveas003 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: './',
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
        try {
            let task = await request.agent.create(context, conf as Object as request.agent.Config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual(undefined);
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(typeof info.headers).assertEqual(undefined);
                expect(info.index).assertEqual(undefined);
                expect(info.metered).assertEqual(undefined);
                expect(info.method).assertEqual(undefined);
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(undefined);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(undefined);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(undefined);
                expect(info.redirect).assertEqual(undefined);
                expect(info.retry).assertEqual(undefined);
                expect(info.roaming).assertEqual(undefined);
                expect(info.saveas).assertEqual('./');
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual(undefined);
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskConfigSaveas003 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskConfigSaveas003 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskConfigSaveas003 test fail!");
        }
    });

    /**
     * @tc.number: testTaskConfigSaveas004
     * @tc.name: testTaskConfigSaveas004
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     saveas to ./filename and other parameters to undefined.
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
    it('testTaskConfigSaveas004', 0, (done: Function) => {
      console.info("===> testTaskConfigSaveas004 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: './test004.txt',
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
        try {
            let task = await request.agent.create(context, conf as Object as request.agent.Config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual(undefined);
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(typeof info.headers).assertEqual(undefined);
                expect(info.index).assertEqual(undefined);
                expect(info.metered).assertEqual(undefined);
                expect(info.method).assertEqual(undefined);
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(undefined);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(undefined);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(undefined);
                expect(info.redirect).assertEqual(undefined);
                expect(info.retry).assertEqual(undefined);
                expect(info.roaming).assertEqual(undefined);
                expect(info.saveas).assertEqual('./test004.txt');
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual(undefined);
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskConfigSaveas004 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskConfigSaveas004 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskConfigSaveas004 test fail!");
        }
    });

    /**
     * @tc.number: testTaskConfigSaveas005
     * @tc.name: testTaskConfigSaveas005
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     saveas to pathname/filename and other parameters to undefined.
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
    it('testTaskConfigSaveas005', 0, (done: Function) => {
      console.info("===> testTaskConfigSaveas005 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: 'a/b/test005.txt',
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
        try {
            let task = await request.agent.create(context, conf as Object as request.agent.Config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual(undefined);
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(typeof info.headers).assertEqual(undefined);
                expect(info.index).assertEqual(undefined);
                expect(info.metered).assertEqual(undefined);
                expect(info.method).assertEqual(undefined);
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(undefined);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(undefined);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(undefined);
                expect(info.redirect).assertEqual(undefined);
                expect(info.retry).assertEqual(undefined);
                expect(info.roaming).assertEqual(undefined);
                expect(info.saveas).assertEqual('a/b/test005.txt');
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual(undefined);
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskConfigSaveas005 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskConfigSaveas005 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskConfigSaveas005 test fail!");
        }
    });

    /**
     * @tc.number: testTaskConfigSaveas006
     * @tc.name: testTaskConfigSaveas006
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     saveas to internal://cache/filename and other parameters to undefined.
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
    it('testTaskConfigSaveas006', 0, (done: Function) => {
      console.info("===> testTaskConfigSaveas006 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: 'internal://cache/test006',
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
        try {
            let task = await request.agent.create(context, conf as Object as request.agent.Config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual(undefined);
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(typeof info.headers).assertEqual(undefined);
                expect(info.index).assertEqual(undefined);
                expect(info.metered).assertEqual(undefined);
                expect(info.method).assertEqual(undefined);
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(undefined);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(undefined);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(undefined);
                expect(info.redirect).assertEqual(undefined);
                expect(info.retry).assertEqual(undefined);
                expect(info.roaming).assertEqual(undefined);
                expect(info.saveas).assertEqual('internal://cache/test006');
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual(undefined);
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskConfigSaveas006 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskConfigSaveas006 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskConfigSaveas006 test fail!");
        }
    });

    /**
     * @tc.number: testTaskConfigSaveas007
     * @tc.name: testTaskConfigSaveas007
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     saveas is whole path which contains error .. and other parameters to undefined.
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
    it('testTaskConfigSaveas007', 0, (done: Function) => {
      console.info("===> testTaskConfigSaveas007 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: '/data/storage/../../el2/base/test_saveas_007',
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
      try {
        request.agent.create(context, config as Object as request.agent.Config, async (err, data) => {
          expect(false).assertTrue();
          console.info("===> testTaskConfigSaveas007 test failed!");
          done();
        });
      } catch (err) {
        expect(true).assertTrue();
        console.info("===> testTaskConfigSaveas007 test success!");
        done();
      }
    });

    /**
     * @tc.number: testTaskConfigSaveas008
     * @tc.name: testTaskConfigSaveas008
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     saveas is file:// with correct package name and other parameters to undefined.
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
    it('testTaskConfigSaveas008', 0, (done: Function) => {
      console.info("===> testTaskConfigSaveas008 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: 'file://com.acts.request/data/storage/el2/base/test008',
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
        try {
            let task = await request.agent.create(context, conf as Object as request.agent.Config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual(undefined);
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(typeof info.headers).assertEqual(undefined);
                expect(info.index).assertEqual(undefined);
                expect(info.metered).assertEqual(undefined);
                expect(info.method).assertEqual(undefined);
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(undefined);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(undefined);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(undefined);
                expect(info.redirect).assertEqual(undefined);
                expect(info.retry).assertEqual(undefined);
                expect(info.roaming).assertEqual(undefined);
                expect(info.saveas).assertEqual('file://com.acts.request/data/storage/el2/base/test008');
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual(undefined);
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskConfigSaveas008 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskConfigSaveas008 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskConfigSaveas008 test fail!");
        }
    });

    /**
     * @tc.number: testTaskConfigSaveas009
     * @tc.name: testTaskConfigSaveas009
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     saveas is file:// with error package name and other parameters to undefined.
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
    it('testTaskConfigSaveas009', 0, (done: Function) => {
      console.info("===> testTaskConfigSaveas009 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: 'file://com.acts.request1111/data/storage/el2/base/test009',
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
      try {
        request.agent.create(context, config as Object as request.agent.Config, async (err, data) => {
          expect(false).assertTrue();
          console.info("===> testTaskConfigSaveas009 test failed!");
          done();
        });
      } catch (err) {
        expect(true).assertTrue();
        console.info("===> testTaskConfigSaveas009 test success!");
        done();
      }
    });

    /**
     * @tc.number: testTaskConfigMetered001
     * @tc.name: testTaskConfigMetered001
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     metered is true and other parameters to undefined.
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
    it('testTaskConfigMetered001', 0, (done: Function) => {
      console.info("===> testTaskConfigMetered001 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: true,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
        try {
            let task = await request.agent.create(context, conf as Object as request.agent.Config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual(undefined);
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(typeof info.headers).assertEqual(undefined);
                expect(info.index).assertEqual(undefined);
                expect(info.metered).assertEqual(true);
                expect(info.method).assertEqual(undefined);
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(undefined);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(undefined);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(undefined);
                expect(info.redirect).assertEqual(undefined);
                expect(info.retry).assertEqual(undefined);
                expect(info.roaming).assertEqual(undefined);
                expect(info.saveas).assertEqual(undefined);
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual(undefined);
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskConfigMetered001 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskConfigMetered001 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskConfigMetered001 test fail!");
        }
    });

    /**
     * @tc.number: testTaskConfigMetered002
     * @tc.name: testTaskConfigMetered002
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     metered is 1 and other parameters to undefined.
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
    it('testTaskConfigMetered002', 0, (done: Function) => {
      console.info("===> testTaskConfigMetered002 test start!");

      let config: Record<string, string | number | undefined> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "title": undefined,
        "description": undefined,
        "mode": undefined,
        "overwrite": undefined,
        "method": undefined,
        "headers": undefined,
        //data: undefined,
        "saveas": undefined,
        "network": undefined,
        "metered": 1,
        "roaming": undefined,
        "retry": undefined,
        "redirect": undefined,
        "proxy": undefined,
        "index": undefined,
        "begins": undefined,
        "ends": undefined,
        "gauge": undefined,
        "precise": undefined,
        "token": undefined,
        "priority": undefined,
        "extras": undefined,
        "multipart": undefined,
        "notification": undefined,
        "minSpeed": undefined,
        "timeout": undefined,
      }

        try {
            let task = await request.agent.create(context, conf as Object as request.agent.Config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual(undefined);
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(typeof info.headers).assertEqual(undefined);
                expect(info.index).assertEqual(undefined);
                expect(info.metered).assertEqual(1);
                expect(info.method).assertEqual(undefined);
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(undefined);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(undefined);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(undefined);
                expect(info.redirect).assertEqual(undefined);
                expect(info.retry).assertEqual(undefined);
                expect(info.roaming).assertEqual(undefined);
                expect(info.saveas).assertEqual(undefined);
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual(undefined);
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskConfigMetered002 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskConfigMetered002 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskConfigMetered002 test fail!");
        }
    });

    /**
     * @tc.number: testTaskConfigRoaming001
     * @tc.name: testTaskConfigRoaming001
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     roaming is true and other parameters to undefined.
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
    it('testTaskConfigRoaming001', 0, (done: Function) => {
      console.info("===> testTaskConfigRoaming001 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: true,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
        try {
            let task = await request.agent.create(context, conf as Object as request.agent.Config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual(undefined);
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(typeof info.headers).assertEqual(undefined);
                expect(info.index).assertEqual(undefined);
                expect(info.metered).assertEqual(undefined);
                expect(info.method).assertEqual(undefined);
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(undefined);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(undefined);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(undefined);
                expect(info.redirect).assertEqual(undefined);
                expect(info.retry).assertEqual(undefined);
                expect(info.roaming).assertEqual(true);
                expect(info.saveas).assertEqual(undefined);
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual(undefined);
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskConfigRoaming001 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskConfigRoaming001 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskConfigRoaming001 test fail!");
        }
    });

    /**
     * @tc.number: testTaskConfigRoaming002
     * @tc.name: testTaskConfigRoaming002
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     roaming is 1 and other parameters to undefined.
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
    it('testTaskConfigRoaming002', 0, (done: Function) => {
      console.info("===> testTaskConfigRoaming002 test start!");

      let config: Record<string, string | number | undefined> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "title": undefined,
        "description": undefined,
        "mode": undefined,
        "overwrite": undefined,
        "method": undefined,
        "headers": undefined,
        //data: undefined,
        "saveas": undefined,
        "network": undefined,
        "metered": undefined,
        "roaming": 1,
        "retry": undefined,
        "redirect": undefined,
        "proxy": undefined,
        "index": undefined,
        "begins": undefined,
        "ends": undefined,
        "gauge": undefined,
        "precise": undefined,
        "token": undefined,
        "priority": undefined,
        "extras": undefined,
        "multipart": undefined,
        "notification": undefined,
        "minSpeed": undefined,
        "timeout": undefined,
      }

        try {
            let task = await request.agent.create(context, conf as Object as request.agent.Config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual(undefined);
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(typeof info.headers).assertEqual(undefined);
                expect(info.index).assertEqual(undefined);
                expect(info.metered).assertEqual(undefined);
                expect(info.method).assertEqual(undefined);
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(undefined);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(undefined);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(undefined);
                expect(info.redirect).assertEqual(undefined);
                expect(info.retry).assertEqual(undefined);
                expect(info.roaming).assertEqual(1);
                expect(info.saveas).assertEqual(undefined);
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual(undefined);
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskConfigRoaming002 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskConfigRoaming002 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskConfigRoaming002 test fail!");
        }
    });

    /**
     * @tc.number: testTaskConfigRetry001
     * @tc.name: testTaskConfigRetry001
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     retry is true and other parameters to undefined.
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
    it('testTaskConfigRetry001', 0, (done: Function) => {
      console.info("===> testTaskConfigRetry001 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: true,
        redirect: undefined,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
        try {
            let task = await request.agent.create(context, conf as Object as request.agent.Config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual(undefined);
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(typeof info.headers).assertEqual(undefined);
                expect(info.index).assertEqual(undefined);
                expect(info.metered).assertEqual(undefined);
                expect(info.method).assertEqual(undefined);
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(undefined);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(undefined);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(undefined);
                expect(info.redirect).assertEqual(undefined);
                expect(info.retry).assertEqual(true);
                expect(info.roaming).assertEqual(undefined);
                expect(info.saveas).assertEqual(undefined);
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual(undefined);
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskConfigRetry001 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskConfigRetry001 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskConfigRetry001 test fail!");
        }
    });

    /**
     * @tc.number: testTaskConfigRetry002
     * @tc.name: testTaskConfigRetry002
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     retry is 1 and other parameters to undefined.
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
    it('testTaskConfigRetry002', 0, (done: Function) => {
      console.info("===> testTaskConfigRetry002 test start!");

      let config: Record<string, string | number | undefined> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "title": undefined,
        "description": undefined,
        "mode": undefined,
        "overwrite": undefined,
        "method": undefined,
        "headers": undefined,
        //data: undefined,
        "saveas": undefined,
        "network": undefined,
        "metered": undefined,
        "roaming": undefined,
        "retry": 1,
        "redirect": undefined,
        "proxy": undefined,
        "index": undefined,
        "begins": undefined,
        "ends": undefined,
        "gauge": undefined,
        "precise": undefined,
        "token": undefined,
        "priority": undefined,
        "extras": undefined,
        "multipart": undefined,
        "notification": undefined,
        "minSpeed": undefined,
        "timeout": undefined,
      }

        try {
            let task = await request.agent.create(context, conf as Object as request.agent.Config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual(undefined);
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(typeof info.headers).assertEqual(undefined);
                expect(info.index).assertEqual(undefined);
                expect(info.metered).assertEqual(undefined);
                expect(info.method).assertEqual(undefined);
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(undefined);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(undefined);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(undefined);
                expect(info.redirect).assertEqual(undefined);
                expect(info.retry).assertEqual(1);
                expect(info.roaming).assertEqual(undefined);
                expect(info.saveas).assertEqual(undefined);
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual(undefined);
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskConfigRetry002 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskConfigRetry002 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskConfigRetry002 test fail!");
        }
    });

    /**
     * @tc.number: testTaskConfigRedirect001
     * @tc.name: testTaskConfigRedirect001
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     redirect is true and other parameters to undefined.
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
    it('testTaskConfigRedirect001', 0, (done: Function) => {
      console.info("===> testTaskConfigRedirect001 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: true,
        proxy: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }
        try {
            let task = await request.agent.create(context, conf as Object as request.agent.Config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual(undefined);
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(typeof info.headers).assertEqual(undefined);
                expect(info.index).assertEqual(undefined);
                expect(info.metered).assertEqual(undefined);
                expect(info.method).assertEqual(undefined);
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(undefined);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(undefined);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(undefined);
                expect(info.redirect).assertEqual(true);
                expect(info.retry).assertEqual(undefined);
                expect(info.roaming).assertEqual(undefined);
                expect(info.saveas).assertEqual(undefined);
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual(undefined);
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskConfigRedirect001 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskConfigRedirect001 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskConfigRedirect001 test fail!");
        }
    });

    /**
     * @tc.number: testTaskConfigRedirect002
     * @tc.name: testTaskConfigRedirect002
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     redirect is 1 and other parameters to undefined.
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
    it('testTaskConfigRedirect002', 0, (done: Function) => {
      console.info("===> testTaskConfigRedirect002 test start!");

      let config: Record<string, string | number | undefined> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "title": undefined,
        "description": undefined,
        "mode": undefined,
        "overwrite": undefined,
        "method": undefined,
        "headers": undefined,
        //data: undefined,
        "saveas": undefined,
        "network": undefined,
        "metered": undefined,
        "roaming": undefined,
        "retry": undefined,
        "redirect": 1,
        "proxy": undefined,
        "index": undefined,
        "begins": undefined,
        "ends": undefined,
        "gauge": undefined,
        "precise": undefined,
        "token": undefined,
        "priority": undefined,
        "extras": undefined,
        "multipart": undefined,
        "notification": undefined,
        "minSpeed": undefined,
        "timeout": undefined,
      }

        try {
            let task = await request.agent.create(context, conf as Object as request.agent.Config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual(undefined);
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(typeof info.headers).assertEqual(undefined);
                expect(info.index).assertEqual(undefined);
                expect(info.metered).assertEqual(undefined);
                expect(info.method).assertEqual(undefined);
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(undefined);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(undefined);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(undefined);
                expect(info.redirect).assertEqual(1);
                expect(info.retry).assertEqual(undefined);
                expect(info.roaming).assertEqual(undefined);
                expect(info.saveas).assertEqual(undefined);
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual(undefined);
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskConfigSaveas002 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskConfigSaveas002 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskConfigSaveas002 test fail!");
        }
    });

    /**
     * @tc.number: testTaskConfigProxy001
     * @tc.name: testTaskConfigProxy001
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
    proxy to 'http://127.0.0.1:8080', and other parameters to undefined.
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
    it('testTaskConfigProxy001', 0, (done: Function) => {
      console.info("===> testTaskConfigProxy001 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: 'https://127.0.0.1:8080',
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }

      try {
        request.agent.create(context, config, async (err, data) => {
          console.info("===> testTaskConfigProxy001 test fail!");
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(err.code).assertEqual(401);
        console.info("===> testTaskConfigProxy001 test success!");
        done();
      }
    });

    /**
     * @tc.number: testTaskConfigProxy002
     * @tc.name: testTaskConfigProxy002
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     proxy to 1, and other parameters to undefined.
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
    it('testTaskConfigProxy002', 0, (done: Function) => {
      console.info("===> testTaskConfigProxy002 test start!");

      let config: Record<string, string | number | undefined> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "title": undefined,
        "description": undefined,
        "mode": undefined,
        "overwrite": undefined,
        "method": undefined,
        "headers": undefined,
        //data: undefined,
        "saveas": undefined,
        "network": undefined,
        "metered": undefined,
        "roaming": undefined,
        "retry": undefined,
        "redirect": undefined,
        "proxy": 1,
        "index": undefined,
        "begins": undefined,
        "ends": undefined,
        "gauge": undefined,
        "precise": undefined,
        "token": undefined,
        "priority": undefined,
        "extras": undefined,
        "multipart": undefined,
        "notification": undefined,
        "minSpeed": undefined,
        "timeout": undefined,
      }

        try {
            let task = await request.agent.create(context, conf as Object as request.agent.Config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual(undefined);
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(typeof info.headers).assertEqual(undefined);
                expect(info.index).assertEqual(undefined);
                expect(info.metered).assertEqual(undefined);
                expect(info.method).assertEqual(undefined);
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(undefined);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(undefined);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(1);
                expect(info.redirect).assertEqual(undefined);
                expect(info.retry).assertEqual(undefined);
                expect(info.roaming).assertEqual(undefined);
                expect(info.saveas).assertEqual(undefined);
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual(undefined);
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskConfigProxy002 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskConfigProxy002 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskConfigProxy002 test fail!");
        }
    });

    /**
     * @tc.number: testTaskConfigIndex001
     * @tc.name: testTaskConfigIndex001
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
    index to 1, and other parameters to undefined.
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
    it('testTaskConfigIndex001', 0, (done: Function) => {
      console.info("===> testTaskConfigIndex001 test start!");

      let config: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        headers: undefined,
        // data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        proxy: undefined,
        index: 1,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined,
        priority: undefined,
        extras: undefined,
        multipart: undefined,
        notification: undefined,
        minSpeed: undefined,
        timeout: undefined,
      }

        try {
            let task = await request.agent.create(context, conf as Object as request.agent.Config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual(undefined);
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(typeof info.headers).assertEqual(undefined);
                expect(info.index).assertEqual(1);
                expect(info.metered).assertEqual(undefined);
                expect(info.method).assertEqual(undefined);
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(undefined);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(undefined);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(undefined);
                expect(info.redirect).assertEqual(undefined);
                expect(info.retry).assertEqual(undefined);
                expect(info.roaming).assertEqual(undefined);
                expect(info.saveas).assertEqual(undefined);
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual(undefined);
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskConfigIndex001 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskConfigIndex001 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskConfigIndex001 test fail!");
        }
    });

    /**
     * @tc.number: testTaskConfigIndex002
     * @tc.name: testTaskConfigIndex002
     * @tc.desc: Set action to request.agent.Action.DOWNLOAD, url to a normal url,
     index to 'test', and other parameters to undefined.
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
    it('testDownloadConfigIndex002', 0, (done: Function) => {
      console.info("===> testDownloadConfigIndex002 test start!");

      let config: Record<string, string | number | undefined> = {
        "action": request.agent.Action.DOWNLOAD,
        "url": "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
        "title": undefined,
        "description": undefined,
        "mode": undefined,
        "overwrite": undefined,
        "method": undefined,
        "headers": undefined,
        //data: undefined,
        "saveas": undefined,
        "network": undefined,
        "metered": undefined,
        "roaming": undefined,
        "retry": undefined,
        "redirect": undefined,
        "proxy": undefined,
        "index": 'test',
        "begins": undefined,
        "ends": undefined,
        "gauge": undefined,
        "precise": undefined,
        "token": undefined,
        "priority": undefined,
        "extras": undefined,
        "multipart": undefined,
        "notification": undefined,
        "minSpeed": undefined,
        "timeout": undefined,
      }

        try {
            let task = await request.agent.create(context, conf as Object as request.agent.Config);
            task.start().then(async () => {
                let info = task.config;
                expect(info.begins).assertEqual(undefined);
                expect(info.description).assertEqual(undefined);
                expect(info.ends).assertEqual(undefined);
                expect(info.extras).assertEqual(undefined);
                expect(info.gauge).assertEqual(undefined);
                expect(typeof info.headers).assertEqual(undefined);
                expect(info.index).assertEqual('test');
                expect(info.metered).assertEqual(undefined);
                expect(info.method).assertEqual(undefined);
                expect(info.minSpeed).assertEqual(undefined);
                expect(info.mode).assertEqual(undefined);
                expect(info.multipart).assertEqual(undefined);
                expect(info.network).assertEqual(undefined);
                expect(info.notification).assertEqual(undefined);
                expect(info.overwrite).assertEqual(undefined);
                expect(info.precise).assertEqual(undefined);
                expect(info.proxy).assertEqual(undefined);
                expect(info.redirect).assertEqual(undefined);
                expect(info.retry).assertEqual(undefined);
                expect(info.roaming).assertEqual(undefined);
                expect(info.saveas).assertEqual(undefined);
                expect(info.timeout).assertEqual(undefined);
                expect(info.title).assertEqual(undefined);
                expect(info.token).assertEqual(undefined);
                expect(info.url).assertEqual('https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt');
                await request.agent.remove(task.tid);
                console.info("===> testTaskConfigIndex002 test success!");
            }).catch((err: Error) => {
                expect(false).assertTrue();
                console.info("===> testTaskConfigIndex002 test fail!");
            });
        } catch (err) {
            expect(false).assertTrue();
            console.info("===> testTaskConfigIndex002 test fail!");
        }
    });
  })
}