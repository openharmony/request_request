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
     * @tc.name: testStartTask001
     * @tc.desc: Test start frontend task for callback
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testStartTask001', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.FRONTEND
        }
        task = await agent.create(conf)
        task.start((err) => {
            if (err) {
                expect(false).assertTrue()
            }
            expect(true).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testStartTask002
     * @tc.desc: Test start frontend task for promise
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testStartTask002', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.FRONTEND
        }
        task = await agent.create(conf)
        task.start().then(() => {
            expect(true).assertTrue()
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testStartTask003
     * @tc.desc: Test start background task for callback
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testStartTask003', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.BACKGROUND
        }
        task = await agent.create(conf)
        task.start((err) => {
            if (err) {
                expect(false).assertTrue()
            }
            expect(true).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testStartTask004
     * @tc.desc: Test start background task for promise
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testStartTask004', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.BACKGROUND
        }
        task = await agent.create(conf)
        task.start().then(() => {
            expect(true).assertTrue()
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testStartTask005
     * @tc.desc: Test start pause start background task for callback
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testStartTask005', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.BACKGROUND
        }
        task = await agent.create(conf)
        await task.start()
        await task.pause()
        task.start((err) => {
            if (err) {
                expect(false).assertTrue()
            }
            expect(true).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testStartTask006
     * @tc.desc: Test start pause start background task for promise
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testStartTask006', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.BACKGROUND
        }
        task = await agent.create(conf)
        await task.start()
        await task.pause()
        task.start().then(() => {
            expect(true).assertTrue()
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testStartTask007
     * @tc.desc: Test start stop start background task for callback
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testStartTask007', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.BACKGROUND
        }
        task = await agent.create(conf)
        await task.start()
        await task.stop()
        task.start((err) => {
            if (err) {
                expect(false).assertTrue()
            }
            expect(true).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testStartTask008
     * @tc.desc: Test start stop start background task for promise
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testStartTask008', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.BACKGROUND
        }
        task = await agent.create(conf)
        await task.start()
        await task.stop()
        task.start().then(() => {
            expect(true).assertTrue()
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testStartTask009
     * @tc.desc: Test start stop start frontend task for callback
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testStartTask009', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.FRONTEND
        }
        task = await agent.create(conf)
        await task.start()
        await task.stop()
        task.start((err) => {
            if (err) {
                expect(false).assertTrue()
            }
            expect(true).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testStartTask010
     * @tc.desc: Test start stop start frontend task for promise
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testStartTask010', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.FRONTEND
        }
        task = await agent.create(conf)
        await task.start()
        await task.stop()
        task.start().then(() => {
            expect(true).assertTrue()
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testStartTask011
     * @tc.desc: Test start two frontend task
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testStartTask011', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.FRONTEND
        }
        task = await agent.create(conf)
        await task.start()
        task.on('progress', function(progress) {
            if (progress.state === agent.State.PAUSED) {
                expect(true).assertTrue()
                done()
            }
        })
        let conf1 = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}/test/`,
            mode: agent.Mode.FRONTEND
        }
        let task1 = await agent.create(conf1)
        await task1.start()
        await sleep(2000)
        agent.remove(context, task1.tid)
        if (fs.accessSync(cacheDir + '/test/test.apk')) {
            fs.unlinkSync(cacheDir + '/test/test.apk')
        }
    })

    /**
     * @tc.name: testStartTask012
     * @tc.desc: Test start two background task
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testStartTask012', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.BACKGROUND
        }
        task = await agent.create(conf)
        await task.start()
        let conf1 = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}/test`,
            mode: agent.Mode.BACKGROUND
        }
        let task1 = await agent.create(conf1)
        await task1.start()
        await sleep(2000)
        agent.remove(context, task1.tid)
        expect(fs.accessSync(cacheDir + '/test/test.apk') && fs.accessSync(cacheDir + '/test.apk')).assertTrue()
        if (fs.accessSync(cacheDir + '/test/test.apk')) {
            fs.unlinkSync(cacheDir + '/test/test.apk')
        }
    })

    /**
     * @tc.name: testPauseTask001
     * @tc.desc: Test pause frontend task for callback
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testPauseTask001', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.FRONTEND
        }
        task = await agent.create(conf)
        task.pause((err) => {
            if (err) {
                expect(true).assertTrue()
            } else {
                expect(false).assertTrue()
            }
            done()
        })
    })

    /**
     * @tc.name: testPauseTask002
     * @tc.desc: Test pause frontend task for promise
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testPauseTask002', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.FRONTEND
        }
        task = await agent.create(conf)
        task.pause().then(() => {
            expect(false).assertTrue()
            done()
        }).catch((err) => {
            expect(true).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testPauseTask003
     * @tc.desc: Test pause background task for callback
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testPauseTask003', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.BACKGROUND
        }
        task = await agent.create(conf)
        task.pause((err) => {
            if (err) {
                expect(false).assertTrue()
            }
            expect(true).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testPauseTask004
     * @tc.desc: Test pause background task for promise
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testPauseTask004', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.BACKGROUND
        }
        task = await agent.create(conf)
        task.pause().then(() => {
            expect(true).assertTrue()
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testPauseTask005
     * @tc.desc: Test pause resume pause background task
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testPauseTask005', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.BACKGROUND
        }
        task = await agent.create(conf)
        await task.pause()
        task.resume().then(() => {
            expect(true).assertTrue()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
        task.pause().then(() => {
            expect(true).assertTrue()
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testPauseTask006
     * @tc.desc: Test pause stop background task
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testPauseTask006', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.BACKGROUND
        }
        task = await agent.create(conf)
        await task.pause()
        task.stop().then(() => {
            expect(true).assertTrue()
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testResumeTask001
     * @tc.desc: Test resume frontend task for callback
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testResumeTask001', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.FRONTEND
        }
        task = await agent.create(conf)
        task.resume((err) => {
            if (err) {
                expect(true).assertTrue()
            } else {
                expect(false).assertTrue()
            }
            done()
        })
    })

    /**
     * @tc.name: testResumeTask002
     * @tc.desc: Test resume frontend task for promise
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testResumeTask002', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.FRONTEND
        }
        task = await agent.create(conf)
        task.resume().then(() => {
            expect(false).assertTrue()
            done()
        }).catch((err) => {
            expect(true).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testResumeTask003
     * @tc.desc: Test resume background task for callback
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testResumeTask003', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.BACKGROUND
        }
        task = await agent.create(conf)
        task.resume((err) => {
            if (err) {
                expect(false).assertTrue()
            }
            expect(true).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testResumeTask004
     * @tc.desc: Test resume background task for promise
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testResumeTask004', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.BACKGROUND
        }
        task = await agent.create(conf)
        task.resume().then(() => {
            expect(true).assertTrue()
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testResumeTask005
     * @tc.desc: Test resume stop resume background task
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testPauseTask005', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.BACKGROUND
        }
        task = await agent.create(conf)
        await task.resume()
        task.stop().then(() => {
            expect(true).assertTrue()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
        task.resume().then(() => {
            expect(true).assertTrue()
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testResumeTask006
     * @tc.desc: Test start resume start background task for callback
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testResumeTask006', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.BACKGROUND
        }
        task = await agent.create(conf)
        await task.start()
        task.resume((err) => {
            if (err) {
                expect(false).assertTrue()
                done()
            }
            expect(true).assertTrue()
            task.start((error) => {
                if (error) {
                    expect(false).assertTrue()
                }
                expect(true).assertTrue()
                done()
            })
        })
    })

    /**
     * @tc.name: testResumeTask007
     * @tc.desc: Test start resume start background task for promise
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testResumeTask007', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.BACKGROUND
        }
        task = await agent.create(conf)
        await task.start()
        task.resume().then(() => {
            expect(true).assertTrue()
            task.start().then(() => {
                expect(true).assertTrue()
                done()
            }).catch((err) => {
                expect(false).assertTrue()
                done()
            })
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testStopTask001
     * @tc.desc: Test stop frontend task for callback
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testStopTask001', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.FRONTEND
        }
        task = await agent.create(conf)
        task.stop((err) => {
            if (err) {
                expect(false).assertTrue()
            }
            expect(true).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testStopTask002
     * @tc.desc: Test stop frontend task for promise
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testStopTask002', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.FRONTEND
        }
        task = await agent.create(conf)
        task.stop().then(() => {
            expect(true).assertTrue()
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testStopTask003
     * @tc.desc: Test stop background task for callback
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testStopTask003', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.BACKGROUND
        }
        task = await agent.create(conf)
        task.stop((err) => {
            if (err) {
                expect(false).assertTrue()
            }
            expect(true).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testStopTask004
     * @tc.desc: Test stop background task for promise
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testStopTask004', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.BACKGROUND
        }
        task = await agent.create(conf)
        task.stop().then(() => {
            expect(true).assertTrue()
            done()
        }).catch((err) => {
            expect(false).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testStopTask005
     * @tc.desc: Test stop pause background task for promise
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testStopTask005', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.BACKGROUND
        }
        task = await agent.create(conf)
        await task.stop()
        task.pause((err) => {
            if (err) {
                expect(false).assertTrue()
            }
            expect(true).assertTrue()
            done()
        })
    })

    /**
     * @tc.name: testOnTask001
     * @tc.desc: Test on task for 'test'
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testOnTask001', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.FRONTEND
        }
        task = await agent.create(conf)
        try {
            task.on('test', function (progress) {})
        } catch (err) {
            expect(err.code).assertEqual(401)
            done()
        }
    })

    /**
     * @tc.name: testOnTask002
     * @tc.desc: Test on task for number
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testOnTask002', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.FRONTEND
        }
        task = await agent.create(conf)
        try {
            task.on(123, function (progress) {})
        } catch (err) {
            expect(err.code).assertEqual(401)
            done()
        }
    })

    /**
     * @tc.name: testOnTask003
     * @tc.desc: Test on task for failed
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testOnTask003', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test1.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.FRONTEND
        }
        task = await agent.create(conf)
        task.on('failed', function () {
            expect(true).assertTrue()
            done()
        })
        await task.start()
    })

    /**
     * @tc.name: testOnTask004
     * @tc.desc: Test on task for completed
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testOnTask004', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.FRONTEND
        }
        task = await agent.create(conf)
        task.on('completed', function () {
            expect(true).assertTrue()
            done()
        })
        await task.start()
    })

    /**
     * @tc.name: testOnTask005
     * @tc.desc: Test on task for progress state RUNNING
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testOnTask005', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.FRONTEND
        }
        task = await agent.create(conf)
        task.on('progress', function (progress) {
            if (progress.state === agent.State.RUNNING) {
                expect(true).assertTrue()
                done()
            }
        })
        await task.start()
    })

    /**
     * @tc.name: testOnTask006
     * @tc.desc: Test on task for progress state RETRYING
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testOnTask006', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test1.apk',
            saveas: `${cacheDir}`,
            timeout: 2,
            retry: true,
            mode: agent.Mode.FRONTEND
        }
        task = await agent.create(conf)
        task.on('progress', function (progress) {
            if (progress.state === agent.State.RETRYING) {
                expect(true).assertTrue()
                done()
            }
        })
        await task.start()
    })

    /**
     * @tc.name: testOnTask007
     * @tc.desc: Test on task for progress state PAUSED
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testOnTask007', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.FRONTEND
        }
        task = await agent.create(conf)
        task.on('progress', function (progress) {
            if (progress.state === agent.State.PAUSED) {
                expect(true).assertTrue()
                done()
            }
        })
        await task.start()
        await task.pause()
    })

    /**
     * @tc.name: testOnTask008
     * @tc.desc: Test on task for progress state STOPPED
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testOnTask008', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.FRONTEND
        }
        task = await agent.create(conf)
        task.on('progress', function (progress) {
            if (progress.state === agent.State.STOPPED) {
                expect(true).assertTrue()
                done()
            }
        })
        await task.start()
        await task.stop()
    })

    /**
     * @tc.name: testOnTask009
     * @tc.desc: Test on task for progress state REMOVED
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testOnTask009', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.FRONTEND
        }
        task = await agent.create(conf)
        task.on('progress', function (progress) {
            if (progress.state === agent.State.REMOVED) {
                expect(true).assertTrue()
                done()
            }
        })
        await agent.remove(context, task.tid)
        task = undefined
    })

    /**
     * @tc.name: testOnTask010
     * @tc.desc: Test on task for progress twice
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testOnTask010', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.FRONTEND
        }
        task = await agent.create(conf)
        try {
            task.on('progress', function (progress) {})
            task.on('progress', function (progress) {})
            expect(true).assertTrue()
            done()
        } catch (err) {
            expect(false).assertTrue()
            done()
        }
    })

    /**
     * @tc.name: testOnTask011
     * @tc.desc: Test on task background task
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testOnTask011', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.BACKGROUND
        }
        task = await agent.create(conf)
        try {
            task.on('progress', function (progress) {})
        } catch (err) {
            expect(err.code).assertEqual(21900005)
            done()
        }
    })

    /**
     * @tc.name: testOffTask001
     * @tc.desc: Test off task for 'test'
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testOffTask001', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.FRONTEND
        }
        task = await agent.create(conf)
        try {
            task.off('test', function () {})
        } catch (err) {
            expect(err.code).assertEqual(401)
            done()
        }
    })

    /**
     * @tc.name: testOffTask002
     * @tc.desc: Test off task for number
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testOffTask002', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.FRONTEND
        }
        task = await agent.create(conf)
        try {
            task.off(123)
        } catch (err) {
            expect(err.code).assertEqual(401)
            done()
        }
    })

    /**
     * @tc.name: testOffTask003
     * @tc.desc: Test off task for failed
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testOffTask003', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test1.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.FRONTEND
        }
        task = await agent.create(conf)
        task.on('failed', function() {
            expect(false).assertTrue()
            done()
        })
        task.off('failed')
        await task.start()
        await sleep(2000)
        expect(true).assertTrue()
        done()
    })

    /**
     * @tc.name: testOffTask004
     * @tc.desc: Test off task for completed
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testOffTask004', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.FRONTEND
        }
        task = await agent.create(conf)
        task.on('completed', function () {
            expect(false).assertTrue()
            done()
        })
        task.off('completed')
        await task.start()
    })

    /**
     * @tc.name: testOffTask005
     * @tc.desc: Test off task for progress
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testOffTask005', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.FRONTEND
        }
        task = await agent.create(conf)
        task.on('progress', function (progress) {
            expect(false).assertTrue()
            done()
        })
        task.off('progress')
        await task.start()
    })

    /**
     * @tc.name: testOffTask006
     * @tc.desc: Test off task twice
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testOffTask006', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.FRONTEND
        }
        task = await agent.create(conf)
        try {
            task.off('completed')
            task.off('completed')
            expect(true).assertTrue()
            done()
        } catch (err) {
            expect(false).assertTrue()
            done()
        }
    })

    /**
     * @tc.name: testOffTask007
     * @tc.desc: Test off background task
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testOffTask006', async function (done) {
        let conf = {
            action: agent.Action.DOWNLOAD,
            url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test.apk',
            saveas: `${cacheDir}`,
            mode: agent.Mode.BACKGROUND
        }
        task = await agent.create(conf)
        try {
            task.off('progress', function (progress) {})
        } catch (err) {
            expect(err.code).assertEqual(21900005)
            done()
        }
    })
})