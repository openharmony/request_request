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
import featureAbility from '@ohos.ability.featureAbility';
import fs from '@ohos.file.fs';

const TAG = "UPLOAD_TEST";

describe('RequestUploadTest', async function () {
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
        console.info('afterEach called');
        if (fs.accessSync(cacheDir + '/test.txt')) {
          fs.unlinkSync(cacheDir + '/test.txt');
        }
    })
    console.log(TAG + "*************Unit Test Begin*************");

    let cacheDir = await featureAbility.getContext().getCacheDir();

    let RequestData = {
      name: 'name',
      value: '123'
    }

    let File = {
      filename: 'test',
      name: 'test',
      uri: 'internal://cache/test.txt',
      type: 'txt'
    }

    let File1 = {
      filename: 'test1',
      name: 'test',
      uri: 'internal://cache/test1.txt',
      type: 'txt'
    }

    let File2 = {
      filename: 'test2',
      name: 'test',
      uri: 'internal://cache/test2.txt',
      type: 'txt'
    }

    let uploadConfig = {
      url: 'http://127.0.0.1',
      header: {
        headers: 'http'
      },
      method: 'POST',
      files: [File],
      data: [RequestData]
    };

    let uploadConfig1 = {
        url: 'http://127.0.0.1',
        header: {
          headers: 'http'
        },
        method: 'PUT',
        files: [File],
        data: [RequestData]
    };

    let uploadConfig2 = {
      url:  'http://127.0.0.1',
      header: {
        headers: 'http'
      },
      index: 1,
      begins: 50,
      ends: 100,
      method: 'POST',
      files: [File1, File2],
      data: [RequestData]
    };

    let uploadConfig3 = {
      url:  'http://127.0.0.1',
      header: {
        headers: 'http'
      },
      index: 1,
      begins: "50",
      ends: 100,
      method: 'POST',
      files: [File1, File2],
      data: [RequestData]
    };

    let uploadConfig4 = {
      url:  'http://127.0.0.1',
      header: {
        headers: 'http'
      },
      index: 1,
      begins: undefined,
      ends: 100,
      method: 'POST',
      files: [File1, File2],
      data: [RequestData]
    };

    /**
     * @tc.number    SUB_REQUEST_UPLOAD_API_0001
     * @tc.name      Test requestUploadTest type = TIMER_TYPE_REALTIME
     * @tc.desc      Test requestUploadTest API functionality.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 0
     */
    it('SUB_REQUEST_UPLOAD_API_0001', 0, async function (done) {
      console.info("-----------------------SUB_REQUEST_UPLOAD_API_0001 is starting-----------------------");
      try {
        console.info("SUB_REQUEST_UPLOAD_API_0001 request.NETWORK_MOBILE: " + request.NETWORK_MOBILE);
        expect(request.NETWORK_MOBILE).assertEqual(1);
        console.info("SUB_REQUEST_UPLOAD_API_0001 request.NETWORK_WIFI: " + request.NETWORK_WIFI);
        expect(request.NETWORK_WIFI).assertEqual(65536);
        console.info("SUB_REQUEST_UPLOAD_API_0001 request.ERROR_CANNOT_RESUME: " + request.ERROR_CANNOT_RESUME);
        expect(request.ERROR_CANNOT_RESUME).assertEqual(0);
        console.info("SUB_REQUEST_UPLOAD_API_0001 request.ERROR_DEVICE_NOT_FOUND: " + request.ERROR_DEVICE_NOT_FOUND);
        expect(request.ERROR_DEVICE_NOT_FOUND).assertEqual(1);
      } catch (err) {
        console.error("SUB_REQUEST_UPLOAD_API_0001 error: " + err);
        expect().assertFail();
      }
      console.info("-----------------------SUB_REQUEST_UPLOAD_API_0001 end-----------------------");
      done();
    });

    /**
     * @tc.number    SUB_REQUEST_UPLOAD_API_0002
     * @tc.name      Test requestUploadTest type = TIMER_TYPE_REALTIME
     * @tc.desc      Test requestUploadTest API functionality.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 0
     */
    it('SUB_REQUEST_UPLOAD_API_0002', 0, async function (done) {
      console.info("-----------------------SUB_REQUEST_UPLOAD_API_0002 is starting-----------------------");
      try {
        console.info("SUB_REQUEST_UPLOAD_API_0002 request.ERROR_FILE_ALREADY_EXISTS: " + request.ERROR_FILE_ALREADY_EXISTS);
        expect(request.ERROR_FILE_ALREADY_EXISTS).assertEqual(2);
        console.info("SUB_REQUEST_UPLOAD_API_0002 request.ERROR_FILE_ERROR: " + request.ERROR_FILE_ERROR);
        expect(request.ERROR_FILE_ERROR).assertEqual(3);
        console.info("SUB_REQUEST_UPLOAD_API_0002 request.ERROR_HTTP_DATA_ERROR: " + request.ERROR_HTTP_DATA_ERROR);
        expect(request.ERROR_HTTP_DATA_ERROR).assertEqual(4);
        console.info("SUB_REQUEST_UPLOAD_API_0002 request.ERROR_INSUFFICIENT_SPACE: " + request.ERROR_INSUFFICIENT_SPACE);
        expect(request.ERROR_INSUFFICIENT_SPACE).assertEqual(5);
      } catch (err) {
        console.error("SUB_REQUEST_UPLOAD_API_0002 error: " + err);
        expect().assertFail();
      }
      console.info("-----------------------SUB_REQUEST_UPLOAD_API_0002 end-----------------------");
      done();
    });

    /**
     * @tc.number    SUB_REQUEST_UPLOAD_API_0003
     * @tc.name      Test requestUploadTest type = TIMER_TYPE_REALTIME
     * @tc.desc      Test requestUploadTest API functionality.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 0
     */
    it('SUB_REQUEST_UPLOAD_API_0003', 0, async function (done) {
      console.info("-----------------------SUB_REQUEST_UPLOAD_API_0003 is starting-----------------------");
      try {
        console.info("SUB_REQUEST_UPLOAD_API_0003 request.ERROR_TOO_MANY_REDIRECTS:" + request.ERROR_TOO_MANY_REDIRECTS);
        expect(request.ERROR_TOO_MANY_REDIRECTS).assertEqual(6);
        console.info("SUB_REQUEST_UPLOAD_API_0003 request.ERROR_UNHANDLED_HTTP_CODE:" + request.ERROR_UNHANDLED_HTTP_CODE);
        expect(request.ERROR_UNHANDLED_HTTP_CODE).assertEqual(7);
        console.info("SUB_REQUEST_UPLOAD_API_0003 request.ERROR_UNKNOWN:" + request.ERROR_UNKNOWN);
        expect(request.ERROR_UNKNOWN).assertEqual(8);
        console.info("SUB_REQUEST_UPLOAD_API_0003 request.PAUSED_QUEUED_FOR_WIFI:" + request.PAUSED_QUEUED_FOR_WIFI);
        expect(request.PAUSED_QUEUED_FOR_WIFI).assertEqual(0);
      } catch (err) {
        console.error("SUB_REQUEST_UPLOAD_API_0003 error: " + err);
        expect().assertFail();
      }
      console.info("-----------------------SUB_REQUEST_UPLOAD_API_0003 end-----------------------");
      done();
    });

    /**
     * @tc.number    SUB_REQUEST_UPLOAD_API_0004
     * @tc.name      Test requestUploadTest type = TIMER_TYPE_REALTIME
     * @tc.desc      Test requestUploadTest API functionality.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 0
     */
    it('SUB_REQUEST_UPLOAD_API_0004', 0, async function (done) {
      console.info("-----------------------SUB_REQUEST_UPLOAD_API_0004 is starting-----------------------");
      try {
        console.info("SUB_REQUEST_UPLOAD_API_0004 request.PAUSED_UNKNOWN:" + request.PAUSED_UNKNOWN);
        expect(request.PAUSED_UNKNOWN).assertEqual(4);
        console.info("SUB_REQUEST_UPLOAD_API_0004 request.PAUSED_WAITING_FOR_NETWORK:" + request.PAUSED_WAITING_FOR_NETWORK);
        expect(request.PAUSED_WAITING_FOR_NETWORK).assertEqual(1);
        console.info("SUB_REQUEST_UPLOAD_API_0004 request.PAUSED_WAITING_TO_RETRY:" + request.PAUSED_WAITING_TO_RETRY);
        expect(request.PAUSED_WAITING_TO_RETRY).assertEqual(2);
        console.info("SUB_REQUEST_UPLOAD_API_0004 request.SESSION_FAILED:" + request.SESSION_FAILED);
        expect(request.SESSION_FAILED).assertEqual(4);
      } catch (err) {
        console.error("SUB_REQUEST_UPLOAD_API_0004 error: " + err);
        expect().assertFail();
      }
      console.info("-----------------------SUB_REQUEST_UPLOAD_API_0004 end-----------------------");
      done();
    });

    /**
     * @tc.number    SUB_REQUEST_UPLOAD_API_0005
     * @tc.name      Test requestUploadTest type = TIMER_TYPE_REALTIME
     * @tc.desc      Test requestUploadTest API functionality.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 0
     */
    it('SUB_REQUEST_UPLOAD_API_0005', 0, async function (done) {
      console.info("-----------------------SUB_REQUEST_UPLOAD_API_0005 is starting-----------------------");
      try {
        console.info("SUB_REQUEST_UPLOAD_API_0005 request.SESSION_PAUSED:" + request.SESSION_PAUSED);
        expect(request.SESSION_PAUSED).assertEqual(3);
        console.info("SUB_REQUEST_UPLOAD_API_0005 request.SESSION_PENDING:" + request.SESSION_PENDING);
        expect(request.SESSION_PENDING).assertEqual(2);
        console.info("SUB_REQUEST_UPLOAD_API_0005 request.SESSION_RUNNING:" + request.SESSION_RUNNING);
        expect(request.SESSION_RUNNING).assertEqual(1);
        console.info("SUB_REQUEST_UPLOAD_API_0005 request.SESSION_SUCCESSFUL:" + request.SESSION_SUCCESSFUL);
        expect(request.SESSION_SUCCESSFUL).assertEqual(0);
      } catch (err) {
        console.error("SUB_REQUEST_UPLOAD_API_0005 error: " + err);
        expect().assertFail();
      }
      console.info("-----------------------SUB_REQUEST_UPLOAD_API_0005 end-----------------------");
      done();
    });

    /**
     * @tc.number    SUB_REQUEST_UPLOAD_API_CALLBACK_0001
     * @tc.name      Test requestUploadTest type = TIMER_TYPE_REALTIME
     * @tc.desc      Test requestUploadTest API functionality.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 1
     */
    it('SUB_REQUEST_UPLOAD_API_CALLBACK_0001', 0, async function (done) {
        let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(file);
        console.info("-----------------------SUB_REQUEST_UPLOAD_API_CALLBACK_0001 is starting-----------------------");
        try {
          console.info("SUB_REQUEST_UPLOAD_API_CALLBACK_0001 uploadConfig: " + JSON.stringify(uploadConfig1));
          request.upload(uploadConfig1, (err, data) => {
            let uploadTask = data;
            console.info("SUB_REQUEST_UPLOAD_API_CALLBACK_0001 progress uploadTask: " + JSON.stringify(uploadTask));
            expect(uploadTask !== undefined).assertEqual(true);

            function ProgressCallback(data1, data2) {
              console.info("SUB_REQUEST_UPLOAD_API_CALLBACK_0001 on data1: " + data1 + ", data1: " + data2);
              expect(true).assertEqual((data1 !== undefined) || (data1 !== "") || (data1 !== {}));
              expect(true).assertEqual((data2 !== undefined) || (data2 != "") || (data2 !== {}));
            }
            uploadTask.on('progress', ProgressCallback);
            uploadTask.off('progress', ProgressCallback);
  
            uploadTask.remove((result) => {
              console.info("SUB_REQUEST_UPLOAD_API_CALLBACK_0001 remove: " + result);
              expect(result).assertEqual(true);
            });
          });
        } catch (err) {
          console.error("SUB_REQUEST_UPLOAD_API_CALLBACK_0001 error: " + err);
          expect().assertFail();
        }
        console.info("-----------------------SUB_REQUEST_UPLOAD_API_CALLBACK_0001 end-----------------------");
        done();
    });
  
    /**
     * @tc.number    : SUB_REQUEST_UPLOAD_API_PROMISE_0001
     * @tc.name      : Use getEntries get the value by mixing the string key
     * @tc.desc      : Mixed strings value can be obtained correctly
     * @tc.size      : MediumTest
     * @tc.type      : Function
     * @tc.level     : Level 1
     */
    it('SUB_REQUEST_UPLOAD_API_PROMISE_0001', 0, async function (done) {
        let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(file);
        console.info("-----------------------SUB_REQUEST_UPLOAD_API_PROMISE_0001 is starting-----------------------");
        try {
          console.info("SUB_REQUEST_UPLOAD_API_PROMISE_0001 UploadConfig: " + JSON.stringify(uploadConfig1));
          request.upload(uploadConfig1).then((data) => {
            let uploadTask = data;
            console.info("SUB_REQUEST_UPLOAD_API_PROMISE_0001 uploadTask: " + uploadTask);
            expect(true).assertEqual((uploadTask !== undefined) || (uploadTask !== "") || (uploadTask !== {}));
            function HeaderReceiveCallback(header) {
              console.info("SUB_REQUEST_UPLOAD_API_PROMISE_0001 on header: " + header);
              expect(true).assertEqual((header !== undefined) || (header !== "") || (header !== {}));
            }
            uploadTask.on('headerReceive', HeaderReceiveCallback);
            uploadTask.off('headerReceive', HeaderReceiveCallback);
  
            uploadTask.remove().then((result)=>{
              console.info("SUB_REQUEST_UPLOAD_API_PROMISE_0001 remove result: " + result);
              expect(result).assertEqual(true);
            });
          });
        } catch (e) {
          console.error("SUB_REQUEST_UPLOAD_API_PROMISE_0001 error: " + JSON.stringify(e));
          expect().assertFail();
        }
        console.info("-----------------------SUB_REQUEST_UPLOAD_API_PROMISE_0001 end-----------------------");
        done();
    });
  
    /**
     * @tc.number    : SUB_REQUEST_UPLOAD_API_PROMISE_0002
     * @tc.name      : Use getEntries get the value by mixing the string key
     * @tc.desc      : Called when the current upload session complete or fail.
     * @tc.size      : MediumTest
     * @tc.type      : Function
     * @tc.level     : Level 1
     */
    it('SUB_REQUEST_UPLOAD_API_PROMISE_0002', 0, async function (done) {
        let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(file);
        console.info("-----------------------SUB_REQUEST_UPLOAD_API_PROMISE_0002 is starting-----------------------");
        try {
          console.info("SUB_REQUEST_UPLOAD_API_PROMISE_0002 UploadConfig: " + JSON.stringify(uploadConfig));
          request.upload(uploadConfig).then((data) => {
            let uploadTask = data;
            console.info("SUB_REQUEST_UPLOAD_API_PROMISE_0002 uploadTask: " + uploadTask);
            expect(true).assertEqual((uploadTask !== undefined) || (uploadTask !== "") || (uploadTask !== {}));
            uploadTask.on('fail', (TaskState) => {
              console.info("SUB_REQUEST_UPLOAD_API_PROMISE_0002 fail TaskState.path: " + TaskState.path +
                ", TaskState.responseCode: " + TaskState.responseCode + ", TaskState.message: " + TaskState.message);

              expect(typeof(TaskState.path) == "string").assertEqual(true);
              expect(typeof(TaskState.responseCode) == "number").assertEqual(true);
              expect(typeof(TaskState.message) == "string").assertEqual(true);
            });
          });
        } catch (e) {
          console.error("SUB_REQUEST_UPLOAD_API_PROMISE_0002 error: " + JSON.stringify(e));
          expect().assertFail();
        }
        console.info("-----------------------SUB_REQUEST_UPLOAD_API_PROMISE_0002 end-----------------------");
        done();
    });
  
    /**
     * @tc.number    : SUB_REQUEST_UPLOAD_API_PROMISE_0003
     * @tc.name      : Use getEntries get the value by mixing the string key
     * @tc.desc      : Called when the current upload session complete or fail.
     * @tc.size      : MediumTest
     * @tc.type      : Function
     * @tc.level     : Level 1
    */
    it('SUB_REQUEST_UPLOAD_API_PROMISE_0003', 0, async function (done) {
        let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
        fs.closeSync(file);
        console.info("-----------------------SUB_REQUEST_UPLOAD_API_PROMISE_0003 is starting-----------------------");
        try {
          console.info("SUB_REQUEST_UPLOAD_API_PROMISE_0003 UploadConfig: " + JSON.stringify(uploadConfig));
          request.upload(uploadConfig).then((data) => {
            let uploadTask = data;
            console.info("SUB_REQUEST_UPLOAD_API_PROMISE_0003 uploadTask: " + uploadTask);
            expect(true).assertEqual((uploadTask !== undefined) || (uploadTask !== "") || (uploadTask !== {}));
  
            uploadTask.off('complete');
            uploadTask.off('fail');
          });
        } catch (e) {
          console.error("SUB_REQUEST_UPLOAD_API_PROMISE_0003 error: " + JSON.stringify(e));
          expect().assertFail();
        }
        console.info("-----------------------SUB_REQUEST_UPLOAD_API_PROMISE_0003 end-----------------------");
        done();
    });

    /**
     * @tc.number    SUB_REQUEST_UPLOADFILE_API_PARTIAL_0001
     * @tc.name      Test function of partial upload.
     * @tc.desc      Test if the partial upload success or fail.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 0
     */
    it('SUB_REQUEST_UPLOADFILE_API_PARTIAL_0001', 0, async function (done) {
      console.info("====>-----------------------SUB_REQUEST_UPLOADFILE_API_PARTIAL_0001 is starting-----------------------");
      try {
        console.info("====>SUB_REQUEST_UPLOADFILE_API_PARTIAL_0001 UploadConfig2 = " + JSON.stringify(uploadConfig2));
        // @ts-ignore
        await request.uploadFile(globalThis.abilityContext, uploadConfig2).then((uploadTask) => {
          console.info("====>SUB_REQUEST_UPLOADFILE_API_PARTIAL_0001 uploadTask = " + uploadTask);
          expect(true).assertEqual((uploadTask != undefined));
          let complete_callback = async (tasks) => {
            try {
              uploadTask.off('complete');
              console.info("====>SUB_REQUEST_UPLOADFILE_API_PARTIAL_0001 tasks0 = " + JSON.stringify(tasks[0])
              + ", path = " + tasks[0].path + ", responseCode" + tasks[0].responseCode + ", message = " + tasks[0].message);
              let bool = await uploadTask.delete();
              console.info("====>SUB_REQUEST_UPLOADFILE_API_PARTIAL_0001 remove result = " + bool);
              done();
            } catch (err) {
              console.info("====>SUB_REQUEST_UPLOADFILE_API_PARTIAL_0001 fail = " + JSON.stringify(err));
              done();
            }
          }

          console.info("====>SUB_REQUEST_UPLOADFILE_API_PARTIAL_0001 headerReceive success");
          uploadTask.on('complete', complete_callback);

        }).catch((err) => {
          console.error("====>SUB_REQUEST_UPLOADFILE_API_PARTIAL_0001 catch err: " + JSON.stringify(err));
        });
      } catch (err) {
        console.error("====>SUB_REQUEST_UPLOADFILE_API_PARTIAL_0001 fail error: " + JSON.stringify(err));
        done();
      }
    });

    /**
     * @tc.number    SUB_REQUEST_UPLOADFILE_API_PARTIAL_0002
     * @tc.name      Test function of partial upload.
     * @tc.desc      Test if the partial upload with abnormal input success or fail.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 0
     */
    it('SUB_REQUEST_UPLOADFILE_API_PARTIAL_0002', 0, async function (done) {
      console.info("====>-----------------------SUB_REQUEST_UPLOADFILE_API_PARTIAL_0002 is starting-----------------------");	  
      try {
        console.info("====>SUB_REQUEST_UPLOADFILE_API_PARTIAL_0002 UploadConfig3 = " + JSON.stringify(UploadConfig3));
        // @ts-ignore
        await request.uploadFile(globalThis.abilityContext, UploadConfig3).then((uploadTask) => {
          console.info("====>SUB_REQUEST_UPLOADFILE_API_PARTIAL_0002 uploadTask = " + uploadTask);
          expect(true).assertEqual((uploadTask != undefined));
          let complete_callback = async (tasks) => {
            try {
              uploadTask.off('complete');
              console.info("====>SUB_REQUEST_UPLOADFILE_API_PARTIAL_0001 tasks0 = " + JSON.stringify(tasks[0])
              + ", path = " + tasks[0].path + ", responseCode" + tasks[0].responseCode + ", message = " + tasks[0].message);
              let bool = await uploadTask.delete();
              console.info("====>SUB_REQUEST_UPLOADFILE_API_PARTIAL_0002 remove result = " + bool);
              done();
            } catch (err) {
              console.info("====>SUB_REQUEST_UPLOADFILE_API_PARTIAL_0002 fail = " + JSON.stringify(err));
              done();
            }
          }

          console.info("====>SUB_REQUEST_UPLOADFILE_API_PARTIAL_0002 headerReceive success");
          uploadTask.on('complete', complete_callback);

        }).catch((err) => {
          console.error("====>SUB_REQUEST_UPLOADFILE_API_PARTIAL_0002 catch err: " + JSON.stringify(err));
        });
      } catch (err) {
        console.error("====>SUB_REQUEST_UPLOADFILE_API_PARTIAL_0002 fail error: " + JSON.stringify(err));
        done();
      }
    });

    /**
     * @tc.number    SUB_REQUEST_UPLOADFILE_API_PARTIAL_0003
     * @tc.name      Test function of partial upload.
     * @tc.desc      Test if the partial upload with abnormal input success or fail.
     * @tc.size      : MEDIUM
     * @tc.type      : Function
     * @tc.level     : Level 0
     */
    it('SUB_REQUEST_UPLOADFILE_API_PARTIAL_0003', 0, async function (done) {
      console.info("====>-----------------------SUB_REQUEST_UPLOADFILE_API_PARTIAL_0003 is starting-----------------------");	  
      try {
        console.info("====>SUB_REQUEST_UPLOADFILE_API_PARTIAL_0003 UploadConfig4 = " + JSON.stringify(UploadConfig4));
        // @ts-ignore
        await request.uploadFile(globalThis.abilityContext, UploadConfig4).then((uploadTask) => {
          console.info("====>SUB_REQUEST_UPLOADFILE_API_PARTIAL_0003 uploadTask = " + uploadTask);
          expect(true).assertEqual((uploadTask != undefined));
          let complete_callback = async (tasks) => {
            try {
              uploadTask.off('complete');
              console.info("====>SUB_REQUEST_UPLOADFILE_API_PARTIAL_0001 tasks0 = " + JSON.stringify(tasks[0])
              + ", path = " + tasks[0].path + ", responseCode" + tasks[0].responseCode + ", message = " + tasks[0].message);
              let bool = await uploadTask.delete();
              console.info("====>SUB_REQUEST_UPLOADFILE_API_PARTIAL_0003 remove result = " + bool);
              done();
            } catch (err) {
              console.info("====>SUB_REQUEST_UPLOADFILE_API_PARTIAL_0003 fail = " + JSON.stringify(err));
              done();
            }
          }

          console.info("====>SUB_REQUEST_UPLOADFILE_API_PARTIAL_0003 headerReceive success");
          uploadTask.on('complete', complete_callback);

        }).catch((err) => {
          console.error("====>SUB_REQUEST_UPLOADFILE_API_PARTIAL_0003 catch err: " + JSON.stringify(err));
        });
      } catch (err) {
        console.error("====>SUB_REQUEST_UPLOADFILE_API_PARTIAL_0003 fail error: " + JSON.stringify(err));
        done();
      }
    });

    console.log(TAG + "*************Unit Test End*************");
})