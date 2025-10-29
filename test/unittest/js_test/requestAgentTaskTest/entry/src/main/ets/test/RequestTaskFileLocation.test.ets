/*
 * Copyright (C) 2025 Huawei Device Co., Ltd.
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
import request from '@ohos.request';
import fs from '@ohos.file.fs';
import commonEvent from '@ohos.commonEventManager';
import common from '@ohos.app.ability.common';
import { BusinessError } from "@ohos.base";

export default function requestFileLocTaskTest() {
  describe('requestFileLocTaskTest', () => {
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
      if (fs.accessSync(cacheDir + '/test.txt')) {
        fs.unlinkSync(cacheDir + '/test.txt');
      }
      if (fs.accessSync(cacheDir + '/README.md')) {
        fs.unlinkSync(cacheDir + '/README.md');
      }
    });

    let context: common.UIAbilityContext = globalThis.abilityContext;
    let cacheDir: String = globalThis.cacheDir;

    /**
     * @tc.number: testDownloadTaskSaveas001
     * @tc.name: testDownloadTaskSaveas001
     * @tc.desc: Test when saveas is undefined
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testDownloadTaskSaveas001', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        saveas: 'testDownloadTaskSaveas001.txt',
        mode: request.agent.Mode.BACKGROUND
      };
      await request.agent.create(context, conf).then(async () => {
        expect(true).assertTrue();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });
    /**
     * @tc.number: testDownloadTaskSaveas002
     * @tc.name: testDownloadTaskSaveas002
     * @tc.desc: Test when saveas is ./
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testDownloadTaskSaveas002', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.BACKGROUND,
        saveas: './'
      };
      await request.agent.create(context, conf).then(async () => {
        expect(true).assertTrue();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });

    /**
     * @tc.number: testDownloadTaskSaveas003
     * @tc.name: testDownloadTaskSaveas003
     * @tc.desc: Test when saveas is ./filename
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testDownloadTaskSaveas003', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.BACKGROUND,
        saveas: './test_saveas_003'
      };
      await request.agent.create(context, conf).then(async () => {
        expect(true).assertTrue();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });

    /**
     * @tc.number: testDownloadTaskSaveas004
     * @tc.name: testDownloadTaskSaveas004
     * @tc.desc: Test when saveas is filename
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testDownloadTaskSaveas004', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.BACKGROUND,
        saveas: 'test_saveas_004'
      };
      await request.agent.create(context, conf).then(async () => {
        expect(true).assertTrue();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });

    /**
     * @tc.number: testDownloadTaskSaveas005
     * @tc.name: testDownloadTaskSaveas005
     * @tc.desc: Test when saveas is pathname/filename
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testDownloadTaskSaveas005', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.BACKGROUND,
        saveas: 'a/b/test_saveas_005'
      };
      await request.agent.create(context, conf).then(async () => {
        expect(true).assertTrue();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });

    /**
     * @tc.number: testDownloadTaskSaveas006
     * @tc.name: testDownloadTaskSaveas006
     * @tc.desc: Test when saveas is internal://cache/filename
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testDownloadTaskSaveas006', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.BACKGROUND,
        saveas: 'internal://cache/test_saveas_006'
      };
      await request.agent.create(context, conf).then(async () => {
        expect(true).assertTrue();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });

    /**
     * @tc.number: testDownloadTaskSaveas007
     * @tc.name: testDownloadTaskSaveas007
     * @tc.desc: Test when saveas is whole path in base
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testDownloadTaskSaveas007', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.BACKGROUND,
        saveas: '/data/storage/el1/base/test_saveas_007'
      };
      await request.agent.create(context, conf).then(async () => {
        expect(true).assertTrue();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });

    /**
     * @tc.number: testDownloadTaskSaveas008
     * @tc.name: testDownloadTaskSaveas008
     * @tc.desc: Test when saveas is whole path but not in base
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testDownloadTaskSaveas008', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.BACKGROUND,
        saveas: '/data/storage/test_saveas_008'
      };
      try {
        await request.agent.create(context, conf).then(async () => {
          expect(false).assertTrue();
          done();
        }).catch((err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(true).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testDownloadTaskSaveas009
     * @tc.name: testDownloadTaskSaveas009
     * @tc.desc: Test when saveas is whole path which contains ..
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testDownloadTaskSaveas009', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.BACKGROUND,
        saveas: '/data/storage/../../data/storage/el1/../el2/base/test_saveas_009'
      };
      await request.agent.create(context, conf).then(async () => {
        expect(true).assertTrue();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });

    /**
     * @tc.number: testDownloadTaskSaveas010
     * @tc.name: testDownloadTaskSaveas010
     * @tc.desc: Test when saveas is whole path which contains error ..
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testDownloadTaskSaveas010', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.BACKGROUND,
        saveas: '/data/storage/../../el2/base/test_saveas_010'
      };
      try {
        await request.agent.create(context, conf).then(async () => {
          expect(false).assertTrue();
          done();
        }).catch((err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(true).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testDownloadTaskSaveas011
     * @tc.name: testDownloadTaskSaveas011
     * @tc.desc: Test when saveas is file:// with correct package name
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testDownloadTaskSaveas011', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.BACKGROUND,
        saveas: 'file://com.acts.request/data/storage/el2/base/test_saveas_011'
      };
      await request.agent.create(context, conf).then(async () => {
        expect(true).assertTrue();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });

    /**
     * @tc.number: testDownloadTaskSaveas012
     * @tc.name: testDownloadTaskSaveas012
     * @tc.desc: Test when saveas is file:// with error package name
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testDownloadTaskSaveas012', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.BACKGROUND,
        saveas: 'file://com.acts.request1111/data/storage/el2/base/test_saveas_012'
      };
      try {
        await request.agent.create(context, conf).then(async () => {
          expect(false).assertTrue();
          done();
        }).catch((err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(true).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testDownloadTaskSaveas013
     * @tc.name: testDownloadTaskSaveas013
     * @tc.desc: Test when saveas is internal://files/filename
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testDownloadTaskSaveas013', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt',
        mode: request.agent.Mode.BACKGROUND,
        saveas: 'internal://files/path_save_as_013/test_saveas_013'
      };
      await request.agent.create(context, conf).then(async () => {
        expect(true).assertTrue();
        done();
      }).catch((err: Error) => {
        expect(false).assertTrue();
        done();
      });
    });

    /**
     * @tc.number: testUploadTaskPath001
     * @tc.name: testUploadTaskPath001
     * @tc.desc: Test when path is ./filename
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testUploadTaskPath001', 0, async (done: Function) => {
      let context: common.UIAbilityContext = globalThis.abilityContext;
      let dir: String = context.cacheDir;
      let file = fs.openSync(dir + '/upload_001', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
          name: 'file',
          value: {
            path: `./upload_001`
          }
        }]
      };
      try {
        await request.agent.create(context, conf).then(async () => {
          if (fs.accessSync(dir + '/upload_001')) {
            fs.unlinkSync(dir + '/upload_001');
          }
          expect(true).assertTrue();
          done();
        }).catch((err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testUploadTaskPath002
     * @tc.name: testUploadTaskPath002
     * @tc.desc: Test when path is filename
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testUploadTaskPath002', 0, async (done: Function) => {
      let context: common.UIAbilityContext = globalThis.abilityContext;
      let dir: String = context.cacheDir;
      let file = fs.openSync(dir + '/upload_002', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
          name: 'file',
          value: {
            path: `./upload_002`
          }
        }]
      };
      try {
        await request.agent.create(context, conf).then(async () => {
          if (fs.accessSync(dir + '/upload_002')) {
            fs.unlinkSync(dir + '/upload_002');
          }
          expect(true).assertTrue();
          done();
        }).catch((err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testUploadTaskPath003
     * @tc.name: testUploadTaskPath003
     * @tc.desc: Test when path is internal://cache/filename
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testUploadTaskPath003', 0, async (done: Function) => {
      let context: common.UIAbilityContext = globalThis.abilityContext;
      let dir: String = context.cacheDir;
      let file = fs.openSync(dir + '/upload_003', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
          name: 'file',
          value: {
            path: `internal://cache/upload_003`
          }
        }]
      };
      try {
        await request.agent.create(context, conf).then(async () => {
          if (fs.accessSync(dir + '/upload_003')) {
            fs.unlinkSync(dir + '/upload_003');
          }
          expect(true).assertTrue();
          done();
        }).catch((err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testUploadTaskPath004
     * @tc.name: testUploadTaskPath004
     * @tc.desc: Test when path is internal://files/filename
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testUploadTaskPath004', 0, async (done: Function) => {
      let context: common.UIAbilityContext = globalThis.abilityContext;
      let dir: String = context.filesDir;
      let file = fs.openSync(dir + '/upload_004', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
          name: 'file',
          value: {
            path: `internal://files/upload_004`
          }
        }]
      };
      try {
        await request.agent.create(context, conf).then(async () => {
          if (fs.accessSync(dir + '/upload_004')) {
            fs.unlinkSync(dir + '/upload_004');
          }
          expect(true).assertTrue();
          done();
        }).catch((err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testUploadTaskPath005
     * @tc.name: testUploadTaskPath005
     * @tc.desc: Test when path is whole path in base
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testUploadTaskPath005', 0, async (done: Function) => {
      let context: common.UIAbilityContext = globalThis.abilityContext;
      let dir: String = context.cacheDir;
      let file = fs.openSync(dir + '/upload_005', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
          name: 'file',
          value: {
            path: dir + `/upload_005`
          }
        }]
      };
      try {
        await request.agent.create(context, conf).then(async () => {
          if (fs.accessSync(dir + '/upload_005')) {
            fs.unlinkSync(dir + '/upload_005');
          }
          expect(true).assertTrue();
          done();
        }).catch((err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testUploadTaskPath006
     * @tc.name: testUploadTaskPath006
     * @tc.desc: Test when path is whole path but not in base
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testUploadTaskPath006', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
          name: 'file',
          value: {
            path: `/data/storage/upload_006`
          }
        }]
      };
      try {
        await request.agent.create(context, conf).then(async () => {
          expect(false).assertTrue();
          done();
        }).catch((err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(true).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testUploadTaskPath007
     * @tc.name: testUploadTaskPath007
     * @tc.desc: Test when path is whole path which contains ..
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testUploadTaskPath007', 0, async (done: Function) => {
      let context: common.UIAbilityContext = globalThis.abilityContext.getApplicationContext();
      let dir: String = context.cacheDir;
      let file = fs.openSync(dir + '/upload_007', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
          name: 'file',
          value: {
            path: '/data/storage/../../data/storage/el1/../el2/base/cache/upload_007'
          }
        }]
      };
      try {
        await request.agent.create(context, conf).then(async () => {
          if (fs.accessSync(dir + '/upload_007')) {
            fs.unlinkSync(dir + '/upload_007');
          }
          expect(true).assertTrue();
          done();
        }).catch((err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testUploadTaskPath008
     * @tc.name: testUploadTaskPath008
     * @tc.desc: Test when path is whole path which contains error ..
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testUploadTaskPath008', 0, async (done: Function) => {
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
          name: 'file',
          value: {
            path: '/data/storage/../../el2/base/upload_008'
          }
        }]
      };
      try {
        await request.agent.create(context, conf).then(async () => {
          expect(false).assertTrue();
          done();
        }).catch((err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(true).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testUploadTaskPath009
     * @tc.name: testUploadTaskPath009
     * @tc.desc: Test when path is file:// with correct package name
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testUploadTaskPath009', 0, async (done: Function) => {
      let context: common.UIAbilityContext = globalThis.abilityContext.getApplicationContext();
      let dir: String = context.cacheDir;
      let file = fs.openSync(dir + '/upload_009', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
          name: 'file',
          value: {
            path: 'file://com.acts.request/data/storage/el2/base/cache/upload_009'
          }
        }]
      };
      try {
        await request.agent.create(context, conf).then(async () => {
          if (fs.accessSync(dir + '/upload_009')) {
            fs.unlinkSync(dir + '/upload_009');
          }
          expect(true).assertTrue();
          done();
        }).catch((err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(false).assertTrue();
        done();
      }
    });

    /**
     * @tc.number: testUploadTaskPath010
     * @tc.name: testUploadTaskPath010
     * @tc.desc: Test when path is file:// with error package name
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.require: issues#1573
     */
    it('testUploadTaskPath010', 0, async (done: Function) => {
      let context: common.UIAbilityContext = globalThis.abilityContext.getApplicationContext();
      let dir: String = context.cacheDir;
      let file = fs.openSync(dir + '/upload_010', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.closeSync(file);
      let conf: request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'http://127.0.0.1',
        data: [{
          name: 'file',
          value: {
            path: 'file://com.acts.request1111/data/storage/el2/base/cache/upload_010'
          }
        }]
      };
      try {
        await request.agent.create(context, conf).then(async () => {
          if (fs.accessSync(dir + '/upload_010')) {
            fs.unlinkSync(dir + '/upload_010');
          }
          expect(false).assertTrue();
          done();
        }).catch((err: Error) => {
          expect(false).assertTrue();
          done();
        });
      } catch (err) {
        expect(true).assertTrue();
        done();
      }
    });
  })
}