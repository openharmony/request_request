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
import Server from '../common/iTcpServer'

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
          console.error('testGetDownloadInfo005 downloadInfo undefined.');
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
          console.error('testGetDownloadInfo005 downloadInfo2 undefined.');
          expect(false).assertTrue();
          done();
        } else {
          expect(total_time != downloadInfo2.performance.totalTime).assertTrue();
          done();
        }
      } catch (err) {
        console.error('testGetDownloadInfo005 download error.');
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testGetDownloadInfo006
     * @tc.name: testGetDownloadInfo006
     * @tc.desc: getDownloadInfo can get ip information
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Set download info list size to 10 via setDownloadInfoListSize.
     *           2. Start local http server and Call download(), wait 300 milliseconds, then get its download info.
     *           3. If the first obtained download info is undefined, print error and assert failure; 
     *              if not, assert ip is 127.0.0.1
     * @tc.require: issues#1573
     */
    it('testGetDownloadInfo006', 0, async (done: Function) => {
      let server = new Server();
      try {
        server.serverStart();
        cacheDownload.setDownloadInfoListSize(10);
        cacheDownload.download("http://127.0.0.1:8078", {});
        await sleep(300);
        let downloadInfo: cacheDownload.DownloadInfo | undefined =
          cacheDownload.getDownloadInfo("http://127.0.0.1:8078");
        if (downloadInfo == undefined) {
          console.error('testGetDownloadInfo006 downloadInfo undefined.');
          expect(false).assertTrue();
          server.serverClose();
          done();
        } else {
          expect(downloadInfo?.network.ip == '127.0.0.1').assertTrue();
          server.serverClose();
          done();
        }
      } catch (err) {
        console.error('testGetDownloadInfo006 download error.');
        expect(false).assertTrue();
        server.serverClose();
        done();
      }
    });

    /**
     * @tc.number: testCacheStrategy001
     * @tc.name: testCacheStrategy001
     * @tc.desc: Test scenario where the cache policy is set to LAZY when the cache exists
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Invoke the cacheDownload.setDownloadInfoListSize interface to set the size of the download information list.
     *           2. Invoke cacheDownload.download to download url 
     *              https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt and wait for 5s.
     *           3. Invoke cacheDownload.getDownloadInfo to obtain url 
     *              https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt download information.
     *           4. Set the cache refresh policy in options to LAZY, invoke cacheDownload.download to download url 
     *              https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt again, and wait for 5 seconds.
     *           5. Invoke the cacheDownload.getDownloadInfo interface to obtain the url 
     *              https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt download information again.
     *           6. Compare the obtained download information.
     * @tc.require: issues#1650
     */
    it('testCacheStrategy001', 0, async (done: Function) => {
      try {
        let forceOptions: cacheDownload.CacheDownloadOptions = {
            cacheStrategy : cacheDownload.CacheStrategy.FORCE
        };
        cacheDownload.setDownloadInfoListSize(1);
        cacheDownload.download("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt",
          forceOptions);
        await sleep(5000);
        let downloadInfo =
          cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt");
        let totalTime = 0;
        let dnsTime = 0;
        let connectTime = 0;
        let tlsTime = 0;
        let firstSendTime = 0;
        let firstReceiveTime = 0;
        let redirectTime = 0;
        if (downloadInfo == undefined) {
          console.error('testCacheStrategy001 downloadInfo undefined.');
          expect(false).assertTrue();
          done();
        } else {
          totalTime = downloadInfo.performance.totalTime;
          dnsTime = downloadInfo.performance.dnsTime;
          connectTime = downloadInfo.performance.connectTime;
          tlsTime = downloadInfo.performance.tlsTime;
          firstSendTime = downloadInfo.performance.firstSendTime;
          firstReceiveTime = downloadInfo.performance.firstReceiveTime;
          redirectTime = downloadInfo.performance.redirectTime;
        }
        
        let lazyOptions: cacheDownload.CacheDownloadOptions = {
            cacheStrategy : cacheDownload.CacheStrategy.LAZY
        };
        cacheDownload.download("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt",
          lazyOptions);
        await sleep(5000);
        let downloadInfo2 =
          cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt");
        if (downloadInfo2 == undefined) {
          console.error('testCacheStrategy001 downloadInfo2 undefined.');
          expect(false).assertTrue();
          done();
        } else {
          expect(totalTime).assertEqual(downloadInfo2.performance.totalTime);
          expect(dnsTime).assertEqual(downloadInfo2.performance.dnsTime);
          expect(connectTime).assertEqual(downloadInfo2.performance.connectTime);
          expect(tlsTime).assertEqual(downloadInfo2.performance.tlsTime);
          expect(firstSendTime).assertEqual(downloadInfo2.performance.firstSendTime);
          expect(firstReceiveTime).assertEqual(downloadInfo2.performance.firstReceiveTime);
          expect(redirectTime).assertEqual(downloadInfo2.performance.redirectTime);

          done();
        }
      } catch (err) {
        console.error('testCacheStrategy001 download error.');
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testCacheStrategy002
     * @tc.name: testCacheStrategy002
     * @tc.desc: Test the scenario where the cache policy is set to LAZY when the cache does not exist
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Invoke the cacheDownload.setDownloadInfoListSize interface to set the size of the download information list.
     *           2. Set the cache refresh policy in options to LAZY, invoke cacheDownload.download to download url 
     *              https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt, and wait for 5 seconds.
     *           3. Invoke the cacheDownload.getDownloadInfo interface to obtain the url 
     *               https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt download information again.
     * @tc.require: issues#1650
     */
    it('testCacheStrategy002', 0, async (done: Function) => {
      try {
        let lazyOptions: cacheDownload.CacheDownloadOptions = {
            cacheStrategy : cacheDownload.CacheStrategy.LAZY
        };
        cacheDownload.setMemoryCacheSize(0);
        cacheDownload.setFileCacheSize(0);
        cacheDownload.setDownloadInfoListSize(0);
        cacheDownload.setDownloadInfoListSize(1);
        cacheDownload.download("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt",
          lazyOptions);
        await sleep(5000);
        let downloadInfo =
          cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt");
        if (downloadInfo == undefined) {
          console.error('testCacheStrategy002 downloadInfo undefined.');
           expect(false).assertTrue();
        }
        done();
      } catch (err) {
        console.error('testCacheStrategy002 download error.');
         expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testCacheStrategy003
     * @tc.name: testCacheStrategy003
     * @tc.desc: Test the scenario where the cache policy is set to Force when the cache exists
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Invoke the cacheDownload.setDownloadInfoListSize interface to set the size of the download information list.
     *           2. Invoke cacheDownload.download to download url 
     *              https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt and wait for 5s.
     *           3. Invoke the cacheDownload.download interface to download the url 
     *              https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt and wait for 5 seconds. Invoke the 
     *              cacheDownload.getDownloadInfo interface to obtain the url 
     *              https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt download information.
     *           4. Set the cache refresh policy in options to Force, invoke cacheDownload.download to download url 
     *              https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt again, and wait for 5s.
     *           5. Invoke the cacheDownload.getDownloadInfo interface to obtain the url 
     *              https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt download information again.
     *           6. Compare the obtained download information.
     * @tc.require: issues#1650
     */
    it('testCacheStrategy003', 0, async (done: Function) => {
      try {
        let forceOptions: cacheDownload.CacheDownloadOptions = {
            cacheStrategy : cacheDownload.CacheStrategy.FORCE
        };
        cacheDownload.setDownloadInfoListSize(1);
        cacheDownload.download("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt",
          forceOptions);
        await sleep(5000);
        let downloadInfo =
          cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt");
        let totalTime = 0;
        if (downloadInfo == undefined) {
          console.error('testCacheStrategy003 downloadInfo undefined.');
          expect(false).assertTrue();
          done();
        } else {
          totalTime = downloadInfo.performance.totalTime;
        }

        cacheDownload.download("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt",
          forceOptions);
        await sleep(5000);
        let downloadInfo2 =
          cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt");
        if (downloadInfo2 == undefined) {
          console.error('testCacheStrategy003 downloadInfo2 undefined.');
          expect(false).assertTrue();
        } else {
          expect(totalTime != downloadInfo2.performance.totalTime).assertTrue();
        }
        done();
      } catch (err) {
        console.error('testCacheStrategy003 download error.');
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testCacheStrategy004
     * @tc.name: testCacheStrategy004
     * @tc.desc: Test the scenario where the cache policy is set to Force when the cache does not exist
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Invoke the cacheDownload.setDownloadInfoListSize interface to set the size of the download information list.
     *           2. Set the cache refresh policy in options to LAZY, invoke cacheDownload.download to download url 
     *              https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt, and wait for 5 seconds.
     *           3. Invoke the cacheDownload.getDownloadInfo interface to obtain the url 
     *              https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt download information again.
     * @tc.require: issues#1650
     */
    it('testCacheStrategy004', 0, async (done: Function) => {
      try {
        let forceOptions: cacheDownload.CacheDownloadOptions = {
            cacheStrategy : cacheDownload.CacheStrategy.FORCE
        };
        cacheDownload.setDownloadInfoListSize(0);
        cacheDownload.setDownloadInfoListSize(1);
        cacheDownload.download("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt",
          forceOptions);
        await sleep(5000);
        let downloadInfo =
          cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt");
        if (downloadInfo == undefined) {
          console.error('testCacheStrategy004 downloadInfo undefined.');
          expect(false).assertTrue();
        }
        done();
      } catch (err) {
        console.error('testCacheStrategy004 download error.');
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testCacheStrategy005
     * @tc.name: testCacheStrategy005
     * @tc.desc: Verifying CacheStrategy Enumerated Value Scenarios
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Verify that the value of CacheStrategy.FORCE is 0.
     *           2. Verify that the value of CacheStrategy.LAZY is 1.
     * @tc.require: issues#1650
     */
    it('testCacheStrategy005', 0, async (done: Function) => {
      try {
        expect(cacheDownload.CacheStrategy.FORCE).assertEqual(0);
        expect(cacheDownload.CacheStrategy.LAZY).assertEqual(1);
      catch (err) {
        expect(false).assertTrue();
      }
      done();
    });

    /**
     * @tc.number: testCacheStrategy006
     * @tc.name: testCacheStrategy006
     * @tc.desc: Test the scenario where the cache policy is not set when the cache exists
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Invoke the cacheDownload.setDownloadInfoListSize interface to set the size of the download information list.
     *           2. Invoke cacheDownload.download to download url 
     *              https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt and wait for 5s.
     *           3. Invoke the cacheDownload.download interface to download the url 
     *              https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt and wait for 5 seconds. Invoke the 
     *              cacheDownload.getDownloadInfo interface to obtain the url 
     *              https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt download information.
     *           4. Do not configure the cache policy, invoke cacheDownload.download to download url 
     *              https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt again, and wait for 5s.
     *           5. Invoke the cacheDownload.getDownloadInfo interface to obtain the url 
     *              https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt download information again.
     *           6. Compare the obtained download information.
     * @tc.require: issues#1650
     */
    it('testCacheStrategy006', 0, async (done: Function) => {
      try {
        let options: cacheDownload.CacheDownloadOptions = {};
        cacheDownload.setDownloadInfoListSize(1);
        cacheDownload.download("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt",
          options);
        await sleep(5000);
        let downloadInfo =
          cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt");
        let totalTime = 0;
        if (downloadInfo == undefined) {
          console.error('testCacheStrategy006 downloadInfo undefined.');
          expect(false).assertTrue();
          done();
        } else {
          totalTime = downloadInfo.performance.totalTime;
        }

        cacheDownload.download("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt",
          options);
        await sleep(5000);
        let downloadInfo2 =
          cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt");
        if (downloadInfo2 == undefined) {
          console.error('testCacheStrategy006 downloadInfo2 undefined.');
          expect(false).assertTrue();
        } else {
          expect(totalTime != downloadInfo2.performance.totalTime).assertTrue();
        }
        done();
      } catch (err) {
        console.error('testCacheStrategy006 download error.');
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testCacheStrategy007
     * @tc.name: testCacheStrategy007
     * @tc.desc: Test the scenario where the cache exists and the cache policy is set to undefined
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Invoke the cacheDownload.setDownloadInfoListSize interface to set the size of the download information list.
     *           2. Invoke cacheDownload.download to download url 
     *              https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt and wait for 5s.
     *           3. Invoke the cacheDownload.download interface to download the url 
     *              https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt and wait for 5 seconds. Invoke the 
     *              cacheDownload.getDownloadInfo interface to obtain the url 
     *              https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt download information.
     *           4. Set the cache policy to undefined, invoke cacheDownload.download to download url 
     *              https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt again, and wait for 5s.
     *           5. Invoke the cacheDownload.getDownloadInfo interface to obtain the url 
     *              https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt download information again.
     *           6. Compare the obtained download information.
     * @tc.require: issues#1650
     */
    it('testCacheStrategy007', 0, async (done: Function) => {
      try {
        let options: cacheDownload.CacheDownloadOptions = {
            cacheStrategy : undefined
        };
        cacheDownload.setDownloadInfoListSize(1);
        cacheDownload.download("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt",
          options);
        await sleep(5000);
        let downloadInfo =
          cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt");
        let totalTime = 0;
        if (downloadInfo == undefined) {
          console.error('testCacheStrategy007 downloadInfo undefined.');
          expect(false).assertTrue();
          done();
        } else {
          totalTime = downloadInfo.performance.totalTime;
        }

        cacheDownload.download("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt",
          options);
        await sleep(5000);
        let downloadInfo2 =
          cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt");
        if (downloadInfo2 == undefined) {
          console.error('testCacheStrategy007 downloadInfo2 undefined.');
          expect(false).assertTrue();
        } else {
          expect(totalTime != downloadInfo2.performance.totalTime).assertTrue();
        }
        done();
      } catch (err) {
        console.error('testCacheStrategy007 download error.');
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testCacheStrategy008
     * @tc.name: testCacheStrategy008
     * @tc.desc: Test the scenario where the cache exists and the cache policy is set to String
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Invoke the cacheDownload.setDownloadInfoListSize interface to set the size of the download information list.
     *           2. Invoke cacheDownload.download to download url 
     *              https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt and wait for 5s.
     *           3. Invoke the cacheDownload.download interface to download the url 
     *              https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt and wait for 5 seconds. Invoke the 
     *              cacheDownload.getDownloadInfo interface to obtain the url 
     *              https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt download information.
     *           4. Set the cache policy to string, invoke cacheDownload.download to download url 
     *              https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt again, and wait for 5s.
     *           5. Invoke the cacheDownload.getDownloadInfo interface to obtain the url 
     *              https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt download information again.
     *           6. Compare the obtained download information.
     * @tc.require: issues#1650
     */
    it('testCacheStrategy008', 0, async (done: Function) => {
      try {
        let options: cacheDownload.CacheDownloadOptions = {
            cacheStrategy : "test"
        };
        cacheDownload.setDownloadInfoListSize(1);
        cacheDownload.download("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt",
          options);
        await sleep(5000);
        let downloadInfo =
          cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt");
        let totalTime = 0;
        if (downloadInfo == undefined) {
          console.error('testCacheStrategy008 downloadInfo undefined.');
          expect(false).assertTrue();
          done();
        } else {
          totalTime = downloadInfo.performance.totalTime;
        }

        cacheDownload.download("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt",
          options);
        await sleep(5000);
        let downloadInfo2 =
          cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt");
        if (downloadInfo2 == undefined) {
          console.error('testCacheStrategy008 downloadInfo2 undefined.');
          expect(false).assertTrue();
        } else {
          expect(totalTime != downloadInfo2.performance.totalTime).assertTrue();
        }
        done();
      } catch (err) {
        console.error('testCacheStrategy008 download error.');
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testCacheStrategy009
     * @tc.name: testCacheStrategy009
     * @tc.desc: Test the scenario where the cache exists and the cache policy is set to 3
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Invoke the cacheDownload.setDownloadInfoListSize interface to set the size of the download information list.
     *           2. Invoke cacheDownload.download to download url 
     *              https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt and wait for 5s.
     *           3. Invoke the cacheDownload.download interface to download the url 
     *              https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt and wait for 5 seconds. Invoke the 
     *              cacheDownload.getDownloadInfo interface to obtain the url 
     *              https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt download information.
     *           4. Set the cache policy to 3, invoke cacheDownload.download to download url 
     *              https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt again, and wait for 5s.
     *           5. Invoke the cacheDownload.getDownloadInfo interface to obtain the url 
     *              https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt download information again.
     *           6. Compare the obtained download information.
     * @tc.require: issues#1650
     */
    it('testCacheStrategy009', 0, async (done: Function) => {
      try {
        let options: cacheDownload.CacheDownloadOptions = {
            cacheStrategy : 3
        };
        cacheDownload.setDownloadInfoListSize(1);
        cacheDownload.download("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt",
          options);
        await sleep(5000);
        let downloadInfo =
          cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt");
        let totalTime = 0;
        if (downloadInfo == undefined) {
          console.error('testCacheStrategy009 downloadInfo undefined.');
          expect(false).assertTrue();
          done();
        } else {
          totalTime = downloadInfo.performance.totalTime;
        }

        cacheDownload.download("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt",
          options);
        await sleep(5000);
        let downloadInfo2 =
          cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt");
        if (downloadInfo2 == undefined) {
          console.error('testCacheStrategy009 downloadInfo2 undefined.');
          expect(false).assertTrue();
        } else {
          expect(totalTime != downloadInfo2.performance.totalTime).assertTrue();
        }
        done();
      } catch (err) {
        console.error('testCacheStrategy009 download error.');
        expect(false).assertTrue();
        done();
      }
    });
  })
}