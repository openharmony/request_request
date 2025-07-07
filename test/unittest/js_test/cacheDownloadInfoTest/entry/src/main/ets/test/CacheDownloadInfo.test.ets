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

// @ts-nocheck
import { describe, beforeAll, beforeEach, afterEach, afterAll, it, expect } from "@ohos/hypium";
import cacheDownload from '@ohos.request.cacheDownload';

export default function cacheDownloadInfo() {
  describe('cacheDownloadInfo', function () {
    beforeAll(function () {
      console.info('beforeAll called');
    })

    afterAll(function () {
      console.info('afterAll called');
    })

    beforeEach(function () {
      console.info('beforeEach called');
    })

    afterEach(async function () {
      console.info('afterEach called');
    })

    function sleep(ms) {
      return new Promise(resolve => setTimeout(resolve, ms));
    }

    /**
     * @tc.name: testSetInfoListSize001
     * @tc.desc: setDownloadInfoListSize sets the size to be less than 0
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testSetInfoListSize001', 0, async function (done) {
      try {
        cacheDownload.setDownloadInfoListSize(-1);
        expect(false).assertTrue();
        done();
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.name: testSetInfoListSize002
     * @tc.desc: setDownloadInfoListSize sets the size to be greater than 8192
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testSetInfoListSize002', 0, async function (done) {
      try {
        cacheDownload.setDownloadInfoListSize(8193);
        expect(false).assertTrue();
        done();
      } catch (err) {
        expect(err.code).assertEqual(401);
        done();
      }
    })

    /**
     * @tc.name: testSetInfoListSize003
     * @tc.desc: setDownloadInfoListSize sets the size to be greater than 8192
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testSetInfoListSize003', 0, async function (done) {
      try {
        cacheDownload.setDownloadInfoListSize(8192);
        done();
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    })

    /**
     * @tc.name: testGetDownloadInfo001
     * @tc.desc: getDownloadInfoList updates the same url info
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testGetDownloadInfo001', 0, async function (done) {
      try {
        let options: cacheDownload.CacheDownloadOptions = {};
        cacheDownload.setDownloadInfoListSize(1);
        cacheDownload.download("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt",
                  options);
        await sleep(5000);
        let downloadInfo = cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt");
        let total_time = 0;
        if (downloadInfo == undefined) {
            expect(false).assertTrue();
            done();
        } else {
            total_time = downloadInfo.performance.totalTime;
        }
        cacheDownload.download("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt",
                  options);
        await sleep(5000);
        let downloadInfo2 = cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt");
        if (downloadInfo2 == undefined) {
            expect(false).assertTrue();
            done();
        } else {
          expect(total_time != downloadInfo2.performance.totalTime).assertTrue();
          done();
        }
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    })

    /**
     * @tc.name: testGetDownloadInfo002
     * @tc.desc: The set size is valid and can save the info of the number of sizes.
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testGetDownloadInfo002', 0, async function (done) {
      try {
        let options: cacheDownload.CacheDownloadOptions = {};
        cacheDownload.setDownloadInfoListSize(1);
        cacheDownload.download("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt",
                  options);
        await sleep(5000);
        let downloadInfo = cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt");
        if (downloadInfo == undefined) {
          expect(false).assertTrue();
          done();
        }
        cacheDownload.download("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/edge_picture.jpg",
                  options);
        await sleep(5000);
        let downloadInfo2 = cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/edge_picture.jpg");
        if (downloadInfo2 == undefined) {
          expect(false).assertTrue();
          done();
        }
        let downloadInfo_1 = cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt");
        if (downloadInfo_1 == undefined) {
          done();
        } else {
          expect(false).assertTrue();
          done();
        }
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    })

    /**
     * @tc.name: testGetDownloadInfo003
     * @tc.desc: remove the oldest information and save the latest one
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testGetDownloadInfo003', 0, async function (done) {
      try {
        let options: cacheDownload.CacheDownloadOptions = {};
        cacheDownload.setDownloadInfoListSize(2);
        cacheDownload.download("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt",
                  options);
        await sleep(5000);
        let downloadInfo = cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt");
        if (downloadInfo == undefined) {
          expect(false).assertTrue();
          done();
        }
        cacheDownload.download("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/edge_picture.jpg",
                  options);
        await sleep(5000);
        let downloadInfo2 = cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/edge_picture.jpg");
        if (downloadInfo2 == undefined) {
          expect(false).assertTrue();
          done();
        }
        cacheDownload.download("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/audio_test.mp3",
                  options);
        await sleep(5000);
        let downloadInfo3 = cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/audio_test.mp3");
        if (downloadInfo3 == undefined) {
          expect(false).assertTrue();
          done();
        }
        let downloadInfo_1 = cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt");
        if (downloadInfo_1 == undefined) {
          done();
        } else {
          expect(false).assertTrue();
          done();
        }
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    })

    /**
     * @tc.name: testGetDownloadInfo004
     * @tc.desc: getDownloadInfo can update the corresponding url information to the latest
     * @tc.type: FUNC
     * @tc.require:
     */
    it('testGetDownloadInfo004', 0, async function (done) {
      try {
        let options: cacheDownload.CacheDownloadOptions = {};
        cacheDownload.setDownloadInfoListSize(2);
        cacheDownload.download("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt",
                  options);
        await sleep(5000);
        let downloadInfo = cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt");
        if (downloadInfo == undefined) {
          expect(false).assertTrue();
          done();
        }
        cacheDownload.download("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/edge_picture.jpg",
                  options);
        await sleep(5000);
        let downloadInfo2 = cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/edge_picture.jpg");
        if (downloadInfo2 == undefined) {
          expect(false).assertTrue();
          done();
        }

        let downloadInfo_1 = cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt");
        if (downloadInfo_1 == undefined) {
          expect(false).assertTrue();
          done();
        }
        cacheDownload.download("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/audio_test.mp3",
                  options);
        await sleep(5000);
        let downloadInfo3 = cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/audio_test.mp3");
        if (downloadInfo3 == undefined) {
          expect(false).assertTrue();
          done();
        }
        let downloadInfo_2 = cacheDownload.getDownloadInfo("https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/edge_picture.jpg");
        if (downloadInfo_2 == undefined) {
          done();
        } else {
          expect(false).assertTrue();
          done();
        }
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    })
    
  })
}