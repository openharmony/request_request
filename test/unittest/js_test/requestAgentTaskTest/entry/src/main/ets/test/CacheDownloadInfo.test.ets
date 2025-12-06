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

    let cbSuccess1 = () => {
      console.info("cbSuccess1");
      cacheDownload.offDownloadSuccess("https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt", cbSuccess1);
    }

    let cbError1 = () => {
      console.info("cbError1");
      cacheDownload.offDownloadError("https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test123.txt", cbError1);
    }

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
    /**
     * @tc.number: testDownloadSuccess001
     * @tc.name: testDownloadSuccess001
     * @tc.desc: Test the scenario of successful download with success callback
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     * @tc.step: 1. Register download success callback
     *           2. Invoke cacheDownload.download to download a valid file
     *           3. Verify the success callback is triggered within timeout
     * @tc.require: issues#1650
     */
    it('testDownloadSuccess001', 0, async () => {
      const url = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt";

      // 创建Promise包装回调，显式指定类型
      const successPromise: Promise<void> = new Promise<void>((resolve: () => void) => {
        cacheDownload.onDownloadSuccess(url, () => {
          resolve();
        });
      });

      // 创建超时Promise，显式指定类型
      const timeoutPromise: Promise<never> = new Promise<never>((_resolve, reject: (error: Error) => void) => {
        setTimeout(() => {
          reject(new Error('Timeout after 5 seconds'));
        }, 5000);
      });

      cacheDownload.download(url, {});

      try {
        // 使用Promise.race，显式指定返回类型
        await Promise.race<Promise<void>[]>([successPromise, timeoutPromise]);

        // 成功执行到这说明回调已触发
        expect(true).assertTrue();
      } catch (err) {
        // 超时或其他错误
        console.error('testDownloadSuccess001 download error:', (err as Error).message);
        expect(false).assertTrue();
      }
    });
    /**
     * @tc.number: testDownloadSuccess002
     * @tc.name: testDownloadSuccess002
     * @tc.desc: Test multiple download requests with success callback
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     * @tc.step: 1. Register download success callback
     *           2. Invoke cacheDownload.download twice to download a valid file
     *           3. Verify the success callback is triggered for both downloads
     * @tc.require: issues#1650
     */
    it('testDownloadSuccess002', 0, async () => {
      const url = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt";
      let callbackCount = 0;
      
      // 创建Promise包装回调
      const successPromise = new Promise<void>((resolve) => {
        cacheDownload.onDownloadSuccess(url, () => {
          callbackCount++;
          if (callbackCount === 2) resolve();
        });
      });

      // 创建超时Promise
      const timeoutPromise = new Promise<never>((_, reject) => 
        setTimeout(() => reject(new Error('Timeout after 5 seconds')), 5000)
      );

      try {
        cacheDownload.download(url, {});
        cacheDownload.download(url, {});
        
        // 等待回调执行（带超时）
        await Promise.race([successPromise, timeoutPromise]);
        
        // 成功断言
        expect(true).assertTrue();
      } catch (err) {
        console.error('testDownloadSuccess002 download error:', (err as Error).message);
        expect(false).assertTrue();
      }
    });

    /**
     * @tc.number: testDownloadSuccess003
     * @tc.name: testDownloadSuccess003
     * @tc.desc: Test nested success callbacks scenario
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     * @tc.step: 1. Register download success callback that registers another callback
     *           2. Invoke cacheDownload.download to download a valid file
     *           3. Verify both callbacks are triggered correctly
     * @tc.require: issues#1650
     */
    it('testDownloadSuccess003', 0, async () => {
      const url = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt";
      
      // 创建Promise包装嵌套回调
      const successPromise = new Promise<void>((resolve) => {
        cacheDownload.onDownloadSuccess(url, () => {
          console.log('testDownloadSuccess003 first success');
          cacheDownload.onDownloadSuccess(url, () => {
            console.log('testDownloadSuccess003 second success');
            resolve();
          });
        });
      });

      const timeoutPromise = new Promise<never>((_, reject) => 
        setTimeout(() => reject(new Error('Timeout after 5 seconds')), 5000)
      );

      try {
        cacheDownload.download(url, {});
        await Promise.race([successPromise, timeoutPromise]);
        expect(true).assertTrue();
      } catch (err) {
        console.error('testDownloadSuccess003 download error:', (err as Error).message);
        expect(false).assertTrue();
      }
    });

    /**
     * @tc.number: testDownloadSuccess004
     * @tc.name: testDownloadSuccess004
     * @tc.desc: Test unregister success callback immediately after registration
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     * @tc.step: 1. Register download success callback
     *           2. Immediately unregister the callback
     *           3. Invoke cacheDownload.download to download a valid file
     *           4. Verify the callback is not triggered
     * @tc.require: issues#1650
     */
    it('testDownloadSuccess004', 0, async () => {
      const url = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt";
      let callbackCalled = false;
      
      // 创建Promise包装回调
      const successPromise = new Promise<void>((resolve) => {
        cacheDownload.onDownloadSuccess(url, () => {
          callbackCalled = true;
          resolve();
        });
      });

      const timeoutPromise = new Promise<void>((resolve) => 
        setTimeout(() => resolve(), 5000)
      );

      try {
        // 注册后立即取消
        cacheDownload.offDownloadSuccess(url);
        cacheDownload.download(url, {});
        
        // 等待5秒看回调是否被调用
        await Promise.race([successPromise, timeoutPromise]);
        
        // 验证回调未被调用
        expect(callbackCalled).assertFalse();
      } catch (err) {
        console.error('testDownloadSuccess004 download error:', (err as Error).message);
        expect(false).assertTrue();
      }
    });

    /**
     * @tc.number: testDownloadSuccess005
     * @tc.name: testDownloadSuccess005
     * @tc.desc: Test unregistering multiple success callbacks
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     * @tc.step: 1. Register download success callback
     *           2. Unregister all callbacks
     *           3. Invoke cacheDownload.download to download a valid file
     *           4. Verify the callback is not triggered
     * @tc.require: issues#1650
     */
    it('testDownloadSuccess005', 0, async () => {
      const url = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt";
      let callbackCalled = false;
      
      const successPromise = new Promise<void>((resolve) => {
        cacheDownload.onDownloadSuccess(url, () => {
          callbackCalled = true;
          resolve();
        });
      });

      const timeoutPromise = new Promise<void>((resolve) => 
        setTimeout(() => resolve(), 5000)
      );

      try {
        // 注册多个回调后全部取消
        cacheDownload.offDownloadSuccess(url);
        cacheDownload.download(url, {});
        
        await Promise.race([successPromise, timeoutPromise]);
        expect(callbackCalled).assertFalse();
      } catch (err) {
        console.error('testDownloadSuccess005 download error:', (err as Error).message);
        expect(false).assertTrue();
      }
    });

    /**
     * @tc.number: testDownloadSuccess006
     * @tc.name: testDownloadSuccess006
     * @tc.desc: Test multiple success callbacks execution order
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     * @tc.step: 1. Register multiple download success callbacks
     *           2. Invoke cacheDownload.download to download a valid file
     *           3. Verify all callbacks are triggered in registration order
     * @tc.require: issues#1650
     */
    it('testDownloadSuccess006', 0, async () => {
      const url = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt";
      let cbString = "";
      
      // 创建Promise等待两个回调执行
      const successPromise = new Promise<void>((resolve) => {
        let count = 0;
        const checkComplete = () => {
          if (count === 2) resolve();
        };
        
        cacheDownload.onDownloadSuccess(url, () => {
          cbString += "a";
          count++;
          checkComplete();
        });
        
        cacheDownload.onDownloadSuccess(url, () => {
          cbString += "b";
          count++;
          checkComplete();
        });
      });

      const timeoutPromise = new Promise<never>((_, reject) => 
        setTimeout(() => reject(new Error('Timeout after 5 seconds')), 5000)
      );

      try {
        cacheDownload.download(url, {});
        await Promise.race([successPromise, timeoutPromise]);
        expect(cbString).assertEqual("ab");
      } catch (err) {
        console.error('testDownloadSuccess006 download error:', (err as Error).message);
        expect(false).assertTrue();
      }
    });

    /**
     * @tc.number: testDownloadSuccess007
     * @tc.name: testDownloadSuccess007
     * @tc.desc: Test callback override behavior for success callbacks
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     * @tc.step: 1. Register multiple download success callbacks
     *           2. Invoke cacheDownload.download to download a valid file
     *           3. Verify only the last registered callback is triggered
     * @tc.require: issues#1650
     */
    it('testDownloadSuccess007', 0, async () => {
      const url = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt";
      let cbString = "";
      
      const successPromise = new Promise<void>((resolve) => {
        cacheDownload.onDownloadSuccess(url, () => {
          cbString += "b";
          resolve();
        });
      });

      const timeoutPromise = new Promise<never>((_, reject) => 
        setTimeout(() => reject(new Error('Timeout after 5 seconds')), 5000)
      );

      try {
        // 注册多个回调，最后一个覆盖前面的
        cacheDownload.onDownloadSuccess(url, () => { cbString += "a"; });
        cacheDownload.onDownloadSuccess(url, () => { cbString += "b"; });
        cacheDownload.onDownloadSuccess(url, () => { cbString += "a"; });
        
        cacheDownload.download(url, {});
        await Promise.race([successPromise, timeoutPromise]);
        expect(cbString).assertEqual("b");
      } catch (err) {
        console.error('testDownloadSuccess007 download error:', (err as Error).message);
        expect(false).assertTrue();
      }
    });

    /**
     * @tc.number: testDownloadSuccess008
     * @tc.name: testDownloadSuccess008
     * @tc.desc: Test selective unregistration of success callbacks
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     * @tc.step: 1. Register multiple named download success callbacks
     *           2. Unregister only one specific callback
     *           3. Invoke cacheDownload.download to download a valid file
     *           4. Verify only the remaining callback is triggered
     * @tc.require: issues#1650
     */
    it('testDownloadSuccess008', 0, async () => {
      const url = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt";
      let cbString = "";
      
      const successPromise = new Promise<void>((resolve) => {
        cacheDownload.onDownloadSuccess(url, () => {
          cbString += "b";
          resolve();
        });
      });

      const timeoutPromise = new Promise<never>((_, reject) => 
        setTimeout(() => reject(new Error('Timeout after 5 seconds')), 5000)
      );

      try {
        const callbackA = () => { cbString += "a"; };
        const callbackB = () => { cbString += "b"; };
        
        cacheDownload.onDownloadSuccess(url, callbackA);
        cacheDownload.onDownloadSuccess(url, callbackB);
        
        // 取消第一个回调
        cacheDownload.offDownloadSuccess(url, callbackA);
        
        cacheDownload.download(url, {});
        await Promise.race([successPromise, timeoutPromise]);
        expect(cbString).assertEqual("b");
      } catch (err) {
        console.error('testDownloadSuccess008 download error:', (err as Error).message);
        expect(false).assertTrue();
      }
    });

    /**
     * @tc.number: testDownloadSuccess009
     * @tc.name: testDownloadSuccess009
     * @tc.desc: Test unregistration of multiple identical success callbacks
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     * @tc.step: 1. Register the same callback function multiple times
     *           2. Unregister the callback once
     *           3. Invoke cacheDownload.download to download a valid file
     *           4. Verify the callback is not triggered
     * @tc.require: issues#1650
     */
    it('testDownloadSuccess009', 0, async () => {
      const url = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt";
      let cbString = "";
      
      const successPromise = new Promise<void>((resolve) => {
        // 设置超时检查
        setTimeout(() => {
          resolve();
        }, 3000);
      });

      try {
        const callback = () => { cbString += "a"; };
        
        // 注册多个相同回调
        cacheDownload.onDownloadSuccess(url, callback);
        cacheDownload.onDownloadSuccess(url, callback);
        
        // 取消所有该回调
        cacheDownload.offDownloadSuccess(url, callback);
        
        cacheDownload.download(url, {});
        await successPromise;
        expect(cbString).assertEqual("");
      } catch (err) {
        console.error('testDownloadSuccess009 download error:', (err as Error).message);
        expect(false).assertTrue();
      }
    });

    /**
     * @tc.number: testDownloadSuccess010
     * @tc.name: testDownloadSuccess010
     * @tc.desc: Test unregistering all success callbacks
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     * @tc.step: 1. Register multiple download success callbacks
     *           2. Unregister all callbacks using offDownloadSuccess without parameters
     *           3. Invoke cacheDownload.download to download a valid file
     *           4. Verify no callbacks are triggered
     * @tc.require: issues#1650
     */
    it('testDownloadSuccess010', 0, async () => {
      const url = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt";
      let cbString = "";
      
      const successPromise = new Promise<void>((resolve) => {
        setTimeout(() => resolve(), 3000);
      });

      try {
        cacheDownload.onDownloadSuccess(url, () => { cbString += "a"; });
        cacheDownload.onDownloadSuccess(url, () => { cbString += "a"; });
        
        // 取消所有回调
        cacheDownload.offDownloadSuccess(url);
        
        cacheDownload.download(url, {});
        await successPromise;
        expect(cbString).assertEqual("");
      } catch (err) {
        console.error('testDownloadSuccess010 download error:', (err as Error).message);
        expect(false).assertTrue();
      }
    });

    /**
     * @tc.number: testDownloadSuccess011
     * @tc.name: testDownloadSuccess011
     * @tc.desc: Test unregistering another callback from within a success callback
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     * @tc.step: 1. Register multiple download success callbacks
     *           2. Make one callback unregister another callback
     *           3. Invoke cacheDownload.download to download a valid file
     *           4. Verify only the first callback is triggered
     * @tc.require: issues#1650
     */
    it('testDownloadSuccess011', 0, async () => {
      const url = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt";
      let cbString = "";
      
      const successPromise = new Promise<void>((resolve) => {
        cacheDownload.onDownloadSuccess(url, () => {
          cbString += "a";
          resolve();
        });
      });

      const timeoutPromise = new Promise<never>((_, reject) => 
        setTimeout(() => reject(new Error('Timeout after 5 seconds')), 5000)
      );

      try {
        const callbackB = () => { cbString += "b"; };
        
        cacheDownload.onDownloadSuccess(url, () => {
          cbString += "a";
          // 在回调中取消另一个回调
          cacheDownload.offDownloadSuccess(url, callbackB);
        });
        
        cacheDownload.onDownloadSuccess(url, callbackB);
        
        cacheDownload.download(url, {});
        await Promise.race([successPromise, timeoutPromise]);
        expect(cbString).assertEqual("a");
      } catch (err) {
        console.error('testDownloadSuccess011 download error:', (err as Error).message);
        expect(false).assertTrue();
      }
    });

    /**
     * @tc.number: testDownloadSuccess012
     * @tc.name: testDownloadSuccess012
     * @tc.desc: Test unregistering other callbacks during multiple downloads
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     * @tc.step: 1. Register multiple download success callbacks
     *           2. Make one callback unregister another callback during first download
     *           3. Invoke cacheDownload.download twice
     *           4. Verify callback behavior across both downloads
     * @tc.require: issues#1650
     */
    it('testDownloadSuccess012', 0, async () => {
      const url = "https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/bird.png";
      let cbString = "";
      let downloadCounter = 0;
      
      // 定义回调函数
      const callbackB = () => {
        cbString += "b";
      };
      
      const callbackA = () => {
        cbString += "a";
        downloadCounter++;
        
        // 在第一次下载的回调中取消callbackB
        if (downloadCounter === 1) {
          cacheDownload.offDownloadSuccess(url, callbackB);
        }
      };

      // 创建Promise等待两次下载完成
      const downloadPromise = new Promise<void>((resolve) => {
        // 使用轮询检查是否两次下载都已完成
        const checkInterval = setInterval(() => {
          if (downloadCounter >= 2) {
            clearInterval(checkInterval);
            resolve();
          }
        }, 100);
      });

      const timeoutPromise = new Promise<never>((_, reject) => 
        setTimeout(() => reject(new Error('Timeout after 15 seconds')), 15000)
      );

      try {
        // 注册回调
        cacheDownload.onDownloadSuccess(url, callbackB);
        cacheDownload.onDownloadSuccess(url, callbackA);
        
        // 发起两次下载
        cacheDownload.download(url, {});
        cacheDownload.download(url, {});
        
        // 等待两次下载完成
        await Promise.race([downloadPromise, timeoutPromise]);
        
        console.log(`testDownloadSuccess012 cbString: ${cbString}, downloadCounter: ${downloadCounter}`);
        
        expect(cbString).assertEqual("baa");
      } catch (err) {
        console.error('testDownloadSuccess012 download error:', (err as Error).message);
        expect(false).assertTrue();
      }
    });

    /**
     * @tc.number: testDownloadSuccess013
     * @tc.name: testDownloadSuccess013
     * @tc.desc: Test self-unregistering success callback during multiple downloads
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     * @tc.step: 1. Register a self-unregistering success callback and a normal callback
     *           2. Invoke cacheDownload.download twice
     *           3. Verify the self-unregistering callback only triggers once
     * @tc.require: issues#1650
     */
    it('testDownloadSuccess013', 0, async () => {
      const url = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt";
      let cbString = "";
      let downloadCounter = 0;
      
      // 定义自解绑回调函数
      const cbSuccess1 = () => {
        cbString += "a";
        // 自解绑
        cacheDownload.offDownloadSuccess(url, cbSuccess1);
      };

      // 普通回调函数
      const callbackB = () => {
        cbString += "b";
        downloadCounter++;
      };

      // 创建Promise等待两次下载完成
      const downloadPromise = new Promise<void>((resolve) => {
        // 使用轮询检查是否两次下载都已完成
        const checkInterval = setInterval(() => {
          if (downloadCounter >= 2) {
            clearInterval(checkInterval);
            resolve();
          }
        }, 100);
      });

      const timeoutPromise = new Promise<never>((_, reject) => 
        setTimeout(() => reject(new Error('Timeout after 15 seconds')), 15000)
      );

      try {
        // 注册回调
        cacheDownload.onDownloadSuccess(url, cbSuccess1);
        cacheDownload.onDownloadSuccess(url, callbackB);
        
        // 发起两次下载
        cacheDownload.download(url, {});
        cacheDownload.download(url, {});
        
        // 等待两次下载完成
        await Promise.race([downloadPromise, timeoutPromise]);
        
        console.log(`testDownloadSuccess013 cbString: ${cbString}, downloadCounter: ${downloadCounter}`);
        
        expect(cbString).assertEqual("abb");
      } catch (err) {
        console.error('testDownloadSuccess013 download error:', (err as Error).message);
        expect(false).assertTrue();
      }
    });

    /**
     * @tc.number: testDownloadError001
     * @tc.name: testDownloadError001
     * @tc.desc: Test basic error callback registration and triggering
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     * @tc.step: 1. Register download error callback
     *           2. Invoke cacheDownload.download with invalid URL
     *           3. Verify error callback is triggered
     * @tc.require: issues#1650
     */
    it('testDownloadError001', 0, async () => {
      const url = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test123.txt";
      
      const errorPromise = new Promise<void>((resolve) => {
        cacheDownload.onDownloadError(url, (error: cacheDownload.DownloadError) => {
          resolve();
        });
      });

      const timeoutPromise = new Promise<never>((_, reject) => 
        setTimeout(() => reject(new Error('Timeout after 5 seconds')), 5000)
      );

      try {
        cacheDownload.download(url, {});
        await Promise.race([errorPromise, timeoutPromise]);
        expect(true).assertTrue();
      } catch (err) {
        console.error('testDownloadError001 download error:', (err as Error).message);
        expect(false).assertTrue();
      }
    });

    /**
     * @tc.number: testDownloadError002
     * @tc.name: testDownloadError002
     * @tc.desc: Test error callback triggering for multiple downloads
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     * @tc.step: 1. Register download error callback
     *           2. Invoke cacheDownload.download twice with invalid URL
     *           3. Verify error callback is triggered twice
     * @tc.require: issues#1650
     */
    it('testDownloadError002', 0, async () => {
      const url = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test123.txt";
      let errorCount = 0;
      
      const errorPromise = new Promise<void>((resolve) => {
        cacheDownload.onDownloadError(url, (error: cacheDownload.DownloadError) => {
          errorCount++;
          if (errorCount === 2) resolve();
        });
      });

      const timeoutPromise = new Promise<never>((_, reject) => 
        setTimeout(() => reject(new Error('Timeout after 10 seconds')), 10000)
      );

      try {
        cacheDownload.download(url, {});
        cacheDownload.download(url, {});
        await Promise.race([errorPromise, timeoutPromise]);
        expect(true).assertTrue();
      } catch (err) {
        console.error('testDownloadError002 download error:', (err as Error).message);
        expect(false).assertTrue();
      }
    });

    /**
     * @tc.number: testDownloadError003
     * @tc.name: testDownloadError003
     * @tc.desc: Test dynamic callback registration during error handling
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     * @tc.step: 1. Register first error callback
     *           2. In first callback, register second error callback
     *           3. Invoke cacheDownload.download with invalid URL
     *           4. Verify both callbacks are triggered
     * @tc.require: issues#1650
     */
    it('testDownloadError003', 0, async () => {
      const url = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test123.txt";
      
      const errorPromise = new Promise<void>((resolve) => {
        cacheDownload.onDownloadError(url, (error: cacheDownload.DownloadError) => {
          console.log('testDownloadError003 first error');
          cacheDownload.onDownloadError(url, (error: cacheDownload.DownloadError) => {
            console.log('testDownloadError003 second error');
            resolve();
          });
        });
      });

      const timeoutPromise = new Promise<never>((_, reject) => 
        setTimeout(() => reject(new Error('Timeout after 5 seconds')), 5000)
      );

      try {
        cacheDownload.download(url, {});
        await Promise.race([errorPromise, timeoutPromise]);
        expect(true).assertTrue();
      } catch (err) {
        console.error('testDownloadError003 download error:', (err as Error).message);
        expect(false).assertTrue();
      }
    });

    /**
     * @tc.number: testDownloadError004
     * @tc.name: testDownloadError004
     * @tc.desc: Test callback unregistration before download
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     * @tc.step: 1. Register download error callback
     *           2. Immediately unregister the callback
     *           3. Invoke cacheDownload.download with invalid URL
     *           4. Verify callback is not triggered
     * @tc.require: issues#1650
     */
    it('testDownloadError004', 0, async () => {
      const url = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test123.txt";
      let callbackCalled = false;
      
      const errorPromise = new Promise<void>((resolve) => {
        cacheDownload.onDownloadError(url, () => {
          callbackCalled = true;
          resolve();
        });
      });

      const timeoutPromise = new Promise<void>((resolve) => 
        setTimeout(() => resolve(), 5000)
      );

      try {
        // 注册后立即取消
        cacheDownload.offDownloadError(url);
        cacheDownload.download(url, {});
        
        await Promise.race([errorPromise, timeoutPromise]);
        expect(callbackCalled).assertFalse();
      } catch (err) {
        console.error('testDownloadError004 download error:', (err as Error).message);
        expect(false).assertTrue();
      }
    });

    /**
     * @tc.number: testDownloadError005
     * @tc.name: testDownloadError005
     * @tc.desc: Test unregistering all callbacks before download
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     * @tc.step: 1. Register download error callback
     *           2. Unregister all callbacks
     *           3. Invoke cacheDownload.download with invalid URL
     *           4. Verify callback is not triggered
     * @tc.require: issues#1650
     */
    it('testDownloadError005', 0, async () => {
      const url = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test123.txt";
      let callbackCalled = false;
      
      const errorPromise = new Promise<void>((resolve) => {
        cacheDownload.onDownloadError(url, () => {
          callbackCalled = true;
          resolve();
        });
      });

      const timeoutPromise = new Promise<void>((resolve) => 
        setTimeout(() => resolve(), 5000)
      );

      try {
        // 注册多个回调后全部取消
        cacheDownload.offDownloadError(url);
        cacheDownload.download(url, {});
        
        await Promise.race([errorPromise, timeoutPromise]);
        expect(callbackCalled).assertFalse();
      } catch (err) {
        console.error('testDownloadError005 download error:', (err as Error).message);
        expect(false).assertTrue();
      }
    });

    /**
     * @tc.number: testDownloadError006
     * @tc.name: testDownloadError006
     * @tc.desc: Test multiple error callbacks execution order
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     * @tc.step: 1. Register two error callbacks
     *           2. Invoke cacheDownload.download with invalid URL
     *           3. Verify both callbacks are triggered in registration order
     * @tc.require: issues#1650
     */
    it('testDownloadError006', 0, async () => {
      const url = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test123.txt";
      let cbString = "";
      
      const errorPromise = new Promise<void>((resolve) => {
        let count = 0;
        const checkComplete = () => {
          if (count === 2) resolve();
        };
        
        cacheDownload.onDownloadError(url, () => {
          cbString += "a";
          count++;
          checkComplete();
        });
        
        cacheDownload.onDownloadError(url, () => {
          cbString += "b";
          count++;
          checkComplete();
        });
      });

      const timeoutPromise = new Promise<never>((_, reject) => 
        setTimeout(() => reject(new Error('Timeout after 5 seconds')), 5000)
      );

      try {
        cacheDownload.download(url, {});
        await Promise.race([errorPromise, timeoutPromise]);
        expect(cbString).assertEqual("ab");
      } catch (err) {
        console.error('testDownloadError006 download error:', (err as Error).message);
        expect(false).assertTrue();
      }
    });

    /**
     * @tc.number: testDownloadError007
     * @tc.name: testDownloadError007
     * @tc.desc: Test selective callback unregistration
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     * @tc.step: 1. Register two different error callbacks
     *           2. Unregister only the first callback
     *           3. Invoke cacheDownload.download with invalid URL
     *           4. Verify only the second callback is triggered
     * @tc.require: issues#1650
     */
    it('testDownloadError007', 0, async () => {
      const url = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test123.txt";
      let cbString = "";
      
      const errorPromise = new Promise<void>((resolve) => {
        cacheDownload.onDownloadError(url, () => {
          cbString += "b";
          resolve();
        });
      });

      const timeoutPromise = new Promise<never>((_, reject) => 
        setTimeout(() => reject(new Error('Timeout after 5 seconds')), 5000)
      );

      try {
        const callbackA = () => { cbString += "a"; };
        const callbackB = () => { cbString += "b"; };
        
        cacheDownload.onDownloadError(url, callbackA);
        cacheDownload.onDownloadError(url, callbackB);
        
        // 取消第一个回调
        cacheDownload.offDownloadError(url, callbackA);
        
        cacheDownload.download(url, {});
        await Promise.race([errorPromise, timeoutPromise]);
        expect(cbString).assertEqual("b");
      } catch (err) {
        console.error('testDownloadError007 download error:', (err as Error).message);
        expect(false).assertTrue();
      }
    });

    /**
     * @tc.number: testDownloadError008
     * @tc.name: testDownloadError008
     * @tc.desc: Test selective callback unregistration with same function
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     * @tc.step: 1. Register two named error callbacks
     *           2. Unregister only the first named callback
     *           3. Invoke cacheDownload.download with invalid URL
     *           4. Verify only the second callback is triggered
     * @tc.require: issues#1650
     */
    it('testDownloadError008', 0, async () => {
      const url = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test123.txt";
      let cbString = "";
      
      const errorPromise = new Promise<void>((resolve) => {
        cacheDownload.onDownloadError(url, () => {
          cbString += "b";
          resolve();
        });
      });

      const timeoutPromise = new Promise<never>((_, reject) => 
        setTimeout(() => reject(new Error('Timeout after 5 seconds')), 5000)
      );

      try {
        const callbackA = () => { cbString += "a"; };
        const callbackB = () => { cbString += "b"; };
        
        cacheDownload.onDownloadError(url, callbackA);
        cacheDownload.onDownloadError(url, callbackB);
        
        // 取消第一个回调
        cacheDownload.offDownloadError(url, callbackA);
        
        cacheDownload.download(url, {});
        await Promise.race([errorPromise, timeoutPromise]);
        expect(cbString).assertEqual("b");
      } catch (err) {
        console.error('testDownloadError008 download error:', (err as Error).message);
        expect(false).assertTrue();
      }
    });

    /**
     * @tc.number: testDownloadError009
     * @tc.name: testDownloadError009
     * @tc.desc: Test unregistering duplicate callbacks
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     * @tc.step: 1. Register the same error callback multiple times
     *           2. Unregister the callback
     *           3. Invoke cacheDownload.download with invalid URL
     *           4. Verify callback is not triggered
     * @tc.require: issues#1650
     */
    it('testDownloadError009', 0, async () => {
      const url = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test123.txt";
      let cbString = "";
      
      const errorPromise = new Promise<void>((resolve) => {
        setTimeout(() => resolve(), 3000);
      });

      try {
        const callback = () => { cbString += "a"; };
        
        // 注册多个相同回调
        cacheDownload.onDownloadError(url, callback);
        cacheDownload.onDownloadError(url, callback);
        
        // 取消所有该回调
        cacheDownload.offDownloadError(url, callback);
        
        cacheDownload.download(url, {});
        await errorPromise;
        expect(cbString).assertEqual("");
      } catch (err) {
        console.error('testDownloadError009 download error:', (err as Error).message);
        expect(false).assertTrue();
      }
    });

    /**
     * @tc.number: testDownloadError010
     * @tc.name: testDownloadError010
     * @tc.desc: Test unregistering all anonymous callbacks
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     * @tc.step: 1. Register two anonymous error callbacks
     *           2. Unregister all callbacks
     *           3. Invoke cacheDownload.download with invalid URL
     *           4. Verify no callbacks are triggered
     * @tc.require: issues#1650
     */
    it('testDownloadError010', 0, async () => {
      const url = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test123.txt";
      let cbString = "";
      
      const errorPromise = new Promise<void>((resolve) => {
        setTimeout(() => resolve(), 3000);
      });

      try {
        cacheDownload.onDownloadError(url, () => { cbString += "a"; });
        cacheDownload.onDownloadError(url, () => { cbString += "a"; });
        
        // 取消所有回调
        cacheDownload.offDownloadError(url);
        
        cacheDownload.download(url, {});
        await errorPromise;
        expect(cbString).assertEqual("");
      } catch (err) {
        console.error('testDownloadError010 download error:', (err as Error).message);
        expect(false).assertTrue();
      }
    });

    /**
     * @tc.number: testDownloadError011
     * @tc.name: testDownloadError011
     * @tc.desc: Test unregistering callbacks during error handling
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     * @tc.step: 1. Register two error callbacks
     *           2. In first callback, unregister the second callback
     *           3. Invoke cacheDownload.download with invalid URL
     *           4. Verify only the first callback completes execution
     * @tc.require: issues#1650
     */
    it('testDownloadError011', 0, async () => {
      const url = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test123.txt";
      let cbString = "";
      
      const errorPromise = new Promise<void>((resolve) => {
        cacheDownload.onDownloadError(url, () => {
          cbString += "a";
          resolve();
        });
      });

      const timeoutPromise = new Promise<never>((_, reject) => 
        setTimeout(() => reject(new Error('Timeout after 5 seconds')), 5000)
      );

      try {
        const callbackB = () => { cbString += "b"; };
        
        cacheDownload.onDownloadError(url, () => {
          cbString += "a";
          // 在回调中取消另一个回调
          cacheDownload.offDownloadError(url, callbackB);
        });
        
        cacheDownload.onDownloadError(url, callbackB);
        
        cacheDownload.download(url, {});
        await Promise.race([errorPromise, timeoutPromise]);
        expect(cbString).assertEqual("a");
      } catch (err) {
        console.error('testDownloadError011 download error:', (err as Error).message);
        expect(false).assertTrue();
      }
    });

    /**
     * @tc.number: testDownloadError012
     * @tc.name: testDownloadError012
     * @tc.desc: Test unregistering other callbacks during multiple download errors
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     * @tc.step: 1. Register multiple download error callbacks
     *           2. Make one callback unregister another callback during first download
     *           3. Invoke cacheDownload.download twice with invalid URLs
     *           4. Verify callback behavior across both downloads
     * @tc.require: issues#1650
     */
    it('testDownloadError012', 0, async () => {
      const url = "https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/bird123.png";
      let cbString = "";
      let downloadCounter = 0;
      
      // 定义回调函数
      const callbackB = (error: cacheDownload.DownloadError) => {
        cbString += "b";
      };
      
      const callbackA = (error: cacheDownload.DownloadError) => {
        cbString += "a";
        downloadCounter++;
        
        // 在第一次下载的回调中取消callbackB
        if (downloadCounter === 1) {
          cacheDownload.offDownloadError(url, callbackB);
        }
      };

      // 创建Promise等待两次下载完成
      const downloadPromise = new Promise<void>((resolve) => {
        // 使用轮询检查是否两次下载都已完成
        const checkInterval = setInterval(() => {
          if (downloadCounter >= 2) {
            clearInterval(checkInterval);
            resolve();
          }
        }, 100);
      });

      const timeoutPromise = new Promise<never>((_, reject) => 
        setTimeout(() => reject(new Error('Timeout after 15 seconds')), 15000)
      );

      try {
        // 注册回调
        cacheDownload.onDownloadError(url, callbackB);
        cacheDownload.onDownloadError(url, callbackA);
        
        // 发起两次下载
        cacheDownload.download(url, {});
        cacheDownload.download(url, {});
        
        // 等待两次下载完成
        await Promise.race([downloadPromise, timeoutPromise]);
        
        console.log(`testDownloadError012 cbString: ${cbString}, downloadCounter: ${downloadCounter}`);
        
        expect(cbString).assertEqual("baa");
      } catch (err) {
        console.error('testDownloadError012 download error:', (err as Error).message);
        expect(false).assertTrue();
      }
    });
    /**
     * @tc.number: testDownloadError013
     * @tc.name: testDownloadError013
     * @tc.desc: Test self-unregistering error callback during multiple downloads
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     * @tc.step: 1. Register a self-unregistering error callback and a normal callback
     *           2. Invoke cacheDownload.download twice with invalid URLs
     *           3. Verify the self-unregistering callback only triggers once
     * @tc.require: issues#1650
     */
    it('testDownloadError013', 0, async () => {
      const url = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test123.txt";
      let cbString = "";
      let downloadCounter = 0;
      
      // 定义自解绑回调函数
      const cbError1 = (error: cacheDownload.DownloadError) => {
        cbString += "a";
        // 自解绑
        cacheDownload.offDownloadError(url, cbError1);
      };

      // 普通回调函数
      const callbackB = (error: cacheDownload.DownloadError) => {
        cbString += "b";
        downloadCounter++;
      };

      // 创建Promise等待两次下载完成
      const downloadPromise = new Promise<void>((resolve) => {
        // 使用轮询检查是否两次下载都已完成
        const checkInterval = setInterval(() => {
          if (downloadCounter >= 2) {
            clearInterval(checkInterval);
            resolve();
          }
        }, 100);
      });

      const timeoutPromise = new Promise<never>((_, reject) => 
        setTimeout(() => reject(new Error('Timeout after 15 seconds')), 15000)
      );

      try {
        // 注册回调
        cacheDownload.onDownloadError(url, cbError1);
        cacheDownload.onDownloadError(url, callbackB);
        
        // 发起两次下载
        cacheDownload.download(url, {});
        cacheDownload.download(url, {});
        
        // 等待两次下载完成
        await Promise.race([downloadPromise, timeoutPromise]);
        
        console.log(`testDownloadError013 cbString: ${cbString}, downloadCounter: ${downloadCounter}`);
        
        expect(cbString).assertEqual("abb");
      } catch (err) {
        console.error('testDownloadError013 download error:', (err as Error).message);
        expect(false).assertTrue();
      }
    });
  })
}