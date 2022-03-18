# Copyright (c) 2021 Huawei Device Co., Ltd.
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

// @ts-nocheck
import request from '@ohos.request';
import {describe, expect, it} from 'deccjsunit/index.ets'

let DownloadConfig = {
  url: 'https://mirror.bjtu.edu.cn/kernel/linux/libs/libc5/libc5.cvs.tar.gz',// Resource address.
  header: { }, // Adds an HTTP or HTTPS header to be included with the upload request.
  enableMetered: true,
  enableRoaming: true, // Allows download in a roaming network.
  description: 'a', // Sets the description of a download session.
  networkType: 1, // Sets the network type allowed for download.
  filePath: '/data/abc1.txt', // Sets the path for downloads.
  title: 'a', // Sets a download session title.
}
let DownloadConfig1 = {
  url: 'https://mirror.bjtu.edu.cn/kernel/linux/devel/binutils/binutils-2.10.91.0.2-1.i386.rpm',// Resource address.
  header: { }, // Adds an HTTP or HTTPS header to be included with the upload request.
  enableMetered: true,
  enableRoaming: true, // Allows download in a roaming network.
  description: 'a1', // Sets the description of a download session.
  networkType: 1, // Sets the network type allowed for download.
  filePath: '/system/lib/abc1.txt', // Sets the path for downloads.
  title: 'a1', // Sets a download session title.
}
let DownloadConfig2 = {
  url: 'https://mirror.bjtu.edu.cn/kernel/linux/devel/binutils/binutils-2.11.90.0.23-2.11.90.0.24.sign',// Resource address.
  header: { }, // Adds an HTTP or HTTPS header to be included with the upload request.
  enableMetered: true,
  enableRoaming: true, // Allows download in a roaming network.
  description: 'a2', // Sets the description of a download session.
  networkType: 1, // Sets the network type allowed for download.
  filePath: '/system/lib/abc2.txt', // Sets the path for downloads.
  title: 'a2', // Sets a download session title.
}
let DownloadConfig3 = {
  header: { }, // Adds an HTTP or HTTPS header to be included with the upload request.
  enableMetered: true,
  enableRoaming: true, // Allows download in a roaming network.
}
let DownloadConfig4 = {
  url: 'https://mirror.bjtu.edu.cn/kernel/linux/devel/binutils/autoconf-2.13-10.tar.gz',// Resource address.
  header: { }, // Adds an HTTP or HTTPS header to be included with the upload request.
  enableMetered: true,
  enableRoaming: true, // Allows download in a roaming network.
  description: 'a2', // Sets the description of a download session.
  networkType: 1, // Sets the network type allowed for download.
  filePath: '/system/lib/abc2.txt', // Sets the path for downloads.
  title: 'a2', // Sets a download session title.
}
let DownloadConfig5 = {
  url: 'https://mirror.bjtu.edu.cn/kernel/linux/devel/binutils/automake-1.4-8.3.tar.gz',// Resource address.
  header: { }, // Adds an HTTP or HTTPS header to be included with the upload request.
  enableMetered: true,
  enableRoaming: true, // Allows download in a roaming network.
  description: 'a2', // Sets the description of a download session.
  networkType: 1, // Sets the network type allowed for download.
  filePath: '/system/lib/abc2.txt', // Sets the path for downloads.
  title: 'a2', // Sets a download session title.
}
let DownloadConfig6 = {
  url: 'https://mirror.bjtu.edu.cn/kernel/linux/devel/binutils/binutils-2.10.91.0.2-2.10.91.0.4.diff.gz',// Resource address.
  header: { }, // Adds an HTTP or HTTPS header to be included with the upload request.
  enableMetered: true,
  enableRoaming: true, // Allows download in a roaming network.
  description: 'a2', // Sets the description of a download session.
  networkType: 1, // Sets the network type allowed for download.
  filePath: '/system/lib/abc2.txt', // Sets the path for downloads.
  title: 'a2', // Sets a download session title.
}
let DownloadConfig7 = {
  url: 'https://mirror.bjtu.edu.cn/kernel/linux/devel/binutils/binutils-2.11.90.0.23-2.11.90.0.24.sign',// Resource address.
  header: { }, // Adds an HTTP or HTTPS header to be included with the upload request.
  enableMetered: true,
  enableRoaming: true, // Allows download in a roaming network.
  description: 'a2', // Sets the description of a download session.
  networkType: 1, // Sets the network type allowed for download.
  filePath: '/system/lib/abc2.txt', // Sets the path for downloads.
  title: 'a2', // Sets a download session title.
}
let DownloadConfig8 = {
  url: 'https://mirror.bjtu.edu.cn/kernel/linux/devel/binutils/binutils-2.11.90.0.5-2.11.90.0.6.diff.gz',// Resource address.
  header: { }, // Adds an HTTP or HTTPS header to be included with the upload request.
  enableMetered: true,
  enableRoaming: true, // Allows download in a roaming network.
  description: 'a2', // Sets the description of a download session.
  networkType: 1, // Sets the network type allowed for download.
  filePath: '/system/lib/abc2.txt', // Sets the path for downloads.
  title: 'a2', // Sets a download session title.
}
let DownloadConfig9 = {
  url: 'https://mirror.bjtu.edu.cn/kernel/linux/devel/binutils/binutils-2.11.90.0.6-2.11.90.0.7.diff.gz',// Resource address.
  header: { }, // Adds an HTTP or HTTPS header to be included with the upload request.
  enableMetered: true,
  enableRoaming: true, // Allows download in a roaming network.
  description: 'a2', // Sets the description of a download session.
  networkType: 1, // Sets the network type allowed for download.
  filePath: '/system/lib/abc2.txt', // Sets the path for downloads.
  title: 'a2', // Sets a download session title.
}
let DownloadConfig10 = {
  url: 'https://mirror.bjtu.edu.cn/kernel/linux/devel/binutils/binutils-2.11.90.0.7-2.11.90.0.8.diff.gz',// Resource address.
  header: { }, // Adds an HTTP or HTTPS header to be included with the upload request.
  enableMetered: true,
  enableRoaming: true, // Allows download in a roaming network.
  description: 'a2', // Sets the description of a download session.
  networkType: 1, // Sets the network type allowed for download.
  filePath: '/system/lib/abc2.txt', // Sets the path for downloads.
  title: 'a2', // Sets a download session title.
}
let DownloadConfig11 = {
  url: 'https://mirror.bjtu.edu.cn/kernel/linux/devel/binutils/binutils-2.11.90.0.8-2.11.90.0.15.diff.gz',// Resource address.
  header: { }, // Adds an HTTP or HTTPS header to be included with the upload request.
  enableMetered: true,
  enableRoaming: true, // Allows download in a roaming network.
  description: 'a2', // Sets the description of a download session.
  networkType: 1, // Sets the network type allowed for download.
  filePath: '/system/lib/abc2.txt', // Sets the path for downloads.
  title: 'a2', // Sets a download session title.
}
let DownloadConfig12 = {
  url: 'https://mirror.bjtu.edu.cn/kernel/linux/devel/binutils/binutils-2.11.92.0.5-2.11.92.0.7.diff.gz',// Resource address.
  header: { }, // Adds an HTTP or HTTPS header to be included with the upload request.
  enableMetered: true,
  enableRoaming: true, // Allows download in a roaming network.
  description: 'a2', // Sets the description of a download session.
  networkType: 1, // Sets the network type allowed for download.
  filePath: '/system/lib/abc2.txt', // Sets the path for downloads.
  title: 'a2', // Sets a download session title.
}
let DownloadConfig13 = {
  url: 'https://mirror.bjtu.edu.cn/kernel/linux/devel/binutils/binutils-2.11.92.0.7-2.11.92.0.10.diff.gz',// Resource address.
  header: { }, // Adds an HTTP or HTTPS header to be included with the upload request.
  enableMetered: true,
  enableRoaming: true, // Allows download in a roaming network.
  description: 'a2', // Sets the description of a download session.
  networkType: 1, // Sets the network type allowed for download.
  filePath: '/system/lib/abc2.txt', // Sets the path for downloads.
  title: 'a2', // Sets a download session title.
}
let DownloadConfig14 = {
  url: 'https://mirror.bjtu.edu.cn/kernel/linux/devel/binutils/binutils-2.12.90.0.1-2.12.90.0.3.diff.gz',// Resource address.
  header: { }, // Adds an HTTP or HTTPS header to be included with the upload request.
  enableMetered: true,
  enableRoaming: true, // Allows download in a roaming network.
  description: 'a2', // Sets the description of a download session.
  networkType: 1, // Sets the network type allowed for download.
  filePath: '/system/lib/abc2.txt', // Sets the path for downloads.
  title: 'a2', // Sets a download session title.
}
let DownloadConfig15 = {
  url: 'https://mirror.bjtu.edu.cn/kernel/linux/devel/binutils/binutils-2.12.90.0.11-2.12.90.0.12.diff.gz',// Resource address.
  header: { }, // Adds an HTTP or HTTPS header to be included with the upload request.
  enableMetered: true,
  enableRoaming: true, // Allows download in a roaming network.
  description: 'a2', // Sets the description of a download session.
  networkType: 1, // Sets the network type allowed for download.
  filePath: '/system/lib/abc2.txt', // Sets the path for downloads.
  title: 'a2', // Sets a download session title.
}
let DownloadConfig16 = {
  url: 'https://mirror.bjtu.edu.cn/kernel/linux/devel/binutils/binutils-2.12.90.0.14-2.12.90.0.15.diff.gz',// Resource address.
  header: { }, // Adds an HTTP or HTTPS header to be included with the upload request.
  enableMetered: true,
  enableRoaming: true, // Allows download in a roaming network.
  description: 'a2', // Sets the description of a download session.
  networkType: 1, // Sets the network type allowed for download.
  filePath: '/system/lib/abc2.txt', // Sets the path for downloads.
  title: 'a2', // Sets a download session title.
}
const SLEEP_TIME = 1000

