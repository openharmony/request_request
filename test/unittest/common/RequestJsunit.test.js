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


let DownloadConfig = {
    //https://sf3-cn.feishucdn.com/obj/ee-appcenter/6d6bc5/Feishu-win32_ia32-5.10.6-signed.exe
    //  url: 'https://sf3-cn.feishucdn.com/obj/ee-appcenter/6d6bc5/Feishu-win32_ia32-5.10.6-signed.exe', // Resource address.
    //url: 'https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png', // Resource address.
    url: 'http://192.168.8.128:8080/HFS_SERVER/123.rar',
    // url: 'http://sf3-cn.feishucdn.com/obj/ee-appcenter/6d6bc5/Feishu-win32_ia32-5.10.6-signed.exe',
    filePath: '/data/storage/el2/base/haps/entry/files/123.rar', // Sets the path for downloads./data/accounts/account_0/appdata/com.example.downloaddemo/files/picture.png
    enableMetered: false,
    enableRoaming: false,
    networkType: 65536, //65536 wifi  1 sim卡网络
    background: true,
}

describe('requestTest',function () {

   console.log(TAG + "*************Unit Test Begin*************");


    /**
     * @tc.name: downloadTest001
     * @tc.desc see if download starts correctly
     * @tc.type: FUNC
     * @tc.require:
     */
    it('downloadTest001', 0, function () {
        console.log(TAG + "************* downloadTest001 start *************");

        request.download(DownloadConfig).then((DownloadTask) => {
            console.log(keyStr + 'download start, DownloadTask: ' + DownloadTask);
            console.log(keyStr + 'download start, DownloadTask: ' + JSON.stringify(DownloadTask));
            DownloadTask.on('progress',(receivedSize, totalSize) => {
                expect(totalSize == 0).assertEqual(false)
                expect(receivedSize == 0).assertEqual(false);
            })
        })

        console.log(TAG + "************* downloadTest001 end *************");
    })

    console.log(TAG + "*************Unit Test End*************");
})