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

import {describe, beforeAll, beforeEach, afterEach, afterAll, it, expect} from 'deccjsunit/index';
import { agent } from '@ohos.request';
import featureAbility from '@ohos.ability.featureAbility'
import fs from '@ohos.file.fs';

describe('RequestCreateTaskTest', function () {
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
        value: [ fileSpec ]
    }]

    /**
     * @tc.name: testTaskAction001
     * @tc.desc: Test create task when lack action
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskAction001', function (done)  {
        let conf = {
            url: 'http://127.0.0.1',
        }
        try {
            agent.create(context, conf, (err) => {
                if (err) {
                    expect(err.code).assertEqual(401)
                    done()
                } else {
                    expect(false).assertTrue();
                    done()
                }
            })
        } catch (err) {
            expect(err.code).assertEqual(401)
            done()
        }
    })

    /**
     * @tc.name: testTaskAction002
     * @tc.desc: Test create task when action is string
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskAction002', async function (done)  {
        let conf = {
            action: 'UPLOAD',
            url: 'http://127.0.0.1'
        }
        try {
            agent.create(context, conf, (err) => {
                if (err) {
                    expect(err.code).assertEqual(401)
                    done()
                } else {
                    expect(false).assertTrue();
                    done()
                }
            })
        } catch (err) {
            expect(err.code).assertEqual(401)
            done()
        }
    })

    /**
     * @tc.name: testTaskAction003
     * @tc.desc: Test create task when action is 2
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskAction003', function (done)  {
        let conf = {
            action: 2,
            url: 'http://127.0.0.1'
        }
        agent.create(context, conf, (err) => {
            if (err) {
                expect(true).assertTrue()
                done()
            } else {
                expect(false).assertTrue();
                done()
            }
        })
    })

    /**
     * @tc.name: testTaskAction004
     * @tc.desc: Test create task when action is UPLOAD
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskAction004',  function (done)  {
        let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(file);
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
            data: formItem
        }
        agent.create(context, conf, async (err, data) => {
            if (err) {
                expect(false).assertTrue()
                done()
            }
            expect(true).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testTaskAction005
     * @tc.desc: Test create task when action is DOWNLOAD
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskAction005', function (done)  {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            gauge: true
        }
        agent.create(context, conf, async (err, data) => {
            if (err) {
                expect(false).assertTrue()
                done()
            }
            data.on('completed', function (progress) {
                if (fs.accessSync(`${cacheDir}/test.apk`)) {
                    expect(true).assertTrue()
                    done()
                }
            })
        })
    })

    /**
     * @tc.name: testTaskUrl001
     * @tc.desc: Test create task when lack url
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskUrl001', function (done)  {
        let conf = {
            action: agent.Action.DOWNLOAD,
        }
        try {
            agent.create(context, conf, (err) => {
                if (err) {
                    expect(err.code).assertEqual(401)
                    done()
                } else {
                    expect(false).assertTrue();
                    done()
                }
            })
        } catch (err) {
            expect(err.code).assertEqual(401)
            done()
        }
    })

    /**
     * @tc.name: testTaskUrl002
     * @tc.desc: Test create task when url is empty
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskUrl002', function (done)  {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: '',
        }
        agent.create(context, conf, (err) => {
            if (err) {
                expect(true).assertTrue()
                done()
            } else {
                expect(false).assertTrue();
                done()
            }
        })
    })

    /**
     * @tc.name: testTaskUrl003
     * @tc.desc: Test create task when url is not support download
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskUrl003', function (done)  {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/openharmony/request_request',
        }
        agent.create(context, conf, (err) => {
            if (err) {
                expect(err.code).assertEqual(13400003)
                done()
            } else {
                expect(false).assertTrue();
                done()
            }
        })
    })

    /**
     * @tc.name: testTaskUrl004
     * @tc.desc: Test create task when url is not support upload
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskUrl004', function (done)  {
        let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(file);
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'https://gitee.com/openharmony/request_request',
            data: formItem
        }
        agent.create(context, conf, (err) => {
            if (err) {
                expect(err.code).assertEqual(13400003)
                done()
            } else {
                expect(false).assertTrue();
                done()
            }
        })
    })

    /**
     * @tc.name: testTaskTitle001
     * @tc.desc: Test create task when title is given
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskTitle001', async function (done) {
        let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(file);
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
            data: {
                name: 'test',
                value: {
                    path: `${cacheDir}/test.txt`
                },
            },
            title: 'upload test.txt'
        }
        task = await agent.create(context, conf);
        expect(task.title).assertEqual('upload test.txt')
        done()
    })

    /**
     * @tc.name: testTaskTitle002
     * @tc.desc: Test create task when title is number
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskTitle002', async function (done) {
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
            data: {
                name: 'test',
                value: {
                    path: `${cacheDir}/test.txt`
                },
            },
            title: 123
        }
        task = await agent.create(context, conf);
        expect(task.title).assertEqual("")
        done()
    })

    /**
     * @tc.name: testTaskDescription001
     * @tc.desc: Test create task when description is given
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskDescription001', async function (done) {
        let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(file);
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
            data: {
                name: 'test',
                value: {
                    path: `${cacheDir}/test.txt`
                },
            },
            description: 'test upload'
        }
        task = await agent.create(context, conf);
        expect(task.description).assertEqual('test upload')
        expect(task.conf.mode).assertEqual(agent.Mode.BACKGROUND)
        done()
    })

    /**
     * @tc.name: testTaskDescription002
     * @tc.desc: Test create task when description is number
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskDescription002', async function (done) {
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
            data: {
                name: 'test',
                value: {
                    path: `${cacheDir}/test.txt`
                },
            },
            title: 123
        }
        task = await agent.create(context, conf);
        expect(task.description).assertEqual("")
        done()
    })

    /**
     * @tc.name: testTaskMode001
     * @tc.desc: Test create task when mode is FRONTEND
     * @tc.type: FUNC
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
        task = await agent.create(context, conf);
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
     * @tc.name: testTaskMode002
     * @tc.desc: Test create task when mode is BACKGROUND
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskMode002', async function (done) {
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
            data: {
                name: 'test',
                value: {
                    path: `${cacheDir}/test.txt`
                },
            },
            mode: agent.Mode.BACKGROUND
        }
        task = await agent.create(context, conf);
        task.start().then(() => {
            expect(true).assertTrue()
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testTaskMode003
     * @tc.desc: Test create task when mode is string
     * @tc.type: FUNC
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
        task = await agent.create(context, conf);
        task.start().then(() => {
            expect(task.conf.mode).assertEqual(agent.Mode.BACKGROUND)
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testTaskCover001
     * @tc.desc: Test create task when cover is true and file exists
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskCover001', async function (done) {
        let file = fs.openSync(cacheDir + '/test.apk', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(file);
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            data: {
                name: 'test',
                value: {
                    path: `${cacheDir}/test.apk`
                },
            },
            cover: true
        }
        task = await agent.create(context, conf);
        task.start().then(() => {
            expect(true).assertTrue()
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testTaskCover002
     * @tc.desc: Test create task when cover is true and file not exists
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskCover002', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            data: {
                name: 'test',
                value: {
                    path: `${cacheDir}/test.apk`
                },
            },
            cover: true
        }
        task = await agent.create(context, conf);
        task.start().then(() => {
            expect(true).assertTrue()
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testTaskCover003
     * @tc.desc: Test create task when cover is false and file exists
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskCover003', async function (done) {
        let file = fs.openSync(cacheDir + '/test.apk', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(file);
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            data: {
                name: 'test',
                value: {
                    path: `${cacheDir}/test.apk`
                },
            },
            cover: false
        }
        task = await agent.create(context, conf);
        task.start().then(() => {
            expect(true).assertTrue()
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testTaskCover004
     * @tc.desc: Test create task when cover is false and file not exists
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskCover004', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            data: {
                name: 'test',
                value: {
                    path: `${cacheDir}/test.apk`
                },
            },
            cover: false
        }
        task = await agent.create(context, conf);
        task.start().then(() => {
            expect(true).assertTrue()
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testTaskCover005
     * @tc.desc: Test create task when cover is string
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskCover005', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            data: {
                name: 'test',
                value: {
                    path: `${cacheDir}/test.apk`
                },
            },
            cover: "true"
        }
        task = await agent.create(context, conf);
        task.start().then(() => {
            expect(task.conf.cover).assertEqual(true)
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testTaskMethod001
     * @tc.desc: Test create task when method is POST for upload
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskMethod001', async function (done) {
        let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(file);
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
            data: {
                name: 'test',
                value: {
                    path: `${cacheDir}/test.txt`
                },
            },
            method: 'POST'
        }
        task = await agent.create(context, conf);
        task.start().then(() => {
            expect(true).assertTrue()
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testTaskMethod002
     * @tc.desc: Test create task when method is POST for download
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskMethod002', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            method: 'POST'
        }
        task = await agent.create(context, conf);
        task.start().then(() => {
            expect(true).assertTrue()
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testTaskMethod003
     * @tc.desc: Test create task when method is number
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskMethod003', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            method: 123
        }
        task = await agent.create(context, conf);
        task.start().then(() => {
            expect(task.conf.method).assertEqual('GET')
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testTaskMethod004
     * @tc.desc: Test create task when method is empty
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskMethod004', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            method: ''
        }
        task = await agent.create(context, conf);
        task.start().then(() => {
            expect(task.conf.method).assertEqual('GET')
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testTaskMethod005
     * @tc.desc: Test create task when method is GET for upload
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskMethod005', async function (done) {
        let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(file);
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
            data: {
                name: 'test',
                value: {
                    path: `${cacheDir}/test.txt`
                },
            },
            method: 'GET'
        }
        task = await agent.create(context, conf);
        task.start().then(() => {
            expect(true).assertTrue()
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testTaskMethod006
     * @tc.desc: Test create task when method is PUT for download
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskMethod006', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            method: 'PUT'
        }
        task = await agent.create(context, conf);
        task.start().then(() => {
            expect(true).assertTrue()
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testTaskHeaders001
     * @tc.desc: Test create task when headers content-type is application/json but data is file for upload
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskHeaders001', async function (done) {
        let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(file);
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
            data: {
                name: 'test',
                value: {
                    path: `${cacheDir}/test.txt`
                },
            },
            headers: JSON.stringify({'content-type': 'application/json'}),
        }
        task = await agent.create(context, conf);
        task.start().then(() => {
            expect(true).assertTrue()
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testTaskHeaders002
     * @tc.desc: Test create task when lack headers for upload
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskHeaders002', async function (done) {
        let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(file);
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
            data: {
                name: 'test',
                value: {
                    path: `${cacheDir}/test.txt`
                },
            },
        }
        task = await agent.create(context, conf);
        task.start().then(() => {
            expect(task.conf.headers).assertEqual('multipart/form-data')
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testTaskHeaders003
     * @tc.desc: Test create task when lack headers for download
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskHeaders003', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
        }
        task = await agent.create(context, conf);
        task.start().then(() => {
            expect(task.conf.headers).assertEqual('application/json')
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testTaskSaveas001
     * @tc.desc: Test create task when lack saveas is number for download
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskSaveas001', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            mode: agent.Mode.FRONTEND,
            saveas: 123
        }
        task = await agent.create(context, conf);
        task.on('completed', function() {
            if (fs.accessSync(`${cacheDir}/test.apk`)) {
                expect(true).assertTrue()
                done()
            }
        })
        task.start()
    })

    /**
     * @tc.name: testTaskData001
     * @tc.desc: Test create task when data lack name
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskData001', function (done)  {
        let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(file);
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
            data: {
                value: {
                    path: `${cacheDir}/test.txt`
                },
            }
        }
        try {
            agent.create(context, conf, (err) => {
                if (err) {
                    expect(err.code).assertEqual(401)
                    done()
                } else {
                    expect(false).assertTrue();
                    done()
                }
            })
        } catch (err) {
            expect(err.code).assertEqual(401)
            done()
        }
    })

    /**
     * @tc.name: testTaskData002
     * @tc.desc: Test create task when data name is number
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskData002', function (done)  {
        let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(file);
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
            data: {
                name: 123,
                value: {
                    path: `${cacheDir}/test.txt`
                },
            }
        }
        try {
            agent.create(context, conf, (err) => {
                if (err) {
                    expect(err.code).assertEqual(401)
                    done()
                } else {
                    expect(false).assertTrue();
                    done()
                }
            })
        } catch (err) {
            expect(err.code).assertEqual(401)
            done()
        }
    })

    /**
     * @tc.name: testTaskData003
     * @tc.desc: Test create task when data lack value
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskData003', function (done)  {
        let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(file);
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
            data: {
                name: 'test'
            }
        }
        try {
            agent.create(context, conf, (err) => {
                if (err) {
                    expect(err.code).assertEqual(401)
                    done()
                } else {
                    expect(false).assertTrue();
                    done()
                }
            })
        } catch (err) {
            expect(err.code).assertEqual(401)
            done()
        }
    })

    /**
     * @tc.name: testTaskData004
     * @tc.desc: Test create task when data value is number
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskData004', function (done)  {
        let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(file);
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
            data: {
                name: 'test',
                value: 123
            }
        }
        try {
            agent.create(context, conf, (err) => {
                if (err) {
                    expect(err.code).assertEqual(401)
                    done()
                } else {
                    expect(false).assertTrue();
                    done()
                }
            })
        } catch (err) {
            expect(err.code).assertEqual(401)
            done()
        }
    })

    /**
     * @tc.name: testTaskData005
     * @tc.desc: Test create task when data path is '', path is not exits
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskData005', function (done)  {
        let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(file);
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
        try {
            agent.create(context, conf, (err) => {
                if (err) {
                    expect(err.code).assertEqual(401)
                    done()
                } else {
                    expect(false).assertTrue();
                    done()
                }
            })
        } catch (err) {
            expect(err.code).assertEqual(401)
            done()
        }
    })

    /**
     * @tc.name: testTaskData006
     * @tc.desc: Test create task when data path is number
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskData006', function (done)  {
        let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(file);
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
        try {
            agent.create(context, conf, (err) => {
                if (err) {
                    expect(err.code).assertEqual(401)
                    done()
                } else {
                    expect(false).assertTrue();
                    done()
                }
            })
        } catch (err) {
            expect(err.code).assertEqual(401)
            done()
        }
    })


    /**
     * @tc.name: testTaskData007
     * @tc.desc: Test create task when data path is not access permission
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskData007', function (done)  {
        let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(file);
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
        agent.create(context, conf, (err) => {
            if (err) {
                expect(err.code).assertEqual(13400001)
                done()
            } else {
                expect(false).assertTrue();
                done()
            }
        })
    })

    /**
     * @tc.name: testTaskData008
     * @tc.desc: Test create task when data filename is number
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskData008', function (done)  {
        let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(file);
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
            data: {
                name: 'test',
                value: {
                    path: `${cacheDir}/test.txt`,
                    filename: 123
                }
            }
        }
        agent.create(context, conf, (err) => {
            if (err) {
                expect(false).assertTrue()
                done()
            } else {
                expect(true).assertTrue();
                done()
            }
        })
    })


    /**
     * @tc.name: testTaskData009
     * @tc.desc: Test create task when data mimetype is number
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskData009', function (done)  {
        let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(file);
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
            data: {
                name: 'test',
                value: {
                    path: `${cacheDir}/test.txt`,
                    mimetype: 123
                }
            }
        }
        agent.create(context, conf, (err) => {
            if (err) {
                expect(false).assertTrue()
                done()
            } else {
                expect(true).assertTrue();
                done()
            }
        })
    })

    /**
     * @tc.name: testTaskData010
     * @tc.desc: Test create task when data path and filename is different
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskData010', function (done)  {
        let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(file);
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
            data: {
                name: 'test',
                value: {
                    path: `${cacheDir}/test.txt`,
                    filename: 'a.txt'
                }
            }
        }
        agent.create(context, conf, (err) => {
            if (err) {
                expect(false).assertTrue()
                done()
            } else {
                expect(true).assertTrue();
                done()
            }
        })
    })

    /**
     * @tc.name: testTaskData011
     * @tc.desc: Test create task when data two files for upload
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskData011', function (done)  {
        let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(file);
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
            data: {
                name: 'test',
                value: [
                    {
                        path: `${cacheDir}/test.txt`,
                    },
                    {
                        path: `${cacheDir}/test.txt`,
                    },
                ]
            }
        }
        agent.create(context, conf, (err) => {
            if (err) {
                expect(false).assertTrue()
                done()
            } else {
                expect(true).assertTrue();
                done()
            }
        })
    })

    /**
     * @tc.name: testTaskData012
     * @tc.desc: Test create task when data value is string
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskData012', function (done)  {
        let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(file);
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
            data: {
                name: 'test',
                value: 'test'
            }
        }
        agent.create(context, conf, (err) => {
            if (err) {
                expect(false).assertTrue()
                done()
            } else {
                expect(true).assertTrue();
                done()
            }
        })
    })

    /**
     * @tc.name: testTaskData013
     * @tc.desc: Test create task when data path and filename is same
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskData013', function (done)  {
        let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(file);
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
            data: {
                name: 'test',
                value: {
                    path: `${cacheDir}/test.txt`,
                    filename: 'test.txt'
                }
            }
        }
        agent.create(context, conf, (err) => {
            if (err) {
                expect(false).assertTrue()
                done()
            } else {
                expect(true).assertTrue();
                done()
            }
        })
    })

    /**
     * @tc.name: testTaskNetwork001
     * @tc.desc: Test create task when network is 3
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskNetwork001', async function (done) {
        let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(file);
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
            data: {
                name: 'test',
                value: {
                    path: `${cacheDir}/test.txt`,
                }
            },
            network: 3
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
     * @tc.name: testTaskNetwork002
     * @tc.desc: Test create task when network is string
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskNetwork002', async function (done) {
        let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(file);
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
            data: {
                name: 'test',
                value: {
                    path: `${cacheDir}/test.txt`,
                }
            },
            network: "ANY"
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
     * @tc.name: testTaskNetwork003
     * @tc.desc: Test create task when network is WIFI for DOWNLOAD
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskNetwork003', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
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
     * @tc.name: testTaskNetwork004
     * @tc.desc: Test create task when network is any for UPLOAD
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskNetwork004', async function (done) {
        let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(file);
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
            data: {
                name: 'test',
                value: {
                    path: `${cacheDir}/test.txt`,
                }
            },
            network: agent.NetWork.WIFI
        }

        task = await agent.create(context,  conf);
        task.start().then(() => {
            expect(true).assertTrue()
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testTaskRetry001
     * @tc.desc: Test create task when retry is true for frontend
     * @tc.type: FUNC
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
        task.start().then(() => {
            expect(true).assertTrue()
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testTaskRetry002
     * @tc.desc: Test create task when retry is true for background
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskRetry002', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            mode: agent.Mode.BACKGROUND,
            retry: true,
            gauge: true
        }
        task = await agent.create(context, conf);
        task.start().then(() => {
            expect(true).assertTrue()
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testTaskRetry003
     * @tc.desc: Test create task when retry is string
     * @tc.type: FUNC
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
     * @tc.name: testTaskRetry004
     * @tc.desc: Test create task when retry is false for frontend
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskRetry004', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test1.apk',
            mode: agent.Mode.FRONTEND,
            retry: false
        }
        task = await agent.create(context, conf);
        task.on('failed', function () {
            expect(true).assertTrue()
            done()
        })
        task.start()
    })

    /**
     * @tc.name: testTaskRetry005
     * @tc.desc: Test create task when retry is false for background
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskRetry005', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test1.apk',
            mode: agent.Mode.FRONTEND,
            retry: false
        }
        task.on('failed', function () {
            expect(true).assertTrue()
            done()
        })
        task = await agent.create(context, conf);
        task.start()
    })

    /**
     * @tc.name: testTaskIndex001
     * @tc.desc: Test create task when index is string
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskIndex001', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            index: '0',
            mode: agent.Mode.FRONTEND
        }
        task = await agent.create(context, conf);
        task.on('completed', function() {
            expect(fs.statSync(`${cacheDir}/test.apk`).size() >= 1042000).assertTrue()
            done()
        })
        task.start()
    })

    /**
     * @tc.name: testTaskIndex002
     * @tc.desc: Test create task when index is 0 and begins greater than ends
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskIndex002', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            index: 0,
            begins: 10,
            ends: 5
        }
        try {
            task = await agent.create(context, conf);
            task.start().then(() => {
                expect(false).assertTrue()
                done()
            }).catch(async (err) => {
                expect(err.code).assertEqual(401)
                done()
            })
        } catch (err) {
            expect(err.code).assertEqual(401)
            done()
        }
    })

    /**
     * @tc.name: testTaskIndex003
     * @tc.desc: Test create task when index is 1 but only one file for upload
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskIndex003', async function (done) {
        let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(file);
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
            index: 1
        }
        try {
            task = await agent.create(context, conf);
            task.start().then(() => {
                expect(false).assertTrue()
                done()
            }).catch(async (err) => {
                expect(err.code).assertEqual(401)
                done()
            })
        } catch (err) {
            expect(err.code).assertEqual(401)
            done()
        }
    })

    /**
     * @tc.name: testTaskIndex004
     * @tc.desc: Test create task when index is 0 and begins is 5 and ends is 10 for download
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskIndex004', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            mode: agent.Mode.FRONTEND,
            data: [
                {
                    path: `${cacheDir}`,
                    filename: 'test.apk'
                },
            ],
            begins: 5,
            ends: 10,
            gauge: true
        }
        task = await agent.create(context, conf);
        await task.start()
        task.on('completed', function(err, progress) {
            expect(progress.state).assertEqual(agent.State.COMPLETED)
            expect(progress.index).assertEqual(0)
            expect(progress.processed).assertEqual(5)
            expect(progress.sizes[0]).assertEqual(5)
            done()
        })
    })

    /**
     * @tc.name: testTaskIndex005
     * @tc.desc: Test create task when index is 0 and begins is 5 and ends is 10 for upload
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskIndex005', async function (done) {
        let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(file);
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
            data: {
                name: 'test',
                value: {
                    path: `${cacheDir}/test.txt`,
                }
            },
            begins: 5,
            ends: 10
        }
        task = await agent.create(context, conf);
        task.start().then(() => {
            expect(true).assertTrue()
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testTaskIndex006
     * @tc.desc: Test create task when index is 0 and begins is 5 and ends is not exists for download
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskIndex006', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            mode: agent.Mode.FRONTEND,
            begins: 5,
            gauge: true
        }
        task = await agent.create(context, conf);
        await task.start()
        task.on('completed', function(err, progress) {
            expect(progress.sizes[0]).assertLess(1042000)
            done()
        })
    })

    /**
     * @tc.name: testTaskIndex007
     * @tc.desc: Test create task when index is 0 and begins is 5 and ends is not exists for upload
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskIndex007', async function (done) {
        let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(file);
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
            data: {
                name: 'test',
                value: {
                    path: `${cacheDir}/test.txt`,
                }
            },
            begins: 5,
        }
        task = await agent.create(context, conf);
        task.start().then(() => {
            expect(true).assertTrue()
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testTaskIndex008
     * @tc.desc: Test create task when index is 0 and begins is not exists and ends is 10 for download
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskIndex008', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            mode: agent.Mode.FRONTEND,
            ends: 10,
            gauge: true
        }
        task = await agent.create(context, conf);
        await task.start()
        task.on('completed', function(err, progress) {
            expect(progress.sizes[0]).assertLess(15)
            done()
        })
    })

    /**
     * @tc.name: testTaskIndex009
     * @tc.desc: Test create task when index is 0 and begins is 5 and ends is not exists for upload
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskIndex009', async function (done) {
        let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(file);
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
            data: {
                name: 'test',
                value: {
                    path: `${cacheDir}/test.txt`,
                }
            },
            ends: 10,
        }
        task = await agent.create(context, conf);
        task.start().then(() => {
            expect(true).assertTrue()
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testTaskIndex009
     * @tc.desc: Test create task when index is 1 but have two files for upload
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskIndex009', async function (done) {
        let test1 = fs.openSync(cacheDir + '/test1.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(test1);
        let test2 = fs.openSync(cacheDir + '/test2.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(test2);
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
            data: [
                {
                    path: `${cacheDir}`,
                    filename: 'test1.txt'
                },
                {
                    path: `${cacheDir}`,
                    filename: 'test2.txt'
                },
            ],
            index: 1,
        }
        task = await agent.create(context, conf);
        task.start().then(() => {
            fs.unlinkSync(cacheDir + '/test1.txt')
            fs.unlinkSync(cacheDir + '/test2.txt')
            expect(true).assertTrue()
            done()
        }).catch(async (err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testTaskAbility001
     * @tc.desc: Test create task when ability is number
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskAbility001', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            ability: 123
        }
        task = await agent.create(context, conf);
        task.start().then(() => {
            expect(true).assertTrue()
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testTaskAbility002
     * @tc.desc: Test create task when the ability is not exists
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskAbility002', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            ability: 'com.test'
        }
        try {
            task.create(context, conf).then(()=> {
                expect(false).assertTrue()
                done()
            }).catch((err) => {
                expect(err.code).assertEqual(401)
                done()
            })
        } catch (err) {
            expect(err.code).assertEqual(401)
            done()
        }
    })

    /**
     * @tc.name: testTaskAbility003
     * @tc.desc: Test create task when the ability is exists
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskAbility003', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            ability: 'com.acts.request'
        }
        task = await agent.create(conf);
        task.create(context, conf).then(()=> {
            expect(true).assertTrue()
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testTaskToken001
     * @tc.desc: Test create task when token is 7 bytes
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskToken001', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            token: 'abcdef'
        }
        try {
            agent.create(conf).then((data) => {
                expect(false).assertTrue()
                done()
            }).catch((err) => {
                expect(err.code).assertEqual(401)
                done()
            })
        } catch (err) {
            expect(err.code).assertEqual(401)
            done()
        }
    })

    /**
     * @tc.name: testTaskToken002
     * @tc.desc: Test create task when token is 2049 bytes
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskToken002', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            token: 'a'.padStart(2049, 'a')
        }
        try {
            agent.create(conf).then((data) => {
                expect(false).assertTrue()
                done()
            }).catch((err) => {
                expect(err.code).assertEqual(401)
                done()
            })
        } catch (err) {
            expect(err.code).assertEqual(401)
            done()
        }
    })

    /**
     * @tc.name: testTaskToken003
     * @tc.desc: Test create task when token is number
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskToken003', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            data: [{
                path: `${cacheDir}`
            }],
            token: 666
        }
        try {
            agent.create(conf).then((data) => {
                expect(false).assertTrue()
                done()
            }).catch((err) => {
                expect(err.code).assertEqual(401)
                done()
            })
        } catch (err) {
            expect(err.code).assertEqual(401)
            done()
        }
    })

    /**
     * @tc.name: testTaskToken004
     * @tc.desc: Test create task when token is 2048 bytes
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testTaskToken004', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            data: [{
                path: `${cacheDir}`
            }],
            token: 'a'.padStart(2048, 'a')
        }
        agent.create(conf).then((data) => {
            expect(true).assertTrue()
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })
})