export default function exampleTestJsunit() {

  describe('appInfoTest', function () {
    function sleep(numberMillis)
    {
      var now = new Date();
      var exitTime = now.getTime() + numberMillis;
      while (true) {
        now = new Date();
        if (now.getTime() > exitTime)
        return;
      }
    }
    console.log("-----------------------Download_Test is starting-----------------------");
    /*
     * @tc.number  Download_Test_0100
     * @tc.name    Create file downloads
     * @tc.desc    Function test
     * @tc.level   0
     */
    it("Download_Test_0100", 0, async function(done){
      console.log("Download_Test_DownloadConfig", JSON.stringify(DownloadConfig));
      console.log("------------------start Download_Test_0100-------------------");
      try{
        console.log("Show me the log");
        request.download(DownloadConfig, (err, data) => {
            console.log("Download_Test_0100: register download issue successful, result = " +data);
            expect(typeof(data) == 'number').assertTrue();
          done();
        });
        sleep(SLEEP_TIME);
      } catch (error) {
        console.log("logMessage Download_Test_0100: error = " + error);
      }
      console.log("------------------end Download_Test_0100-------------------");
    });
    /*
     * @tc.number  Download_Test_0200
     * @tc.name    Create a file download, monitor the download process
     * @tc.desc    Function test
     * @tc.level   0
     */
    it("Download_Test_0200", 0, async function(done){
      console.log("------------------start Download_Test_0200 on progress-------------------");
      try{
        let eventType = 'progress'
        request.download(DownloadConfig4, (err, data) => {
          console.log("Download_Test_0200: register download  issue successful, result = " +data);
          data.on(eventType,(err1,data1,data2) =>{
            console.log("Download_Test_0200: , on progress result data1 = "+data1);
            console.log("Download_Test_0200: , on progress result data2 = "+data2);
            expect(typeof(data1) == 'number').assertTrue();
            expect(typeof(data2) == 'number').assertTrue();
            done();
          })
          sleep(SLEEP_TIME);
//          done();
        });
        sleep(SLEEP_TIME);
      } catch (error) {
        console.log("logMessage pause: error = " + error);
      }
    });
    /*
     * @tc.number  Download_Test_0300
     * @tc.name    Create file download, monitor download failure
     * @tc.desc    Function test
     * @tc.level   0
     */
    it("Download_Test_0300", 0, async function(done){
      console.log("------------------start Download_Test_0300 on fail-------------------");
      try{
        let isEnter = 1
        let eventType = 'fail'
        request.download(DownloadConfig5, (err, data) => {
          console.log("Download_Test_0300: register download issue successful, result = " +data);
          data.on(eventType,(err1,data1) =>{
            isEnter = 2
            console.log("Download_Test_0300: , on fail result = "+data1);
            expect(isEnter == 2).assertTrue();
            done();
          })
//          done();
        });
        sleep(SLEEP_TIME);
      } catch (error) {
        console.log("logMessage on fail: error = " + error);
      }
    });
    /*
     * @tc.number  Download_Test_0400
     * @tc.name    Create file download, monitor download completion
     * @tc.desc    Function test
     * @tc.level   0
     */
    it("Download_Test_0400", 0, async function(done){
      console.log("------------------start Download_Test_0400-------------------");
      try{
        let isEnter = 1
        let eventType = 'complete';
        request.download(DownloadConfig6, (err, data) => {
          console.log("Download_Test_0400: register download issue successful, result = " +data);
          data.on(eventType,(err1,data1) =>{
            isEnter = 2
            console.log("Download_Test_0400: , on complete result = "+data1);
            expect(isEnter == 2).assertTrue();
            done();
          })
//          done();
        });
        sleep(SLEEP_TIME);
      } catch (error) {
        console.log("logMessage remove: error = " + error);
      }
    });
    /*
     * @tc.number  Download_Test_0500
     * @tc.name    Create a file download, monitor and cancel the monitor download process
     * @tc.desc    Function test
     * @tc.level   0
     */
    it("Download_Test_0500", 0, async function(done){
      console.log("------------------start Download_Test_0500-------------------");
      try{
        let isEnter = 1
        let eventType = 'progress';
        request.download(DownloadConfig7, (err, data) => {
          console.log("Download_Test_0500: register download issue successful, result = " +data);
          data.on(eventType,(err1,data1) =>{
            console.log("Download_Test_0500: , on progress result = "+data1);
          });
          sleep(SLEEP_TIME);
          data.off(eventType,(err1,data1,data2) =>{
            isEnter = 2
            console.log("Download_Test_0500: , off progress result = "+data1);
            console.log("Download_Test_0500: , off progress result = "+data2);
            expect(isEnter == 2).assertTrue();
            done();
          });
//          done();
        });
        sleep(SLEEP_TIME);
      } catch (error) {
        console.log("Download_Test_0500 queryMimeType: error = " + error);
      }
    });
    /*
     * @tc.number  Download_Test_0600
     * @tc.name    Create file downloads, monitor and cancel monitoring download failures
     * @tc.desc    Function test
     * @tc.level   0
     */
    it("Download_Test_0600", 0, async function(done){
      console.log("------------------start Download_Test_0600-------------------");
      try{
        let isEnter = 1
        let eventType = 'fail';
        request.download(DownloadConfig8, (err, data) => {
          console.log("Download_Test_0600: register download issue successful, result = " +data);
          data.on(eventType,(err,data) =>{
            console.log("Download_Test_0600: register on fail  issue successful, result = " +data);
          });
          sleep(SLEEP_TIME);
          data.off(eventType,(err,data) =>{
            isEnter = 2
            console.log("Download_Test_0600: register off fail  issue successful, result = " +data);
            expect(isEnter == 2).assertTrue();
            done();
          });
        });
        sleep(SLEEP_TIME);
      } catch (error) {
        console.log("Download_Test_0600 off fail: error = " + error);
      }
    });
    /*
     * @tc.number  Download_Test_0700
     * @tc.name    Create file download, listen for download completion and cancel listening for download completion
     * @tc.desc    Function test
     * @tc.level   0
     */
    it("Download_Test_0700", 0, async function(done){
      console.log("------------------start Download_Test_0700-------------------");
      try{
        let isEnter = 1
        let eventType = 'complete';
        request.download(DownloadConfig9, (err, data) => {
          console.log("Download_Test_0700: register download issue successful, result = " +data);
          data.on(eventType,(err1,data1) =>{
            console.log("Download_Test_0700: , on complete result = "+data1);
          })
          sleep(SLEEP_TIME);
          data.off(eventType,(err1,data1) =>{
            isEnter = 2
            console.log("Download_Test_0700: , off complete result = "+data1);
            expect(isEnter == 2).assertTrue();
            done();
          })
        });
        sleep(SLEEP_TIME);
      } catch (error) {
        console.log("Download_Test_0700 off complete: error = " + error);
      }
    });
    /*
     * @tc.number  Download_Test_0800
     * @tc.name    Create file download, pause file download
     * @tc.desc    Function test
     * @tc.level   0
     */
    it("Download_Test_0800", 0, async function(done){
      console.log("------------------start Download_Test_0800-------------------");
      try{
        request.download(DownloadConfig10, (err, data) => {
          let eventType = 'pause';
          console.log("Download_Test_0800: register download issue successful, result = " +data);
          data.on(eventType,(err1,data1) =>{
            console.log("Download_Test_0800: , on pause result = "+data1);
          })
          data.pause((err1,data1) =>{
            console.log("Download_Test_0800: , off pause result = "+data1);
            expect(data1==true).assertTrue();
            done();
          })
        });
        sleep(SLEEP_TIME);
      } catch (error) {
        console.log("Download_Test_0800 pause: error = " + error);
      }
    });
    /*
     * @tc.number  Download_Test_0900
     * @tc.name    Create a file download, pause the file download and resume the download
     * @tc.desc    Function test
     * @tc.level   0
     */
    it("Download_Test_0900", 0, async function(done){
      console.log("------------------start Download_Test_0900-------------------");
      try{
        request.download(DownloadConfig11, (err, data) => {
          console.log("Download_Test_0900: register download issue successful, result = " +data);
          data.pause((err1,data1) =>{
            console.log("Download_Test_0900: , pause result = "+data1);
          })
          data.resume((err1,data1) =>{
            console.log("Download_Test_0900: , resume result = "+data1);
            expect(data1==true).assertTrue();
            done();
          })
        });
        sleep(SLEEP_TIME);
      } catch (error) {
        console.log("Download_Test_0900 resume: error = " + error);
      }
    });
    /*
     * @tc.number  Download_Test_1000
     * @tc.name    Create file download, cancel file download
     * @tc.desc    Function test
     * @tc.level   0
     */
    it("Download_Test_1000", 0, async function(done){
      console.log("------------------start Download_Test_1000-------------------");
      try{
        request.download(DownloadConfig12, (err, data) => {
          console.log("Download_Test_1000: register download issue successful, result = " +data);
          data.remove((err1,data1) =>{
            console.log("Download_Test_1000: , remove result = "+data1);
            expect(data1==true).assertTrue();
            done();
          })
        });
        sleep(SLEEP_TIME);
      } catch (error) {
        console.log("Download_Test_1000 remove: error = " + error);
      }
    });
    /*
     * @tc.number  Download_Test_1100
     * @tc.name    Create multiple file downloads
     * @tc.desc    Function test
     * @tc.level   0
     */
    it("Download_Test_1100", 0, async function(done){
      console.log("------------------start Download_Test_1100-------------------");
      try{
        request.download(DownloadConfig13, (err, data) => {
          console.log("Download_Test_1100: register download issue successful, result = " +data);
          expect(typeof(data) == 'object').assertTrue();
        });
        request.download(DownloadConfig12, (err, data) => {
          console.log("Download_Test_1100: register download issue successful, result = " +data);
          expect(typeof(data) == 'object').assertTrue();
        });
        request.download(DownloadConfig2, (err, data) => {
          console.log("Download_Test_1100: register download issue successful, result = " +data);
          expect(typeof(data) == 'object').assertTrue();
          done();
        });
        sleep(SLEEP_TIME);
      } catch (error) {
        console.log("Download_Test_1100 multiple: error = " + error);
      }
    });
    /*
     * @tc.number  Download_Test_1200
     * @tc.name    Create file downloads and receive download file MIME type
     * @tc.desc    Function test
     * @tc.level   0
     */
    it("Download_Test_1200", 0, async function(done){
      console.log("------------------start Download_Test_1200-------------------");
      try{
        request.download(DownloadConfig14, (err, data) => {
          console.log("Download_Test_1200: register download issue successful, result = " +data);
          data.queryMimeType((err1,data1) =>{
            console.log("Download_Test_1200: register queryMimeType  issue successful, result ="+data1);
            expect(typeof(data1) == 'string').assertTrue();
          })
          done();
        });
        sleep(SLEEP_TIME);
      } catch (error) {
        console.log("Download_Test_1200 queryMimeType: error = " + error);
      }
    });
    /*
     * @tc.number  Download_Test_1300
     * @tc.name    Create file downloads and receive download info
     * @tc.desc    Function test
     * @tc.level   0
     */
    it("Download_Test_1300", 0, async function(done){
      console.log("------------------start Download_Test_1300-------------------");
      try{
        request.download(DownloadConfig15, (err, data) => {
          console.log("Download_Test_1300: register download issue successful, result = " +data);
          data.query((err1,data1) =>{
            console.log("Download_Test_1300: register query  issue successful, result ="+JSON.stringify(data1));
            expect(typeof(data1) == 'object').assertTrue();
          })
          done();
        });
      } catch (error) {
        console.log("Download_Test_1300 query: error = " + error);
      }
    });
    /*
     * @tc.number  Download_Test_1400
     * @tc.name    Create file downloads and missing parameters
     * @tc.desc    Function test
     * @tc.level   0
     */
    it("Download_Test_1400", 0, async function(done){
      console.log("------------------start Download_Test_1400-------------------");
      try{
        request.download(DownloadConfig3, (err, data) => {
          console.log("Download_Test_1400: register download issue successful, result = " +data);
          expect(data==obj).assertTrue();
          done();
        });
        sleep(SLEEP_TIME);
      } catch (error) {
        console.log("Download_Test_1400 missing parameters: error = " + error);
      }
    });
  })
}

