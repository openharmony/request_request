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

import { describe, beforeAll, beforeEach, afterEach, afterAll, it, expect } from "@ohos/hypium";
import cacheDownload from '@ohos.request.cacheDownload';

export default function cacheDownloadInfo() {
  describe('cacheDownloadInfo', () => {
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
      cacheDownload.setDownloadInfoListSize(0);
    });

    let sleep: (ms: number) => Promise<void> = async (ms: number): Promise<void> => {
      return new Promise((resolve) => {
        setTimeout(resolve, ms);
      });
    };

    /**
     * @tc.number: testSetInfoListSize001
     * @tc.name: testSetInfoListSize001
     * @tc.desc: setDownloadInfoListSize sets the size to be less than 0
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Call setDownloadInfoListSize with parameter -1.
     *           2. Verify that the call fails and returns error code 401.
     * @tc.require: issues#1573
     */
    it('testSetInfoListSize001', 0, async (done: Function) => {
      try {
        cacheDownload.setDownloadInfoListSize(-1);
        expect(false).assertTrue();
        done();
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });

    /**
     * @tc.number: testSetInfoListSize002
     * @tc.name: testSetInfoListSize002
     * @tc.desc: setDownloadInfoListSize sets the size to be greater than 8192
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Call setDownloadInfoListSize with parameter 8193.
     *           2. Verify that the call fails and returns error code 401.
     * @tc.require: issues#1573
     */
    it('testSetInfoListSize002', 0, async (done: Function) => {
      try {
        cacheDownload.setDownloadInfoListSize(8193);
        expect(false).assertTrue();
        done();
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    });

    /**
     * @tc.number: testSetInfoListSize003
     * @tc.name: testSetInfoListSize003
     * @tc.desc: setDownloadInfoListSize sets the size to be greater than 8192
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Call setDownloadInfoListSize with parameter 8192.
     *           2. Verify that the call succeeds without throwing an error.
     * @tc.require: issues#1573
     */
    it('testSetInfoListSize003', 0, async (done: Function) => {
      try {
        cacheDownload.setDownloadInfoListSize(8192);
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testGetDownloadInfo001
     * @tc.name: testGetDownloadInfo001
     * @tc.desc: getDownloadInfoList updates the same url info
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Set download info list size to 1 via setDownloadInfoListSize.
     *           2. Call download twice with the same test URL and wait 5 seconds each time.
     *           3. Get download info twice via getDownloadInfo and verify totalTime is different.
     * @tc.require: issues#1573
     */
    it('testGetDownloadInfo001', 0, async (done: Function) => {
      try {
        let options: cacheDownload.CacheDownloadOptions = {};
        cacheDownload.setDownloadInfoListSize(1);
        cacheDownload.download("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt",
          options);
        await sleep(5000);
        let downloadInfo =
          cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt");
        let total_time = 0;
        if (downloadInfo == undefined) {
          console.error('testGetDownloadInfo001 downloadInfo undefined.');
          expect(false).assertTrue();
          done();
        } else {
          total_time = downloadInfo.performance.totalTime;
        }
        cacheDownload.download("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt",
          options);
        await sleep(5000);
        let downloadInfo2 =
          cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt");
        if (downloadInfo2 == undefined) {
          console.error('testGetDownloadInfo001 downloadInfo2 undefined.');
          expect(false).assertTrue();
          done();
        } else {
          expect(total_time != downloadInfo2.performance.totalTime).assertTrue();
          done();
        }
      } catch (err) {
        console.error('testGetDownloadInfo001 download error.');
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testGetDownloadInfo002
     * @tc.name: testGetDownloadInfo002
     * @tc.desc: The set size is valid and can save the info of the number of sizes.
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Set download info list size to 1 via setDownloadInfoListSize.
     *           2. Call download with two different test URLs and wait 5 seconds each time.
     *           3. Verify the first URL's info is cleared and the second URL's info exists.
     * @tc.require: issues#1573
     */
    it('testGetDownloadInfo002', 0, async (done: Function) => {
      try {
        let options: cacheDownload.CacheDownloadOptions = {};
        cacheDownload.setDownloadInfoListSize(1);
        cacheDownload.download("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt",
          options);
        await sleep(5000);
        let downloadInfo =
          cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt");
        if (downloadInfo == undefined) {
          console.error('testGetDownloadInfo002 downloadInfo undefined.');
          expect(false).assertTrue();
          done();
        }
        cacheDownload.download("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/edge_picture.jpg",
          options);
        await sleep(5000);
        let downloadInfo2 =
          cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/edge_picture.jpg");
        if (downloadInfo2 == undefined) {
          console.error('testGetDownloadInfo002 downloadInfo2 undefined.');
          expect(false).assertTrue();
          done();
        }
        let downloadInfo_1 =
          cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt");
        if (downloadInfo_1 == undefined) {
          done();
        } else {
          console.error('testGetDownloadInfo002 downloadInfo_1 is not undefined.');
          expect(false).assertTrue();
          done();
        }
      } catch (err) {
        console.error('testGetDownloadInfo002 download error.');
        expect(false).assertTrue();
        done();
      }
    });
    
    /**
     * @tc.number: testGetDownloadInfo003
     * @tc.name: testGetDownloadInfo003
     * @tc.desc: remove the oldest information and save the latest one
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Set download info list size to 2 via setDownloadInfoListSize.
     *           2. Call download with three different test URLs and wait 5 seconds each time.
     *           3. Verify the oldest URL's info is cleared and the latest two exist.
     * @tc.require: issues#1573
     */
    it('testGetDownloadInfo003', 0, async (done: Function) => {
      try {
        let options: cacheDownload.CacheDownloadOptions = {};
        cacheDownload.setDownloadInfoListSize(2);
        cacheDownload.download("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt",
          options);
        await sleep(5000);
        let downloadInfo =
          cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt");
        if (downloadInfo == undefined) {
          console.error('testGetDownloadInfo003 downloadInfo undefined.');
          expect(false).assertTrue();
          done();
        }
        cacheDownload.download("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/edge_picture.jpg",
          options);
        await sleep(5000);
        let downloadInfo2 =
          cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/edge_picture.jpg");
        if (downloadInfo2 == undefined) {
          console.error('testGetDownloadInfo003 downloadInfo2 undefined.');
          expect(false).assertTrue();
          done();
        }
        cacheDownload.download("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/audio_test.mp3",
          options);
        await sleep(5000);
        let downloadInfo3 =
          cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/audio_test.mp3");
        if (downloadInfo3 == undefined) {
          console.error('testGetDownloadInfo003 downloadInfo3 undefined.');
          expect(false).assertTrue();
          done();
        }
        let downloadInfo_1 =
          cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt");
        if (downloadInfo_1 == undefined) {
          done();
        } else {
          console.error('testGetDownloadInfo003 downloadInfo_1 is not undefined.');
          expect(false).assertTrue();
          done();
        }
      } catch (err) {
        console.error('testGetDownloadInfo003 download error.');
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testGetDownloadInfo004
     * @tc.name: testGetDownloadInfo004
     * @tc.desc: getDownloadInfo can update the corresponding url information to the latest
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Initialize empty download options,
     *              set download info list size to 2 via setDownloadInfoListSize.
     *           2. Call download() for "test400K.txt" URL with the above options,
     *              wait 5 seconds, then get its download info; 
     *              if info is undefined, print error and assert failure.
     *           3. Call download() for "edge_picture.jpg" URL with the same options,
     *              wait 5 seconds, then get its download info; 
     *              if info is undefined, print error and assert failure.
     *           4. Re-get download info of "test400K.txt" URL;
     *              if info is undefined, print error and assert failure.
     *           5. Call download() for "audio_test.mp3" URL with the same options,
     *              wait 5 seconds, then get its download info; 
     *              if info is undefined, print error and assert failure.
     *           6. Get download info of "edge_picture.jpg" URL again;
     *              if info is undefined, execute done() normally; 
     *              if not, print error and assert failure.
     * @tc.require: issues#1573
     */
    it('testGetDownloadInfo004', 0, async (done: Function) => {
      try {
        let options: cacheDownload.CacheDownloadOptions = {};
        cacheDownload.setDownloadInfoListSize(2);
        cacheDownload.download("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt",
          options);
        await sleep(5000);
        let downloadInfo =
          cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt");
        if (downloadInfo == undefined) {
          console.error('testGetDownloadInfo004 downloadInfo undefined.');
          expect(false).assertTrue();
          done();
        }
        cacheDownload.download("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/edge_picture.jpg",
          options);
        await sleep(5000);
        let downloadInfo2 =
          cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/edge_picture.jpg");
        if (downloadInfo2 == undefined) {
          console.error('testGetDownloadInfo004 downloadInfo2 undefined.');
          expect(false).assertTrue();
          done();
        }
        let downloadInfo_1 =
          cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt");
        if (downloadInfo_1 == undefined) {
          console.error('testGetDownloadInfo004 downloadInfo2 undefined.');
          expect(false).assertTrue();
          done();
        }
        cacheDownload.download("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/audio_test.mp3",
          options);
        await sleep(5000);
        let downloadInfo3 =
          cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/audio_test.mp3");
        if (downloadInfo3 == undefined) {
          console.error('testGetDownloadInfo004 downloadInfo3 undefined.');
          expect(false).assertTrue();
          done();
        }
        let downloadInfo_2 =
          cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/edge_picture.jpg");
        if (downloadInfo_2 == undefined) {
          done();
        } else {
          console.error('testGetDownloadInfo004 downloadInfo_2 is not undefined.');
          expect(false).assertTrue();
          done();
        }
      } catch (err) {
        console.error('testGetDownloadInfo004 download error.');
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testGetDownloadInfo005
     * @tc.name: testGetDownloadInfo005
     * @tc.desc: getDownloadInfoList updates the same url info with headers
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Initialize download options with headers {"key":"123","value":"456"}, 
     *              set download info list size to 1 via setDownloadInfoListSize.
     *           2. Call download() for "test400K.txt" URL with the above options,
     *              wait 5 seconds, then get its download info.
     *           3. If the first obtained download info is undefined, print error and assert failure; 
     *              if not, record its performance.totalTime to total_time variable.
     *           4. Call download() for the same "test400K.txt" URL with the same options again, wait 5 seconds,
     *              then get its download info (named downloadInfo2).
     *           5. If downloadInfo2 is undefined, print error and assert failure; 
     *              if not, assert that total_time is not equal to downloadInfo2.performance.totalTime,
     *              then execute done().
     * @tc.require: issues#1573
     */
    it('testGetDownloadInfo005', 0, async (done: Function) => {
      try {
        let options: cacheDownload.CacheDownloadOptions = {
          headers: {
            "key": "123",
            "value": "456",
          }
        };
        cacheDownload.setDownloadInfoListSize(1);
        cacheDownload.download("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt",
          options);
        await sleep(5000);
        let downloadInfo: cacheDownload.DownloadInfo | undefined =
          cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt");
        let total_time = 0;
        if (downloadInfo == undefined) {
          console.error('testGetDownloadInfo001 downloadInfo undefined.');
          expect(false).assertTrue();
          done();
        } else {
          total_time = downloadInfo.performance.totalTime;
        }
        cacheDownload.download("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt",
          options);
        await sleep(5000);
        let downloadInfo2: cacheDownload.DownloadInfo | undefined =
          cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt");
        if (downloadInfo2 == undefined) {
          console.error('testGetDownloadInfo001 downloadInfo2 undefined.');
          expect(false).assertTrue();
          done();
        } else {
          expect(total_time != downloadInfo2.performance.totalTime).assertTrue();
          done();
        }
      } catch (err) {
        console.error('testGetDownloadInfo001 download error.');
        expect(false).assertTrue();
        done();
      }
    });
  })
}