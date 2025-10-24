/*
 * Copyright (c) 2023 Huawei Device Co., Ltd.
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

import cacheDownloadInfo from './CacheDownloadInfo.test.ets';
import requestCreateTaskTest from './RequestCreateTask.test.ets';
import requestBasicConfigTest from './RequestBasicConfig.test.ets';
import requestOperateTaskTest from './RequestOperateTask.test.ets';
import requestCallbackTaskTest from './RequestCallbackTask.test.ets';
import requestDownloadFileTest from './RequestDownloadFile.test.ets';
import requestUploadFileTest from './RequestUploadFile.test.ets';
import requestHttpConfigTest from './RequestCreateHttpConf.test.ets';
import requestTaskGroupTest from './RequestTaskGroup.test.ets';
import requestCreateFileConfTest from './RequestCreateFileConf.test.ets';
import requestNewUploadTest from './RequestNewUpload.test.ets';
import requestDataConfigCreateTest from './RequestDataConfigCreate.ets';
import requestFileLocTaskTest from './RequestTaskFileLocation.test.ets';
import requestHttpLimitTest from './RequestHttpLimit.test.ets';
import requestForeignKitTest from './RequestForeignKit.test.ets';

export default function testsuite() {
  cacheDownloadInfo();
  requestCreateTaskTest();
  requestBasicConfigTest();
  requestOperateTaskTest();
  requestCallbackTaskTest();
  requestDownloadFileTest();
  requestUploadFileTest();
  requestHttpConfigTest();
  requestTaskGroupTest();
  requestCreateFileConfTest();
  requestNewUploadTest();
  requestDataConfigCreateTest();
  requestFileLocTaskTest();
  requestHttpLimitTest();
  requestForeignKitTest();
}