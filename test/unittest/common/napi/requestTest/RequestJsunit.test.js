/*
 * Copyright (C) 2022 Huawei Device Co., Ltd.
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
import request from '@ohos.request';

const TAG = "REQUEST_TEST";
let keyStr = 'download test ';

describe('RequestTest', function () {
    beforeAll(function () {
        console.info('beforeAll called')
    })

    afterAll(function () {
        console.info('afterAll called')
    })

    beforeEach(function () {
        console.info('beforeEach called')
    })

    afterEach(function () {
        console.info('afterEach called')
    })
    console.log(TAG + "*************Unit Test Begin*************");

    let downloadTask;
    let downloadConfig;

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0001
     * @tc.desc      alled when the current download session is in process.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0001', 0, async function (done) {
        downloadConfig = {
            url: 'https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png',
            header: {
                headers: 'http'
            },
            enableMetered: false,
            enableRoaming: false,
            description: 'XTS download test!',
            networkType: request.NETWORK_WIFI,
            filePath: `/data/storage/el2/base/haps/entry/files/`,
            title: 'XTS download test!',
            background: false
        }
        console.info("====>-----------------------SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0001 is starting-----------------------");
        downloadConfig.filePath += 'SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0001.txt'
        request.download(downloadConfig, (err, data)=>{
            try{
                downloadTask = data;
                console.info("====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0001 downloadTask: " + JSON.stringify(downloadTask) );
                expect(true).assertEqual(downloadTask != undefined);
                downloadTask.on('progress', (data1, data2) => {
                    try{
                        console.info("====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0001 on data1 =" + data1);
                        console.info("====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0001 on data2 =" + data2);
                        expect(true).assertEqual(data1 != undefined);
                        expect(true).assertEqual(data2 != undefined);
                        if (data1 == data2){
                            expect(true).assertTrue();
                            downloadTask.remove()
                            done();
                        }
                    }catch(err){
                        console.error("====>SUB_REQUEST_DOWNLOAD_API_PROMISE_0001 throw_error: " + err);
                        done();
                    }
                });
            }catch(err){
                console.error("====>SUB_REQUEST_DOWNLOAD_API_PROMISE_0001 error: " + err);
                done();
            }
        });
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0002
     * @tc.desc       Called when the current download session complete、pause or remove.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0002', 0, async function (done) {
        downloadConfig = {
            url: 'https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png',
            header: {
                headers: 'http'
            },
            enableMetered: false,
            enableRoaming: false,
            description: 'XTS download test!',
            networkType: request.NETWORK_WIFI,
            filePath: `/data/storage/el2/base/haps/entry/files/`,
            title: 'XTS download test!',
            background: false
        }
        console.info("====>-----------------------SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0002 is starting-----------------------");
        downloadConfig.filePath += 'SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0002.txt'
        request.download(downloadConfig, (err, data)=>{
            downloadTask = data;
            console.info("====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0002 downloadTask: " + downloadTask);
            try{
                expect(true).assertEqual(downloadTask != undefined);
                downloadTask.on('complete', () => {
                    try{
                        console.info('====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0002 task completed.')
                        expect(true).assertTrue();
                    }catch(err){
                        console.error("====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0002 throw_error: " + err);
                    }
                    done();
                });
            }catch(err){
                console.error("====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0002 error: " + err);
                done();
            }
        });
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0003
     * @tc.desc       Called when the current download session complete、pause or remove.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0003', 0, async function (done) {
        downloadConfig = {
            url: 'https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png',
            header: {
                headers: 'http'
            },
            enableMetered: false,
            enableRoaming: false,
            description: 'XTS download test!',
            networkType: request.NETWORK_WIFI,
            filePath: `/data/storage/el2/base/haps/entry/files/`,
            title: 'XTS download test!',
            background: false
        }
        console.info("====>-----------------------SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0003 is starting-----------------------");
        downloadConfig.filePath += 'SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0003.txt'
        request.download(downloadConfig, (err, data)=>{
            downloadTask = data;
            console.info("====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0003 downloadTask: " + downloadTask);
            try{
                expect(true).assertEqual(downloadTask != undefined);
                downloadTask.on('pause', () => {
                    try{
                        console.info('====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0003 task completed.')
                        expect(true).assertTrue();
                    }catch(err){
                        console.error("====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0003 throw_error: " + err);
                    }
                    downloadTask.remove()
                    done();
                });
                downloadTask.on('progress', (data1, data2) => {
                    if(data1 > 0){
                        downloadTask.pause()
                    }
                });
            }catch(err){
                console.error("====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0003 error: " + err);
                done();
            }
        });
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0004
     * @tc.desc       Called when the current download session complete、pause or remove.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0004', 0, async function (done) {
        downloadConfig = {
            url: 'https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png',
            header: {
                headers: 'http'
            },
            enableMetered: false,
            enableRoaming: false,
            description: 'XTS download test!',
            networkType: request.NETWORK_WIFI,
            filePath: `/data/storage/el2/base/haps/entry/files/`,
            title: 'XTS download test!',
            background: false
        }
        console.info("====>-----------------------SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0004 is starting-----------------------");
        downloadConfig.filePath += 'SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0004.txt'
        request.download(downloadConfig, (err, data)=>{
            downloadTask = data;
            console.info("====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0004 downloadTask: " + downloadTask);
            try{
                expect(downloadTask != undefined).assertEqual(true);
                downloadTask.on('remove', () => {
                    try{
                        console.info('====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0004 remove completed')
                        expect(true).assertTrue();
                    }catch(err){
                        console.error("====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0004 throw_error: " + err);
                    }
                    done();
                });
                downloadTask.on('progress', (data1, data2) => {
                    if(data1 > 0){
                        downloadTask.remove()
                    }
                });
            }catch(err){
                console.error("====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0004 error: " + err);
                done();
            }
        });
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0005
     * @tc.desc      Called when the current download session fails.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0005', 0, async function (done) {
        downloadConfig = {
            url: 'https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.pngkfkew',
            header: {
                headers: 'http'
            },
            enableMetered: false,
            enableRoaming: false,
            description: 'XTS download test!',
            networkType: request.NETWORK_WIFI,
            filePath: `/data/storage/el2/base/haps/entry/files/`,
            title: 'XTS download test!',
            background: false
        }
        console.info("====>-----------------------SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0005 is starting-----------------------");
        downloadConfig.filePath += 'SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0005.txt'
        downloadConfig.url += '123456'
        request.download(downloadConfig, (err, data)=>{
            downloadTask = data;
            console.info("====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0005 downloadTask: " + downloadTask);
            try{
                expect(downloadTask != undefined).assertEqual(true);
                downloadTask.on('fail',(err)=>{
                    try{
                        console.info('====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0005 fail completed')
                        expect(true).assertTrue();
                    }catch(err){
                        console.error("====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0005 throw_error: " + err);
                    }
                    done();
                })
            }catch(err){
                console.error("====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0005 error: " + err);
                done();
            }
        });
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0001
     * @tc.desc      alled when the current download session is in process.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0001', 0, async function (done) {
        downloadConfig = {
            url: 'https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png',
            header: {
                headers: 'http'
            },
            enableMetered: false,
            enableRoaming: false,
            description: 'XTS download test!',
            networkType: request.NETWORK_WIFI,
            filePath: `/data/storage/el2/base/haps/entry/files/`,
            title: 'XTS download test!',
            background: false
        }
        console.info("====>-----------------------SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0001 is starting-----------------------");
        downloadConfig.filePath += 'SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0001.txt'
        request.download(downloadConfig, (err, data)=>{
            try{
                downloadTask = data;
                console.info("====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0001 downloadTask: " + downloadTask);
                expect(downloadTask != undefined).assertEqual(true);
                downloadTask.on('progress', (data1, data2) => {});
                downloadTask.off('progress', (data1, data2) => {
                    try{
                        console.info("====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0001 on data1 =" + data1);
                        console.info("====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0001 on data2 =" + data2);
                        expect(data1).assertEqual(0);
                        expect(data2).assertEqual(0);
                    }catch(err){
                        console.error("====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0001 off_err: " + err);
                    }
                    downloadTask.remove()
                    done();
                });
            }catch(err){
                console.error("====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0001 error: " + err);
                done();
            }
        });
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0002
     * @tc.desc      alled when the current download session complete、pause or remove.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0002', 0, async function (done) {
        downloadConfig = {
            url: 'https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png',
            header: {
                headers: 'http'
            },
            enableMetered: false,
            enableRoaming: false,
            description: 'XTS download test!',
            networkType: request.NETWORK_WIFI,
            filePath: `/data/storage/el2/base/haps/entry/files/`,
            title: 'XTS download test!',
            background: false
        }
        console.info("====>-----------------------SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0002 is starting-----------------------");
        downloadConfig.filePath += 'SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0002.txt'
        request.download(downloadConfig, (err, data)=>{
            downloadTask = data;
            console.info("====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0002 downloadTask: " + downloadTask);
            try{
                expect(downloadTask != undefined).assertEqual(true);
                downloadTask.off('complete', () => {
                    try{
                        console.info('====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0002 task complete.')
                        expect(true).assertTrue();
                    }catch(err){
                        console.error("====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0002 off_err: " + err);
                    }
                    downloadTask.remove()
                    done();
                });
            }catch(err){
                console.error("====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0002 error: " + err);
                done();
            }
        });
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0003
     * @tc.desc      alled when the current download session complete、pause or remove.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0003', 0, async function (done) {
        downloadConfig = {
            url: 'https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png',
            header: {
                headers: 'http'
            },
            enableMetered: false,
            enableRoaming: false,
            description: 'XTS download test!',
            networkType: request.NETWORK_WIFI,
            filePath: `/data/storage/el2/base/haps/entry/files/`,
            title: 'XTS download test!',
            background: false
        }
        console.info("====>-----------------------SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0003 is starting-----------------------");
        downloadConfig.filePath += 'SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0003.txt'
        request.download( downloadConfig, (err, data)=>{
            downloadTask = data;
            console.info("====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0003 downloadTask: " + downloadTask);
            try{
                expect(downloadTask != undefined).assertEqual(true);
                downloadTask.off('pause', () => {
                    try{
                        console.info('====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0003 task complete.')
                        expect(true).assertTrue();
                    }catch(err){
                        console.error("====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0003 off_err: " + err);
                    }
                    downloadTask.remove()
                    done();
                });
            }catch(err){
                console.error("====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0003 error: " + err);
                done();
            }
        });
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0004
     * @tc.desc      alled when the current download session complete、pause or remove.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0004', 0, async function (done) {
        downloadConfig = {
            url: 'https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png',
            header: {
                headers: 'http'
            },
            enableMetered: false,
            enableRoaming: false,
            description: 'XTS download test!',
            networkType: request.NETWORK_WIFI,
            filePath: `/data/storage/el2/base/haps/entry/files/`,
            title: 'XTS download test!',
            background: false
        }
        console.info("====>-----------------------SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0004 is starting-----------------------");
        downloadConfig.filePath += 'SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0004.txt'
        request.download( downloadConfig, (err, data)=>{
            downloadTask = data;
            console.info("====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0004 downloadTask: " + downloadTask);
            try{
                expect(downloadTask != undefined).assertEqual(true);
                downloadTask.off('remove', (data) => {
                    try{
                        console.info('====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0003 remove data:' +JSON.stringify(data))
                        expect(true).assertTrue();
                    }catch(err){
                        console.error("====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0003 off_err: " + err);
                    }
                    downloadTask.remove()
                    done();
                });
            }catch(err){
                console.error("====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0004 error: " + err);
                done();
            }
        });
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0005
     * @tc.desc      Called when the current download session fails.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0005', 0, async function (done) {
        downloadConfig = {
            url: 'https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png',
            header: {
                headers: 'http'
            },
            enableMetered: false,
            enableRoaming: false,
            description: 'XTS download test!',
            networkType: request.NETWORK_WIFI,
            filePath: `/data/storage/el2/base/haps/entry/files/`,
            title: 'XTS download test!',
            background: false
        }
        console.info("====>-----------------------SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0005 is starting-----------------------");
        downloadConfig.filePath += 'SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0005.txt'
        request.download( downloadConfig, (err, data)=>{
            downloadTask = data;
            console.info("====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0005 downloadTask: " + downloadTask);
            try{
                expect(downloadTask != undefined).assertEqual(true);
                downloadTask.on('fail', (data) => {});
                downloadTask.off('fail', (data) => {
                    try{
                        console.info('====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0005 fail_data:' + JSON.stringify(data))
                        expect(data).assertEqual(0)
                        console.info("====>-----------------------SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0005 end-----------------------");
                    }catch(err){
                        console.error("====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0005 fail_error: " + err);
                    }
                    downloadTask.remove()
                    done();
                });
            }catch(err){
                console.error("====>SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0005 error: " + err);
                done();
            }
        });
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_REMOVE_0001
     * @tc.desc      Deletes a download session and the downloaded files.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_REMOVE_0001', 0, async function (done) {
        downloadConfig = {
            url: 'https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png',
            header: {
                headers: 'http'
            },
            enableMetered: false,
            enableRoaming: false,
            description: 'XTS download test!',
            networkType: request.NETWORK_WIFI,
            filePath: `/data/storage/el2/base/haps/entry/files/`,
            title: 'XTS download test!',
            background: false
        }
        console.info("====>-----------------------SUB_REQUEST_DOWNLOAD_API_REMOVE_0001 is starting-----------------------");
        downloadConfig.filePath += 'SUB_REQUEST_DOWNLOAD_API_REMOVE_0001.txt'
        request.download( downloadConfig, (err, data)=>{
            downloadTask = data;
            console.info("====>SUB_REQUEST_DOWNLOAD_API_REMOVE_0001 downloadTask: " + downloadTask);
            try{
                expect(downloadTask != undefined).assertEqual(true);
                downloadTask.remove((err, data)=>{
                    try{
                        if(err) {
                            console.error('====>SUB_REQUEST_DOWNLOAD_API_REMOVE_0001 Failed to remove the download task.');
                            expect().assertFail();
                        }
                        if (data) {
                            console.info('====>SUB_REQUEST_DOWNLOAD_API_REMOVE_0001 removed data:' + JSON.stringify(data));
                            expect(data == true).assertTrue();
                        } else {
                            console.error('====>SUB_REQUEST_DOWNLOAD_API_REMOVE_0001 Failed to remove the download task.');
                            expect().assertFail();
                        }
                    }catch(err){
                        console.info('====>SUB_REQUEST_DOWNLOAD_API_REMOVE_0001 remove_throw_err:' + JSON.stringify(err))
                    }
                    done();
                });
            }catch(err){
                console.error("====>SUB_REQUEST_DOWNLOAD_API_REMOVE_0001 error: " + err);
                done();
            }
        })
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_REMOVE_0002
     * @tc.desc      Deletes a download session and the downloaded files.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_REMOVE_0002', 0, async function (done) {
        downloadConfig = {
            url: 'https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png',
            header: {
                headers: 'http'
            },
            enableMetered: false,
            enableRoaming: false,
            description: 'XTS download test!',
            networkType: request.NETWORK_WIFI,
            filePath: `/data/storage/el2/base/haps/entry/files/`,
            title: 'XTS download test!',
            background: false
        }
        console.info("====>-----------------------SUB_REQUEST_DOWNLOAD_API_REMOVE_0002 is starting-----------------------");
        downloadConfig.filePath += 'SUB_REQUEST_DOWNLOAD_API_REMOVE_0002.txt'
        request.download( downloadConfig, (err, data)=>{
            downloadTask = data;
            console.info("====>SUB_REQUEST_DOWNLOAD_API_REMOVE_0002 downloadTask: " + downloadTask);
            try{
                expect(downloadTask != undefined).assertEqual(true);
                downloadTask.remove().then(data => {
                    if (data) {
                        console.info('====>SUB_REQUEST_DOWNLOAD_API_REMOVE_0002 removed data:' + JSON.stringify(data));
                        expect(data == true).assertTrue();
                    } else {
                        console.error('====>SUB_REQUEST_DOWNLOAD_API_REMOVE_0002 Failed to remove the download task.');
                        expect().assertFail();
                    }
                    done();
                }).catch((err) => {
                    console.error('====>SUB_REQUEST_DOWNLOAD_API_REMOVE_0002 Failed to remove the download task.');
                    done();
                })
            }catch(err){
                console.error("====>SUB_REQUEST_DOWNLOAD_API_REMOVE_0002 error: " + err);
                done();
            }
        })
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_PAUSE_0001
     * @tc.desc      Pause a download session.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_PAUSE_0001', 0, async function (done) {
        downloadConfig = {
            url: 'https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png',
            header: {
                headers: 'http'
            },
            enableMetered: false,
            enableRoaming: false,
            description: 'XTS download test!',
            networkType: request.NETWORK_WIFI,
            filePath: `/data/storage/el2/base/haps/entry/files/`,
            title: 'XTS download test!',
            background: false
        }
        console.info("====>-----------------------SUB_REQUEST_DOWNLOAD_API_PAUSE_0001 is starting-----------------------");
        downloadConfig.filePath += 'SUB_REQUEST_DOWNLOAD_API_PAUSE_0001.txt'
        request.download( downloadConfig, (err, data)=>{
            downloadTask = data;
            console.info("====>SUB_REQUEST_DOWNLOAD_API_PAUSE_0001 downloadTask: " + downloadTask);
            expect(downloadTask != undefined).assertEqual(true);
            try{
                downloadTask.pause(()=>{
                    try{
                        console.info('====>SUB_REQUEST_DOWNLOAD_API_PAUSE_0001 Download task pause success.');
                        expect(true).assertTrue();
                    }catch(err){
                        console.info('====>SUB_REQUEST_DOWNLOAD_API_PAUSE_0001 pause_throw_err:' + JSON.stringify(err))
                    }
                    done();
                })
            }catch(err){
                console.error("====>SUB_REQUEST_DOWNLOAD_API_PAUSE_0001 error: " + JSON.stringify(err));
                done();
            }
        })
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_PAUSE_0002
     * @tc.desc      Pause a download session.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_PAUSE_0002', 0, async function (done) {
        downloadConfig = {
            url: 'https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png',
            header: {
                headers: 'http'
            },
            enableMetered: false,
            enableRoaming: false,
            description: 'XTS download test!',
            networkType: request.NETWORK_WIFI,
            filePath: `/data/storage/el2/base/haps/entry/files/`,
            title: 'XTS download test!',
            background: false
        }
        console.info("====>-----------------------SUB_REQUEST_DOWNLOAD_API_PAUSE_0002 is starting-----------------------");
        downloadConfig.filePath += 'SUB_REQUEST_DOWNLOAD_API_PAUSE_0002.txt'
        request.download( downloadConfig, (err, data)=>{
            downloadTask = data;
            console.info("====>SUB_REQUEST_DOWNLOAD_API_PAUSE_0002 downloadTask: " + downloadTask);
            try{
                expect(downloadTask != undefined).assertEqual(true);
                downloadTask.pause().then(() => {
                    console.info('====>SUB_REQUEST_DOWNLOAD_API_PAUSE_0002 Download task pause success.');
                    expect(true).assertTrue();
                    done();
                }).catch((err) => {
                    console.error("====>SUB_REQUEST_DOWNLOAD_API_PAUSE_0002 throw_error: " + JSON.stringify(err));
                    done();
                })
            }catch(err){
                console.error("====>SUB_REQUEST_DOWNLOAD_API_PAUSE_0002 error: " + JSON.stringify(err));
                done();
            }
        })
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_REMUSE_0001
     * @tc.desc      Resume a paused download session.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_REMUSE_0001', 0, async function (done) {
        downloadConfig = {
            url: 'https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png',
            header: {
                headers: 'http'
            },
            enableMetered: false,
            enableRoaming: false,
            description: 'XTS download test!',
            networkType: request.NETWORK_WIFI,
            filePath: `/data/storage/el2/base/haps/entry/files/`,
            title: 'XTS download test!',
            background: false
        }
        console.info("====>-----------------------SUB_REQUEST_DOWNLOAD_API_REMUSE_0001 is starting-----------------------");
        downloadConfig.filePath += 'SUB_REQUEST_DOWNLOAD_API_REMUSE_0001.txt'
        request.download( downloadConfig, (err, data)=>{
            downloadTask = data;
            console.info("====>SUB_REQUEST_DOWNLOAD_API_REMUSE_0001 downloadTask: " + downloadTask);
            expect(downloadTask != undefined).assertEqual(true);
            try{
                downloadTask.resume(()=>{
                    try{
                        console.info('====>SUB_REQUEST_DOWNLOAD_API_REMUSE_0001 Download task resume success.');
                        expect(true).assertTrue();
                    }catch(err){
                        console.info('====>SUB_REQUEST_DOWNLOAD_API_REMUSE_0001 pause_throw_err:' + JSON.stringify(err))
                    }
                    downloadTask.remove()
                    done();
                })
            }catch(err){
                console.error("====>SUB_REQUEST_DOWNLOAD_API_REMUSE_0001 error: " + JSON.stringify(err));
                done();
            }
        })
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_REMUSE_0002
     * @tc.desc      Resume a paused download session.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_REMUSE_0002', 0, async function (done) {
        downloadConfig = {
            url: 'https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png',
            header: {
                headers: 'http'
            },
            enableMetered: false,
            enableRoaming: false,
            description: 'XTS download test!',
            networkType: request.NETWORK_WIFI,
            filePath: `/data/storage/el2/base/haps/entry/files/`,
            title: 'XTS download test!',
            background: false
        }
        console.info("====>-----------------------SUB_REQUEST_DOWNLOAD_API_REMUSE_0002 is starting-----------------------");
        downloadConfig.filePath += 'SUB_REQUEST_DOWNLOAD_API_REMUSE_0002.txt'
        request.download( downloadConfig, (err, data)=>{
            downloadTask = data;
            console.info("====>SUB_REQUEST_DOWNLOAD_API_REMUSE_0002 downloadTask: " + downloadTask);
            try{
                expect(downloadTask != undefined).assertEqual(true);
                downloadTask.resume().then(() => {
                    console.info('====>SUB_REQUEST_DOWNLOAD_API_REMUSE_0002 Download task resume success.');
                    expect(true).assertTrue();
                    downloadTask.remove()
                    done()
                }).catch((err) => {
                    console.error("====>SUB_REQUEST_DOWNLOAD_API_REMUSE_0002 throw_error: " + JSON.stringify(err));
                    done();
                })
            }catch(err){
                console.error("====>SUB_REQUEST_DOWNLOAD_API_REMUSE_0002 error: " + JSON.stringify(err));
                done();
            }
        })
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_QUERY_0001
     * @tc.desc      Queries download information of a session, which is defined in DownloadSession.DownloadInfo.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_QUERY_0001', 0, async function (done) {
        downloadConfig = {
            url: 'https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png',
            header: {
                headers: 'http'
            },
            enableMetered: false,
            enableRoaming: false,
            description: 'XTS download test!',
            networkType: request.NETWORK_WIFI,
            filePath: `/data/storage/el2/base/haps/entry/files/`,
            title: 'XTS download test!',
            background: false
        }
        console.info("====>-----------------------SUB_REQUEST_DOWNLOAD_API_QUERY_0001 is starting-----------------------");
        downloadConfig.filePath += 'SUB_REQUEST_DOWNLOAD_API_QUERY_0001.txt'
        request.download( downloadConfig, (err, data)=>{
            downloadTask = data;
            console.info("====>SUB_REQUEST_DOWNLOAD_API_QUERY_0001 downloadTask: " + downloadTask);
            try{
                expect(downloadTask != undefined).assertEqual(true);
                downloadTask.query((err, downloadInfo)=>{
                    try{
                        if(err) {
                            console.error('====>SUB_REQUEST_DOWNLOAD_API_QUERY_0001 Failed to query: ' + JSON.stringify(err));
                            expect().assertFail();
                        } else {
                            console.info('====>SUB_REQUEST_DOWNLOAD_API_QUERY_0001 query info: '+ downloadInfo.description);
                            console.info('====>SUB_REQUEST_DOWNLOAD_API_QUERY_0001 query info: '+ downloadInfo.downloadedBytes);
                            console.info('====>SUB_REQUEST_DOWNLOAD_API_QUERY_0001 query info: '+ downloadInfo.downloadId);
                            console.info('====>SUB_REQUEST_DOWNLOAD_API_QUERY_0001 query info: '+ downloadInfo.failedReason);
                            console.info('====>SUB_REQUEST_DOWNLOAD_API_QUERY_0001 query info: '+ downloadInfo.fileName);
                            console.info('====>SUB_REQUEST_DOWNLOAD_API_QUERY_0001 query info: '+ downloadInfo.filePath);
                            console.info('====>SUB_REQUEST_DOWNLOAD_API_QUERY_0001 query info: '+ downloadInfo.pausedReason);
                            console.info('====>SUB_REQUEST_DOWNLOAD_API_QUERY_0001 query info: '+ downloadInfo.status);
                            console.info('====>SUB_REQUEST_DOWNLOAD_API_QUERY_0001 query info: '+ downloadInfo.targetURI);
                            console.info('====>SUB_REQUEST_DOWNLOAD_API_QUERY_0001 query info: '+ downloadInfo.downloadTitle);
                            console.info('====>SUB_REQUEST_DOWNLOAD_API_QUERY_0001 query info: '+ downloadInfo.downloadTotalBytes);
                            expect(true).assertTrue();
                        }
                    }catch(err){
                        console.error("====>SUB_REQUEST_DOWNLOAD_API_QUERY_0001 throw_error: " + JSON.stringify(err));
                    }
                    done();
                })
            }catch(err){
                console.error("====>SUB_REQUEST_DOWNLOAD_API_QUERY_0001 error: " + JSON.stringify(err));
                done();
            }
        })
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_QUERY_0002
     * @tc.desc      Queries download information of a session, which is defined in DownloadSession.DownloadInfo.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_QUERY_0002', 0, async function (done) {
        downloadConfig = {
            url: 'https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png',
            header: {
                headers: 'http'
            },
            enableMetered: false,
            enableRoaming: false,
            description: 'XTS download test!',
            networkType: request.NETWORK_WIFI,
            filePath: `/data/storage/el2/base/haps/entry/files/`,
            title: 'XTS download test!',
            background: false
        }
        console.info("====>-----------------------SUB_REQUEST_DOWNLOAD_API_QUERY_0002 is starting-----------------------");
        downloadConfig.filePath += 'SUB_REQUEST_DOWNLOAD_API_QUERY_0002.txt'
        request.download( downloadConfig, (err, data)=>{
            downloadTask = data;
            console.info("====>SUB_REQUEST_DOWNLOAD_API_QUERY_0002 downloadTask: " + downloadTask);
            try{
                expect(downloadTask != undefined).assertEqual(true);
                downloadTask.query().then((downloadInfo)=>{
                    try{
                        if(err) {
                            console.error('====>SUB_REQUEST_DOWNLOAD_API_QUERY_0002 Failed to query: ' + JSON.stringify(err));
                            expect().assertFail();
                        } else {
                            console.info('====>SUB_REQUEST_DOWNLOAD_API_QUERY_0002 query info: '+ downloadInfo.description);
                            console.info('====>SUB_REQUEST_DOWNLOAD_API_QUERY_0002 query info: '+ downloadInfo.downloadedBytes);
                            console.info('====>SUB_REQUEST_DOWNLOAD_API_QUERY_0002 query info: '+ downloadInfo.downloadId);
                            console.info('====>SUB_REQUEST_DOWNLOAD_API_QUERY_0002 query info: '+ downloadInfo.failedReason);
                            console.info('====>SUB_REQUEST_DOWNLOAD_API_QUERY_0002 query info: '+ downloadInfo.fileName);
                            console.info('====>SUB_REQUEST_DOWNLOAD_API_QUERY_0002 query info: '+ downloadInfo.filePath);
                            console.info('====>SUB_REQUEST_DOWNLOAD_API_QUERY_0002 query info: '+ downloadInfo.pausedReason);
                            console.info('====>SUB_REQUEST_DOWNLOAD_API_QUERY_0002 query info: '+ downloadInfo.status);
                            console.info('====>SUB_REQUEST_DOWNLOAD_API_QUERY_0002 query info: '+ downloadInfo.targetURI);
                            console.info('====>SUB_REQUEST_DOWNLOAD_API_QUERY_0002 query info: '+ downloadInfo.downloadTitle);
                            console.info('====>SUB_REQUEST_DOWNLOAD_API_QUERY_0002 query info: '+ downloadInfo.downloadTotalBytes);
                            expect(true).assertTrue();
                        }
                        done();
                    }catch(err){
                        console.error("====>SUB_REQUEST_DOWNLOAD_API_QUERY_0002 error: " + JSON.stringify(err));
                        done();
                    }
                }).catch((err)=>{
                    console.error("====>SUB_REQUEST_DOWNLOAD_API_QUERY_0002  catch_error: " + JSON.stringify(err));
                    expect().assertFail();
                    done();
                })
            }catch(err){
                console.error("====>SUB_REQUEST_DOWNLOAD_API_QUERY_0002 error: " + JSON.stringify(err));
                done();
            }
        })
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_QUERYMINETYPE_0001
     * @tc.desc      Queries the MIME type of the download file.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_QUERYMINETYPE_0001', 0, async function (done) {
        downloadConfig = {
            url: 'https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png',
            header: {
                headers: 'http'
            },
            enableMetered: false,
            enableRoaming: false,
            description: 'XTS download test!',
            networkType: request.NETWORK_WIFI,
            filePath: `/data/storage/el2/base/haps/entry/files/`,
            title: 'XTS download test!',
            background: false
        }
        console.info("====>---------------------SUB_REQUEST_DOWNLOAD_API_QUERYMINETYPE_0001 is starting---------------------");
        downloadConfig.filePath += 'SUB_REQUEST_DOWNLOAD_API_QUERYMINETYPE_0001.txt'
        request.download( downloadConfig, (err, data)=>{
            downloadTask = data;
            console.info("====>SUB_REQUEST_DOWNLOAD_API_QUERYMINETYPE_0001 downloadTask: " + downloadTask);
            try{
                expect(downloadTask != undefined).assertEqual(true);
                downloadTask.on('complete', () => {
                    try{
                        downloadTask.queryMimeType((err, data)=>{
                            try{
                                if(err) {
                                    console.error('====>SUB_REQUEST_DOWNLOAD_API_QUERYMINETYPE_0001 Failed to queryMimeType the download task.');
                                    expect().assertFail();
                                }
                                if (data) {
                                    console.info('====>SUB_REQUEST_DOWNLOAD_API_QUERYMINETYPE_0001 data:' + JSON.stringify(data));
                                    expect(typeof data == "string").assertTrue();
                                } else {
                                    console.error("====>SUB_REQUEST_DOWNLOAD_API_QUERYMINETYPE_0001 error: " + JSON.stringify(err));
                                    expect().assertFail();
                                }
                            }catch(err){
                                console.error("====>SUB_REQUEST_DOWNLOAD_API_QUERYMINETYPE_0001 throw_error: " + JSON.stringify(err));
                            }
                            done();
                        });
                        console.info('====>SUB_REQUEST_DOWNLOAD_API_QUERYMINETYPE_0001 task completed.')
                        expect(true).assertTrue();
                    }catch(err){
                        console.error("====>SUB_REQUEST_DOWNLOAD_API_QUERYMINETYPE_0001 throw_error: " + err);
                    }
                    done();
                });
            }catch(err){
                console.error("====>SUB_REQUEST_DOWNLOAD_API_QUERYMINETYPE_0001 error: " + JSON.stringify(err));
                done();
            }
        })
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_QUERYMINETYPE_0002
     * @tc.desc      Queries the MIME type of the download file.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_QUERYMINETYPE_0002', 0, async function (done) {
        downloadConfig = {
            url: 'https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png',
            header: {
                headers: 'http'
            },
            enableMetered: false,
            enableRoaming: false,
            description: 'XTS download test!',
            networkType: request.NETWORK_WIFI,
            filePath: `/data/storage/el2/base/haps/entry/files/`,
            title: 'XTS download test!',
            background: false
        }
        console.info("====>-------------------SUB_REQUEST_DOWNLOAD_API_QUERYMINETYPE_0002 is starting----------------------");
        downloadConfig.filePath += 'SUB_REQUEST_DOWNLOAD_API_QUERYMINETYPE_0002.txt'
        request.download( downloadConfig, (err, data)=>{
            downloadTask = data;
            console.info("====>SUB_REQUEST_DOWNLOAD_API_QUERYMINETYPE_0002 downloadTask: " + downloadTask);
            try{
                expect(downloadTask != undefined).assertEqual(true);
                downloadTask.on('complete', () => {
                    try{
                        downloadTask.queryMimeType().then(data => {
                            if (data) {
                                console.info('====>SUB_REQUEST_DOWNLOAD_API_QUERYMINETYPE_0002 Download task queryMimeType.');
                                expect(data == true).assertTrue();
                            } else {
                                console.error('====>SUB_REQUEST_DOWNLOAD_API_QUERYMINETYPE_0002 Failed to queryMimeType the download task.');
                                expect().assertFail();
                            }
                            done();
                        }).catch((err) => {
                            console.error("====>SUB_REQUEST_DOWNLOAD_API_QUERYMINETYPE_0002 throw_error: " + JSON.stringify(err));
                            expect().assertFail();
                            done();
                        })
                        console.info('====>SUB_REQUEST_DOWNLOAD_API_QUERYMINETYPE_0002 task completed.')
                        expect(true).assertTrue();
                    }catch(err){
                        console.error("====>SUB_REQUEST_DOWNLOAD_API_QUERYMINETYPE_0002 throw_error: " + err);
                    }
                    done();
                });
            }catch(err){
                console.error("====>SUB_REQUEST_DOWNLOAD_API_QUERYMINETYPE_0002 error: " + JSON.stringify(err));
                done();
            }
        })
    });
    console.log(TAG + "*************Unit Test End*************");
})
