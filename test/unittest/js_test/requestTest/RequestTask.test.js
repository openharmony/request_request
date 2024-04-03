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

import { describe, beforeAll, beforeEach, afterEach, afterAll, it, expect } from 'deccjsunit/index';
import { agent } from '@ohos.request';
import featureAbility from '@ohos.ability.featureAbility'
import fs from '@ohos.file.fs';

describe('RequestTaskTest', function () {
    beforeAll(function () {
        console.info('beforeAll called')
    })

    afterAll(function () {
        console.info('afterAll called')
    })

    beforeEach(function () {
        console.info('beforeEach called')
    })

    afterEach(async function () {
        console.info('afterEach called')
        if (fs.accessSync(cacheDir + '/test.txt')) {
            fs.unlinkSync(cacheDir + '/test.txt')
        }
        if (fs.accessSync(cacheDir + '/test.apk')) {
            fs.unlinkSync(cacheDir + '/test.apk')
        }
        if (task !== undefined) {
            await agent.remove(context, task.tid)
            task = undefined
        }
    })

    function sleep(ms) {
        return new Promise(resolve => setTimeout(resolve, ms))
    }

    let task;
    let context = featureAbility.getContext();
    let cacheDir = '/data/storage/el2/base/haps/entry/files';
    let fileSpec = {
        path: `${cacheDir}/test.txt`
    }
    let formItem = [{
        name: 'test',
        type: `${cacheDir}`,
        value: [fileSpec]
    }]

    function errorParamCreate(conf, code) {
        agent.create(conf, (err) => {
            if (err) {
                expect(err.code).assertEqual(code)
                done()
            } else {
                expect(false).assertTrue();
                done()
            }
        })
    }

    function createLamdaApi10(conf, isError, isNotError) {
        agent.create(conf, (err) => {
            if (err) {
                expect(isError).assertTrue()
                done()
            } else {
                expect(isNotError).assertTrue();
                done()
            }
        })
    }

    async function createApi10Task(conf) {
        task = await agent.create(conf);
        task.start().then(() => {
            expect(true).assertTrue()
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    }

    async function createApi10GetTask(conf) {
        task = await agent.create(conf);
        task.start().then(() => {
            expect(task.conf.method).assertEqual('GET')
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    }

    function wrapTryCatch(conf, code) {
        try {
            errorParamCreate(conf, code)
        } catch (err) {
            expect(err.code).assertEqual(code)
            done()
        }
    }

    let globalConf = {
        action: agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: {
            name: 'test',
            value: {
                path: `${cacheDir}/test.txt`
            },
        }
    }

    function openSyncFile(fileName) {
        let file = fs.openSync(cacheDir + '/' + fileName, fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(file);
    }

    /**
     * @tc.number: testTaskAction001
     * @tc.name: testTaskAction001
     * @tc.desc: Test create task when lack action
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskAction001', function (done) {
        let conf = {
            url: 'http://127.0.0.1',
        }
        expect(true).assertTrue();
        wrapTryCatch(conf, 401);
    })

    /**
     * @tc.number: testTaskAction002
     * @tc.name: testTaskAction002
     * @tc.desc: Test create task when action is string
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskAction002', async function (done) {
        let conf = {
            action: 'UPLOAD',
            url: 'http://127.0.0.1'
        }
        expect(true).assertTrue();
        wrapTryCatch(conf, 401);
    })

    /**
     * @tc.number: testTaskAction003
     * @tc.name: testTaskAction003
     * @tc.desc: Test create task when action is 2
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskAction003', function (done) {
        let conf = {
            action: 2,
            url: 'http://127.0.0.1'
        }
        expect(true).assertTrue();
        createLamdaApi10(conf, true, false)
    })

    /**
     * @tc.number: testTaskAction004
     * @tc.name: testTaskAction004
     * @tc.desc: Test create task when action is UPLOAD
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskAction004', function (done) {
        openSyncFile('test.txt');
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
            data: formItem
        }
        agent.create(conf, async (err, data) => {
            if (err) {
                expect(false).assertTrue()
                done()
            }
            expect(true).assertTrue()
            done()
        })
    })

    /**
     * @tc.number: testTaskAction005
     * @tc.name: testTaskAction005
     * @tc.desc: Test create task when action is DOWNLOAD
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskAction005', function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
            saveas: `${cacheDir}`
        }
        agent.create(conf, async (err, data) => {
            if (err) {
                expect(false).assertTrue()
                done()
            }
            data.on('completed', function (progress) {
                if (fs.accessSync(`${cacheDir}/test.txt`)) {
                    expect(true).assertTrue()
                    done()
                }
            })
        })
    })

    /**
     * @tc.number: testTaskUrl001
     * @tc.name: testTaskUrl001
     * @tc.desc: Test create task when lack url
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskUrl001', function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
        }
        expect(true).assertTrue();
        wrapTryCatch(conf, 401);
    })

    /**
     * @tc.number: testTaskUrl002
     * @tc.name: testTaskUrl002
     * @tc.desc: Test create task when url is empty
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskUrl002', function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: '',
        };
        expect(true).assertTrue();
        createLamdaApi10(conf, true, false);
    })

    /**
     * @tc.number: testTaskUrl003
     * @tc.name: testTaskUrl003
     * @tc.desc: Test create task when url is not support download
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskUrl003', function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/openharmony/request_request',
        }
        expect(true).assertTrue();
        errorParamCreate(conf, 13400003)
    })

    /**
     * @tc.number: testTaskUrl004
     * @tc.name: testTaskUrl004
     * @tc.desc: Test create task when url is not support upload
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskUrl004', function (done) {
        openSyncFile('test.txt');
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'https://gitee.com/openharmony/request_request',
            data: formItem
        }
        expect(true).assertTrue();
        errorParamCreate(conf, 13400003)
    })

    /**
     * @tc.number: testTaskTitle001
     * @tc.name: testTaskTitle001
     * @tc.desc: Test create task when title is given
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskTitle001', async function (done) {
        openSyncFile('test.txt')
        let tmpConf = JSON.parse(JSON.stringify(globalConf));
        tmpConf.title = 'upload test.txt';
        task = await agent.create(tmpConf);
        expect(task.title).assertEqual('upload test.txt')
        done()
    })

    /**
     * @tc.number: testTaskTitle002
     * @tc.name: testTaskTitle002
     * @tc.desc: Test create task when title is number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskTitle002', async function (done) {
        let tmpConf = JSON.parse(JSON.stringify(globalConf));
        tmpConf.title = 123;
        task = await agent.create(tmpConf);
        expect(task.title).assertEqual("")
        done()
    })

    /**
     * @tc.number: testTaskDescription001
     * @tc.name: testTaskDescription001
     * @tc.desc: Test create task when description is given
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskDescription001', async function (done) {
        openSyncFile('test.txt')
        let tmpConf = JSON.parse(JSON.stringify(globalConf));
        tmpConf.description = 'test upload'
        task = await agent.create(tmpConf);
        expect(task.description).assertEqual('test upload')
        expect(task.conf.mode).assertEqual(agent.Mode.BACKGROUND)
        done()
    })

    /**
     * @tc.number: testTaskDescription002
     * @tc.name: testTaskDescription002
     * @tc.desc: Test create task when description is number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskDescription002', async function (done) {
        let tmpConf = JSON.parse(JSON.stringify(globalConf));
        tmpConf.title = 123;
        task = await agent.create(tmpConf);
        expect(task.description).assertEqual("")
        done()
    })

    /**
     * @tc.number: testTaskMode001
     * @tc.name: testTaskMode001
     * @tc.desc: Test create task when mode is FRONTEND
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskMode001', async function (done) {
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
            data: {
                name: 'test',
                value: {
                    path: `${cacheDir}/test.txt`
                },
            },
            mode: agent.Mode.FRONTEND
        }
        task = await agent.create(conf);
        task.start().then(() => {
            agent.remove(context, task.tid)
            expect(true).assertTrue()
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.number: testTaskMode002
     * @tc.name: testTaskMode002
     * @tc.desc: Test create task when mode is BACKGROUND
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskMode002', async function (done) {
        let conf = JSON.parse(JSON.stringify(globalConf));
        conf.mode = agent.Mode.BACKGROUND;
        expect(true).assertTrue();
        await createApi10Task(conf);
    })

    /**
     * @tc.number: testTaskMode003
     * @tc.name: testTaskMode003
     * @tc.desc: Test create task when mode is string
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskMode003', async function (done) {
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
            data: {
                name: 'test',
                value: {
                    path: `${cacheDir}/test.txt`
                },
            },
            mode: "BACKGROUND"
        }
        task = await agent.create(conf);
        task.start().then(() => {
            expect(task.conf.mode).assertEqual(agent.Mode.BACKGROUND)
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.number: testTaskCover001
     * @tc.name: testTaskCover001
     * @tc.desc: Test create task when cover is true and file exists
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskCover001', async function (done) {
        openSyncFile('test.txt')
        let conf = JSON.parse(JSON.stringify(globalConf));
        conf.url = 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt';
        conf.action = agent.Action.DOWNLOAD;
        conf.saveas = 'testTaskCover001.txt';
        conf.cover = true;
        expect(true).assertTrue()
        await createApi10Task(conf);
    })

    /**
     * @tc.number: testTaskCover002
     * @tc.name: testTaskCover002
     * @tc.desc: Test create task when cover is true and file not exists
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskCover002', async function (done) {
        let conf = JSON.parse(JSON.stringify(globalConf));
        conf.url = 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt';
        conf.action = agent.Action.DOWNLOAD;
        conf.saveas = 'testTaskCover002.txt';
        conf.cover = true;
        expect(true).assertTrue()
        await createApi10Task(conf);
    })

    /**
     * @tc.number: testTaskCover003
     * @tc.name: testTaskCover003
     * @tc.desc: Test create task when cover is false and file exists
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskCover003', async function (done) {
        openSyncFile('test.txt')
        let conf = JSON.parse(JSON.stringify(globalConf));
        conf.url = 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt';
        conf.action = agent.Action.DOWNLOAD;
        conf.saveas = 'testTaskCover003.txt';
        conf.cover = false;
        expect(true).assertTrue()
        await createApi10Task(conf);
    })

    /**
     * @tc.number: testTaskCover004
     * @tc.name: testTaskCover004
     * @tc.desc: Test create task when cover is false and file not exists
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskCover004', async function (done) {
        let conf = JSON.parse(JSON.stringify(globalConf));
        conf.url = 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt';
        conf.action = agent.Action.DOWNLOAD;
        conf.saveas = 'testTaskCover004.txt';
        conf.cover = false;
        expect(true).assertTrue()
        await createApi10Task(conf);
    })

    /**
     * @tc.number: testTaskCover005
     * @tc.name: testTaskCover005
     * @tc.desc: Test create task when cover is string
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskCover005', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
            data: {
                name: 'test',
                value: {
                    path: `${cacheDir}/test.txt`
                },
            },
            cover: "true"
        }
        task = await agent.create(conf);
        task.start().then(() => {
            expect(task.conf.cover).assertEqual(true)
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.number: testTaskMethod001
     * @tc.name: testTaskMethod001
     * @tc.desc: Test create task when method is POST for upload
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskMethod001', async function (done) {
        openSyncFile('test.txt');
        let tmpConf = JSON.parse(JSON.stringify(globalConf));
        tmpConf.method = 'POST';
        expect(true).assertTrue()
        await createApi10Task(conf);
    })

    /**
     * @tc.number: testTaskMethod002
     * @tc.name: testTaskMethod002
     * @tc.desc: Test create task when method is POST for download
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskMethod002', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
            saveas: `${cacheDir}/testTaskMethod002.txt`,
            method: 'POST'
        }
        expect(true).assertTrue()
        await createApi10Task(conf);
    })

    /**
     * @tc.number: testTaskMethod003
     * @tc.name: testTaskMethod003
     * @tc.desc: Test create task when method is number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskMethod003', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
            saveas: `${cacheDir}/testTaskMethod003.txt`,
            method: 123
        }
        expect(true).assertTrue()
        await createApi10GetTask(conf)
    })

    /**
     * @tc.number: testTaskMethod004
     * @tc.name: testTaskMethod004
     * @tc.desc: Test create task when method is empty
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskMethod004', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
            saveas: `${cacheDir}/testTaskMethod004.txt`,
            method: ''
        }
        expect(true).assertTrue()
        await createApi10GetTask(conf)
    })

    /**
     * @tc.number: testTaskMethod005
     * @tc.name: testTaskMethod005
     * @tc.desc: Test create task when method is GET for upload
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskMethod005', async function (done) {
        openSyncFile('test.txt');
        let tmpConf = JSON.parse(JSON.stringify(globalConf));
        tmpConf.method = 'GET';
        expect(true).assertTrue()
        await createApi10Task(conf);
    })

    /**
     * @tc.number: testTaskMethod006
     * @tc.name: testTaskMethod006
     * @tc.desc: Test create task when method is PUT for download
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskMethod006', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
            saveas: `${cacheDir}/testTaskMethod006.txt`,
            method: 'PUT'
        }
        expect(true).assertTrue()
        await createApi10Task(conf);
    })

    /**
     * @tc.number: testTaskHeaders001
     * @tc.name: testTaskHeaders001
     * @tc.desc: Test create task when headers content-type is application/json but data is file for upload
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskHeaders001', async function (done) {
        openSyncFile('test.txt');
        let tmpConf = JSON.parse(JSON.stringify(globalConf));
        tmpConf.headers = JSON.stringify({ 'content-type': 'application/json' });
        expect(true).assertTrue()
        await createApi10Task(conf);
    })

    /**
     * @tc.number: testTaskHeaders002
     * @tc.name: testTaskHeaders002
     * @tc.desc: Test create task when lack headers for upload
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskHeaders002', async function (done) {
        openSyncFile('test.txt');
        let tmpConf = JSON.parse(JSON.stringify(globalConf));
        task = await agent.create(tmpConf);
        task.start().then(() => {
            expect(task.conf.headers).assertEqual('multipart/form-data')
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.number: testTaskHeaders003
     * @tc.name: testTaskHeaders003
     * @tc.desc: Test create task when lack headers for download
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskHeaders003', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
            saveas: 'testTaskHeaders003.txt',
        }
        task = await agent.create(conf);
        task.start().then(() => {
            expect(task.conf.headers).assertEqual('application/json')
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.number: testTaskSaveas001
     * @tc.name: testTaskSaveas001
     * @tc.desc: Test create task when lack saveas is number for download
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskSaveas001', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
            saveas: 123
        }
        task = await agent.create(conf);
        task.on('completed', function () {
            if (fs.accessSync(`${cacheDir}/test.txt`)) {
                expect(true).assertTrue()
                done()
            }
        })
        task.start()
    })

    /**
     * @tc.number: testTaskData001
     * @tc.name: testTaskData001
     * @tc.desc: Test create task when data lack name
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskData001', function (done) {
        openSyncFile('test.txt');
        let conf = JSON.parse(JSON.stringify(globalConf));
        conf.data = {
            value: {
                path: `${cacheDir}/test.txt`
            },
        };
        expect(true).assertTrue()
        wrapTryCatch(conf, 401);
    })

    /**
     * @tc.number: testTaskData002
     * @tc.name: testTaskData002
     * @tc.desc: Test create task when data name is number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskData002', function (done) {
        openSyncFile('test.txt');
        let conf = JSON.parse(JSON.stringify(globalConf));
        conf.data.name = 123;
        expect(true).assertTrue()
        wrapTryCatch(conf, 401);
    })

    /**
     * @tc.number: testTaskData003
     * @tc.name: testTaskData003
     * @tc.desc: Test create task when data lack value
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskData003', function (done) {
        openSyncFile('test.txt');
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
            data: {
                name: 'test'
            }
        }
        expect(true).assertTrue()
        wrapTryCatch(conf, 401);
    })

    /**
     * @tc.number: testTaskData004
     * @tc.name: testTaskData004
     * @tc.desc: Test create task when data value is number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskData004', function (done) {
        openSyncFile('test.txt');
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
            data: {
                name: 'test',
                value: 123
            }
        }
        expect(true).assertTrue()
        wrapTryCatch(conf, 401);
    })

    /**
     * @tc.number: testTaskData005
     * @tc.name: testTaskData005
     * @tc.desc: Test create task when data path is '', path is not exits
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskData005', function (done) {
        openSyncFile('test.txt');
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
            data: {
                name: 'test',
                value: {
                    path: ''
                }
            }
        }
        expect(true).assertTrue()
        wrapTryCatch(conf, 401);
    })

    /**
     * @tc.number: testTaskData006
     * @tc.name: testTaskData006
     * @tc.desc: Test create task when data path is number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskData006', function (done) {
        openSyncFile('test.txt');
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
            data: {
                name: 'test',
                value: {
                    path: 123
                }
            }
        }
        expect(true).assertTrue()
        wrapTryCatch(conf, 401);
    })


    /**
     * @tc.number: testTaskData007
     * @tc.name: testTaskData007
     * @tc.desc: Test create task when data path is not access permission
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskData007', function (done) {
        openSyncFile('test.txt');
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
            data: {
                name: 'test',
                value: {
                    path: 'system/etc/init.cfg'
                }
            }
        }
        expect(true).assertTrue()
        errorParamCreate(conf, 13400001);
    })

    /**
     * @tc.number: testTaskData008
     * @tc.name: testTaskData008
     * @tc.desc: Test create task when data filename is number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskData008', function (done) {
        openSyncFile('test.txt');
        let conf = JSON.parse(JSON.stringify(globalConf));
        conf.data.value.fileName = 123;
        expect(true).assertTrue()
        createLamdaApi10(conf, false, true)
    })


    /**
     * @tc.number: testTaskData009
     * @tc.name: testTaskData009
     * @tc.desc: Test create task when data mimetype is number
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskData009', function (done) {
        openSyncFile('test.txt');
        let conf = JSON.parse(JSON.stringify(globalConf));
        conf.data.value.mimetype = 123;
        expect(true).assertTrue()
        createLamdaApi10(conf, false, true)
    })

    /**
     * @tc.number: testTaskData010
     * @tc.name: testTaskData010
     * @tc.desc: Test create task when data path and filename is different
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskData010', function (done) {
        openSyncFile('test.txt');
        let conf = JSON.parse(JSON.stringify(globalConf));
        conf.data.value.fileName = 'a.txt';
        expect(true).assertTrue()
        createLamdaApi10(conf, false, true)
    })

    /**
     * @tc.number: testTaskData011
     * @tc.name: testTaskData011
     * @tc.desc: Test create task when data two files for upload
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskData011', function (done) {
        openSyncFile('test.txt');
        let conf = JSON.parse(JSON.stringify(globalConf));
        conf.data.value = [
            {
                path: `${cacheDir}/test.txt`,
            },
            {
                path: `${cacheDir}/test.txt`,
            },
        ];
        expect(true).assertTrue()
        createLamdaApi10(conf, false, true)
    })

    /**
     * @tc.number: testTaskData012
     * @tc.name: testTaskData012
     * @tc.desc: Test create task when data value is string
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskData012', function (done) {
        openSyncFile('test.txt');
        let conf = JSON.parse(JSON.stringify(globalConf));
        conf.data.value = 'test';
        expect(true).assertTrue()
        createLamdaApi10(conf, false, true)
    })

    /**
     * @tc.number: testTaskData013
     * @tc.name: testTaskData013
     * @tc.desc: Test create task when data path and filename is same
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskData013', function (done) {
        openSyncFile('test.txt');
        let conf = JSON.parse(JSON.stringify(globalConf));
        conf.data.value.fileName = 'test.txt';
        expect(true).assertTrue()
        createLamdaApi10(conf, false, true)
    })

    /**
     * @tc.number: testTaskNetwork001
     * @tc.name: testTaskNetwork001
     * @tc.desc: Test create task when network is 3
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskNetwork001', async function (done) {
        openSyncFile('test.txt');
        let conf = JSON.parse(JSON.stringify(globalConf));
        conf.network = 3;
        task.create(context, conf).then(() => {
            expect(true).assertTrue()
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.number: testTaskNetwork002
     * @tc.name: testTaskNetwork002
     * @tc.desc: Test create task when network is string
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskNetwork002', async function (done) {
        let conf = JSON.parse(JSON.stringify(globalConf));
        conf.network = "ANY";
        task.create(context, conf).then(() => {
            expect(true).assertTrue()
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.number: testTaskNetwork003
     * @tc.name: testTaskNetwork003
     * @tc.desc: Test create task when network is WIFI for DOWNLOAD
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskNetwork003', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
            saveas: 'testTaskNetwork003.txt',
            network: agent.NetWork.WIFI
        }
        task.create(context, conf).then(() => {
            expect(true).assertTrue()
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.number: testTaskNetwork004
     * @tc.name: testTaskNetwork004
     * @tc.desc: Test create task when network is WIFI for UPLOAD
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskNetwork004', async function (done) {
        let conf = JSON.parse(JSON.stringify(globalConf));
        conf.network = agent.NetWork.WIFI;
        expect(true).assertTrue()
        await createApi10Task(conf);
    })

    /**
     * @tc.number: testTaskRetry001
     * @tc.name: testTaskRetry001
     * @tc.desc: Test create task when retry is true for frontend
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskRetry001', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'xxx',
            mode: agent.Mode.FRONTEND,
            retry: true
        }
        task = await agent.create(context, conf);
        task.on('failed', function () {
            expect(true).assertTrue()
            done()
        })
        task.start()
    })

    /**
     * @tc.number: testTaskRetry002
     * @tc.name: testTaskRetry002
     * @tc.desc: Test create task when retry is true for background
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskRetry002', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
            mode: agent.Mode.BACKGROUND,
            saveas: 'testTaskRetry002.txt',
            retry: true
        }
        task = await agent.create(conf);
        task.on('progress', function (progress) {
            if (progress.state === agent.State.RETRYING) {
                expect(true).assertTrue()
                done()
            }
        })
        task.start()
    })

    /**
     * @tc.number: testTaskRetry003
     * @tc.name: testTaskRetry003
     * @tc.desc: Test create task when retry is string
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskRetry003', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'xxx',
            mode: agent.Mode.FRONTEND,
            retry: 'true'
        }
        task = await agent.create(context, conf);
        task.on('failed', function () {
            expect(true).assertTrue()
            done()
        })
        task.start()
    })

    /**
     * @tc.number: testTaskRetry004
     * @tc.name: testTaskRetry004
     * @tc.desc: Test create task when retry is false for frontend
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskRetry004', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test1.txt',
            mode: agent.Mode.FRONTEND,
            retry: false
        }
        task = await agent.create(conf);
        task.on('failed', function () {
            expect(true).assertTrue()
            done()
        })
        task.start()
    })

    /**
     * @tc.number: testTaskRetry005
     * @tc.name: testTaskRetry005
     * @tc.desc: Test create task when retry is false for background
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.require:
     */
    it('testTaskRetry005', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test1.txt',
            mode: agent.Mode.FRONTEND,
            retry: false
        }
        task.on('failed', function () {
            expect(true).assertTrue()
            done()
        })
        task = await agent.create(conf);
        task.start()
    })
})