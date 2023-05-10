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

describe('PermissionTest', function () {
    beforeAll(function () {
        console.info('beforeAll called')
        let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(file)
    })

    afterAll(function () {
        console.info('afterAll called')
        if (fs.accessSync(cacheDir + '/test.txt')) {
            fs.unlinkSync(cacheDir + '/test.txt')
        }
    })

    beforeEach(function () {
        console.info('beforeEach called')
    })

    afterEach(async function () {
        console.info('afterEach called')
        if (task !== undefined) {
            await agent.remove(context, task.tid)
            task = undefined
        }
    })

    let context = featureAbility.getContext()
    let cacheDir = '/data/storage/el2/base/haps/entry/files/';
    let task = undefined

    /**
     * @tc.name: testCreateTask001
     * @tc.desc: Test start task for callback no permission
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testCreateTask001', async function (done) {
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
        }
        task.create(conf, (err) => {
            if (err) {
                expect(err.code).assertEqual(201)
            } else {
                expect(false).assertTrue()
            }
            done()
        })
    })

    /**
     * @tc.name: testCreateTask002
     * @tc.desc: Test create task for promise no permission
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testCreateTask002', async function (done) {
        let conf = {
            action: agent.Action.UPLOAD,
            url: 'http://127.0.0.1',
        }
        task.create().then((task) => {
            expect(false).assertTrue()
            done()
        }).catch((err) => {
            expect(err.code).assertEqual(201)
            done()
        })
    })

    /**
     * @tc.name: testSearchTask001
     * @tc.desc: Test search task for callback no permission
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testSearchTask001', async function (done) {
        let filter = {
            bundle: 'com.acts.request'
        }
        agent.search(context, filter, (err) => {
            if (err) {
                expect(err.code).assertEqual(202)
            } else {
                expect(false).assertTrue()
            }
            done()
        })
    })

    /**
     * @tc.name: testSearchTask002
     * @tc.desc: Test search task for promise no permission
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testSearchTask002', async function (done) {
        let filter = {
            bundle: 'com.acts.request'
        }
        agent.search(context, filter).then((err) => {
            expect(false).assertTrue()
            done()
        }).catch((err) => {
            expect(err.code).assertEqual(202)
            done()
        })
    })

    /**
     * @tc.name: testQueryTask001
     * @tc.desc: Test query task for callback no permission
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testQueryTask001', function (done) {
        agent.query(context, '123', (err) => {
            if (err) {
                expect(err.code).assertEqual(202)
            } else {
                expect(false).assertTrue()
            }
            done()
        })
    })

    /**
     * @tc.name: testQueryTask002
     * @tc.desc: Test query task for promise no permission
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testQueryTask002', async function (done) {
        agent.query(context, '123').then((err) => {
            expect(false).assertTrue()
            done()
        }).catch((err) => {
            expect(err.code).assertEqual(202)
            done()
        })
    })

    /**
     * @tc.name: testClearTask001
     * @tc.desc: Test clear task for callback no permission
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testClearTask001', function (done) {
        agent.query(context, ['123'], (err) => {
            if (err) {
                expect(err.code).assertEqual(202)
            } else {
                expect(false).assertTrue()
            }
            done()
        })
    })

    /**
     * @tc.name: testClearTask002
     * @tc.desc: Test clear task for promise no permission
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testClearTask002', async function (done) {
        agent.clear(context, ['123']).then((err) => {
            expect(false).assertTrue()
            done()
        }).catch((err) => {
            expect(err.code).assertEqual(202)
            done()
        })
    })
})