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

    /**
     * @tc.name: downloadTest001
     * @tc.desc download parameter verification
     * @tc.type: FUNC
     * @tc.require: 000000
     */
    it('downloadTest001', 0, function () {
        console.log(TAG + "************* downloadTest001 start *************");
        let config;
        try {
            request.download(config);
        } catch (err) {
            expect(true).assertEqual(true);
        }
        console.log(TAG + "************* downloadTest001 end *************");
    });

    let downloadTask;
    let downloadConfig = {
        url: 'http://127.0.0.1',
        header: {
            headers: 'http'
        },
        enableMetered: false,
        enableRoaming: false,
        description: 'XTS download test!',
        networkType: request.NETWORK_WIFI,
        filePath: 'test.txt',
        title: 'XTS download test!',
        background: true
    }

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_CALLBACK_0001
     * @tc.desc      Starts a download session.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_CALLBACK_0001', 0, async function (done) {
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_CALLBACK_0001 is starting-----------------------");
        try {
            request.download(downloadConfig, (data)=>{
                downloadTask = data;
                console.info("SUB_REQUEST_DOWNLOAD_API_CALLBACK_0001 downloadTask: " + downloadTask);
                expect(downloadTask !== undefined).assertEqual(true);
            });
        } catch (err) {
            console.error("SUB_REQUEST_DOWNLOAD_API_CALLBACK_0001 error: " + err);
            expect().assertFail();
        }
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_CALLBACK_0001 end-----------------------");
        done();
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_PROMISE_0001
     * @tc.desc      Starts a download session.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_PROMISE_0001', 0, async function (done) {
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_PROMISE_0001 is starting-----------------------");
        request.download(downloadConfig).then(data => {
            downloadTask = data;
            console.info("SUB_REQUEST_DOWNLOAD_API_PROMISE_0001 downloadTask: " + downloadTask);
            expect(downloadTask !== undefined).assertEqual(true);
        }).catch(err => {
            console.error("SUB_REQUEST_DOWNLOAD_API_PROMISE_0001 error: " + err);
            expect().assertFail();
        })
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_PROMISE_0001 end-----------------------");
        done();
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0001
     * @tc.desc      alled when the current download session is in process.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0001', 0, async function (done) {
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0001 is starting-----------------------");
        request.download( downloadConfig, (data)=>{
            downloadTask = data;
            console.info("SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0001 downloadTask: " + downloadTask);
            expect(true).assertEqual(downloadTask !== undefined);
            downloadTask.on('progress', (data1, data2) => {
                console.info("SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0001 on data1 =" + data1);
                console.info("SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0001 on data2 =" + data2);
                expect(true).assertEqual(data1 !== undefined);
                expect(true).assertEqual(data2 !== undefined);
            });
        });

        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0001 end-----------------------");
        done();
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0002
     * @tc.desc       Called when the current download session complete、pause or remove.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0002', 0, async function (done) {
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0002 is starting-----------------------");
        request.download( downloadConfig, (data)=>{
            downloadTask = data;
            console.info("SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0002 downloadTask: " + downloadTask);
            expect(true).assertEqual(downloadTask !== undefined);
            try{
                downloadTask.on('complete', () => {
                    console.info('SUB_REQUEST_DOWNLOAD_API_CALLBACK_0002 task completed.')
                });
            }catch(err){
                console.error("SUB_REQUEST_DOWNLOAD_API_CALLBACK_0002 error: " + err);
                expect().assertFail();
            }
        });
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0002 end-----------------------");
        done();
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0003
     * @tc.desc       Called when the current download session complete、pause or remove.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0003', 0, async function (done) {
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0003 is starting-----------------------");
        request.download( downloadConfig, (data)=>{
            downloadTask = data;
            console.info("SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0003 downloadTask: " + downloadTask);
            expect(true).assertEqual(downloadTask !== undefined);
            try{
                downloadTask.on('pause', () => {
                    console.info('SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0003 task pause.')
                });
            }catch(err){
                console.error("SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0003 error: " + err);
                expect().assertFail();
            }
        });
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0003 end-----------------------");
        done();
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0004
     * @tc.desc       Called when the current download session complete、pause or remove.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0004', 0, async function (done) {
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0004 is starting-----------------------");
        request.download( downloadConfig, (data)=>{
            downloadTask = data;
            console.info("SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0004 downloadTask: " + downloadTask);
            expect(downloadTask !== undefined).assertEqual(true);
            try{
                downloadTask.on('remove', () => {
                    console.info('SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0004 task remove.')
                });
            }catch(err){
                console.error("SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0004 error: " + err);
                expect().assertFail();
            }
        });
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0004 end-----------------------");
        done();
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0005
     * @tc.desc      Called when the current download session fails.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0005', 0, async function (done) {
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0005 is starting-----------------------");
        request.download( downloadConfig, (data)=>{
            downloadTask = data;
            console.info("SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0005 downloadTask: " + downloadTask);
            expect(downloadTask !== undefined).assertEqual(true);
            try{
                downloadTask.on('remove', () => {
                    console.info('SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0005 task remove.')
                });
            }catch(err){
                console.error("SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0005 error: " + err);
                expect().assertFail();
            }
        });
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_ON_0005 end-----------------------");
        done();
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0001
     * @tc.desc      alled when the current download session is in process.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0001', 0, async function (done) {
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0001 is starting-----------------------");
        request.download( downloadConfig, (data)=>{
            downloadTask = data;
            console.info("SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0001 downloadTask: " + downloadTask);
            expect(downloadTask !== undefined).assertEqual(true);
            downloadTask.off('progress', (data1, data2) => {
                console.info("SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0001 on data1 =" + data1);
                console.info("SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0001 on data2 =" + data2);
                expect(data1 !== undefined).assertEqual(true);
                expect(data2 !== undefined).assertEqual(true);
            });
        });
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0001 end-----------------------");
        done();
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0002
     * @tc.desc      alled when the current download session complete、pause or remove.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0002', 0, async function (done) {
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0002 is starting-----------------------");
        request.download( downloadConfig, (data)=>{
            downloadTask = data;
            console.info("SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0002 downloadTask: " + downloadTask);
            expect(downloadTask !== undefined).assertEqual(true);
            try{
                downloadTask.off('complete', () => {
                    console.info('SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0002 task complete.')
                });
            }catch(err){
                console.error("SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0002 error: " + err);
                expect().assertFail();
            }
        });
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0002 end-----------------------");
        done();
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0003
     * @tc.desc      alled when the current download session complete、pause or remove.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0003', 0, async function (done) {
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0003 is starting-----------------------");
        request.download( downloadConfig, (data)=>{
            downloadTask = data;
            console.info("SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0003 downloadTask: " + downloadTask);
            expect(downloadTask !== undefined).assertEqual(true);
            try{
                downloadTask.off('pause', () => {
                    console.info('SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0003 task pause.')
                });
            }catch(err){
                console.error("SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0003 error: " + err);
                expect().assertFail();
            }
        });
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0002 end-----------------------");
        done();
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0004
     * @tc.desc      alled when the current download session complete、pause or remove.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0004', 0, async function (done) {
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0004 is starting-----------------------");
        request.download( downloadConfig, (data)=>{
            downloadTask = data;
            console.info("SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0004 downloadTask: " + downloadTask);
            expect(downloadTask !== undefined).assertEqual(true);
            try{
                downloadTask.off('remove', () => {
                    console.info('SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0004 task remove.')
                });
            }catch(err){
                console.error("SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0004 error: " + err);
                expect().assertFail();
            }
        });
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0004 end-----------------------");
        done();
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0005
     * @tc.desc      Called when the current download session fails.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0005', 0, async function (done) {
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0005 is starting-----------------------");
        request.download( downloadConfig, (data)=>{
            downloadTask = data;
            console.info("SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0005 downloadTask: " + downloadTask);
            expect(downloadTask !== undefined).assertEqual(true);
            try{
                downloadTask.off('pause', () => {
                    console.info('SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0005 task complete.')
                });
            }catch(err){
                console.error("SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0005 error: " + err);
                expect().assertFail();
            }
        });
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0002 end-----------------------");
        done();
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_REMOVE_0001
     * @tc.desc      Deletes a download session and the downloaded files.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_REMOVE_0001', 0, async function (done) {
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_REMOVE_0001 is starting-----------------------");
        request.download( downloadConfig, (data)=>{
            downloadTask = data;
            console.info("SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0005 downloadTask: " + downloadTask);
            expect(downloadTask !== undefined).assertEqual(true);
            downloadTask.remove((err, data)=>{
                if(err) {
                    console.error('SUB_REQUEST_DOWNLOAD_API_REMOVE_0001 Failed to remove the download task.');
                    expect().assertFail();
                }
                if (data) {
                    console.info('SUB_REQUEST_DOWNLOAD_API_REMOVE_0001 Download task removed.');
                    expect(data === true).assertTrue();
                } else {
                    console.error('SUB_REQUEST_DOWNLOAD_API_REMOVE_0001 Failed to remove the download task.');
                    expect().assertFail();
                }
            });
        })
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_REMOVE_0001 end-----------------------");
        done();
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_REMOVE_0002
     * @tc.desc      Deletes a download session and the downloaded files.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_REMOVE_0002', 0, async function (done) {
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_REMOVE_0002 is starting-----------------------");
        request.download( downloadConfig, (data)=>{
            downloadTask = data;
            console.info("SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0005 downloadTask: " + downloadTask);
            expect(downloadTask !== undefined).assertEqual(true);
            downloadTask.remove().then(data => {
                if (data) {
                    console.info('SUB_REQUEST_DOWNLOAD_API_REMOVE_0002 Download task removed.');
                    expect(data === true).assertTrue();
                } else {
                    console.error('SUB_REQUEST_DOWNLOAD_API_REMOVE_0002 Failed to remove the download task.');
                    expect().assertFail();
                }
            }).catch((err) => {
                console.error('SUB_REQUEST_DOWNLOAD_API_REMOVE_0002 Failed to remove the download task.');
                expect().assertFail();
            })
        })
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_REMOVE_0002 end-----------------------");
        done();
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_PAUSE_0001
     * @tc.desc      Pause a download session.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_PAUSE_0001', 0, async function (done) {
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_PAUSE_0001 is starting-----------------------");
        request.download( downloadConfig, (data)=>{
            downloadTask = data;
            console.info("SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0005 downloadTask: " + downloadTask);
            expect(downloadTask !== undefined).assertEqual(true);
            try{
                downloadTask.pause(()=>{
                    console.info('SUB_REQUEST_DOWNLOAD_API_PAUSE_0001 Download task pause success.');
                    expect(true).assertTrue();
                })
            }catch(err){
                console.error('Failed to pause the download task pause. because: ' + JSON.stringify(err));
                expect().assertFail();
            }
        })
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_PAUSE_0001 end-----------------------");
        done();
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_PAUSE_0002
     * @tc.desc      Pause a download session.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_PAUSE_0002', 0, async function (done) {
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_PAUSE_0002 is starting-----------------------");
        request.download( downloadConfig, (data)=>{
            downloadTask = data;
            console.info("SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0005 downloadTask: " + downloadTask);
            expect(downloadTask !== undefined).assertEqual(true);
            downloadTask.pause().then(() => {
                console.info('SUB_REQUEST_DOWNLOAD_API_PAUSE_0002 Download task pause success.');
                expect(true).assertTrue();
            }).catch((err) => {
                console.error('Failed to pause the download task pause. because: ' + JSON.stringify(err));
                expect().assertFail();
            })
        })
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_PAUSE_0002 end-----------------------");
        done();
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_REMUSE_0001
     * @tc.desc      Resume a paused download session.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_REMUSE_0001', 0, async function (done) {
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_REMUSE_0001 is starting-----------------------");
        request.download( downloadConfig, (data)=>{
            downloadTask = data;
            console.info("SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0005 downloadTask: " + downloadTask);
            expect(downloadTask !== undefined).assertEqual(true);
            try{
                downloadTask.resume(()=>{
                    console.info('SUB_REQUEST_DOWNLOAD_API_REMUSE_0001 Download task resume success.');
                    expect(true).assertTrue();
                })
            }catch(err){
                console.error('Failed to pause the download task resume. because: ' + JSON.stringify(err));
                expect().assertFail();
            }
        })
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_REMUSE_0001 end-----------------------");
        done();
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_REMUSE_0002
     * @tc.desc      Resume a paused download session.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_REMUSE_0002', 0, async function (done) {
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_REMUSE_0002 is starting-----------------------");
        request.download( downloadConfig, (data)=>{
            downloadTask = data;
            console.info("SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0005 downloadTask: " + downloadTask);
            expect(downloadTask !== undefined).assertEqual(true);
            downloadTask.resume().then(() => {
                console.info('SUB_REQUEST_DOWNLOAD_API_REMUSE_0002 Download task resume success.');
                expect(true).assertTrue();
            }).catch((err) => {
                console.error('Failed to pause the download task resume. because: ' + JSON.stringify(err));
                expect().assertFail();
            })
        })
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_REMUSE_0002 end-----------------------");
        done();
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_QUERY_0001
     * @tc.desc      Queries download information of a session, which is defined in DownloadSession.DownloadInfo.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_QUERY_0001', 0, async function (done) {
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_QUERY_0001 is starting-----------------------");
        request.download( downloadConfig, (data)=>{
            downloadTask = data;
            console.info("SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0005 downloadTask: " + downloadTask);
            expect(downloadTask !== undefined).assertEqual(true);
            try{
                downloadTask.query((err, downloadInfo)=>{
                    if(err) {
                        console.error('SUB_REQUEST_DOWNLOAD_API_QUERY_0001 Failed to query: ' + JSON.stringify(err));
                        expect().assertFail();
                    } else {
                        console.info('SUB_REQUEST_DOWNLOAD_API_QUERY_0001 query info: '+ downloadInfo.description);
                        console.info('SUB_REQUEST_DOWNLOAD_API_QUERY_0001 query info: '+ downloadInfo.downloadedBytes);
                        console.info('SUB_REQUEST_DOWNLOAD_API_QUERY_0001 query info: '+ downloadInfo.downloadId);
                        console.info('SUB_REQUEST_DOWNLOAD_API_QUERY_0001 query info: '+ downloadInfo.failedReason);
                        console.info('SUB_REQUEST_DOWNLOAD_API_QUERY_0001 query info: '+ downloadInfo.fileName);
                        console.info('SUB_REQUEST_DOWNLOAD_API_QUERY_0001 query info: '+ downloadInfo.filePath);
                        console.info('SUB_REQUEST_DOWNLOAD_API_QUERY_0001 query info: '+ downloadInfo.pausedReason);
                        console.info('SUB_REQUEST_DOWNLOAD_API_QUERY_0001 query info: '+ downloadInfo.status);
                        console.info('SUB_REQUEST_DOWNLOAD_API_QUERY_0001 query info: '+ downloadInfo.targetURI);
                        console.info('SUB_REQUEST_DOWNLOAD_API_QUERY_0001 query info: '+ downloadInfo.downloadTitle);
                        console.info('SUB_REQUEST_DOWNLOAD_API_QUERY_0001 query info: '+ downloadInfo.downloadTotalBytes);
                        expect(true).assertTrue();
                    }
                })
            }catch(err){
                console.error('Failed to pause the download task query. because: ' + JSON.stringify(err));
                expect().assertFail();
            }
        })
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_QUERY_0001 end-----------------------");
        done();
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_QUERY_0002
     * @tc.desc      Queries download information of a session, which is defined in DownloadSession.DownloadInfo.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_QUERY_0002', 0, async function (done) {
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_QUERY_0002 is starting-----------------------");
        request.download( downloadConfig, (data)=>{
            downloadTask = data;
            console.info("SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0005 downloadTask: " + downloadTask);
            expect(downloadTask !== undefined).assertEqual(true);
            downloadTask.query().then((err, downloadInfo)=>{
                if(err) {
                    console.error('SUB_REQUEST_DOWNLOAD_API_QUERY_0002 Failed to query: ' + JSON.stringify(err));
                    expect().assertFail();
                } else {
                    console.info('SUB_REQUEST_DOWNLOAD_API_QUERY_0001 query info: '+ downloadInfo.description);
                    console.info('SUB_REQUEST_DOWNLOAD_API_QUERY_0001 query info: '+ downloadInfo.downloadedBytes);
                    console.info('SUB_REQUEST_DOWNLOAD_API_QUERY_0001 query info: '+ downloadInfo.downloadId);
                    console.info('SUB_REQUEST_DOWNLOAD_API_QUERY_0001 query info: '+ downloadInfo.failedReason);
                    console.info('SUB_REQUEST_DOWNLOAD_API_QUERY_0001 query info: '+ downloadInfo.fileName);
                    console.info('SUB_REQUEST_DOWNLOAD_API_QUERY_0001 query info: '+ downloadInfo.filePath);
                    console.info('SUB_REQUEST_DOWNLOAD_API_QUERY_0001 query info: '+ downloadInfo.pausedReason);
                    console.info('SUB_REQUEST_DOWNLOAD_API_QUERY_0001 query info: '+ downloadInfo.status);
                    console.info('SUB_REQUEST_DOWNLOAD_API_QUERY_0001 query info: '+ downloadInfo.targetURI);
                    console.info('SUB_REQUEST_DOWNLOAD_API_QUERY_0001 query info: '+ downloadInfo.downloadTitle);
                    console.info('SUB_REQUEST_DOWNLOAD_API_QUERY_0001 query info: '+ downloadInfo.downloadTotalBytes);
                    expect(true).assertTrue();
                }
            }).catch(err => {
                console.error('Failed to pause the download task query. because: ' + JSON.stringify(err));
                expect().assertFail();
            })
        })
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_QUERY_0002 end-----------------------");
        done();
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_QUERYMINETYPE_0001
     * @tc.desc      Queries the MIME type of the download file.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_QUERYMINETYPE_0001', 0, async function (done) {
        console.info("---------------------SUB_REQUEST_DOWNLOAD_API_QUERYMINETYPE_0001 is starting---------------------");
        request.download( downloadConfig, (data)=>{
            downloadTask = data;
            console.info("SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0005 downloadTask: " + downloadTask);
            expect(downloadTask !== undefined).assertEqual(true);
            downloadTask.queryMimeType((err, data)=>{
                if(err) {
                    console.error('SUB_REQUEST_DOWNLOAD_API_QUERYMINETYPE_0001 Failed to queryMimeType the download task.');
                    expect().assertFail();
                }
                if (data) {
                    console.info('SUB_REQUEST_DOWNLOAD_API_QUERYMINETYPE_0001 Download task queryMimeType.');
                    expect(typeof data == "string").assertTrue();
                } else {
                    console.error('SUB_REQUEST_DOWNLOAD_API_QUERYMINETYPE_0001 Failed to queryMimeType the download task.');
                    expect().assertFail();
                }
            });
        })
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_QUERYMINETYPE_0001 end-----------------------");
        done();
    });

    /**
     * @tc.number    SUB_REQUEST_DOWNLOAD_API_QUERYMINETYPE_0002
     * @tc.desc      Queries the MIME type of the download file.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 2
     */
    it('SUB_REQUEST_DOWNLOAD_API_QUERYMINETYPE_0002', 0, async function (done) {
        console.info("-------------------SUB_REQUEST_DOWNLOAD_API_QUERYMINETYPE_0002 is starting----------------------");
        request.download( downloadConfig, (data)=>{
            downloadTask = data;
            console.info("SUB_REQUEST_DOWNLOAD_API_DOWNLOADTASK_OFF_0005 downloadTask: " + downloadTask);
            expect(downloadTask !== undefined).assertEqual(true);
            downloadTask.queryMimeType().then(data => {
                if (data) {
                    console.info('SUB_REQUEST_DOWNLOAD_API_QUERYMINETYPE_0002 Download task queryMimeType.');
                    expect(data === true).assertTrue();
                } else {
                    console.error('SUB_REQUEST_DOWNLOAD_API_QUERYMINETYPE_0002 Failed to queryMimeType the download task.');
                    expect().assertFail();
                }
            }).catch((err) => {
                console.error('SUB_REQUEST_DOWNLOAD_API_QUERYMINETYPE_0002 Failed to queryMimeType the download task.');
                expect().assertFail();
            })
        })
        console.info("-----------------------SUB_REQUEST_DOWNLOAD_API_QUERYMINETYPE_0002 end-----------------------");
        done();
    });
    console.log(TAG + "*************Unit Test End*************");
})
