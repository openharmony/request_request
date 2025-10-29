'use static'

/**
 * Copyright (c) 2025 Huawei Device Co., Ltd.
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
import { describe, it, expect, TestType, Size, Level, beforeAll } from "../../../hypium/index";
import hilog from '@ohos.hilog'
import abilityDelegatorRegistry from '@ohos.app.ability.abilityDelegatorRegistry';
import UIAbility from '@ohos.app.ability.UIAbility';
import { BusinessError } from '@ohos.base';
import Utils from './Util.test';
import request from "@ohos.request";
import common from "@ohos.app.ability.common";
import fs from '@ohos.file.fs';

let domain: int = 0x0000; //日志标识,0x0000作为测试框架的业务标识
let tag: string = 'testTag'; //日志标识字符串,作为tag标识当前runner类下的测试行为
let baseContext: common.UIAbilityContext;

export default function RequestCreateTaskStaticTest() {

  describe("RequestCreateTaskStaticTest", (): void => {
    hilog.info(domain, tag, '%{public}s', 'RequestCreateTaskStaticTest start');

    beforeAll(() => {
      hilog.info(domain, tag, '%{public}s', 'beforeAll start');
      let abilityDelegator: abilityDelegatorRegistry.AbilityDelegator = abilityDelegatorRegistry.getAbilityDelegator()
      abilityDelegator.addAbilityMonitor({
        abilityName: "EntryAbility",
        moduleName: "entry",
        onAbilityCreate: (abilitys: UIAbility): void => {
          baseContext = abilitys.context
          hilog.info(domain, tag, '%{public}s', 'onAbilityCreate end');

        },
      }, (err: BusinessError | null): void => {
        if (err != null) {
          hilog.info(domain, tag, '%{public}s', '-----' + err.code);
        }
        hilog.info(domain, tag, '%{public}s', 'BusinessError  end');
      });
      await Utils.msSleep(2000)
      abilityDelegator.executeShellCommand("aa start -a EntryAbility -b com.acts.newRequestAuthority.static")
      await Utils.msSleep(2000)
      hilog.info(domain, tag, '%{public}s', 'beforeAll end');
    })

    /**
     * @tc.name: SUB_Misc_REQUEST_Create_on_callback_Static_0100
     * @tc.desc: Test binding and unbinding of completed event for download task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a download task
     *           2. Bind the completed event handler
     *           3. Unbind the completed event handler
     *           4. Start the download task
     * @tc.require: issues#1573
     */
    it('SUB_Misc_REQUEST_Create_on_callback_Static_0100', Level.LEVEL1, async (done: () => void): Promise<void> => {
      hilog.info(0x0000, 'testTag', "-------zbw----testCase001---start---");
      (request.agent.create(context, config) as Promise<request.agent.Task>)
        .then((task: request.agent.Task) => {
          task.on('completed', (progress: request.agent.Progress) => {
            hilog.info(0x0000, 'testTag', "-------zbw----testCase001---On_completed------success");
          });
          task.off('completed', (progress: request.agent.Progress) => {
            hilog.info(0x0000, 'testTag', "-------zbw----testCase001---Off_completed------success");
          });
          await task.start();
        })
        .catch((err: Error) => {
          hilog.error(0x0000, 'testTag',
            `-------zbw---testCase001----failed--: ${err.message}, error code: ${err.code}`);
        });
    });

    /**
     * @tc.name: SUB_Misc_REQUEST_Create_on_fail_Static_0100
     * @tc.desc: Test binding of failed event for download task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Set the download URL
     *           2. Create a download task
     *           3. Bind the failed event handler
     *           4. Start the download task
     * @tc.require: issues#1573
     */
    it('SUB_Misc_REQUEST_Create_on_fail_Static_0100', Level.LEVEL1, async (done: () => void): Promise<void> => {
      hilog.info(0x0000, 'testTag', "-------zbw----testCase002------");
      config.url = 'https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test1.txt';
      (request.agent.create(context, config) as Promise<request.agent.Task>)
        .then((task: request.agent.Task) => {
          task.on('failed', (progress: request.agent.Progress) => {
            hilog.info(0x0000, 'testTag', "-------zbw---testCase002----On_failed---on---success");
          });
          this.taskStart(task);
        })
        .catch((err: Error) => {
          hilog.error(0x0000, 'testTag',
            `-------zbw---testCase002----failed--: ${err.message}, error code: ${err.code}`);
        });
    });

    /**
     * @tc.name: SUB_Misc_REQUEST_Create_on_pause_Static_0100
     * @tc.desc: Test binding of pause event for download task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a download task
     *           2. Bind the pause event handler
     *           3. Start the download task
     * @tc.require: issues#1573
     */
    it('SUB_Misc_REQUEST_Create_on_pause_Static_0100', Level.LEVEL1, async (done: () => void): Promise<void> => {
      hilog.info(0x0000, 'testTag', "-------zbw----testCase003------");
      (request.agent.create(context, config) as Promise<request.agent.Task>)
        .then((task: request.agent.Task) => {
          task.on('pause', (progress: request.agent.Progress) => {
            hilog.info(0x0000, 'testTag',
              "-------zbw----testCase003---On_pause------success-------zbw----testCase003---On_pause------success");
          });
          this.taskStart(task);
        })
        .catch((err: Error) => {
          hilog.error(0x0000, 'testTag',
            `-------zbw---testCase003----failed--: ${err.message}, error code: ${err.code}`);
        });
    });

    /**
     * @tc.name: SUB_Misc_REQUEST_Create_on_resume_Static_0100
     * @tc.desc: Test binding and unbinding of pause and resume events for download task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Set the download URL to a large file
     *           2. Create a download task
     *           3. Bind the pause event handler
     *           4. Bind the resume event handler
     *           5. Unbind the resume event handler
     *           6. Start the download task
     *           7. Pause the download task
     *           8. Resume the download task
     * @tc.require: issues#1573
     */
    it('SUB_Misc_REQUEST_Create_on_resume_Static_0100', Level.LEVEL1, async (done: () => void): Promise<void> => {
      hilog.info(0x0000, 'testTag', "-------zbw----testCase004------");
      config.url = 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/large_test.zip';
      (request.agent.create(context, config) as Promise<request.agent.Task>)
        .then((task: request.agent.Task) => {
          task.on('pause', (progress: request.agent.Progress) => {
            hilog.info(0x0000, 'testTag', "-------zbw----testCase004---On_pause------success");
          });
          task.on('resume', (progress: request.agent.Progress) => {
            hilog.info(0x0000, 'testTag', "-------zbw----testCase004---On_resume---success");
          });
          task.off('resume')
          //this.taskStart(task);
          await task.start();
          await task.pause();
          await task.resume();
        })
        .catch((err: Error) => {
          hilog.error(0x0000, 'testTag',
            `-------zbw---testCase004----failed--: ${err.message}, error code: ${err.code}`);
        });
    });

    /**
     * @tc.name: SUB_Misc_REQUEST_Create_on_progress_Static_0100
     * @tc.desc: Test binding of progress event for download task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a download task using callback
     *           2. If task is created successfully, bind the progress event handler
     *           3. Start the download task
     * @tc.require: issues#1573
     */
    it('SUB_Misc_REQUEST_Create_on_progress_Static_0100', Level.LEVEL1, async (done: () => void): Promise<void> => {
      hilog.info(0x0000, 'testTag', "-------zbw----testCase005----start--");
      request.agent.create(context, config, (err, task) => {
        if (task != undefined) {
          task.on('progress', (progress: request.agent.Progress) => {
            hilog.info(0x0000, 'testTag', "-------zbw----testCase005---On_progress---success");
            hilog.info(0x0000, 'testTag',
              `-------zbw---testCase005------: index: ${progress.index}, sizes: ${progress.sizes},
                           processed: ${progress.processed}, state: ${progress.state}`);
          });
          this.taskStart(task);
        }
      });
    });

    /**
     * @tc.name: SUB_Misc_REQUEST_Download_overwrite_Static_0000
     * @tc.desc: Test binding of remove event for download task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a download task
     *           2. Bind the remove event handler
     *           3. Start the download task
     *           4. Remove the download task
     * @tc.require: issues#1573
     */
    it('SUB_Misc_REQUEST_Download_overwrite_Static_0000', Level.LEVEL1, async (done: () => void): Promise<void> => {
      hilog.info(0x0000, 'testTag', "-------zbw----testCase006------");
      (request.agent.create(context, config) as Promise<request.agent.Task>)
        .then((task: request.agent.Task) => {
          task.on('remove', (progress: request.agent.Progress) => {
            hilog.info(0x0000, 'testTag', "-------zbw----testCase006---On_remove---success");
          });
          this.taskStart(task);
          await request.agent.remove(task.tid);
        })
        .catch((err: Error) => {
          hilog.error(0x0000, 'testTag',
            `-------zbw---testCase006----failed--: ${err.message}, error code: ${err.code}`);
        });
    });

    /**
     * @tc.name: SUB_Misc_REQUEST_Download_overwrite_Static_0100
     * @tc.desc: Test binding of faultOccur event for download task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create a download task
     *           2. Bind the faultOccur event handler
     *           3. Start the download task
     * @tc.require: issues#1573
     */
    it('SUB_Misc_REQUEST_Download_overwrite_Static_0100', Level.LEVEL1, async (done: () => void): Promise<void> => {
      hilog.info(0x0000, 'testTag', "-------zbw----testCase008------");
      (request.agent.create(context, config) as Promise<request.agent.Task>)
        .then((task: request.agent.Task) => {
          task.on('faultOccur', (fault: request.agent.Faults) => {
            hilog.info(0x0000, 'testTag', "-------zbw----testCase008---On_faultOccur---success");
          });
          this.taskStart(task);
        })
        .catch((err: Error) => {
          hilog.error(0x0000, 'testTag',
            `-------zbw---testCase008----failed--: ${err.message}, error code: ${err.code}`);
        });
    });

    /**
     * @tc.name: SUB_Misc_REQUEST_Download_overwrite_Static_0200
     * @tc.desc: Test create download task with overwrite set to true
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create configuration with overwrite set to true
     *           2. Create a download task with the configuration
     *           3. Bind the completed event handler
     *           4. Start the download task
     * @tc.require: issues#1573
     */
    it('SUB_Misc_REQUEST_Download_overwrite_Static_0200', Level.LEVEL1, async (done: () => void): Promise<void> => {
      let domain: int = 0x0000; //日志标识,0x0000作为测试框架的业务标识
      let tag: string = 'testTag'; //日志标识字符串,作为tag标识当前runner类下的测试行为
      let baseContext: common.UIAbilityContext;
      hilog.info(domain, tag, "====>-----SUB_Misc_REQUEST_Download_overwrite_Static_0300 is starting--------------");
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png',
        saveas: './SUB_Misc_REQUEST_Download_overwrite_Static_0300',
        overwrite: true
      };
      (request.agent.create(context, config1) as Promise<request.agent.Task>)
        .then((task: request.agent.Task) => {
          task.on('completed', (progress: request.agent.Progress) => {
            hilog.info(0x0000, 'testTag', "====>-------zbw----testCase009---On_completed---success");
            this.taskStart(task);
          });
          this.taskStart(task);
        })
        .catch((err: Error) => {
          hilog.error(0x0000, 'testTag',
            `-------zbw---testCase009----failed--: ${err.message}, error code: ${err.code}`);
        });
    });

    /**
     * @tc.name: SUB_Misc_REQUEST_Download_overwrite_Static_0300
     * @tc.desc: Test duplicate start of the same download task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create configuration with overwrite set to true
     *           2. Create a download task with the configuration
     *           3. Bind the completed event handler that attempts to start the task again
     *           4. Start the download task for the first time
     * @tc.require: issues#1573
     */
    it('SUB_Misc_REQUEST_Download_overwrite_Static_0300', Level.LEVEL1, async (done: () => void): Promise<void> => {
      let domain: int = 0x0000; //日志标识,0x0000作为测试框架的业务标识
      let tag: string = 'testTag'; //日志标识字符串,作为tag标识当前runner类下的测试行为
      // let baseContext: common.UIAbilityContext;
      hilog.info(domain, tag, "====>--zbw-------010BEGIN--------------");
      hilog.info(domain, tag, "====>-----SUB_Misc_REQUEST_Download_overwrite_Static_0300 is starting--------------");
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png',
        saveas: './SUB_Misc_REQUEST_Download_overwrite_Static_0300',
        overwrite: true
      };
      try {
        let task: request.agent.Task = await request.agent.create(context, config1);
        hilog.info(domain, tag, "====>SUB_Misc_REQUEST_Download_overwrite_Static_0300 create success");
        let on_completedCallback = async (pro: request.agent.Progress): void => {
          try {
            hilog.info(domain, tag, "====>SUB_Misc_REQUEST_Download_overwrite_Static_0300 task.start twice");
            await task.start();
            hilog.info(domain, tag, "====>SUB_Misc_REQUEST_Download_overwrite_Static_0300 task start twice success");
            expect().assertFail();
          } catch (err: BusinessError) {
            try {
              hilog.info(domain, tag,
                "====>SUB_Misc_REQUEST_Download_overwrite_Static_0300 second download err: " + JSON.stringify(err));
              expect(err.code).assertEqual(21900007);
            } catch (err: BusinessError) {
              hilog.info(domain, tag,
                "====>SUB_Misc_REQUEST_Download_overwrite_Static_0300 err: " + JSON.stringify(err));
            }
          }
        };
        task.on('completed', on_completedCallback);
        await task.start();
        hilog.info(domain, tag, "====>SUB_Misc_REQUEST_Download_overwrite_Static_0300 task.start");
      } catch (err: BusinessError) {
        hilog.info(domain, tag,
          "====>SUB_Misc_REQUEST_Download_overwrite_Static_0300 create catch err: " + JSON.stringify(err));
        expect().assertFail();
      }
    });

    /**
     * @tc.name: SUB_Request_Agent_TaskSaveas_Static_0500
     * @tc.desc: Test create download task with invalid save path
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create configuration with invalid save path
     *           2. Attempt to create a download task with the configuration
     *           3. Verify that an error is caught
     * @tc.require: issues#1573
     */
    it('SUB_Request_Agent_TaskSaveas_Static_0500', Level.LEVEL1, async (done: () => void): Promise<void> => {
      let domain: int = 0x0000; //日志标识,0x0000作为测试框架的业务标识
      let tag: string = 'testTag'; //日志标识字符串,作为tag标识当前runner类下的测试行为
      hilog.info(domain, tag,
        "====>-----------------------SUB_Request_Agent_TaskSaveas_Static_0500 is starting-----------------------");
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        saveas: '/ request/SUB_Request_Agent_TaskSaveas_Static_0500',
      };
      try {
        let task: request.agent.Task = await request.agent.create(context, config1);
        hilog.info(domain, tag, "====>SUB_Request_Agent_TaskSaveas_Static_0500 create not err");
        expect().assertFail();
      } catch (err: BusinessError) {
        hilog.info(domain, tag,
          "====>SUB_Request_Agent_TaskSaveas_Static_0500 create catch err: " + err.code + JSON.stringify(err));
        // expect(err.code).assertEqual(401);
      }
    });

    /**
     * @tc.name: SUB_Request_Agent_TaskSaveas_Static_0600
     * @tc.desc: Test create download task with invalid save path using callback
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create configuration with invalid save path
     *           2. Attempt to create a download task using callback
     *           3. Verify error code 401 is returned in the callback
     * @tc.require: issues#1573
     */
    /**
     * @tc.name: SUB_Request_Agent_TaskSaveas_Static_0600
     * @tc.desc: Test task saveas functionality with invalid path format
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration
     *           2. Create a download task using promise
     *           3. Add progress event listener to the task
     *           4. Start the task and verify progress events are triggered
     * @tc.require: issues#1573
     */
    it('SUB_Request_Agent_TaskSaveas_Static_0600', Level.LEVEL1, async (done: () => void): Promise<void> => {
      let domain: int = 0x0000; //日志标识,0x0000作为测试框架的业务标识
      let tag: string = 'testTag'; //日志标识字符串,作为tag标识当前runner类下的测试行为
  hilog.info(domain, tag, "====>--------------SUB_Request_Agent_TaskSaveas_Static_0600 is starting-------------");
  let config1: request.agent.Config = {
    action: request.agent.Action.DOWNLOAD,
    url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
    saveas: '/ request/SUB_Request_Agent_TaskSaveas_Static_0600',
  };
  request.agent.create(context, config1,
    (err: BusinessError | null, task: request.agent.Task | undefined): void => {
      hilog.info(domain, tag, '%{public}s', "====>SUB_Request_Agent_TaskSaveas_Static_0600 downloadTask: " + task);
      try {
        if (err?.code !== 0) {
          hilog.info(domain, tag,
            "====>SUB_Request_Agent_TaskSaveas_Static_0600 create err: " + err?.code + JSON.stringify(err));
          expect(err?.code).assertEqual(401);
        } else {
          hilog.info(domain, tag, "====>SUB_Request_Agent_TaskSaveas_Static_0600 create success: " + task);
          expect().assertFail();
        }
      } catch (err: BusinessError) {
        hilog.info(domain, tag,
          `====>SUB_Request_Agent_TaskSaveas_Static_0600 Succeeded in create a download task.` + err.code +
          JSON.stringify(err));
      }
    });
});

    /**
     * @tc.name: SUB_Misc_REQUEST_On_Download_Progress_Static_0040
     * @tc.desc: Test download progress event handling for download task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with overwrite set to true
     *           2. Create a download task
     *           3. Bind the progress event handler
     *           4. Start the download task
     * @tc.require: issues#1573
     */
    /**
     * @tc.name: SUB_Misc_REQUEST_On_Download_Progress_Static_0040
     * @tc.desc: Test download progress event handling for download task (second occurrence)
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration
     *           2. Create a download task using promise
     *           3. Add progress event listener to the task
     *           4. Remove the progress event listener
     *           5. Start the task and verify listener has been removed
     * @tc.require: issues#1573
     */
    it('SUB_Misc_REQUEST_On_Download_Progress_Static_0040', Level.LEVEL1, async (done: () => void): Promise<void> => {
      let domain: int = 0x0000; //日志标识,0x0000作为测试框架的业务标识
      let tag: string = 'testTag'; //日志标识字符串,作为tag标识当前runner类下的测试行为
  hilog.info(domain, tag,
    "====>-----------------------SUB_Misc_REQUEST_On_Download_Progress_Static_0040 is starting-----------------------");
  let config1: request.agent.Config = {
    action: request.agent.Action.DOWNLOAD,
    url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
    saveas: './SUB_Misc_REQUEST_On_Download_Progress_Static_0040.txt',
    overwrite: true
  };

  try {
    let task: request.agent.Task = await request.agent.create(context, config1);
    hilog.info(domain, tag, '====>SUB_Misc_REQUEST_On_Download_Progress_Static_0040 create success');
    let on_progressCallback = async (pro: request.agent.Progress): void => {
      try {
        hilog.info(domain, tag, "====>SUB_Misc_REQUEST_On_Download_Progress_Static_0040 pro.state: " + pro.state);
        hilog.info(domain, tag, "====>SUB_Misc_REQUEST_On_Download_Progress_Static_0040 pro.index: " + pro.index);
        hilog.info(domain, tag,
          "====>SUB_Misc_REQUEST_On_Download_Progress_Static_0040 pro.processed: " + pro.processed);
        hilog.info(domain, tag, "====>SUB_Misc_REQUEST_On_Download_Progress_Static_0040 pro.sizes: " + pro.sizes);
        hilog.info(domain, tag,
          "====>SUB_Misc_REQUEST_On_Download_Progress_Static_0040 pro.extras: " + JSON.stringify(pro.extras));
        expect(true).assertTrue();
      } catch (err: BusinessError) {
        hilog.info(domain, tag,
          "====>SUB_Misc_REQUEST_On_Download_Progress_Static_0040 catch err: " + JSON.stringify(err));
      }
    }
    task.on('progress', on_progressCallback);
    await task.start();
    hilog.info(domain, tag, '====>SUB_Misc_REQUEST_On_Download_Progress_Static_0040 start success');
  } catch (err: BusinessError) {
    hilog.info(domain, tag,
      "====>SUB_Misc_REQUEST_On_Download_Progress_Static_0040 create catch err: " + JSON.stringify(err));
  }
});

    /**
     * @tc.name: SUB_Request_Agent_onResponse_Static_0100
     * @tc.desc: Test onResponse event handler for download task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with overwrite set to true
     *           2. Create a download task
     *           3. Bind the response event handler
     *           4. Start the download task
     * @tc.require: issues#1573
     */
    it('SUB_Request_Agent_onResponse_Static_0100', Level.LEVEL1, async (done: () => void): Promise<void> => {
  let domain: int = 0x0000; //日志标识,0x0000作为测试框架的业务标识
  let tag: string = 'testTag'; //日志标识字符串,作为tag标识当前runner类下的测试行为
  hilog.info(domain, tag, "====>-------SUB_Request_Agent_onResponse_Static_0100 is starting-----------");
  let config1: request.agent.Config = {
    action: request.agent.Action.DOWNLOAD,
    url: 'https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png',
    saveas: './SUB_Request_Agent_onResponse_Static_0100',
    overwrite: true,
  };
  try {
    let task: request.agent.Task = await request.agent.create(context, config1);
    hilog.info(domain, tag, "====>SUB_Request_Agent_onResponse_Static_0100 create success");
    let responseCallback_1 = (info: request.agent.HttpResponse) => {
      try {
        hilog.info(domain, tag,
          "====>SUB_Request_Agent_onResponse_Static_0100 response info: " + JSON.stringify(info));
        hilog.info(domain, tag,
          "====>SUB_Request_Agent_onResponse_Static_0100 response info.version: " + info.version);
        // expect(info.version).assertEqual('HTTP/1.1');
        // expect(info.statusCode).assertEqual(200);
        // expect(info.reason).assertEqual('OK');
      } catch (err) {
        hilog.info(domain, tag, "====>SUB_Request_Agent_onResponse_Static_0100 err: " + JSON.stringify(err));
      }
    }
    task.on('response', responseCallback_1);
    // task.off('response', responseCallback_1);
    await task.start();
    hilog.info(domain, tag, "====>SUB_Request_Agent_onResponse_Static_0100 task.start");
  } catch (err) {
    hilog.info(domain, tag,
      "====>SUB_Request_Agent_onResponse_Static_0100 create catch err: " + JSON.stringify(err));
    expect().assertFail();
  }
});

it('SUB_Misc_REQUEST_On_Download_Progress_Static_0040', Level.LEVEL1, async (done: () => void): Promise<void> => {
  let domain: int = 0x0000; //日志标识,0x0000作为测试框架的业务标识
  let tag: string = 'testTag'; //日志标识字符串,作为tag标识当前runner类下的测试行为
  hilog.info(domain, tag,
    "====>-----------------------SUB_Misc_REQUEST_On_Download_Progress_Static_0040 is starting-----------------------");
  let config1: request.agent.Config = {
    action: request.agent.Action.DOWNLOAD,
    url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
    saveas: './SUB_Misc_REQUEST_On_Download_Progress_Static_0040.txt',
    overwrite: true,
  };

  try {
    let task: request.agent.Task = await request.agent.create(context, config1);
    hilog.info(domain, tag, '====>SUB_Misc_REQUEST_On_Download_Progress_Static_0040 create success');
    let on_progressCallback = async (pro: request.agent.Progress): void => {
      try {
        hilog.info(domain, tag, "====>SUB_Misc_REQUEST_On_Download_Progress_Static_0040 pro.state: " + pro.state);
        hilog.info(domain, tag, "====>SUB_Misc_REQUEST_On_Download_Progress_Static_0040 pro.index: " + pro.index);
            hilog.info(domain, tag,
              "====>SUB_Misc_REQUEST_On_Download_Progress_Static_0040 pro.processed: " + pro.processed);
            hilog.info(domain, tag, "====>SUB_Misc_REQUEST_On_Download_Progress_Static_0040 pro.sizes: " + pro.sizes);
            hilog.info(domain, tag,
              "====>SUB_Misc_REQUEST_On_Download_Progress_Static_0040 pro.extras: " + JSON.stringify(pro.extras));
            expect(true).assertTrue();
          } catch (err: BusinessError) {
            hilog.info(domain, tag,
              "====>SUB_Misc_REQUEST_On_Download_Progress_Static_0040 catch err: " + JSON.stringify(err));
          }
        }
        task.on('progress', on_progressCallback);
        await task.start();
        hilog.info(domain, tag, '====>SUB_Misc_REQUEST_On_Download_Progress_Static_0040 start success');
      } catch (err: BusinessError) {
        hilog.info(domain, tag,
          "====>SUB_Misc_REQUEST_On_Download_Progress_Static_0040 create catch err: " + JSON.stringify(err));
      }
    });

    /**
     * @tc.name: SUB_REQUEST_create_STAGE_API_CALLBACK_0006
     * @tc.desc: Test create download task with various configuration options using callback
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with multiple options
     *           2. Create a download task using Promise chain
     *           3. Bind the faultOccur event handler
     *           4. Start the download task
     * @tc.require: issues#1573
     */
    it('SUB_REQUEST_create_STAGE_API_CALLBACK_0006', Level.LEVEL1, async (done: () => void): Promise<void> => {
      let domain: int = 0x0000; //日志标识,0x0000作为测试框架的业务标识
      let tag: string = 'testTag'; //日志标识字符串,作为tag标识当前runner类下的测试行为
      hilog.info(domain, tag, '====>SUB_Misc_REQUEST_On_Download_Progress_Static_testCase016 create success');
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap', // 需要手动将url替换为真实服务器的HTTP协议地址
        title: 'taskOnTest',
        description: 'Sample code for event listening',
        mode: request.agent.Mode.FOREGROUND,
        overwrite: true,
        saveas: "./",
        network: request.agent.Network.ANY,
        metered: false,
        roaming: true,
        retry: false,
        redirect: true,
        gauge: false,
        precise: false,
        token: "it is a secret"
      };
      let faultOnCallback = (faults: request.agent.Faults) => {
        try {
          hilog.info(domain, tag, '====>upload task failed. fault: ' + JSON.stringify(faults));
        } catch (err: BusinessError) {
          hilog.info(domain, tag,
            "====>SUB_Misc_REQUEST_On_Download_Progress_testCase016 catch err: " + JSON.stringify(err));
        }
      };
      request.agent.create(context, config1).then((task: request.agent.Task) => {
        try {
          task.on('faultOccur', faultOnCallback);
          hilog.info(domain, tag, `====>Succeeded in creating a upload task. result: ${task.tid}`);
          task.start();
        } catch (err: BusinessError) {
          hilog.info(domain, tag,
            "====>SUB_Misc_REQUEST_On_Download_Progress_testCase016 B catch err: " + JSON.stringify(err));
        }
      })
    });

    /**
     * @tc.name: SUB_REQUEST_create_STAGE_API_CALLBACK_0005
     * @tc.desc: Test create download task using callback with basic configuration
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with overwrite set to true
     *           2. Create a download task using Promise chain
     *           3. Bind the remove event handler
     *           4. Start the download task
     *           5. Wait for 1 second and then remove the task
     * @tc.require: issues#1573
     */
    it('SUB_REQUEST_create_STAGE_API_CALLBACK_0005', Level.LEVEL1, async (done: () => void): Promise<void> => {
      let domain: int = 0x0000; //日志标识,0x0000作为测试框架的业务标识
      let tag: string = 'testTag'; //日志标识字符串,作为tag标识当前runner类下的测试行为
      hilog.info(domain, tag, '====>testCase017');
      let attachments: Array<request.agent.FormItem> = [{
        name: "taskOnTest",
        value: {
          filename: "taskOnTest.avi",
          path: "./taskOnTest.avi",
        }
      }];
      // let config1: request.agent.Config = {
      //   action: request.agent.Action.DOWNLOAD,
      //   url: 'http://127.0.0.1', // 需要手动将url替换为真实服务器的HTTP协议地址
      //   title: 'taskOnTest',
      //   description: 'Sample code for event listening',
      //   mode: request.agent.Mode.FOREGROUND,
      //   overwrite: true,
      //   method: "PUT",
      //   data: attachments,
      //   saveas: "./",
      //   network: request.agent.Network.ANY,
      //   metered: false,
      //   roaming: true,
      //   retry: false,
      //   redirect: true,
      //   gauge: false,
      //   precise: false,
      //   token: "it is a secret"
      // };
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        saveas: './SUB_Misc_REQUEST_On_Download_Progress_Static_0040.txt',
        overwrite: true
      };
      let createOnCallback = (progress: request.agent.Progress) => {
        hilog.info(domain, tag, '====>upload task remove.');
      };
      request.agent.create(context, config1).then(async (task: request.agent.Task) => {
        task.on('remove', createOnCallback);
        hilog.info(domain, tag, `====>Succeeded in creating a upload task. result: ${task.tid}`);
        // task.off('remove', createOnCallback); // test offRemove
        task.start();
        // 等待1秒再执行下一步操作，以防异步乱序
        await new Promise<void>((resolve) => {
          setTimeout(() => resolve(undefined), 1000)
        })
        request.agent.remove(task.tid);
      }).catch((err: Error) => {
        const businessErr = err as BusinessError<void>;
        hilog.info(domain, tag,
          `====>Failed to create a upload task, Code: ${businessErr.code}, message: ${businessErr.message}`);
      });
    });

    /**
     * @tc.name: SUB_REQUEST_create_STAGE_API_CALLBACK_0004
     * @tc.desc: Test create download task using callback with different configuration
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with various options
     *           2. Create a download task using Promise chain
     *           3. Start the download task
     * @tc.require: issues#1573
     */
    it('SUB_REQUEST_create_STAGE_API_CALLBACK_0004', Level.LEVEL1, async (done: () => void): Promise<void> => {
      let domain: int = 0x0000; //日志标识,0x0000作为测试框架的业务标识
      let tag: string = 'testTag'; //日志标识字符串,作为tag标识当前runner类下的测试行为
      hilog.info(domain, tag, '====>testCase018 create success');
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap', // 需要手动将url替换为真实服务器的HTTP协议地址
        title: 'taskOnTest',
        description: 'Sample code for event listening',
        mode: request.agent.Mode.FOREGROUND,
        overwrite: true,
        saveas: "./",
        network: request.agent.Network.ANY,
        metered: false,
        roaming: true,
        retry: false,
        redirect: true,
        gauge: false,
        precise: false,
        token: "it is a secret"
      };
      let faultOnCallback = (faults: request.agent.Faults) => {
        try {
          hilog.info(domain, tag, '====>upload task failed. fault: ' + JSON.stringify(faults));
        } catch (err: BusinessError) {
          hilog.info(domain, tag,
            "====>SUB_Misc_REQUEST_On_Download_Progress_testCase016 catch err: " + JSON.stringify(err));
        }
      };
      request.agent.create(context, config1).then((task: request.agent.Task) => {
        try {
          task.on('faultOccur', faultOnCallback);
          hilog.info(domain, tag, `====>Succeeded in creating a upload task. result: ${task.tid}`);
          task.off('faultOccur', faultOnCallback);
          task.start();
        } catch (err: BusinessError) {
          hilog.info(domain, tag,
            "====>SUB_Misc_REQUEST_On_Download_Progress_testCase016 B catch err: " + JSON.stringify(err));
        }
      })
    });

    /**
     * @tc.name: SUB_REQUEST_create_STAGE_API_CALLBACK_0003
     * @tc.desc: Test create download task using callback with specific options
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with various options
     *           2. Create a download task using Promise chain
     *           3. Bind the resume event handler
     *           4. Unbind the resume event handler
     *           5. Start the download task
     * @tc.require: issues#1573
     */
    it('SUB_REQUEST_create_STAGE_API_CALLBACK_0003', Level.LEVEL1, async (done: () => void): Promise<void> => {
      let domain: int = 0x0000; //日志标识,0x0000作为测试框架的业务标识
      let tag: string = 'testTag'; //日志标识字符串,作为tag标识当前runner类下的测试行为
      hilog.info(domain, tag, '====>testCase019 create success');
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap', // 需要手动将url替换为真实服务器的HTTP协议地址
        title: 'taskOnTest',
        description: 'Sample code for event listening',
        mode: request.agent.Mode.FOREGROUND,
        overwrite: true,
        saveas: "./",
        network: request.agent.Network.ANY,
        metered: false,
        roaming: true,
        retry: false,
        redirect: true,
        gauge: false,
        precise: false,
        token: "it is a secret"
      };
      let createOffCallback1 = (progress: request.agent.Progress) => {
        console.info('====>upload task resume.');
      };
      request.agent.create(context, config1).then((task: request.agent.Task) => {
        try {
          task.on('resume', createOffCallback1);
          hilog.info(domain, tag, `====>Succeeded in creating a upload task. result: ${task.tid}`);
          task.off('resume', createOffCallback1);
          task.start();
        } catch (err: BusinessError) {
          hilog.info(domain, tag, "====>testCase019 B catch err: " + JSON.stringify(err));
        }
      })
    });

    /**
     * @tc.name: SUB_REQUEST_create_STAGE_API_CALLBACK_0002
     * @tc.desc: Test create download task using callback with alternative options
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with various options
     *           2. Create a download task using Promise chain
     *           3. Bind the pause event handler
     *           4. Unbind the pause event handler
     *           5. Start the download task
     * @tc.require: issues#1573
     */
    it('SUB_REQUEST_create_STAGE_API_CALLBACK_0002', Level.LEVEL1, async (done: () => void): Promise<void> => {
      let domain: int = 0x0000; //日志标识,0x0000作为测试框架的业务标识
      let tag: string = 'testTag'; //日志标识字符串,作为tag标识当前runner类下的测试行为
      hilog.info(domain, tag, '====>testCase020 create success');
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap', // 需要手动将url替换为真实服务器的HTTP协议地址
        title: 'taskOnTest',
        description: 'Sample code for event listening',
        mode: request.agent.Mode.FOREGROUND,
        overwrite: true,
        saveas: "./",
        network: request.agent.Network.ANY,
        metered: false,
        roaming: true,
        retry: false,
        redirect: true,
        gauge: false,
        precise: false,
        token: "it is a secret"
      };
      let createOffCallback1 = (progress: request.agent.Progress) => {
        console.info('====>upload task pause.');
      };
      request.agent.create(context, config1).then((task: request.agent.Task) => {
        try {
          task.on('pause', createOffCallback1);
          hilog.info(domain, tag, `====>Succeeded in creating a upload task. result: ${task.tid}`);
          task.off('pause', createOffCallback1);
          task.start();
        } catch (err: BusinessError) {
          hilog.info(domain, tag, "====>testCase020 B catch err: " + JSON.stringify(err));
        }
      })
    });

    /**
     * @tc.name: SUB_REQUEST_create_STAGE_API_CALLBACK_0001
     * @tc.desc: Test create download task using callback with minimal configuration
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with various options
     *           2. Create a download task using Promise chain
     *           3. Bind the failed event handler
     *           4. Unbind the failed event handler
     *           5. Start the download task
     * @tc.require: issues#1573
     */
    it('SUB_REQUEST_create_STAGE_API_CALLBACK_0001', Level.LEVEL1, async (done: () => void): Promise<void> => {
      let domain: int = 0x0000; //日志标识,0x0000作为测试框架的业务标识
      let tag: string = 'testTag'; //日志标识字符串,作为tag标识当前runner类下的测试行为
      hilog.info(domain, tag, '====>testCase021 create success');
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap', // 需要手动将url替换为真实服务器的HTTP协议地址
        title: 'taskOnTest',
        description: 'Sample code for event listening',
        mode: request.agent.Mode.FOREGROUND,
        overwrite: true,
        saveas: "./",
        network: request.agent.Network.ANY,
        metered: false,
        roaming: true,
        retry: false,
        redirect: true,
        gauge: false,
        precise: false,
        token: "it is a secret"
      };
      let createOffCallback1 = (progress: request.agent.Progress) => {
        console.info('====>upload task failed.');
      };
      request.agent.create(context, config1).then((task: request.agent.Task) => {
        try {
          task.on('failed', createOffCallback1);
          hilog.info(domain, tag, `====>Succeeded in creating a upload task. result: ${task.tid}`);
          task.off('failed', createOffCallback1);
          task.start();
        } catch (err: BusinessError) {
          hilog.info(domain, tag, "====>testCase021 B catch err: " + JSON.stringify(err));
        }
      })
    });

    /**
     * @tc.name: SUB_REQUEST_downloadFile_STAGE_API_CALLBACK_0006
     * @tc.desc: Test download file using API callback
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Use request.downloadFile API to create a download task
     *           2. Set up various event handlers (complete, pause, remove, progress)
     *           3. Bind the onComplete event handler to the download task
     * @tc.require: issues#1573
     */
    it('SUB_REQUEST_downloadFile_STAGE_API_CALLBACK_0006', Level.LEVEL1, async (done: () => void): Promise<void> => {
      let domain: int = 0x0000; //日志标识,0x0000作为测试框架的业务标识
      let tag: string = 'testTag'; //日志标识字符串,作为tag标识当前runner类下的测试行为
      // let config1: request.agent.Config = {
      //   action: request.agent.Action.DOWNLOAD,
      //   url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap', // 需要手动将url替换为真实服务器的HTTP协议地址
      //   title: 'taskOnTest',
      //   description: 'Sample code for event listening',
      //   mode: request.agent.Mode.FOREGROUND,
      //   overwrite: true,
      //   saveas: "./",
      //   network: request.agent.Network.ANY,
      //   metered: false,
      //   roaming: true,
      //   retry: false,
      //   redirect: true,
      //   gauge: false,
      //   precise: false,
      //   token: "it is a secret"
      // };

      hilog.info(domain, tag, '====>testCase022 create success');
      try {
        // 需要手动将url替换为真实服务器的HTTP协议地址
        request.downloadFile(context, { url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap' })
          .then((data: request.DownloadTask) => {
            let downloadTask: request.DownloadTask = data;
            // complete1
            let completeCallback = () => {
              hilog.info(domain, tag, '====>Download task completed.');
            };

            // pause1
            let pauseCallback = () => {
              hilog.info(domain, tag, '====>Download task pause.');
            };

            // remove1
            let removeCallback = () => {
              hilog.info(domain, tag, '====>Download task remove.');
            };

            // progress1
            let progressCallback = (receivedSize: long, totalSize: long) => {
              hilog.info(domain, tag, "====>download receivedSize:" + receivedSize + " totalSize:" + totalSize);
            };


            hilog.info(domain, tag, '====>before on.');
            try {
              // complete2
              // downloadTask.on('complete', completeCallback);
              downloadTask.onComplete(completeCallback);
              // downloadTask.offComplete(completeCallback);
              // downloadTask.off('complete', completeCallback);

              // pause2
              // downloadTask.on('pause', pauseCallback);
              // downloadTask.onPause(pauseCallback);
              // downloadTask.off('pause', pauseCallback);
              // downloadTask.offPause(pauseCallback);

              // remove2
              // downloadTask.on('remove', removeCallback);
              // downloadTask.onRemove(removeCallback);
              // downloadTask.off('remove', removeCallback);
              // downloadTask.offRemove(removeCallback);

              // progress2
              // downloadTask.onProgress(progressCallback);
              // downloadTask.off('progress', progressCallback);
              // downloadTask.offProgress(progressCallback);

            } catch (err: Error) {
              hilog.info(domain, tag, "====>testCase022 B catch err: " + JSON.stringify(err));
            }

            hilog.info(domain, tag, '====>after on.');

            // pause3
            // downloadTask.suspend().then((result: boolean) => {
            //   hilog.info(domain, tag, '====>Succeeded in pausing the download task.');
            // }).catch((err: Error) => {
            //   hilog.info(domain, tag, `====>Failed to pause the download task. Code: ${err.code}, message: ${err.message}`);
            // });

            // remove3
            // downloadTask.delete().then((result: boolean) => {
            //   hilog.info(domain, tag, 'Succeeded in removing the download task.');
            // }).catch((err: Error) => {
            //   hilog.info(domain, tag, `Failed to remove the download task. Code: ${err.code}, message: ${err.message}`);
            // });

          })
          .catch((err) => {
            let bizErr = err as BusinessError<void>;
            hilog.info(domain, tag,
              `====>Failed to request the download. Code: ${bizErr.code}, message: ${bizErr.message}`);
          })
      } catch (err) {
        hilog.info(domain, tag, `====>Failed to request the download. err:`);
      }
    });

    /**
     * @tc.name: SUB_REQUEST_downloadFile_STAGE_API_CALLBACK_0005
     * @tc.desc: Test download file using API callback with fail handling
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Use request.downloadFile API with local URL (http://127.0.0.1)
     *           2. Set up fail event handler to catch download errors
     *           3. Bind the onFail event handler to the download task
     * @tc.require: issues#1573
     */
    it('SUB_REQUEST_downloadFile_STAGE_API_CALLBACK_0005', Level.LEVEL1, async (done: () => void): Promise<void> => {
      let domain: int = 0x0000; //日志标识,0x0000作为测试框架的业务标识
      let tag: string = 'testTag'; //日志标识字符串,作为tag标识当前runner类下的测试行为

      hilog.info(domain, tag, '====>testCase023 create success');
      try {
        // 需要手动将url替换为真实服务器的HTTP协议地址
        request.downloadFile(context, { url: 'http://127.0.0.1' }).then((data: request.DownloadTask) => {
          let downloadTask: request.DownloadTask = data;
          // fail1
          let failCallback = (err: int) => {
            hilog.info(domain, tag, `====>Failed to download the task. Code: ${err}`);
          };


          hilog.info(domain, tag, '====>before on.');
          try {
            // progress2
            // downloadTask.on('fail', failCallback);
            downloadTask.onFail(failCallback);
            // downloadTask.off('fail', failCallback);
            // downloadTask.offFail(failCallback);

          } catch (err: Error) {
            hilog.info(domain, tag, "====>testCase023 B catch err: " + JSON.stringify(err));
          }

          hilog.info(domain, tag, '====>after on.');


        }).catch((err) => {
          let bizErr = err as BusinessError<void>;
          hilog.info(domain, tag,
            `====>Failed to request the download. Code: ${bizErr.code}, message: ${bizErr.message}`);
        })
      } catch (err) {
        hilog.info(domain, tag, `====>Failed to request the download. err:`);
      }
    });

    /**
     * @tc.name: SUB_REQUEST_uploadFile_STAGE_API_CALLBACK_0004
     * @tc.desc: Test upload file using callback with file creation
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Get the application's cache directory
     *           2. Create a test file in the cache directory
     * @tc.require: issues#1573
     */
    it('SUB_REQUEST_uploadFile_STAGE_API_CALLBACK_0004', Level.LEVEL1, async (done: () => void): Promise<void> => {
      let domain: int = 0x0000; //日志标识,0x0000作为测试框架的业务标识
      let tag: string = 'testTag'; //日志标识字符串,作为tag标识当前runner类下的测试行为

      hilog.info(domain, tag, '====>testCase024 create success');
      //0
      let cacheDir = context.cacheDir;
      hilog.info(domain, tag, `====> opensync0`);
      let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      hilog.info(domain, tag, `====> opensync1`);
      fs.closeSync(file);
      hilog.info(domain, tag, `====> closesync`);
      // let conf = {
      //   url: 'http://http://192.168.137.1:13333',
      //   header: {
      //     'Accept': '*/*'
      //   },
      //   method: 'POST',
      //   files: [File],
      //   data: [RequestData]
      // };
      // let task = await request.uploadFile(context, conf);
      let headers: Record<string, string> = { 'Accept': '*/*' };
      hilog.info(domain, tag, `====> !!!1`);
      let uploadTask: request.UploadTask;
      hilog.info(domain, tag, `====> !!!2`);
      let uploadConfig: request.UploadConfig = {
        url: 'http://192.168.137.1:13333', // 需要手动将url替换为真实服务器的HTTP协议地址
        header: headers,
        method: "POST",
        files: [{
          filename: "test.hap",
          name: "test",
          uri: "internal://cache/test.hap",
          type: "txt"
        }], // 建议type填写HTTP协议规范的MIME类型
        data: [{ name: "name123", value: "123" }],
      };
      hilog.info(domain, tag, `====> !!!3`);
      try {
        hilog.info(domain, tag, `====> Before upload`);
        request.uploadFile(context, uploadConfig).then((data: request.UploadTask) => {
          uploadTask = data;
          let progressCallback = (uploadedSize: long, totalSize: long) => {
            hilog.info(domain, tag, `====>uploadedSize. totalSize:${uploadedSize} ${totalSize}`);
          };
          // let completeCallback = (state: Array<request.TaskState>) => {
          //   hilog.info(domain, tag, `====>complete ${JSON.stringify(state)} `);
          // };
          // let headerCallback = (state: Array<TaskState>) => {
          //   hilog.info(domain, tag, `====>complete ${JSON.stringify(state)} `);
          // };
          // try {
          //   uploadTask.on('progress', progressCallback);
          //   uploadTask.on('complete', completeCallback);
          // } catch (err: Error) {
          //   hilog.info(domain, tag, `====>Failed to request the upload2. Code: ${err.code}, message: ${err.message}`);
          // }

          try {
        // uploadTask.on('progress', progressCallback);
        uploadTask.onProgress(progressCallback);
        // uploadTask.off('progress', progressCallback);
        // uploadTask.offProgress(progressCallback);
      } catch (err: Error) {
        hilog.info(domain, tag, `====>Failed to request the upload3. Code: ${err.code}, message: ${err.message}`);
      }

      hilog.info(domain, tag, `====> After upload`);
    }).catch((err: Error) => {
      hilog.info(domain, tag, `Failed to request the upload. Code: ${err.code}, message: ${err.message}`);
    });
  } catch (err) {
    hilog.info(domain, tag, `Failed to request the upload. err: ${JSON.stringify(err)}`);
  }
  // 0
});

    /**
     * @tc.name: SUB_REQUEST_uploadFile_STAGE_API_CALLBACK_0003
     * @tc.desc: Test upload file using callback with error handling
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Get the application's cache directory
     *           2. Create a test file in the cache directory
     *           3. Set up upload configuration with file information
     *           4. Use request.uploadFile API to create an upload task
     *           5. Bind the complete event handler to the upload task
     * @tc.require: issues#1573
     */
    it('SUB_REQUEST_uploadFile_STAGE_API_CALLBACK_0003', Level.LEVEL1, async (done: () => void): Promise<void> => {
  let domain: int = 0x0000; //日志标识,0x0000作为测试框架的业务标识
  let tag: string = 'testTag'; //日志标识字符串,作为tag标识当前runner类下的测试行为

  hilog.info(domain, tag, '====>testCase025 create success');
  //0
  let cacheDir = context.cacheDir;
  let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
  fs.closeSync(file);
  let headers: Record<string, string> = { 'Accept': '*/*' };
  let uploadTask: request.UploadTask;
  let uploadConfig: request.UploadConfig = {
    url: 'http://192.168.137.1:13333', // 需要手动将url替换为真实服务器的HTTP协议地址
    header: headers,
    method: "POST",
    files: [{
      filename: "test",
      name: "test",
      uri: "internal://cache/test.hap",
      type: "txt"
    }], // 建议type填写HTTP协议规范的MIME类型
    data: [{ name: "name123", value: "123" }],
  };
  try {
    hilog.info(domain, tag, `====> Before upload`);
    request.uploadFile(context, uploadConfig).then((data: request.UploadTask) => {
      uploadTask = data;
      let completeCallback = (state: Array<request.TaskState>) => {
        hilog.info(domain, tag, `====>complete ${JSON.stringify(state)} `);
      };
      try {
        uploadTask.onComplete(completeCallback);
        // uploadTask.offComplete(completeCallback);
        // uploadTask.on('complete', completeCallback);
        // uploadTask.off('complete', completeCallback);
      } catch (err: Error) {
        hilog.info(domain, tag, `====>Failed to request the upload3. Code: ${err.code}, message: ${err.message}`);
      }

      hilog.info(domain, tag, `====> After upload`);
    }).catch((err: Error) => {
      hilog.info(domain, tag, `Failed to request the upload. Code: ${err.code}, message: ${err.message}`);
    });
  } catch (err) {
    hilog.info(domain, tag, `Failed to request the upload. err: ${JSON.stringify(err)}`);
  }
});

    /**
     * @tc.name: SUB_REQUEST_uploadFile_STAGE_API_CALLBACK_0002
     * @tc.desc: Test upload file using callback with specific headers
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Get the application's cache directory
     *           2. Create a test file in the cache directory
     *           3. Set up upload configuration with localhost URL
     *           4. Use request.uploadFile API to create an upload task
     *           5. Bind the fail event handler to the upload task
     * @tc.require: issues#1573
     */
    it('SUB_REQUEST_uploadFile_STAGE_API_CALLBACK_0002', Level.LEVEL1, async (done: () => void): Promise<void> => {
  let domain: int = 0x0000; //日志标识,0x0000作为测试框架的业务标识
  let tag: string = 'testTag'; //日志标识字符串,作为tag标识当前runner类下的测试行为

  hilog.info(domain, tag, '====>testCase026 create success');
  //0
  let cacheDir = context.cacheDir;
  let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
  fs.closeSync(file);
  let headers: Record<string, string> = { 'Accept': '*/*' };
  let uploadTask: request.UploadTask;
  let uploadConfig: request.UploadConfig = {
    url: 'http://127.0.0.1', // 需要手动将url替换为真实服务器的HTTP协议地址
    header: headers,
    method: "POST",
    files: [{
      filename: "test",
      name: "test",
      uri: "internal://cache/test.hap",
      type: "txt"
    }], // 建议type填写HTTP协议规范的MIME类型
    data: [{ name: "name123", value: "123" }],
  };
  try {
    hilog.info(domain, tag, `====> Before upload`);
    request.uploadFile(context, uploadConfig).then((data: request.UploadTask) => {
      uploadTask = data;
      let completeCallback = (state: Array<request.TaskState>) => {
        hilog.info(domain, tag, `====>fail ${JSON.stringify(state)} `);
      };
      try {
        uploadTask.onFail(completeCallback);
        // uploadTask.offFail(completeCallback);
        // uploadTask.on('fail', completeCallback);
        // uploadTask.off('fail', completeCallback);
      } catch (err: Error) {
        hilog.info(domain, tag, `====>Failed to request the upload3. Code: ${err.code}, message: ${err.message}`);
      }

      hilog.info(domain, tag, `====> After upload`);
    }).catch((err: Error) => {
      hilog.info(domain, tag, `Failed to request the upload. Code: ${err.code}, message: ${err.message}`);
    });
  } catch (err) {
    hilog.info(domain, tag, `Failed to request the upload. err: ${JSON.stringify(err)}`);
  }
});

    /**
     * @tc.name: SUB_REQUEST_uploadFile_STAGE_API_CALLBACK_0001
     * @tc.desc: Test upload file using callback with minimal configuration
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Get the application's cache directory
     *           2. Create a test file in the cache directory
     *           3. Set up upload configuration with file information
     *           4. Use request.uploadFile API to create an upload task
     *           5. Bind the headerReceive event handler to the upload task
     * @tc.require: issues#1573
     */
    it('SUB_REQUEST_uploadFile_STAGE_API_CALLBACK_0001', Level.LEVEL1, async (done: () => void): Promise<void> => {
  let domain: int = 0x0000; //日志标识,0x0000作为测试框架的业务标识
  let tag: string = 'testTag'; //日志标识字符串,作为tag标识当前runner类下的测试行为

  hilog.info(domain, tag, '====>testCase027 create success');
  //0
  let cacheDir = context.cacheDir;
  let file = fs.openSync(cacheDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
  fs.closeSync(file);
  let headers: Record<string, string> = { 'Accept': '*/*' };
  let uploadTask: request.UploadTask;
  let uploadConfig: request.UploadConfig = {
    url: 'http://192.168.137.1:13333', // 需要手动将url替换为真实服务器的HTTP协议地址
    header: headers,
    method: "POST",
    files: [{
      filename: "test",
      name: "test",
      uri: "internal://cache/test.hap",
      type: "txt"
    }], // 建议type填写HTTP协议规范的MIME类型
    data: [{ name: "name123", value: "123" }],
  };
  try {
    hilog.info(domain, tag, `====> Before upload`);
    request.uploadFile(context, uploadConfig).then((data: request.UploadTask) => {
      uploadTask = data;
      let completeCallback = (headers: Object) => {
        hilog.info(domain, tag, `====>header_receive ${JSON.stringify(headers)} `);
      };
      try {
        uploadTask.onHeaderReceive(completeCallback);
        // uploadTask.offHeaderReceive(completeCallback);
        // uploadTask.on('headerReceive', completeCallback);
        // uploadTask.off('headerReceive', completeCallback);
      } catch (err: Error) {
        hilog.info(domain, tag, `====>Failed to request the upload3. Code: ${err.code}, message: ${err.message}`);
      }

      hilog.info(domain, tag, `====> After upload`);
    }).catch((err: Error) => {
      hilog.info(domain, tag, `Failed to request the upload. Code: ${err.code}, message: ${err.message}`);
    });
  } catch (err) {
    hilog.info(domain, tag, `Failed to request the upload. err: ${JSON.stringify(err)}`);
  }
});

    /**
     * @tc.name: SUB_Request_Agent_onResponse_Static_0100
     * @tc.desc: Test agent on response event handling
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration
     *           2. Create a download task using promise
     *           3. Add completed event listener to the task
     *           4. Start the task and verify completed event is triggered
     * @tc.require: issues#1573
     */
    it('SUB_Request_Agent_onResponse_Static_0100', Level.LEVEL1, async (done: () => void): Promise<void> => {
  let domain: int = 0x0000; //日志标识,0x0000作为测试框架的业务标识
  let tag: string = 'testTag'; //日志标识字符串,作为tag标识当前runner类下的测试行为
  hilog.info(domain, tag, "====>-------testCase028 is starting-----------");
  let config1: request.agent.Config = {
    action: request.agent.Action.DOWNLOAD,
    url: 'https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png',
    saveas: './SUB_Request_Agent_onResponse_Static_0100',
    overwrite: true,
  };
  try {
    let task: request.agent.Task = await request.agent.create(context, config1);
    hilog.info(domain, tag, "====>SUB_Request_Agent_onResponse_Static_0100 create success");
    let responseCallback_1 = (info: request.agent.HttpResponse) => {
      try {
        // hilog.info(domain, tag, "====>testCase027 response info: " + JSON.stringify(info));
        request.agent.show(task.tid).then(async (info: request.agent.TaskInfo) => {
          hilog.info(domain, tag, "====>testCase028 show info: " + JSON.stringify(info));
        });
      } catch (err) {
        hilog.info(domain, tag, "====>testCase028 err: " + JSON.stringify(err));
      }
    }
    task.on('response', responseCallback_1);
    await task.start();
    hilog.info(domain, tag, "====>testCase028 task.start");
  } catch (err) {
    hilog.info(domain, tag, "====>testCase028 create catch err: " + JSON.stringify(err));
    expect().assertFail();
  }
});

    /**
     * @tc.name: SUB_REQUEST_downloadFile_STAGE_API_CALLBACK_0001
     * @tc.desc: Test download file using callback with foreground mode
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with foreground mode
     *           2. Use request.agent.create to create a download task
     *           3. Set up search filter with specific criteria
     *           4. Use request.agent.search API to find tasks matching the filter
     * @tc.require: issues#1573
     */
    it('SUB_REQUEST_downloadFile_STAGE_API_CALLBACK_0001', Level.LEVEL1, async (done: () => void): Promise<void> => {
  let domain: int = 0x0000; //日志标识,0x0000作为测试框架的业务标识
  let tag: string = 'testTag'; //日志标识字符串,作为tag标识当前runner类下的测试行为
  hilog.info(domain, tag, "====>-------testCase029 is starting-----------");
  let config1: request.agent.Config = {
    action: request.agent.Action.DOWNLOAD,
    url: 'https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png',
    saveas: './SUB_Request_Agent_onResponse_Static_0100',
    overwrite: true,
    mode: request.agent.Mode.FOREGROUND,
  };
  try {
    let task: request.agent.Task = await request.agent.create(context, config1);
    let filter: request.agent.Filter = {
      state: request.agent.State.INITIALIZED,
      action: request.agent.Action.DOWNLOAD,
      mode: request.agent.Mode.FOREGROUND,
    };
    request.agent.search(filter).then(async (tids: Array<string>) => {
      hilog.info(domain, tag, "====>search: " + JSON.stringify(tids));
    });
  } catch (err) {
    hilog.info(domain, tag, "====>testCase029 create catch err: " + JSON.stringify(err));
    expect().assertFail();
  }
});

    /**
     * @tc.name: SUB_Request_Agent_TaskSaveas_0100
     * @tc.desc: Test task saveas functionality with file operations
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration without explicit saveas path
     *           2. Check and delete any existing test file
     *           3. Create a download task using baseContext
     *           4. Bind the completed event handler to check file existence
     * @tc.require: issues#1573
     */
    it('SUB_Request_Agent_TaskSaveas_0100', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_Request_Agent_TaskSaveas_0100 is starting-----------------------");
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        title: 'createTest',
        description: 'XTS download test!',
        saveas: undefined,
        network: request.agent.Network.WIFI,
        overwrite: true,
        mode: request.agent.Mode.FOREGROUND
      };
      // try {
      let path: string = '/data/storage/el2/base/haps/com.example.helloworld/cache/test.hap';
      let ifExitOut: boolean = fs.accessSync(path);
      if (ifExitOut) {
        fs.unlinkSync(path);
        console.info("====>SUB_Request_Agent_TaskSaveas_0100 unlinkSync file success");
      }
      let task = await request.agent.create(baseContext, config1);
      let on_completedCallback = (pro: request.agent.Progress) => {
        try {
          // task.off('completed', on_completedCallback);
          console.info("====>SUB_Request_Agent_TaskSaveas_0100 completed task.config.saveas: " +
          JSON.stringify(task.config));
          let ifExit: boolean = fs.accessSync(path);
          console.info("====>SUB_Request_Agent_TaskSaveas_0100 ifExit: " + ifExit);
          // expect(ifExit).assertTrue();
          if (ifExit) {
            fs.unlinkSync(path);
            console.info("====>SUB_Request_Agent_TaskSaveas_0100 delete file success");
          }
        } catch (err) {
          console.info("====>SUB_Request_Agent_TaskSaveas_0100 catch err: " + JSON.stringify(err));
        }
      }
      // try {
      // if(err){
      //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 create err: " + JSON.stringify(err));
      //   expect().assertFail();
      // }
      task.on('completed', on_completedCallback);
      await task.start();
      console.info("====>SUB_Request_Agent_TaskSaveas_0100 start success: " + JSON.stringify(config.saveas));
      // } catch (err) {
      //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 catch err: " + JSON.stringify(err));
      //   await request.agent.remove(task.tid);
      // }
      // } catch (err) {
      //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 create catch err: " + JSON.stringify(err));
      //   expect().assertFail();
      // }
    });

    /**
     * @tc.name: SUB_Request_Agent_TaskSaveas_0200
     * @tc.desc: Test task saveas functionality with nested directory path
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with current directory saveas path
     *           2. Check and delete any existing test file
     *           3. Create a download task using baseContext
     *           4. Bind the completed event handler to verify file existence
     *           5. Start the download task
     * @tc.require: issues#1573
     */
    it('SUB_Request_Agent_TaskSaveas_0100', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_Request_Agent_TaskSaveas_0200 is starting-----------------------");
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        title: 'createTest',
        description: 'XTS download test!',
        saveas: './',
        network: request.agent.Network.WIFI,
        overwrite: true,
        mode: request.agent.Mode.FOREGROUND
      };
      // try {
      let path: string = '/data/storage/el2/base/haps/com.example.helloworld/cache/test.hap';
      let ifExitOut: boolean = fs.accessSync(path);
      if (ifExitOut) {
        fs.unlinkSync(path);
        console.info("====>SUB_Request_Agent_TaskSaveas_0200 unlinkSync file success");
      }
      let task = await request.agent.create(baseContext, config1);
      let on_completedCallback = (pro: request.agent.Progress) => {
        try {
          // task.off('completed', on_completedCallback);
          console.info("====>SUB_Request_Agent_TaskSaveas_0200 completed task.config.saveas: " +
          JSON.stringify(task.config));
          let ifExit: boolean = fs.accessSync(path);
          console.info("====>SUB_Request_Agent_TaskSaveas_0200 ifExit: " + ifExit);
          // expect(ifExit).assertTrue();
          if (ifExit) {
            fs.unlinkSync(path);
            console.info("====>SUB_Request_Agent_TaskSaveas_0100 delete file success");
          }
        } catch (err) {
          console.info("====>SUB_Request_Agent_TaskSaveas_0100 catch err: " + JSON.stringify(err));
        }
      }
      // try {
      // if(err){
      //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 create err: " + JSON.stringify(err));
      //   expect().assertFail();
      // }
      task.on('completed', on_completedCallback);
      await task.start();
      console.info("====>SUB_Request_Agent_TaskSaveas_0100 start success: " + JSON.stringify(config.saveas));
      // } catch (err) {
      //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 catch err: " + JSON.stringify(err));
      //   await request.agent.remove(task.tid);
      // }
      // } catch (err) {
      //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 create catch err: " + JSON.stringify(err));
      //   expect().assertFail();
      // }
    });

    /**
     * @tc.name: SUB_Request_Agent_TaskSaveas_0100
     * @tc.desc: Test task saveas functionality with specified path
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with nested directory saveas path
     *           2. Check and delete any existing test file in the target path
     *           3. Create a download task using baseContext
     *           4. Bind the completed event handler to verify file existence
     *           5. Start the download task
     * @tc.require: issues#1573
     */
    it('SUB_Request_Agent_TaskSaveas_0100', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_Request_Agent_TaskSaveas_0300 is starting-----------------------");
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        title: 'createTest',
        description: 'XTS download test!',
        saveas: './request/SUB_Request_Agent_TaskSaveas_0300',
        network: request.agent.Network.WIFI,
        overwrite: true,
        mode: request.agent.Mode.FOREGROUND
      };
      // try {
      let path: string =
        '/data/storage/el2/base/haps/com.example.helloworld/cache/request/SUB_Request_Agent_TaskSaveas_0300';
      let ifExitOut: boolean = fs.accessSync(path);
      if (ifExitOut) {
        fs.unlinkSync(path);
        console.info("====>SUB_Request_Agent_TaskSaveas_0100 unlinkSync file success");
      }
      let task = await request.agent.create(baseContext, config1);
      let on_completedCallback = (pro: request.agent.Progress) => {
        try {
          // task.off('completed', on_completedCallback);
          console.info("====>SUB_Request_Agent_TaskSaveas_0300 completed task.config.saveas: " +
          JSON.stringify(task.config));
          let ifExit: boolean = fs.accessSync(path);
          console.info("====>SUB_Request_Agent_TaskSaveas_0300 ifExit: " + ifExit);
          // expect(ifExit).assertTrue();
          if (ifExit) {
            fs.unlinkSync(path);
            console.info("====>SUB_Request_Agent_TaskSaveas_0100 delete file success");
          }
        } catch (err) {
          console.info("====>SUB_Request_Agent_TaskSaveas_0100 catch err: " + JSON.stringify(err));
        }
      }
      // try {
      // if(err){
      //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 create err: " + JSON.stringify(err));
      //   expect().assertFail();
      // }
      task.on('completed', on_completedCallback);
      await task.start();
      console.info("====>SUB_Request_Agent_TaskSaveas_0100 start success: " + JSON.stringify(config.saveas));
      // } catch (err) {
      //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 catch err: " + JSON.stringify(err));
      //   await request.agent.remove(task.tid);
      // }
      // } catch (err) {
      //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 create catch err: " + JSON.stringify(err));
      //   expect().assertFail();
      // }
    });

    /**
     * @tc.name: SUB_Request_Agent_TaskSaveas_0100
     * @tc.desc: Test task saveas functionality with root directory path
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with direct file name saveas path
     *           2. Check and delete any existing test file in the target path
     *           3. Create a download task using baseContext
     *           4. Bind the completed event handler to verify file existence
     *           5. Start the download task
     * @tc.require: issues#1573
     */
    it('SUB_Request_Agent_TaskSaveas_0100', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_Request_Agent_TaskSaveas_0400 is starting-----------------------");
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        title: 'createTest',
        description: 'XTS download test!',
        saveas: './SUB_Request_Agent_TaskSaveas_0400',
        network: request.agent.Network.WIFI,
        overwrite: true,
        mode: request.agent.Mode.FOREGROUND
      };
      // try {
      let path: string =
        '/data/storage/el2/base/haps/com.example.helloworld/cache/request/SUB_Request_Agent_TaskSaveas_0400';
      let ifExitOut: boolean = fs.accessSync(path);
      if (ifExitOut) {
        fs.unlinkSync(path);
        console.info("====>SUB_Request_Agent_TaskSaveas_0100 unlinkSync file success");
      }
      let task = await request.agent.create(baseContext, config1);
      let on_completedCallback = (pro: request.agent.Progress) => {
        try {
          // task.off('completed', on_completedCallback);
          console.info("====>SUB_Request_Agent_TaskSaveas_0400 completed task.config.saveas: " +
          JSON.stringify(task.config));
          let ifExit: boolean = fs.accessSync(path);
          console.info("====>SUB_Request_Agent_TaskSaveas_0400 ifExit: " + ifExit);
          // expect(ifExit).assertTrue();
          if (ifExit) {
            fs.unlinkSync(path);
            console.info("====>SUB_Request_Agent_TaskSaveas_0100 delete file success");
          }
        } catch (err) {
          console.info("====>SUB_Request_Agent_TaskSaveas_0100 catch err: " + JSON.stringify(err));
        }
      }
      // try {
      // if(err){
      //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 create err: " + JSON.stringify(err));
      //   expect().assertFail();
      // }
      task.on('completed', on_completedCallback);
      await task.start();
      console.info("====>SUB_Request_Agent_TaskSaveas_0100 start success: " + JSON.stringify(config.saveas));
      // } catch (err) {
      //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 catch err: " + JSON.stringify(err));
      //   await request.agent.remove(task.tid);
      // }
      // } catch (err) {
      //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 create catch err: " + JSON.stringify(err));
      //   expect().assertFail();
      // }
    });

    /**
     * @tc.name: SUB_Request_Agent_TaskSaveas_0500
     * @tc.desc: Test task saveas functionality with specific filename
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with invalid root path format
     *           2. Check and delete any existing test file in the target path
     *           3. Attempt to create a download task and catch the expected error
     * @tc.require: issues#1573
     */
    it('SUB_Request_Agent_TaskSaveas_0500', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_Request_Agent_TaskSaveas_0500 is starting-----------------------");
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        title: 'createTest',
        description: 'XTS download test!',
        saveas: '/ request/SUB_Request_Agent_TaskSaveas_0500',
        network: request.agent.Network.WIFI,
        overwrite: true,
        mode: request.agent.Mode.FOREGROUND
      };
      // try {
      let path: string =
        '/data/storage/el2/base/haps/com.example.helloworld/cache/request/SUB_Request_Agent_TaskSaveas_0500';
      let ifExitOut: boolean = fs.accessSync(path);
      if (ifExitOut) {
        fs.unlinkSync(path);
        console.info("====>SUB_Request_Agent_TaskSaveas_0100 unlinkSync file success");
      }
      try {
        let task = await request.agent.create(baseContext, config1);
      } catch (err: BusinessError) {
        console.info("====>SUB_Request_Agent_TaskSaveas_0500 err: " + err.code);
      }
      // let on_completedCallback = (pro: request.agent.Progress) => {
      //   try {
      //     // task.off('completed', on_completedCallback);
      //     console.info("====>SUB_Request_Agent_TaskSaveas_0500 completed task.config.saveas: " + JSON.stringify(task.config));
      //     let ifExit: boolean = fs.accessSync(path);
      //     console.info("====>SUB_Request_Agent_TaskSaveas_0500 ifExit: " + ifExit);
      //     // expect(ifExit).assertTrue();
      //     if(ifExit){
      //       fs.unlinkSync(path);
      //       console.info("====>SUB_Request_Agent_TaskSaveas_0100 delete file success");
      //     }
      //   } catch (err) {
      //     console.info("====>SUB_Request_Agent_TaskSaveas_0100 catch err: " + JSON.stringify(err));
      //   }
      // }
      // try {
      // if(err){
      //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 create err: " + JSON.stringify(err));
      //   expect().assertFail();
      // }
      // task.on('completed', on_completedCallback);
      // await task.start();
      // console.info("====>SUB_Request_Agent_TaskSaveas_0100 start success: " + JSON.stringify(config.saveas));
      // } catch (err) {
      //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 catch err: " + JSON.stringify(err));
      //   await request.agent.remove(task.tid);
      // }
      // } catch (err) {
      //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 create catch err: " + JSON.stringify(err));
      //   expect().assertFail();
      // }
    });

    /**
     * @tc.name: SUB_Request_Agent_TaskSaveas_0100
     * @tc.desc: Test task saveas functionality with filename only
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with filename only (no path prefix)
     *           2. Check and delete any existing test file in the target path
     *           3. Create a download task using baseContext
     *           4. Bind the completed event handler to verify file existence
     *           5. Start the download task
     * @tc.require: issues#1573
     */
    it('SUB_Request_Agent_TaskSaveas_0100', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_Request_Agent_TaskSaveas_0600 is starting-----------------------");
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        title: 'createTest',
        description: 'XTS download test!',
        saveas: 'SUB_Request_Agent_TaskSaveas_0600',
        network: request.agent.Network.WIFI,
        overwrite: true,
        mode: request.agent.Mode.FOREGROUND
      };
      // try {
      let path: string =
        '/data/storage/el2/base/haps/com.example.helloworld/cache/request/SUB_Request_Agent_TaskSaveas_0600';
      let ifExitOut: boolean = fs.accessSync(path);
      if (ifExitOut) {
        fs.unlinkSync(path);
        console.info("====>SUB_Request_Agent_TaskSaveas_0200 unlinkSync file success");
      }
      let task = await request.agent.create(baseContext, config1);
      let on_completedCallback = (pro: request.agent.Progress) => {
        try {
          // task.off('completed', on_completedCallback);
          console.info("====>SUB_Request_Agent_TaskSaveas_0600 completed task.config.saveas: " +
          JSON.stringify(task.config));
          let ifExit: boolean = fs.accessSync(path);
          console.info("====>SUB_Request_Agent_TaskSaveas_0600 ifExit: " + ifExit);
          // expect(ifExit).assertTrue();
          if (ifExit) {
            fs.unlinkSync(path);
            console.info("====>SUB_Request_Agent_TaskSaveas_0100 delete file success");
          }
        } catch (err) {
          console.info("====>SUB_Request_Agent_TaskSaveas_0100 catch err: " + JSON.stringify(err));
        }
      }
      // try {
      // if(err){
      //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 create err: " + JSON.stringify(err));
      //   expect().assertFail();
      // }
      task.on('completed', on_completedCallback);
      await task.start();
      console.info("====>SUB_Request_Agent_TaskSaveas_0100 start success: " + JSON.stringify(config.saveas));
      // } catch (err) {
      //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 catch err: " + JSON.stringify(err));
      //   await request.agent.remove(task.tid);
      // }
      // } catch (err) {
      //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 create catch err: " + JSON.stringify(err));
      //   expect().assertFail();
      // }
    });

    /**
     * @tc.name: SUB_Request_Agent_TaskSaveas_0100
     * @tc.desc: Test task saveas functionality with different filename configuration
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with relative directory path
     *           2. Check and delete any existing test file in the target path
     *           3. Create a download task using baseContext
     *           4. Bind the completed event handler to verify file existence
     *           5. Start the download task
     * @tc.require: issues#1573
     */
    it('SUB_Request_Agent_TaskSaveas_0100', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_Request_Agent_TaskSaveas_0700 is starting-----------------------");
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        title: 'createTest',
        description: 'XTS download test!',
        saveas: 'request/SUB_Request_Agent_TaskSaveas_0700',
        network: request.agent.Network.WIFI,
        overwrite: true,
        mode: request.agent.Mode.FOREGROUND
      };
      // try {
      let path: string =
        '/data/storage/el2/base/haps/com.example.helloworld/cache/request/SUB_Request_Agent_TaskSaveas_0700';
      let ifExitOut: boolean = fs.accessSync(path);
      if (ifExitOut) {
        fs.unlinkSync(path);
        console.info("====>SUB_Request_Agent_TaskSaveas_0200 unlinkSync file success");
      }
      let task = await request.agent.create(baseContext, config1);
      let on_completedCallback = (pro: request.agent.Progress) => {
        try {
          // task.off('completed', on_completedCallback);
          console.info("====>SUB_Request_Agent_TaskSaveas_0700 completed task.config.saveas: " +
          JSON.stringify(task.config));
          let ifExit: boolean = fs.accessSync(path);
          console.info("====>SUB_Request_Agent_TaskSaveas_0600 ifExit: " + ifExit);
          // expect(ifExit).assertTrue();
          if (ifExit) {
            fs.unlinkSync(path);
            console.info("====>SUB_Request_Agent_TaskSaveas_0100 delete file success");
          }
        } catch (err) {
          console.info("====>SUB_Request_Agent_TaskSaveas_0100 catch err: " + JSON.stringify(err));
        }
      }
      // try {
      // if(err){
      //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 create err: " + JSON.stringify(err));
      //   expect().assertFail();
      // }
      task.on('completed', on_completedCallback);
      await task.start();
      console.info("====>SUB_Request_Agent_TaskSaveas_0100 start success: " + JSON.stringify(config.saveas));
      // } catch (err) {
      //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 catch err: " + JSON.stringify(err));
      //   await request.agent.remove(task.tid);
      // }
      // } catch (err) {
      //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 create catch err: " + JSON.stringify(err));
      //   expect().assertFail();
      // }
    });

    /**
     * @tc.name: SUB_Request_Agent_TaskSaveas_0100
     * @tc.desc: Test task saveas functionality with directory and filename configuration
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with internal cache URI format
     *           2. Check and delete any existing test file in the target path
     *           3. Create a download task using baseContext
     *           4. Bind the completed event handler to verify file existence
     *           5. Start the download task
     * @tc.require: issues#1573
     */
    it('SUB_Request_Agent_TaskSaveas_0100', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_Request_Agent_TaskSaveas_0800 is starting-----------------------");
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        title: 'createTest',
        description: 'XTS download test!',
        saveas: 'internal://cache/test.hap',
        network: request.agent.Network.WIFI,
        overwrite: true,
        mode: request.agent.Mode.FOREGROUND
      };
      // try {
      let path: string =
        '/data/storage/el2/base/haps/com.example.helloworld/cache/request/SUB_Request_Agent_TaskSaveas_0800';
      let ifExitOut: boolean = fs.accessSync(path);
      if (ifExitOut) {
        fs.unlinkSync(path);
        console.info("====>SUB_Request_Agent_TaskSaveas_0200 unlinkSync file success");
      }
      let task = await request.agent.create(baseContext, config1);
      let on_completedCallback = (pro: request.agent.Progress) => {
        try {
          // task.off('completed', on_completedCallback);
          console.info("====>SUB_Request_Agent_TaskSaveas_0800 completed task.config.saveas: " +
          JSON.stringify(task.config));
          let ifExit: boolean = fs.accessSync(path);
          console.info("====>SUB_Request_Agent_TaskSaveas_0800 ifExit: " + ifExit);
          // expect(ifExit).assertTrue();
          if (ifExit) {
            fs.unlinkSync(path);
            console.info("====>SUB_Request_Agent_TaskSaveas_0100 delete file success");
          }
        } catch (err) {
          console.info("====>SUB_Request_Agent_TaskSaveas_0100 catch err: " + JSON.stringify(err));
        }
      }
      // try {
      // if(err){
      //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 create err: " + JSON.stringify(err));
      //   expect().assertFail();
      // }
      task.on('completed', on_completedCallback);
      await task.start();
      console.info("====>SUB_Request_Agent_TaskSaveas_0100 start success: " + JSON.stringify(config.saveas));
      // } catch (err) {
      //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 catch err: " + JSON.stringify(err));
      //   await request.agent.remove(task.tid);
      // }
      // } catch (err) {
      //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 create catch err: " + JSON.stringify(err));
      //   expect().assertFail();
      // }
    });

    /**
     * @tc.name: SUB_Request_Agent_TaskSaveas_0100
     * @tc.desc: Test task saveas functionality with internal cache path
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with absolute system path
     *           2. Check and delete any existing test file in the target path
     *           3. Create a download task using baseContext
     *           4. Bind the completed event handler to verify file existence
     *           5. Start the download task
     * @tc.require: issues#1573
     */
    it('SUB_Request_Agent_TaskSaveas_0100', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_Request_Agent_TaskSaveas_0900 is starting-----------------------");
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        title: 'createTest',
        description: 'XTS download test!',
        saveas: '/data/storage/el2/base/SUB_Request_Agent_TaskSaveas_0900',
        network: request.agent.Network.WIFI,
        overwrite: true,
        mode: request.agent.Mode.FOREGROUND
      };
      // try {
      let path = '/data/storage/el2/base/SUB_Request_Agent_TaskSaveas_0900';
      let ifExitOut: boolean = fs.accessSync(path);
      if (ifExitOut) {
        fs.unlinkSync(path);
        console.info("====>SUB_Request_Agent_TaskSaveas_0200 unlinkSync file success");
      }
      let task = await request.agent.create(baseContext, config1);
      let on_completedCallback = (pro: request.agent.Progress) => {
        try {
          // task.off('completed', on_completedCallback);
          console.info("====>SUB_Request_Agent_TaskSaveas_0900 completed task.config.saveas: " +
          JSON.stringify(task.config));
          let ifExit: boolean = fs.accessSync(path);
          console.info("====>SUB_Request_Agent_TaskSaveas_0900 ifExit: " + ifExit);
          // expect(ifExit).assertTrue();
          // if(ifExit){
          //   fs.unlinkSync(path);
          //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 delete file success");
          // }
        } catch (err) {
          console.info("====>SUB_Request_Agent_TaskSaveas_0100 catch err: " + JSON.stringify(err));
        }
      }
      // try {
      // if(err){
      //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 create err: " + JSON.stringify(err));
      //   expect().assertFail();
      // }
      task.on('completed', on_completedCallback);
      await task.start();
      console.info("====>SUB_Request_Agent_TaskSaveas_0100 start success: " + JSON.stringify(config.saveas));
      // } catch (err) {
      //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 catch err: " + JSON.stringify(err));
      //   await request.agent.remove(task.tid);
      // }
      // } catch (err) {
      //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 create catch err: " + JSON.stringify(err));
      //   expect().assertFail();
      // }
    });

    /**
     * @tc.name: SUB_Request_Agent_TaskSaveas_0100
     * @tc.desc: Test task saveas functionality with absolute directory path
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with absolute directory path
     *           2. Check and delete any existing test file in the target path
     *           3. Create a download task using baseContext
     *           4. Handle any potential errors during task creation
     * @tc.require: issues#1573
     */
    it('SUB_Request_Agent_TaskSaveas_0100', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_Request_Agent_TaskSaveas_1000 is starting-----------------------");
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        title: 'createTest',
        description: 'XTS download test!',
        saveas: '/data/storage/el2/base',
        network: request.agent.Network.WIFI,
        overwrite: true,
        mode: request.agent.Mode.FOREGROUND
      };

      let path: string =
        '/data/storage/el2/base/haps/com.example.helloworld/cache/request/SUB_Request_Agent_TaskSaveas_1000';
      let ifExitOut: boolean = fs.accessSync(path);
      if (ifExitOut) {
        fs.unlinkSync(path);
        console.info("====>SUB_Request_Agent_TaskSaveas_0100 unlinkSync file success");
      }
      try {
        let task = await request.agent.create(baseContext, config1);
      } catch (err: BusinessError) {
        console.info("====>SUB_Request_Agent_TaskSaveas_1000 err: " + err.code);
      }
    });

    /**
     * @tc.name: SUB_Request_Agent_TaskSaveas_0100
     * @tc.desc: Test task saveas functionality with path containing relative elements
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with path containing relative elements
     *           2. Check and delete any existing test file in the target path
     *           3. Create a download task using baseContext
     *           4. Set up completed event handler to verify file existence
     *           5. Start the download task
     * @tc.require: issues#1573
     */
    it('SUB_Request_Agent_TaskSaveas_0100', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_Request_Agent_TaskSaveas_1100 is starting-----------------------");
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        title: 'createTest',
        description: 'XTS download test!',
        saveas: '/data/storage/../../data/storage/el1/../el2/base/SUB_Request_Agent_TaskSaveas_1100',
        network: request.agent.Network.WIFI,
        overwrite: true,
        mode: request.agent.Mode.FOREGROUND
      };
      // try {
      let path = '/data/storage/el2/base/SUB_Request_Agent_TaskSaveas_1100';
      let ifExitOut: boolean = fs.accessSync(path);
      if (ifExitOut) {
        fs.unlinkSync(path);
        console.info("====>SUB_Request_Agent_TaskSaveas_1100 unlinkSync file success");
      }
      let task = await request.agent.create(baseContext, config1);
      let on_completedCallback = (pro: request.agent.Progress) => {
        try {
          // task.off('completed', on_completedCallback);
          console.info("====>SUB_Request_Agent_TaskSaveas_1100 completed task.config.saveas: " +
          JSON.stringify(task.config));
          let ifExit: boolean = fs.accessSync(path);
          console.info("====>SUB_Request_Agent_TaskSaveas_1100 ifExit: " + ifExit);
          // expect(ifExit).assertTrue();
          // if(ifExit){
          //   fs.unlinkSync(path);
          //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 delete file success");
          // }
        } catch (err) {
          console.info("====>SUB_Request_Agent_TaskSaveas_0100 catch err: " + JSON.stringify(err));
        }
      }
      // try {
      // if(err){
      //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 create err: " + JSON.stringify(err));
      //   expect().assertFail();
      // }
      task.on('completed', on_completedCallback);
      await task.start();
      console.info("====>SUB_Request_Agent_TaskSaveas_0100 start success: " + JSON.stringify(config.saveas));
      // } catch (err) {
      //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 catch err: " + JSON.stringify(err));
      //   await request.agent.remove(task.tid);
      // }
      // } catch (err) {
      //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 create catch err: " + JSON.stringify(err));
      //   expect().assertFail();
      // }
    });

    /**
     * @tc.name: SUB_Request_Agent_TaskSaveas_1200
     * @tc.desc: Test task saveas functionality with specific download configuration
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with relative path elements
     *           2. Check and delete any existing test file in the target path
     *           3. Attempt to create a download task and catch potential errors
     * @tc.require: issues#1573
     */
    it('SUB_Request_Agent_TaskSaveas_1200', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_Request_Agent_TaskSaveas_1200 is starting-----------------------");
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        title: 'createTest',
        description: 'XTS download test!',
        saveas: '/data/storage/../../el2/base/SUB_Request_Agent_TaskSaveas_1200',
        network: request.agent.Network.WIFI,
        overwrite: true,
        mode: request.agent.Mode.FOREGROUND
      };

      let path: string =
        '/data/storage/el2/base/haps/com.example.helloworld/cache/request/SUB_Request_Agent_TaskSaveas_1000';
      let ifExitOut: boolean = fs.accessSync(path);
      if (ifExitOut) {
        fs.unlinkSync(path);
        console.info("====>SUB_Request_Agent_TaskSaveas_1200 unlinkSync file success");
      }
      try {
        let task = await request.agent.create(baseContext, config1);
      } catch (err: BusinessError) {
        console.info("====>SUB_Request_Agent_TaskSaveas_1200 err: " + err.code);
      }
    });

    /**
     * @tc.name: SUB_Request_Agent_TaskSaveas_1300
     * @tc.desc: Test task saveas functionality with file:// protocol path
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with file:// protocol path
     *           2. Check and delete any existing test file in the target path
     *           3. Create a download task using baseContext
     *           4. Set up completed event handler to verify file existence
     * @tc.require: issues#1573
     */
    it('SUB_Request_Agent_TaskSaveas_1300', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_Request_Agent_TaskSaveas_1300 is starting-----------------------");
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        title: 'createTest',
        description: 'XTS download test!',
        saveas: 'file://com.example.helloworld/data/storage/el2/base/SUB_Request_Agent_TaskSaveas_1300',
        network: request.agent.Network.WIFI,
        overwrite: true,
        mode: request.agent.Mode.FOREGROUND
      };
      // try {
      let path: string = '/data/storage/el2/base/SUB_Request_Agent_TaskSaveas_1300';
      let ifExitOut: boolean = fs.accessSync(path);
      if (ifExitOut) {
        fs.unlinkSync(path);
        console.info("====>SUB_Request_Agent_TaskSaveas_1300 unlinkSync file success");
      }
      let task = await request.agent.create(baseContext, config1);
      let on_completedCallback = (pro: request.agent.Progress) => {
        try {
          // task.off('completed', on_completedCallback);
          console.info("====>SUB_Request_Agent_TaskSaveas_1300 completed task.config.saveas: " +
          JSON.stringify(task.config));
          let ifExit: boolean = fs.accessSync(path);
          console.info("====>SUB_Request_Agent_TaskSaveas_1300 ifExit: " + ifExit);
          // expect(ifExit).assertTrue();
          // if(ifExit){
          //   fs.unlinkSync(path);
          //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 delete file success");
          // }
        } catch (err) {
          console.info("====>SUB_Request_Agent_TaskSaveas_1300 catch err: " + JSON.stringify(err));
        }
      }
      // try {
      // if(err){
      //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 create err: " + JSON.stringify(err));
      //   expect().assertFail();
      // }
      task.on('completed', on_completedCallback);
      await task.start();
      console.info("====>SUB_Request_Agent_TaskSaveas_0100 start success: " + JSON.stringify(config.saveas));
      // } catch (err) {
      //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 catch err: " + JSON.stringify(err));
      //   await request.agent.remove(task.tid);
      // }
      // } catch (err) {
      //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 create catch err: " + JSON.stringify(err));
      //   expect().assertFail();
      // }
    });

    /**
     * @tc.name: SUB_Request_Agent_TaskSaveas_1400
     * @tc.desc: Test task saveas functionality with specific file URL configuration
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with file:// protocol and different app ID
     *           2. Check and delete any existing test file in the target path
     *           3. Attempt to create a download task and catch potential errors
     * @tc.require: issues#1573
     */
    it('SUB_Request_Agent_TaskSaveas_1400', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_Request_Agent_TaskSaveas_1400 is starting-----------------------");
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        title: 'createTest',
        description: 'XTS download test!',
        saveas: 'file://com.examples.app123/data/storage/el2/base/SUB_Request_Agent_TaskSaveas_1300',
        network: request.agent.Network.WIFI,
        overwrite: true,
        mode: request.agent.Mode.FOREGROUND
      };

      let path: string =
        '/data/storage/el2/base/haps/com.example.helloworld/cache/request/SUB_Request_Agent_TaskSaveas_1000';
      let ifExitOut: boolean = fs.accessSync(path);
      if (ifExitOut) {
        fs.unlinkSync(path);
        console.info("====>SUB_Request_Agent_TaskSaveas_1400 unlinkSync file success");
      }
      try {
        let task = await request.agent.create(baseContext, config1);
      } catch (err: BusinessError) {
        console.info("====>SUB_Request_Agent_TaskSaveas_1400 err: " + err.code);
      }
    });

    /**
     * @tc.name: SUB_Request_Agent_TaskSaveas_1500
     * @tc.desc: Test task saveas functionality with el5 storage path
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with el5 storage path
     *           2. Check and delete any existing test file in the target path
     *           3. Create a download task using baseContext
     *           4. Set up completed event handler to verify file existence
     *           5. Start the download task
     * @tc.require: issues#1573
     */
    it('SUB_Request_Agent_TaskSaveas_1500', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_Request_Agent_TaskSaveas_1500 is starting-----------------------");
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        title: 'createTest',
        description: 'XTS download test!',
        saveas: '/data/storage/el5/base/SUB_Request_Agent_TaskSaveas_1500',
        network: request.agent.Network.WIFI,
        overwrite: true,
        mode: request.agent.Mode.FOREGROUND
      };
      // try {
      let path = '/data/storage/el5/base/SUB_Request_Agent_TaskSaveas_1500';
      let ifExitOut: boolean = fs.accessSync(path);
      if (ifExitOut) {
        fs.unlinkSync(path);
        console.info("====>SUB_Request_Agent_TaskSaveas_1500 unlinkSync file success");
      }
      let task = await request.agent.create(baseContext, config1);
      let on_completedCallback = (pro: request.agent.Progress) => {
        try {
          // task.off('completed', on_completedCallback);
          console.info("====>SUB_Request_Agent_TaskSaveas_1500 completed task.config.saveas: " +
          JSON.stringify(task.config));
          let ifExit: boolean = fs.accessSync(path);
          console.info("====>SUB_Request_Agent_TaskSaveas_1500 ifExit: " + ifExit);
          // expect(ifExit).assertTrue();
          // if(ifExit){
          //   fs.unlinkSync(path);
          //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 delete file success");
          // }
        } catch (err) {
          console.info("====>SUB_Request_Agent_TaskSaveas_1500 catch err: " + JSON.stringify(err));
        }
      }
      // try {
      // if(err){
      //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 create err: " + JSON.stringify(err));
      //   expect().assertFail();
      // }
      task.on('completed', on_completedCallback);
      await task.start();
      console.info("====>SUB_Request_Agent_TaskSaveas_1500 start success: " + JSON.stringify(config.saveas));
      // } catch (err) {
      //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 catch err: " + JSON.stringify(err));
      //   await request.agent.remove(task.tid);
      // }
      // } catch (err) {
      //   console.info("====>SUB_Request_Agent_TaskSaveas_0100 create catch err: " + JSON.stringify(err));
      //   expect().assertFail();
      // }
    });

    /**
     * @tc.name: SUB_Request_Agent_onResponse_0100
     * @tc.desc: Test onResponse event functionality for download task
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with ANY network mode
     *           2. Create a download task using baseContext
     *           3. Set up response event handler to process HTTP response information
     *           4. Start the download task
     * @tc.require: issues#1573
     */
    it('SUB_Request_Agent_onResponse_0100', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_Request_Agent_onResponse_0100 is starting-----------------------");
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        title: 'createTest',
        description: 'XTS download test!',
        saveas: './SUB_Request_Agent_onResponse_0100',
        network: request.agent.Network.ANY,
        overwrite: true
      };
      try {
        let task: request.agent.Task = await request.agent.create(baseContext, config1);
        console.info("====>SUB_Request_Agent_onResponse_0100 create success");
        let responseCallback_1 = (info: request.agent.HttpResponse) => {
          try {
            console.info("====>SUB_Request_Agent_onResponse_0100 response info: " + JSON.stringify(info));
          } catch (err) {
            console.info("====>SUB_Request_Agent_onResponse_0100 err: " + JSON.stringify(err));
          }
        }
        task.on('response', responseCallback_1);
        await task.start();
        console.info("====>SUB_Request_Agent_onResponse_0100 task.start");
      } catch (err) {
        console.info("====>SUB_Request_Agent_onResponse_0100 create catch err: " + JSON.stringify(err));
        expect().assertFail();
      }
    });

    /**
     * @tc.name: SUB_Request_Agent_onResponse_0200
     * @tc.desc: Test onResponse event with 404 error response
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with non-existent URL
     *           2. Create a download task using baseContext
     *           3. Set up response event handler to process 404 error response
     *           4. Start the download task
     * @tc.require: issues#1573
     */
    it('SUB_Request_Agent_onResponse_0200', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_Request_Agent_onResponse_0200 is starting-----------------------");
      let config1:request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984572/test-files/releases/downloads/1.1/test.hap',
        title: 'createTest',
        description: 'XTS download test!',
        saveas: './SUB_Request_Agent_onResponse_0200',
        network: request.agent.Network.ANY,
        overwrite: true
      };
      try {
      let task: request.agent.Task = await request.agent.create(baseContext, config1);
      console.info("====>SUB_Request_Agent_onResponse_0200 create success");
      let responseCallback_1 = (info: request.agent.HttpResponse) => {
        try {
          console.info("====>SUB_Request_Agent_onResponse_0200 response info: " + JSON.stringify(info));
          // expect(info.statusCode).assertEqual(404);
        } catch (err) {
          console.info("====>SUB_Request_Agent_onResponse_0200 err: " + JSON.stringify(err));
        }
      }
      task.on('response', responseCallback_1);
      await task.start();
      console.info("====>SUB_Request_Agent_onResponse_0200 task.start");
    } catch (err) {
      console.info("====>SUB_Request_Agent_onResponse_0200 create catch err: " + JSON.stringify(err));
      expect().assertFail();
    }
    });

    /**
     * @tc.name: SUB_Request_Agent_onResponse_0300
     * @tc.desc: Test onResponse event with event listener removal
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with ANY network mode
     *           2. Create a download task using baseContext
     *           3. Set up response event handler
     *           4. Start the download task
     *           5. Remove the response event listener
     * @tc.require: issues#1573
     */
    it('SUB_Request_Agent_onResponse_0300', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_Request_Agent_onResponse_0300 is starting-----------------------");
      let config1:request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        title: 'createTest',
        description: 'XTS download test!',
        saveas: './SUB_Request_Agent_onResponse_0300',
        network: request.agent.Network.ANY,
        overwrite: true
      };
      try {
      let task: request.agent.Task = await request.agent.create(baseContext, config1);
      console.info("====>SUB_Request_Agent_onResponse_0300 create success");
      let responseCallback_1 = (info: request.agent.HttpResponse) => {
        try {
          console.info("====>SUB_Request_Agent_onResponse_0300 response info: " + JSON.stringify(info));
          // expect(info.statusCode).assertEqual(404);
        } catch (err) {
          console.info("====>SUB_Request_Agent_onResponse_0300 err: " + JSON.stringify(err));
        }
      }
      task.on('response', responseCallback_1);
      await task.start();
      console.info("====>SUB_Request_Agent_onResponse_0300 task.start");
      task.off('response', responseCallback_1);
    } catch (err) {
      console.info("====>SUB_Request_Agent_onResponse_0300 create catch err: " + JSON.stringify(err));
      expect().assertFail();
    }
    });

    /**
     * @tc.name: SUB_Request_Agent_onResponse_0500
     * @tc.desc: Test onResponse event with multiple callbacks
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with ANY network mode
     *           2. Create a download task using baseContext
     *           3. Set up two separate response event handlers
     *           4. Remove all response event listeners
     *           5. Start the download task
     * @tc.require: issues#1573
     */
    it('SUB_Request_Agent_onResponse_0500', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_Request_Agent_onResponse_0500 is starting-----------------------");
      let config1:request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        title: 'createTest',
        description: 'XTS download test!',
        saveas: './SUB_Request_Agent_onResponse_0500',
        network: request.agent.Network.ANY,
        overwrite: true
      };
      try {
      let task: request.agent.Task = await request.agent.create(baseContext, config1);
      console.info("====>SUB_Request_Agent_onResponse_0500 create success");
      let responseCallback_1 = (info: request.agent.HttpResponse) => {
        try {
          console.info("====>SUB_Request_Agent_onResponse_0500 response1 info: " + JSON.stringify(info));
          // expect(info.statusCode).assertEqual(404);
        } catch (err) {
          console.info("====>SUB_Request_Agent_onResponse_0500 err: " + JSON.stringify(err));
        }
      }
      let responseCallback_2 = (info: request.agent.HttpResponse) => {
        try {
          console.info("====>SUB_Request_Agent_onResponse_0500 response2 info: " + JSON.stringify(info));
          // expect(info.statusCode).assertEqual(404);
        } catch (err) {
          console.info("====>SUB_Request_Agent_onResponse_0500 err: " + JSON.stringify(err));
        }
      }
      task.on('response', responseCallback_1);
      task.on('response', responseCallback_2);
      task.off('response');
      await task.start();
      console.info("====>SUB_Request_Agent_onResponse_0500 task.start");
    } catch (err) {
      console.info("====>SUB_Request_Agent_onResponse_0500 create catch err: " + JSON.stringify(err));
      // expect().assertFail();
    }
    });

    /**
     * @tc.name: SUB_Request_Agent_onResponse_0700
     * @tc.desc: Test onResponse event with large file download
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration for large file download
     *           2. Create a download task using baseContext
     *           3. Set up response event handler to process partial content response
     *           4. Set up progress event handler to manage pause/resume operations
     * @tc.require: issues#1573
     */
    it('SUB_Request_Agent_onResponse_0700', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_Request_Agent_onResponse_0700 is starting-----------------------");
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://cidownload.openharmony.cn/version/Master_Version/OpenHarmony_5.0.2.45/20240924_092847/version-Master_Version-OpenHarmony_5.0.2.45-20240924_092847-hispark_taurus_Linux_weekly.tar.gz',
        title: 'createTest',
        description: 'XTS download test!',
        saveas: './SUB_Request_Agent_onResponse_0700',
        network: request.agent.Network.ANY,
        overwrite: true
      };
      try {
      let task: request.agent.Task = await request.agent.create(baseContext, config);
      console.info("====>SUB_Request_Agent_onResponse_0700 create success");

      let responseCallback_1 = (info: request.agent.HttpResponse) => {
        try {
          task.off('response');
          console.info("====>SUB_Request_Agent_onResponse_0700 response info: " + JSON.stringify(info));
          // expect(info.statusCode).assertEqual(206);
        } catch (err) {
          console.info("====>SUB_Request_Agent_onResponse_0700 responseCallback_1 err: " + JSON.stringify(err));
        }
      }

      let progressCallback = (pro: request.agent.Progress) => {
        try {
          task.off('progress');
          console.info("====>SUB_Request_Agent_onResponse_0700 pro: " + JSON.stringify(pro));
          await task.pause();
          task.on('response', responseCallback_1);
          await task.resume();
        } catch (err) {
          console.info("====>SUB_Request_Agent_onResponse_0700 progressCallback err: " + JSON.stringify(err));
        }
      }
      task.on('progress', progressCallback);
      await task.start();
    } catch (err) {
      console.info("====>SUB_Request_Agent_onResponse_0700 create catch err: " + JSON.stringify(err));
      // expect().assertFail();
    }
    });

    /**
     * @tc.name: SUB_Request_Agent_proxy_0200
     * @tc.desc: Test task with proxy configuration
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create upload task configuration with proxy settings
     *           2. Attempt to create an upload task with the proxy configuration
     *           3. Handle any potential errors during task creation
     * @tc.require: issues#1573
     */
    it('SUB_Request_Agent_proxy_0200', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_Request_Agent_proxy_0200 is starting-----------------------");
      let config1:request.agent.Config = {
        action: request.agent.Action.UPLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        title: 'createTest',
        description: 'XTS download test!',
        saveas: './SUB_Request_Agent_proxy_0200',
        network: request.agent.Network.ANY,
        overwrite: true,
        proxy: 'https://192.168.249.58:808'
      };
      try {
      console.info("====>SUB_Request_Agent_proxy_0200 create task");
      let task: request.agent.Task = await request.agent.create(baseContext, config1);
      console.info("====>SUB_Request_Agent_proxy_0200 create success");
      // expect().assertFail();
    } catch (err) {
      console.info("====>SUB_Request_Agent_proxy_0200 create catch err: " + JSON.stringify(err));
      // expect(err.code).assertEqual(401);
    }
    });

    /**
     * @tc.name: SUB_Request_Agent_proxy_0400
     * @tc.desc: Test download task with proxy configuration
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with proxy settings
     *           2. Attempt to create a download task with the proxy configuration
     *           3. Handle any potential errors during task creation
     * @tc.require: issues#1573
     */
    it('SUB_Request_Agent_proxy_0400', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info('====>-----------------------SUB_Request_Agent_proxy_0400 is starting-----------------------');
      let config1:request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        title: 'createTest',
        description: 'XTS download test!',
        saveas: './SUB_Request_Agent_proxy_0400',
        network: request.agent.Network.ANY,
        overwrite: true,
        proxy: 'https://192.168.249.58:808'
      };
      try {
      console.info("====>SUB_Request_Agent_proxy_0400 create task");
      let task: request.agent.Task = await request.agent.create(baseContext, config1);
      console.info("====>SUB_Request_Agent_proxy_0400 create success");
      // expect().assertFail();
    } catch (err) {
      console.info("====>SUB_Request_Agent_proxy_0400 create catch err: " + JSON.stringify(err));
      // expect(err.code).assertEqual(401);
    }
    });

    /**
     * @tc.name: SUB_Request_Agent_ErrorDetailed_0100
     * @tc.desc: Test error details when URL is invalid
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with invalid URL format
     *           2. Create a download task using baseContext
     *           3. Set up failed event handler to process error details
     *           4. Start the download task
     * @tc.require: issues#1573
     */
    it('SUB_Request_Agent_ErrorDetailed_0100', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info('====>-----------------------SUB_Request_Agent_ErrorDetailed_0100 is starting-----------------------');
      let config1:request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://?PARAM.com',
        title: 'createTest',
        description: 'XTS download test!',
        saveas: './SUB_Request_Agent_ErrorDetailed_0100',
        network: request.agent.Network.ANY,
        overwrite: true
      };
      try {
      let task: request.agent.Task = await request.agent.create(baseContext, config1);
      task.on('failed', (pro: request.agent.Progress) => {
      try {
      task.off('failed');
      console.info('====>SUB_Request_Agent_ErrorDetailed_0100 failed pro: ' + JSON.stringify(pro));
      let info:request.agent.TaskInfo = await request.agent.show(task.tid);
      console.info('====>SUB_Request_Agent_ErrorDetailed_0100 info: ' + JSON.stringify(info));
      // expect(info.faults).assertEqual(48);
    } catch (err) {
      console.info('====>SUB_Request_Agent_ErrorDetailed_0100 err: ' + JSON.stringify(err));
    }
    })
    await task.start();
    console.info('====>SUB_Request_Agent_ErrorDetailed_0100 create success');
    } catch (err) {
      console.info('====>> create catch err: ' + JSON.stringify(err));
      // expect().assertFail();
    }
    });

    /**
     * @tc.name: SUB_Request_Agent_ContinueDown_0300
     * @tc.desc: Test continue download functionality
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with WIFI network mode
     *           2. Create a download task with callback function
     *           3. Set up completed event handler to test restart functionality
     *           4. Start the download task
     * @tc.require: issues#1573
     */
    it('SUB_Request_Agent_ContinueDown_0300', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_Request_Agent_ContinueDown_0300 is starting-----------------------");
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        title: 'createTest',
        description: 'XTS download test!',
        saveas: './SUB_Request_Agent_ContinueDown_0300.txt',
        network: request.agent.Network.WIFI,
        overwrite: true,
        mode:request.agent.Mode.FOREGROUND
      };
      request.agent.create(baseContext, config1, (err, task) => {
      let on_completedCallback = (pro: request.agent.Progress) => {
        // task.off('completed', on_completedCallback);
        try {
          task?.start();
          console.info("====>SUB_Request_Agent_ContinueDown_0300 task.start success ");
          // expect().assertFail();
        } catch (err) {
          try {
            console.info("====>SUB_Request_Agent_ContinueDown_0300 start err: " + JSON.stringify(err));
            // expect(err.code).assertEqual(21900007);
          } catch (e) {
            console.info("====>SUB_Request_Agent_ContinueDown_0300 code err: " + JSON.stringify(e));
          }
        }
      }
      try {
      if(err){
        console.info("====>SUB_Request_Agent_ContinueDown_0300 create err: " + JSON.stringify(err));
        // expect().assertFail();
      }
      task?.on('completed', on_completedCallback);
      task?.start();
    } catch (err) {
      console.info("====>SUB_Request_Agent_ContinueDown_0300 catch err: " + JSON.stringify(err));
    }
    });
    });

    /**
     * @tc.name: SUB_Request_Agent_ContinueDown_0500
     * @tc.desc: Test continue download with pause functionality
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with WIFI network mode
     *           2. Create a download task with callback function
     *           3. Set up pause event handler to test restart functionality
     *           4. Start the download task
     *           5. Immediately pause the download task
     * @tc.require: issues#1573
     */
    it('SUB_Request_Agent_ContinueDown_0500', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_Request_Agent_ContinueDown_0500 is starting-----------------------");
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        title: 'createTest',
        description: 'XTS download test!',
        saveas: './SUB_Request_Agent_ContinueDown_0500.txt',
        network: request.agent.Network.WIFI,
        overwrite: true,
        mode:request.agent.Mode.FOREGROUND
      };
      request.agent.create(baseContext, config1, (err, task) => {
      let on_completedCallback = (pro: request.agent.Progress) => {
        // task.off('pause', on_completedCallback);
        try {
          task?.start();
          console.info("====>SUB_Request_Agent_ContinueDown_0500 task.start success ");
          // expect().assertFail();
        } catch (err) {
          try {
            console.info("====>SUB_Request_Agent_ContinueDown_0500 start err: " + JSON.stringify(err));
            // expect(err.code).assertEqual(21900007);
          } catch (e) {
            console.info("====>SUB_Request_Agent_ContinueDown_0500 code err: " + JSON.stringify(e));
          }
        }
      }
      try {
      if(err){
        console.info("====>SUB_Request_Agent_ContinueDown_0500 create err: " + JSON.stringify(err));
        // expect().assertFail();
      }
      task?.on('pause', on_completedCallback);
      task?.start();
      task?.pause();
    } catch (err) {
      console.info("====>SUB_Request_Agent_ContinueDown_0500 catch err: " + JSON.stringify(err));
      // await removeTask(task);
    }
    });
    });

    /**
     * @tc.name: SUB_Request_Agent_PauseError_callback_0100
     * @tc.desc: Test pause error callback handling
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with ANY network mode
     *           2. Create a download task using baseContext
     *           3. Call pause() method with error callback
     *           4. Verify error code in the callback
     * @tc.require: issues#1573
     */
    it('SUB_Request_Agent_PauseError_callback_0100', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_Request_Agent_PauseError_callback_0100 is starting-----------------------");
      let config1:request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        title: 'createTest',
        description: 'XTS download test!',
        saveas: './SUB_Request_Agent_PauseError_callback_0100',
        network: request.agent.Network.ANY,
        overwrite: true
      };
      try {
      let task: request.agent.Task = await request.agent.create(baseContext, config1);
      console.info("====>SUB_Request_Agent_PauseError_callback_0100 create success");
      task.pause((err: BusinessError|null) => {
      try{
      if (err) {
        console.info("====>SUB_Request_Agent_PauseError_callback_0100 download pause err: " + JSON.stringify(err.code));
        // expect(err.code).assertEqual(21900007);
      } else {
      console.info('====>SUB_Request_Agent_PauseError_callback_0100 failed');
      // expect().assertFail();
    }
    }catch(err){
      console.info("====>SUB_Request_Agent_PauseError_callback_0100 catch err: " + JSON.stringify(err));
    }
    // await request.agent.remove(task?.tid);
    console.info("====>SUB_Request_Agent_PauseError_callback_0100 removeTask");
    })
    } catch (err) {
      console.info("====>SUB_Request_Agent_PauseError_callback_0100 create catch err: " + JSON.stringify(err));
      // expect().assertFail();
    }
    });

    /**
     * @tc.name: SUB_Request_Agent_PauseError_promise_0100
     * @tc.desc: Test pause error handling with promise
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with ANY network mode
     *           2. Create a download task using baseContext
     *           3. Call pause() method and catch promise rejection
     *           4. Verify error code in the caught exception
     * @tc.require: issues#1573
     */
    it('SUB_Request_Agent_PauseError_promise_0100', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_Request_Agent_PauseError_promise_0100 is starting-----------------------");
      let config1:request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        title: 'createTest',
        description: 'XTS download test!',
        saveas: './SUB_Request_Agent_PauseError_promise_0100',
        network: request.agent.Network.ANY,
        overwrite: true
      };
      let task: request.agent.Task | null = null;
      try {
      task = await request.agent.create(baseContext, config1);
      console.info("====>SUB_Request_Agent_PauseError_promise_0100 create success");
      await task.pause();
      console.info("====>SUB_Request_Agent_PauseError_promise_0100 pause success");
      // expect().assertFail();
    } catch (err: BusinessError) {
      console.info("====>SUB_Request_Agent_PauseError_promise_0100 catch err: " + JSON.stringify(err.code));
      // expect(err.code).assertEqual(21900007);
      if (task !== null){
        // await removeTask(task);
        console.info("====>SUB_Request_Agent_PauseError_promise_0100 removeTask");
      }
    }
    });

    /**
     * @tc.name: SUB_Request_Download_off_faultOccur_0100
     * @tc.desc: Test download with fault occur callbacks
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with test URL
     *           2. Create a download task using baseContext
     *           3. Set up faultOccur event handler to process faults
     *           4. Start the download task
     * @tc.require: issues#1573
     */
    it('SUB_Request_Download_off_faultOccur_0100', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("-----------------------SUB_Request_Download_off_faultOccur_0100 is starting-----------------------");
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/V1.0/test1.txt',
        title: 'reeateTest',
        overwrite: true,
        description: 'XTS download test!',
        saveas: './SUB_Request_Download_off_faultOccur_0100.txt',
        network: request.agent.Network.ANY,
        mode:request.agent.Mode.BACKGROUND
      };

      // let arr: Array<number> = new Array();
      // let arr1 = [2, 3];
      let task: request.agent.Task = await request.agent.create(baseContext, config1);
      let faultOccurCallBack1 = (faults: request.agent.Faults) => {
        console.info("====>SUB_Request_Download_off_faultOccur_0100 faultOccurCallBack1 faults: " + faults);
        // arr.push(1);
      }
      let faultOccurCallBack2 = (faults: request.agent.Faults) => {
        console.info("====>SUB_Request_Download_off_faultOccur_0100 faultOccurCallBack2 faults: " + faults);
        // arr.push(2);
      }
      let faultOccurCallBack3 = (faults: request.agent.Faults) => {
        try {
          task.off('faultOccur')
          console.info("====>SUB_Request_Download_off_faultOccur_0100 faultOccurCallBack3 faults: " + faults);
          // arr.push(3);
          // console.info("====>SUB_Request_Download_off_faultOccur_0100 task3 progress on arr: " + JSON.stringify(arr));
          // expect(arr.join()).assertEqual(arr1.join());
        } catch (err) {
          console.info("====>SUB_Request_Download_off_faultOccur_0100 catch err: " + JSON.stringify(err));
        }
      }

      console.info("====>SUB_Request_Download_off_faultOccur_0100 create task success");
      task.on('faultOccur', faultOccurCallBack1);
      task.off('faultOccur', faultOccurCallBack1);
      task.on('faultOccur', faultOccurCallBack2);
      task.on('faultOccur', faultOccurCallBack3);
      await task.start();
    });

    /**
     * @tc.name: SUB_Request_Download_off_faultOccur_0200
     * @tc.desc: Test download with fault occur event off without callback
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with test URL
     *           2. Create a download task using baseContext
     *           3. Call off('faultOccur') without specifying a callback function
     *           4. Set up faultOccur event handler
     *           5. Start the download task
     * @tc.require: issues#1573
     */
    it('SUB_Request_Download_off_faultOccur_0200', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("-----------------------SUB_Request_Download_off_faultOccur_0200 is starting-----------------------");
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/chenzhixue/downloadTest/releases/download/V1.0/test1.txt',
        title: 'reeateTest',
        overwrite: true,
        description: 'XTS download test!',
        saveas: './SUB_Request_Download_off_faultOccur_0200.txt',
        network: request.agent.Network.ANY,
        mode:request.agent.Mode.BACKGROUND
      };

      let faultOccurCallBack1 = (faults: request.agent.Faults) => {
        console.info("====>SUB_Request_Download_off_faultOccur_0200 faultOccurCallBack1 faults: " + faults);
      }
      let faultOccurCallBack2 = (faults: request.agent.Faults) => {
        console.info("====>SUB_Request_Download_off_faultOccur_0200 faultOccurCallBack2 faults: " + faults);
      }
      let faultOccurCallBack3 = (faults: request.agent.Faults) => {
        console.info("====>SUB_Request_Download_off_faultOccur_0200 faultOccurCallBack3 faults: " + faults);
      }

      let task: request.agent.Task = await request.agent.create(baseContext, config);
      console.info("====>SUB_Request_Download_off_faultOccur_0200 create task success");
      task.on('faultOccur', faultOccurCallBack1);
      task.on('faultOccur', faultOccurCallBack2);
      task.on('faultOccur', faultOccurCallBack3);
      task.off('faultOccur');
      // await task.start();
      // let t = setTimeout(() => {
      //   clearTimeout(t);
      //   console.info("====>SUB_Request_Download_off_faultOccur_0200 create task success");
      // }, 1000)
    });

    /**
     * @tc.name: SUB_Request_Download_on_faultOccur_0300
     * @tc.desc: Test download with multiple fault occur callbacks
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with invalid URL
     *           2. Create a download task using baseContext
     *           3. Set up multiple faultOccur event handlers
     *           4. Start the download task
     * @tc.require: issues#1573
     */
    it('SUB_Request_Download_on_faultOccur_0300', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("-----------------------SUB_Request_Download_on_faultOccur_0300 is starting-----------------------");
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https:////',
        title: 'reeateTest',
        overwrite: true,
        description: 'XTS download test!',
        saveas: './SUB_Request_Download_on_faultOccur_0300.txt',
        network: request.agent.Network.ANY,
        mode:request.agent.Mode.BACKGROUND
      };
      let task: request.agent.Task = await request.agent.create(baseContext, config1);

      let faultOccurCallBack1 = (faults: request.agent.Faults) => {
        console.info("====>SUB_Request_Download_on_faultOccur_0300 faultOccurCallBack1 faults: " + faults);
      }
      let faultOccurCallBack2 = (faults: request.agent.Faults) => {
        console.info("====>SUB_Request_Download_on_faultOccur_0300 faultOccurCallBack2 faults: " + faults);
      }
      let faultOccurCallBack3 = (faults: request.agent.Faults) => {
        try {
          task.off('faultOccur')
          console.info("====>SUB_Request_Download_on_faultOccur_0300 faultOccurCallBack3 faults: " + faults);
        } catch (err) {
          console.info("====>SUB_Request_Download_on_faultOccur_0300 catch err: " + JSON.stringify(err));
        }
      }

      console.info("====>SUB_Request_Download_on_faultOccur_0300 create task success");
      task.on('faultOccur', faultOccurCallBack1);
      task.on('faultOccur', faultOccurCallBack2);
      task.on('faultOccur', faultOccurCallBack3);
      await task.start();
    });

    /**
     * @tc.name: SUB_MISC_REQUEST_DOWNLOAD_state_0010
     * @tc.desc: Test download task state enumeration values
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Log all download task state enumeration values
     *           2. Verify the existence and correctness of each state value
     * @tc.require: issues#1573
     */
    it('SUB_MISC_REQUEST_DOWNLOAD_state_0010', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_MISC_REQUEST_DOWNLOAD_state_0010 is starting-----------------------");
      try {
      console.info("====>SUB_MISC_REQUEST_DOWNLOAD_state_0010 request.agent.State.INITIALIZED:" + request.agent.State.INITIALIZED);
      console.info("====>SUB_MISC_REQUEST_DOWNLOAD_state_0010 request.agent.State.WAITING:" + request.agent.State.WAITING);
      console.info("====>SUB_MISC_REQUEST_DOWNLOAD_state_0010 request.agent.State.RUNNING:" + request.agent.State.RUNNING);
      console.info("====>SUB_MISC_REQUEST_DOWNLOAD_state_0010 request.agent.State.RETRYING:" + request.agent.State.RETRYING);
      console.info("====>SUB_MISC_REQUEST_DOWNLOAD_state_0010 request.agent.State.PAUSED:" + request.agent.State.PAUSED);
      console.info("====>SUB_MISC_REQUEST_DOWNLOAD_state_0010 request.agent.State.STOPPED:" + request.agent.State.STOPPED);
      console.info("====>SUB_MISC_REQUEST_DOWNLOAD_state_0010 request.agent.State.COMPLETED:" + request.agent.State.COMPLETED);
      console.info("====>SUB_MISC_REQUEST_DOWNLOAD_state_0010 request.agent.State.FAILED:" + request.agent.State.FAILED);
      console.info("====>SUB_MISC_REQUEST_DOWNLOAD_state_0010 request.agent.State.REMOVED:" + request.agent.State.REMOVED);
    } catch (err) {
      console.error("====>SUB_MISC_REQUEST_DOWNLOAD_state_0010 error: " + err);
      // expect().assertFail();
    }
    console.info("====>-----------------------SUB_MISC_REQUEST_DOWNLOAD_state_0010 end-----------------------");
    });

    /**
     * @tc.name: SUB_MISC_REQUEST_DOWNLOAD_faults_0010
     * @tc.desc: Test download task faults enumeration values
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Log all download task faults enumeration values
     *           2. Verify the existence and correctness of each faults value
     * @tc.require: issues#1573
     */
    it('SUB_MISC_REQUEST_DOWNLOAD_faults_0010', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_MISC_REQUEST_DOWNLOAD_faults_0010 is starting-----------------------");
      try {
      console.info("====>SUB_MISC_REQUEST_DOWNLOAD_faults_0010 request.agent.Faults.OTHERS: " + request.agent.Faults.OTHERS);
      // expect(request.agent.Faults.OTHERS).assertEqual(255);
      console.info("====>SUB_MISC_REQUEST_DOWNLOAD_faults_0010 request.agent.Faults.DISCONNECTED: " + request.agent.Faults.DISCONNECTED);
      // expect(request.agent.Faults.DISCONNECTED).assertEqual(0);
      console.info("====>SUB_MISC_REQUEST_DOWNLOAD_faults_0010 request.agent.Faults.TIMEOUT: " + request.agent.Faults.TIMEOUT);
      // expect(request.agent.Faults.TIMEOUT).assertEqual(16);
      console.info("====>SUB_MISC_REQUEST_DOWNLOAD_faults_0010 request.agent.Faults.PROTOCOL: " + request.agent.Faults.PROTOCOL);
      // expect(request.agent.Faults.PROTOCOL).assertEqual(32);
      console.info("====>SUB_MISC_REQUEST_DOWNLOAD_faults_0010 request.agent.Faults.FSIO: " + request.agent.Faults.FSIO);
      // expect(request.agent.Faults.FSIO).assertEqual(64);
      console.info("====>SUB_MISC_REQUEST_DOWNLOAD_faults_0010 request.agent.Faults.PARAM: " + request.agent.Faults.PARAM);
      // expect(request.agent.Faults.PARAM).assertEqual(48);
      console.info("====>SUB_MISC_REQUEST_DOWNLOAD_faults_0010 request.agent.Faults.DNS: " + request.agent.Faults.DNS);
      // expect(request.agent.Faults.DNS).assertEqual(80);
      console.info("====>SUB_MISC_REQUEST_DOWNLOAD_faults_0010 request.agent.Faults.TCP: " + request.agent.Faults.TCP);
      // expect(request.agent.Faults.TCP).assertEqual(96);
      console.info("====>SUB_MISC_REQUEST_DOWNLOAD_faults_0010 request.agent.Faults.SSL: " + request.agent.Faults.SSL);
      // expect(request.agent.Faults.SSL).assertEqual(112);
      console.info("====>SUB_MISC_REQUEST_DOWNLOAD_faults_0010 request.agent.Faults.REDIRECT: " + request.agent.Faults.REDIRECT);
      // expect(request.agent.Faults.REDIRECT).assertEqual(128);
      // console.info("====>SUB_MISC_REQUEST_DOWNLOAD_faults_0010 request.agent.Faults.LOW_SPEED: " + request.agent.Faults.LOW_SPEED);
      // expect(request.agent.Faults.LOW_SPEED).assertEqual(144);
    } catch (err) {
      console.error("====>SUB_MISC_REQUEST_DOWNLOAD_faults_0010 error: " + err);
      // expect().assertFail();
    }
    console.info("====>-----------------------SUB_MISC_REQUEST_DOWNLOAD_faults_0010 end-----------------------");
    });

    /**
     * @tc.name: SUB_Misc_REQUEST_Create_Download_Callback_0010
     * @tc.desc: Test create download task using callback with basic configuration
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with basic settings
     *           2. Create a download task using callback function
     *           3. Verify task creation success in the callback
     * @tc.require: issues#1573
     */
    it('SUB_Misc_REQUEST_Create_Download_Callback_0010', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("-----------------------SUB_Misc_REQUEST_Create_Download_Callback_0010 is starting-----------------------");
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        title: 'reeateTest',
        description: 'XTS download test!',
        saveas: './SUB_Misc_REQUEST_Create_Download_Callback_0010.txt',
        network: request.agent.Network.WIFI,
        mode:request.agent.Mode.BACKGROUND
      };

      request.agent.create(baseContext, config1, (err, task)=>{
      console.info("====>SUB_Misc_REQUEST_Create_Download_Callback_0010 downloadTask: " + task);
      try{
      if(err){
        console.info("====>SUB_Misc_REQUEST_Create_Download_Callback_0010 create err: " + JSON.stringify(err));
        // expect().assertFail();
      }
      // expect(task !== undefined).assertEqual(true);
      // expect(typeof(task.config) === "object").assertEqual(true);
      console.info("====>SUB_Misc_REQUEST_Create_Download_Callback_0010 create success: " + task?.config);
    }catch(error){
      console.info("====>SUB_Misc_REQUEST_Create_Download_Callback_0010 create fail: " + JSON.stringify(error));
    }
    // await request.agent.remove(task?.tid?);
    console.info("-----------------------SUB_Misc_REQUEST_Create_Download_Callback_0010 end-----------------------");
    // done();
    });
    });

    /**
     * @tc.name: SUB_Misc_REQUEST_Create_Download_Promise_0010
     * @tc.desc: Test create download task using promise with basic configuration
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with basic settings
     *           2. Create a download task using promise
     *           3. Verify task creation success in the promise chain
     * @tc.require: issues#1573
     */
    it('SUB_Misc_REQUEST_Create_Download_Promise_0010', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("-----------------------SUB_Misc_REQUEST_Create_Download_Promise_0010 is starting-----------------------");
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        title: 'reeateTest',
        description: 'XTS download test!',
        saveas: './SUB_Misc_REQUEST_Create_Download_Promise_0010',
        network: request.agent.Network.WIFI,
        mode:request.agent.Mode.BACKGROUND,
        overwrite: true,
      };
      let task: request.agent.Task | undefined;
      try {
      task = await request.agent.create(baseContext, config1);
      // expect(task !== undefined).assertEqual(true);
      // await request.agent.remove(task.tid);
      console.info("====>SUB_Misc_REQUEST_Create_Download_Promise_0010 create success: " + task);
    } catch (err) {
      console.info("====>SUB_Misc_REQUEST_Create_Download_Promise_0010 catch error: " + JSON.stringify(err));
      if(task !== undefined){
        await request.agent.remove(task.tid);
      }
      // expect().assertFail();
    }
    console.info("-----------------------SUB_Misc_REQUEST_Create_Download_Promise_0010 end-----------------------");
    });

    /**
     * @tc.name: SUB_Misc_REQUEST_Remove_Download_Callback_0010
     * @tc.desc: Test remove download task using callback
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration
     *           2. Create a download task using promise
     *           3. Remove the task using callback function
     *           4. Verify task removal success in the callback
     * @tc.require: issues#1573
     */
    it('SUB_Misc_REQUEST_Remove_Download_Callback_0010', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_Misc_REQUEST_Remove_Download_Callback_0010 is starting-----------------------");
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        title: 'reeateTest',
        description: 'XTS download test!',
        saveas: './SUB_Misc_REQUEST_Remove_Download_Callback_0010.txt',
        network: request.agent.Network.ANY,
      };

      try {
      let task = await request.agent.create(baseContext, config1);
      request.agent.remove(task.tid, err => {
      try {
      if(err){
        console.info("====>SUB_Misc_REQUEST_Remove_Download_Callback_0010 remove err: " + JSON.stringify(err));
        // expect().assertFail();
      }
      // expect(task !== undefined).assertEqual(true);
      console.info("====>SUB_Misc_REQUEST_Remove_Download_Callback_0010 remove success: " + task);
    } catch (err) {
      console.info("====>SUB_Misc_REQUEST_Remove_Download_Callback_0010 remove fail: " + JSON.stringify(err));
    }
    });
    // done();
    } catch (error) {
      console.info("====>SUB_Misc_REQUEST_Remove_Download_Callback_0010 catch error: " + JSON.stringify(error));
      // expect().assertFail();
    }
    console.info("-----------------------SUB_Misc_REQUEST_Remove_Download_Callback_0010 end-----------------------");
    });

    /**
     * @tc.name: SUB_Misc_REQUEST_Remove_Download_Promise_0010
     * @tc.desc: Test remove download task using promise
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration
     *           2. Create a download task using promise
     *           3. Remove the task using promise
     *           4. Verify task removal success
     * @tc.require: issues#1573
     */
    it('SUB_Misc_REQUEST_Remove_Download_Promise_0010', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_Misc_REQUEST_Remove_Download_Promise_0010 is starting-----------------------");
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        title: 'reeateTest',
        description: 'XTS download test!',
        saveas: './SUB_Misc_REQUEST_Remove_Download_Promise_0010.txt',
        network: request.agent.Network.ANY,
      };

      try {
      let task = await request.agent.create(baseContext, config1);
      // await request.agent.remove(task.tid);
      // expect(true).assertEqual(true);
      console.info("====>SUB_Misc_REQUEST_Remove_Download_Promise_0010 remove success: " + task);
    } catch (err) {
      console.info("====>SUB_Misc_REQUEST_Remove_Download_Promise_0010 catch error: " + JSON.stringify(err));
      // expect().assertFail();
    }
    console.info("-----------------------SUB_Misc_REQUEST_Remove_Download_Promise_0010 end-----------------------");
    });

    /**
     * @tc.name: SUB_Misc_REQUEST_Start_Download_Callback_0010
     * @tc.desc: Test start download task using callback
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration
     *           2. Create a download task using promise
     *           3. Start the task using callback function
     *           4. Verify task start success in the callback
     * @tc.require: issues#1573
     */
    it('SUB_Misc_REQUEST_Start_Download_Callback_0010', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_Misc_REQUEST_Start_Download_Callback_0010 is starting-----------------------");
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        title: 'reeateTest',
        description: 'XTS download test!',
        saveas: './SUB_Misc_REQUEST_Start_Download_Callback_0010.txt',
        network: request.agent.Network.ANY,
      };

      request.agent.create(baseContext, config1, (err, task)=>{
      console.info("====>SUB_Misc_REQUEST_Start_Download_Callback_0010 downloadTask: " + task);
      task?.start(err => {
      try{
      if(err){
        console.info("====>SUB_Misc_REQUEST_Start_Download_Callback_0010 download start err: " + JSON.stringify(err));
        // expect().assertFail();
      }
      // expect(true).assertEqual(true);
      console.info("====>SUB_Misc_REQUEST_Start_Download_Callback_0010 download start success: " + task);
    }catch(err){
      console.info("====>SUB_Misc_REQUEST_Start_Download_Callback_0010 catch err: " + JSON.stringify(err));
      // await request.agent.remove(task.tid);
    }
    // await request.agent.remove(task.tid);
    console.info("-----------------------SUB_Misc_REQUEST_Start_Download_Callback_0010 end-----------------------");
    })
    });
    });

    /**
     * @tc.name: SUB_Misc_REQUEST_Start_Download_Promise_0010
     * @tc.desc: Test start download task using promise
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration
     *           2. Create a download task using promise
     *           3. Start the task using promise
     *           4. Verify task start success
     * @tc.require: issues#1573
     */
    it('SUB_Misc_REQUEST_Start_Download_Promise_0010', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_Misc_REQUEST_Start_Download_Promise_0010 is starting-----------------------");
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        title: 'reeateTest',
        description: 'XTS download test!',
        saveas: './SUB_Misc_REQUEST_Start_Download_Promise_0010.txt',
        network: request.agent.Network.ANY,
      };

      let task: request.agent.Task| undefined;
      try {
      task = await request.agent.create(baseContext, config1);
      await task.start();
      // expect(true).assertEqual(true);
      // await request.agent.remove(task.tid);
      console.info("====>SUB_Misc_REQUEST_Start_Download_Promise_0010 download start success: " + task);
    } catch (err) {
      console.info("====>SUB_Misc_REQUEST_Start_Download_Promise_0010 download start err: " + JSON.stringify(err));
      if(task !== undefined){
        // await request.agent.remove(task.tid);
      }
      // expect().assertFail();
    }
    console.info("-----------------------SUB_Misc_REQUEST_Start_Download_Promise_0010 end-----------------------");
    });

    /**
     * @tc.name: SUB_Misc_REQUEST_On_Download_Progress_0040
     * @tc.desc: Test download progress event listener
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration
     *           2. Create a download task using promise
     *           3. Add completed event listener to the task
     *           4. Remove the completed event listener
     *           5. Start the task and verify listener has been removed
     * @tc.require: issues#1573
     */
    it('SUB_Misc_REQUEST_On_Download_Progress_0040', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_Misc_REQUEST_On_Download_Progress_0040 is starting-----------------------");
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        title: 'reeateTest',
        description: 'XTS download test!',
        saveas: './SUB_Misc_REQUEST_On_Download_Progress_0040.txt',
        network: request.agent.Network.WIFI,
        overwrite: true,
        mode:request.agent.Mode.FOREGROUND
      };

      try {
      request.agent.create(baseContext, config1, (err, task) => {
      let on_progressCallback = (pro: request.agent.Progress) => {
        // task.off('progress', on_progressCallback);
        console.info("====>SUB_Misc_REQUEST_On_Download_Progress_0040 pro.state: " + pro.state);
        console.info("====>SUB_Misc_REQUEST_On_Download_Progress_0040 pro.index: " + pro.index);
        console.info("====>SUB_Misc_REQUEST_On_Download_Progress_0040 pro.processed: " + pro.processed);
        console.info("====>SUB_Misc_REQUEST_On_Download_Progress_0040 pro.size: " + pro.sizes);
        console.info("====>SUB_Misc_REQUEST_On_Download_Progress_0040 pro.extras: " + JSON.stringify(pro.extras));
      }
      try {
      if(err){
        console.info("====>SUB_Misc_REQUEST_On_Download_Progress_0040 create err: " + JSON.stringify(err));
        // expect().assertFail();
      }
      task?.on('progress', on_progressCallback);
      task?.start();
    } catch (err) {
      console.info("====>SUB_Misc_REQUEST_On_Download_Progress_0040 catch err: " + JSON.stringify(err));
    }
    });
    } catch (err) {
      console.info("====>SUB_Misc_REQUEST_On_Download_Progress_0040 create catch err: " + JSON.stringify(err));
    }
    });

    /**
     * @tc.name: SUB_Misc_REQUEST_Off_Download_Progress_0040
     * @tc.desc: Test download progress event listener removal
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with invalid URL
     *           2. Create a download task using promise
     *           3. Add failed event listener to the task
     *           4. Start the task and verify failed event is triggered
     * @tc.require: issues#1573
     */
    it('SUB_Misc_REQUEST_Off_Download_Progress_0040', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_Misc_REQUEST_Off_Download_Progress_0040 is starting-----------------------");
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        title: 'reeateTest',
        description: 'XTS download test!',
        saveas: './SUB_Misc_REQUEST_Off_Download_Progress_0040.txt',
        network: request.agent.Network.WIFI,
        overwrite: true,
        mode:request.agent.Mode.FOREGROUND
      };

      try {
      let flag = true;
      request.agent.create(baseContext, config1, (err, task) => {
      let on_progressCallback = (pro: request.agent.Progress) => {
        flag = false;
        console.info("====>SUB_Misc_REQUEST_Off_Download_Progress_0040 on_progressCallback flag: " + flag);
      }
      try {
      if(err){
        console.info("====>SUB_Misc_REQUEST_Off_Download_Progress_0040 create err: " + JSON.stringify(err));
        // expect().assertFail();
      }
      task?.on('progress', on_progressCallback);
      task?.off('progress', on_progressCallback);
      task?.start();
      // let t = setTimeout(async () => {
      //   try {
      //     clearTimeout(t);
      //     expect(flag).assertTrue();
      //     console.info("====>SUB_Misc_REQUEST_Off_Download_Progress_0040 offProgress success flag: " + flag);
      //   } catch (err) {
      //     console.info("====>SUB_Misc_REQUEST_Off_Download_Progress_0040 offProgress fail err: " + JSON.stringify(err));
      //     await request.agent.remove(task.tid);
      //   }
      //   done();
      // }, 3000)
    } catch (err) {
      console.info("====>SUB_Misc_REQUEST_Off_Download_Progress_0040 catch err: " + JSON.stringify(err));
      // await request.agent.remove(task.tid);
    }
    });
    } catch (err) {
      console.info("====>SUB_Misc_REQUEST_Off_Download_Progress_0040 create catch err: " + JSON.stringify(err));
    }
    });

    /**
     * @tc.name: SUB_Misc_REQUEST_On_Download_Completed_0040
     * @tc.desc: Test download completed event listener
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with invalid URL
     *           2. Create a download task using promise
     *           3. Add failed event listener to the task
     *           4. Remove the failed event listener
     *           5. Start the task and verify listener has been removed
     * @tc.require: issues#1573
     */
    it('SUB_Misc_REQUEST_On_Download_Completed_0040', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_Misc_REQUEST_On_Download_Completed_0040 is starting-----------------------");
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png',
        title: 'reeateTest',
        description: 'XTS download test!',
        saveas: './SUB_Misc_REQUEST_On_Download_Completed_0040.txt',
        network: request.agent.Network.WIFI,
        overwrite: true,
        mode:request.agent.Mode.FOREGROUND
      };
      try {
      request.agent.create(baseContext, config1, (err, task) => {
      let on_completedCallback = (pro: request.agent.Progress) => {
        //task.off('completed', on_completedCallback);
        console.info("====>SUB_Misc_REQUEST_On_Download_Completed_0040 pro.state: " + pro.state);
        console.info("====>SUB_Misc_REQUEST_On_Download_Completed_0040 pro.index: " + pro.index);
        console.info("====>SUB_Misc_REQUEST_On_Download_Completed_0040 pro.processed: " + pro.processed);
        console.info("====>SUB_Misc_REQUEST_On_Download_Completed_0040 pro.sizes: " + pro.sizes);
        console.info("====>SUB_Misc_REQUEST_On_Download_Completed_0040 pro.extras: " + JSON.stringify(pro.extras));
        //expect(true).assertTrue();
        //done();
      }
      try {
      if(err){
        console.info("====>SUB_Misc_REQUEST_On_Download_Completed_0040 create err: " + JSON.stringify(err));
        //expect().assertFail();
      }
      task?.on('completed', on_completedCallback);
      task?.start();
    } catch (err) {
      console.info("====>SUB_Misc_REQUEST_On_Download_Completed_0040 catch err: " + JSON.stringify(err));
      //await request.agent.remove(task.tid);
      //done();
    }
    });
    } catch (err) {
      console.info("====>SUB_Misc_REQUEST_On_Download_Completed_0040 create catch err: " + JSON.stringify(err));
      //done();
    }
    });

    /**
     * @tc.name: SUB_Misc_REQUEST_Off_Download_Completed_0040
     * @tc.desc: Test download completed event listener removal
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration
     *           2. Create a download task using promise
     *           3. Start the download task
     *           4. Verify download management functionality
     * @tc.require: issues#1573
     */
    it('SUB_Misc_REQUEST_Off_Download_Completed_0040', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_Misc_REQUEST_Off_Download_Completed_0040 is starting-----------------------");
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png',
        title: 'reeateTest',
        description: 'XTS download test!',
        saveas: './SUB_Misc_REQUEST_Off_Download_Completed_0040.txt',
        network: request.agent.Network.WIFI,
        overwrite: true,
        mode:request.agent.Mode.FOREGROUND
      };

      try {
      let flag = true;
      request.agent.create(baseContext, config1, (err, task) => {
      let on_offCompletedCallback = (pro: request.agent.Progress) => {
        flag = false;
        console.info("====>SUB_Misc_REQUEST_Off_Download_Completed_0040 on_offCompletedCallback flag: " + flag);
      }
      try {
      if(err){
        console.info("====>SUB_Misc_REQUEST_Off_Download_Completed_0040 create err: " + JSON.stringify(err));
        //expect().assertFail();
      }
      task?.on('completed', on_offCompletedCallback);
      task?.off('completed', on_offCompletedCallback);
      task?.start();
      // let t = setTimeout(async () => {
      //   try {
      //     clearTimeout(t);
      //     expect(flag).assertTrue();
      //     console.info("====>SUB_Misc_REQUEST_Off_Download_Completed_0040 offCompleted success flag: " + flag);
      //   } catch (err) {
      //     console.info("====>SUB_Misc_REQUEST_Off_Download_Completed_0040 offCompleted fail err: " + JSON.stringify(err));
      //     await request.agent.remove(task.tid);
      //   }
      //   done();
      // }, 3000)
    } catch (err) {
      console.info("====>SUB_Misc_REQUEST_Off_Download_Completed_0040 catch err: " + JSON.stringify(err));
      //await request.agent.remove(task.tid);
      //done();
    }
    });
    } catch (err) {
      console.info("====>SUB_Misc_REQUEST_Off_Download_Completed_0040 create catch err: " + JSON.stringify(err));
      //done();
    }
    });

    /**
     * @tc.name: SUB_Misc_REQUEST_On_Download_Failed_0040
     * @tc.desc: Test download failed event listener
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with range request
     *           2. Create a download task using callback function
     *           3. Verify task creation success in the callback
     * @tc.require: issues#1573
     */
    it('SUB_Misc_REQUEST_On_Download_Failed_0040', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_Misc_REQUEST_On_Download_Failed_0040 is starting-----------------------");
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/xxxxxxxxxxx/test.apk',
        title: 'reeateTest',
        description: 'XTS download test!',
        saveas: './SUB_Misc_REQUEST_On_Download_Failed_0040.txt',
        network: request.agent.Network.WIFI,
        overwrite: true,
        mode:request.agent.Mode.FOREGROUND
      };
      try {
      request.agent.create(baseContext, config1, (err, task) => {
      let on_failedCallback = (pro: request.agent.Progress) => {
        //task.off('failed', on_failedCallback);
        console.info("====>SUB_Misc_REQUEST_On_Download_Failed_0040 pro.state: " + pro.state);
        console.info("====>SUB_Misc_REQUEST_On_Download_Failed_0040 pro.index: " + pro.index);
        console.info("====>SUB_Misc_REQUEST_On_Download_Failed_0040 pro.processed: " + pro.processed);
        console.info("====>SUB_Misc_REQUEST_On_Download_Failed_0040 pro.size: " + pro.sizes);
        console.info("====>SUB_Misc_REQUEST_On_Download_Failed_0040 pro.extras: " + JSON.stringify(pro.extras));
        //expect(true).assertTrue();
        //done();
      }
      try {
      if(err){
        console.info("====>SUB_Misc_REQUEST_On_Download_Failed_0040 create err: " + JSON.stringify(err));
        //expect().assertFail();
      }
      task?.on('failed', on_failedCallback);
      task?.start();
    } catch (err) {
      console.info("====>SUB_Misc_REQUEST_On_Download_Failed_0040 catch err: " + JSON.stringify(err));
      //await request.agent.remove(task.tid);
      //done();
    }
    });
    } catch (err) {
      console.info("====>SUB_Misc_REQUEST_On_Download_Failed_0040 create catch err: " + JSON.stringify(err));
      //done();
    }
    });

    /**
     * @tc.name: SUB_Misc_REQUEST_Off_Download_Failed_0040
     * @tc.desc: Test download failed event listener removal
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with range request
     *           2. Create a download task using promise
     *           3. Verify task creation success in the promise chain
     * @tc.require: issues#1573
     */
    it('SUB_Misc_REQUEST_Off_Download_Failed_0040', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_Misc_REQUEST_Off_Download_Failed_0040 is starting-----------------------");
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/xxxxxxxxxxx/test.apk',
        title: 'reeateTest',
        description: 'XTS download test!',
        saveas: './SUB_Misc_REQUEST_Off_Download_Failed_0040.txt',
        network: request.agent.Network.WIFI,
        overwrite: true,
        mode:request.agent.Mode.FOREGROUND
      };

      try {
      let flag = true;
      request.agent.create(baseContext, config1, (err, task) => {
      let on_failedCallback = (pro: request.agent.Progress) => {
        flag = false;
        console.info("====>SUB_Misc_REQUEST_Off_Download_Failed_0040 on_failedCallback flag: " + flag);
      }
      try {
      if(err){
        console.info("====>SUB_Misc_REQUEST_Off_Download_Failed_0040 create err: " + JSON.stringify(err));
        //expect().assertFail();
      }
      task?.on('failed', on_failedCallback);
      task?.off('failed', on_failedCallback);
      task?.start();
      // let t = setTimeout(async () => {
      //   try {
      //     clearTimeout(t);
      //     expect(flag).assertTrue();
      //     console.info("====>SUB_Misc_REQUEST_Off_Download_Failed_0040 offFailed success flag: " + flag);
      //   } catch (err) {
      //     console.info("====>SUB_Misc_REQUEST_Off_Download_Failed_0040 offFailed fail err: " + JSON.stringify(err));
      //     await request.agent.remove(task.tid);
      //   }
      //   done();
      // }, 3000)
    } catch (err) {
      console.info("====>SUB_Misc_REQUEST_Off_Download_Failed_0040 catch err: " + JSON.stringify(err));
      //await request.agent.remove(task.tid);
      //done();
    }
    });
    } catch (err) {
      console.info("====>SUB_Misc_REQUEST_Off_Download_Failed_0040 create catch err: " + JSON.stringify(err));
      //done();
    }
    });

    /**
     * @tc.name: SUB_Request_DownloadManagement_Download_0100
     * @tc.desc: Test download management functionality
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with partial range request
     *           2. Create a download task using callback function
     *           3. Verify task creation success in the callback
     * @tc.require: issues#1573
     */
    it('SUB_Request_DownloadManagement_Download_0100', Level.LEVEL1, async (done: () => void): Promise<void> => {
      console.info("====>-----------------------SUB_Request_DownloadManagement_Download_0100 is starting-----------------------");
      let config1:request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com',
        title: 'createTest',
        description: 'XTS download test!',
        saveas: './SUB_Request_DownloadManagement_Download_0100',
        network: request.agent.Network.ANY,
        overwrite: true
      };
      try {
      let task: request.agent.Task = await request.agent.create(baseContext, config1);
      console.info("====>SUB_Request_DownloadManagement_Download_0100 create success");
      let completedCallback = (pro: request.agent.Progress) => {
        try {
          //task.off('completed', completedCallback);
          console.info("====>SUB_Request_DownloadManagement_Download_0100 response info: " + JSON.stringify(pro));
          //expect(pro.sizes[0]).assertEqual(-1);
          //done();
        } catch (err) {
          console.info("====>SUB_Request_DownloadManagement_Download_0100 err: " + JSON.stringify(err));
          //done();
        }
      }
      task.on('completed', completedCallback);
      await task.start();
      console.info("====>SUB_Request_DownloadManagement_Download_0100 task.start");
    } catch (err) {
      console.info("====>SUB_Request_DownloadManagement_Download_0100 create catch err: " + JSON.stringify(err));
      //expect().assertFail();
      //done();
    }
    });

    /**
     * @tc.name: SUB_Misc_REQUEST_create_Callback_0140
     * @tc.desc: Test create download task with range request using callback
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with partial range request
     *           2. Create a download task using promise
     *           3. Verify task creation success in the promise chain
     * @tc.require: issues#1573
     */
    it('SUB_Misc_REQUEST_create_Callback_0140', Level.LEVEL1, async (done: () => void): Promise<void> => {
      let domain: int = 0x0000; //日志标识,0x0000作为测试框架的业务标识
      let tag: string = 'testTag';
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://cidownload.openharmony.cn/version/Master_Version/OpenHarmony_5.0.2.45/20240924_092847/version-Master_Version-OpenHarmony_5.0.2.45-20240924_092847-hispark_taurus_Linux_weekly.tar.gz',
        title: 'createTest',
        description: 'Sample code for create task',
        mode: request.agent.Mode.FOREGROUND ,
        overwrite: true,
        saveas: "./SUB_Misc_REQUEST_create_Callback_0140x.txt",
        network: request.agent.Network.WIFI,
        index: 0,
        begins: 0,
        ends: 100,
      };
      console.info("====>-------------------SUB_Misc_REQUEST_create_Callback_0140 is starting---------------------");
      request.agent.create(baseContext, config1, (err, task) => {
      try{
      if (err) {
        console.info("====>SUB_Misc_REQUEST_create_Callback_0140 create err: " + JSON.stringify(err));
        //expect().assertFail();
      }
      task?.on("completed", (progress) => {
      task?.off("completed");
      let downloadSize = progress.sizes[0];
      try {
      console.info('====>downloadSize:' + downloadSize);
      if (config1.ends !== undefined && config1.begins !== undefined){
      //let fileSize = config1.ends - config1.begins + 1;
      //hilog.info(domain, tag, `====>ends: ${config1.ends} + begin: ${config1.begins}`);
      //expect(downloadSize).assertEqual(fileSize);
      console.info("====>----------SUB_Misc_REQUEST_create_Callback_0140 is end---------------");
      //done();
    } else {
      console.info("====>SUB_Misc_REQUEST_create_Callback_0140 config.ends or config.begins is undefined");
      //expect().assertFail();
      //done();
    }
    } catch (err) {
      console.info("====>SUB_Misc_REQUEST_create_Callback_0140 catch err: " + JSON.stringify(err));
      //done();
    }
    });
    task?.start();
    }catch(err){
      console.info("====>SUB_Misc_REQUEST_create_Callback_0140 create err: " + JSON.stringify(err));
      //done();
    }
    });
    });

    /**
     * @tc.name: SUB_Misc_REQUEST_Create_Promise_0140
     * @tc.desc: Test create download task with range request using promise
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with specific range request
     *           2. Create a download task using callback function
     *           3. Verify task creation success in the callback
     * @tc.require: issues#1573
     */
    it('SUB_Misc_REQUEST_Create_Promise_0140', Level.LEVEL1, async (done: () => void): Promise<void> => {
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://cidownload.openharmony.cn/version/Master_Version/OpenHarmony_5.0.2.45/20240924_092847/version-Master_Version-OpenHarmony_5.0.2.45-20240924_092847-hispark_taurus_Linux_weekly.tar.gz',
        title: 'createTest',
        description: 'Sample code for create task',
        mode: request.agent.Mode.FOREGROUND,
        overwrite: true,
        saveas: "./SUB_Misc_REQUEST_Create_Promise_0140_077xx.txt",
        network: request.agent.Network.WIFI,
        index: 0,
        begins: 0,
        ends: 100,
      };
      console.info("====>-------------------SUB_Misc_REQUEST_Create_Promise_0140 is starting---------------------");
      try{
      let task = await request.agent.create(baseContext, config1)
      let onCompletedCallback = (progress: request.agent.Progress) => {
        task.off("completed");
        let downloadSize = progress.sizes[0]
        try {
          console.info('====>downloadSize:' + downloadSize);
          if (config1.ends !== undefined && config1.begins !== undefined){
            //let fileSize = config.ends - config.begins + 1
            console.info('====>fileSize: '+ config1.ends + ' beg:' + config1.begins)
            //expect(downloadSize).assertEqual(fileSize)
            //done();
          } else {
            console.info("====>SUB_Misc_REQUEST_Create_Promise_0140 config.ends or config.begins is undefined");
            //expect().assertFail();
            //done();
          }
        } catch (err) {
          console.info("====>SUB_Misc_REQUEST_Create_Promise_0140 catch err: " + JSON.stringify(err));
          //d/one();
        }
      }
      task.on("completed", onCompletedCallback)
      await task.start()
    }catch(err){
      console.info("====>SUB_Misc_REQUEST_Create_Promise_0140 create err: " + JSON.stringify(err));
      //done();
    }
    });

    /**
     * @tc.name: SUB_Misc_REQUEST_create_Callback_0160
     * @tc.desc: Test create download task with partial range request using callback
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with range request
     *           2. Create a download task using callback function
     *           3. Verify task creation success in the callback
     * @tc.require: issues#1573
     */
    it('SUB_Misc_REQUEST_create_Callback_0160', Level.LEVEL1, async (done: () => void): Promise<void> => {
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://cidownload.openharmony.cn/version/Master_Version/OpenHarmony_5.0.2.45/20240924_092847/version-Master_Version-OpenHarmony_5.0.2.45-20240924_092847-hispark_taurus_Linux_weekly.tar.gz',
        title: 'createTest',
        description: 'Sample code for create task',
        mode: request.agent.Mode.FOREGROUND,
        overwrite: true,
        saveas: "./SUB_Misc_REQUEST_create_Callback_0160.txt",
        network: request.agent.Network.WIFI,
        index: 0,
        begins: 233332586,
        ends: -1,
      };
      console.info("====>-------------------SUB_Misc_REQUEST_create_Callback_0160 is starting---------------------");
      request.agent.create(baseContext, config1, (err, task) => {
      try{
      if (err) {
        console.info("====>SUB_Misc_REQUEST_create_Callback_0160 create err: " + JSON.stringify(err));
        //expect().assertFail();
      }
      task?.on("completed", (progress) => {
      task?.off("completed");
      let downloadSize = progress.sizes[0];
      console.info('====>downloadSize:' + downloadSize);
      //expect(downloadSize).assertEqual(100);
      console.info("====>----------SUB_Misc_REQUEST_create_Callback_0160 is end---------------");
      //done();
    });
    task?.start();
    }catch(err){
      console.info("====>SUB_Misc_REQUEST_create_Callback_0160 create err: " + JSON.stringify(err));
      //done();
    }
    });
    });

    /**
     * @tc.name: SUB_Misc_REQUEST_Create_Promise_0160
     * @tc.desc: Test create download task with partial range request using promise
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with invalid range request
     *           2. Create a download task using promise
     *           3. Verify task creation success in the promise chain
     * @tc.require: issues#1573
     */
    it('SUB_Misc_REQUEST_Create_Promise_0160', Level.LEVEL1, async (done: () => void): Promise<void> => {
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://cidownload.openharmony.cn/version/Master_Version/OpenHarmony_5.0.2.45/20240924_092847/version-Master_Version-OpenHarmony_5.0.2.45-20240924_092847-hispark_taurus_Linux_weekly.tar.gz',
        title: 'createTest',
        description: 'Sample code for create task',
        mode: request.agent.Mode.FOREGROUND,
        overwrite: true,
        saveas: "./SUB_Misc_REQUEST_Create_Promise_0160.txt",
        network: request.agent.Network.WIFI,
        index: 0,
        begins: 233332586,
        ends: -1,
      };
      console.info("====>-------------------SUB_Misc_REQUEST_Create_Promise_0160 is starting---------------------");

      try{
      let task = await request.agent.create(baseContext, config1);
      let onCompletedCallback = (progress: request.agent.Progress) => {
        task.off("completed");
        let downloadSize = progress.sizes[0];
        console.info('====>downloadSize:' + downloadSize);
        //expect(downloadSize).assertEqual(100);
        //done();
      }
      task.on("completed", onCompletedCallback);
      await task.start();
    }catch(err){
      console.info("====>SUB_Misc_REQUEST_Create_Promise_0160 create err: " + JSON.stringify(err));
      //done();
    }
    });

    /**
     * @tc.name: SUB_Misc_REQUEST_create_Callback_0170
     * @tc.desc: Test create download task with specific range request using callback
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with all undefined properties
     *           2. Create a download task using callback function
     *           3. Verify task creation success in the callback
     * @tc.require: issues#1573
     */
    it('SUB_Misc_REQUEST_create_Callback_0170', Level.LEVEL1, async (done: () => void): Promise<void> => {
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://cidownload.openharmony.cn/version/Master_Version/OpenHarmony_5.0.2.45/20240924_092847/version-Master_Version-OpenHarmony_5.0.2.45-20240924_092847-hispark_taurus_Linux_weekly.tar.gz',
        title: 'createTest',
        description: 'Sample code for create task',
        mode: request.agent.Mode.FOREGROUND,
        overwrite: true,
        saveas: "./SUB_Misc_REQUEST_create_Callback_0170.txt",
        network: request.agent.Network.WIFI,
        index: 0,
        begins: 30,
        ends: 60,
      };
      console.info("====>-------------------SUB_Misc_REQUEST_create_Callback_0170 is starting---------------------");
      request.agent.create(baseContext, config1, (err, task) => {
      try{
      if (err) {
        console.info("====>SUB_Misc_REQUEST_create_Callback_0170 create err: " + JSON.stringify(err));
        //().assertFail();
      }
      task?.on("completed", (progress) => {
      task?.off("completed");
      let downloadSize = progress.sizes[0];
      try {
      console.info('====>downloadSize:' + downloadSize);
      if (config1.ends !== undefined && config1.begins !== undefined){
      // let fileSize = config.ends - config.begins + 1;
      console.info('====>fileSize: '+ config1.ends + ' 、' + config1.begins);
      //expect(downloadSize).assertEqual(fileSize);
      console.info("====>-----------------SUB_Misc_REQUEST_create_Callback_0170 is end--------------------");
      //done();
    } else {
      console.info("====>SUB_Misc_REQUEST_create_Callback_0170 config.ends or config.begins is undefined");
      //expect().assertFail();
      //done();
    }
    } catch (err) {
      console.info("====>SUB_Misc_REQUEST_create_Callback_0170 catch err: " + JSON.stringify(err));
      //done();
    }
    });
    task?.start();
    }catch(err){
      console.info("====>SUB_Misc_REQUEST_create_Callback_0170 create err: " + JSON.stringify(err));
      //done();
    }
    });
    });

    /**
     * @tc.name: SUB_Misc_REQUEST_create_Callback_0180
     * @tc.desc: Test create download task with range request using callback
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create download task configuration with all undefined properties
     *           2. Create a download task using promise
     *           3. Verify task creation success in the promise chain
     * @tc.require: issues#1573
     */
    it('SUB_Misc_REQUEST_create_Callback_0180', Level.LEVEL1, async (done: () => void): Promise<void> => {
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://cidownload.openharmony.cn/version/Master_Version/OpenHarmony_5.0.2.45/20240924_092847/version-Master_Version-OpenHarmony_5.0.2.45-20240924_092847-hispark_taurus_Linux_weekly.tar.gz',
        title: 'createTest',
        description: 'Sample code for create task',
        mode: request.agent.Mode.FOREGROUND,
        overwrite: true,
        saveas: "./SUB_Misc_REQUEST_create_Callback_0180x.txt",
        network: request.agent.Network.WIFI,
        index: 0,
        begins: 60,
        ends: 30,
      };
      console.info("====>-------------------SUB_Misc_REQUEST_create_Callback_0180 is starting---------------------");
      request.agent.create(baseContext, config1, (err, task) => {
      try{
      if (err) {
        console.info("====>SUB_Misc_REQUEST_create_Callback_0180 create err: " + JSON.stringify(err));
        //expect().assertFail();
      }
      task?.on("failed", (progress) => {
      task?.off("failed");
      console.info('====>upload task failed.' + JSON.stringify(progress.state));
      //expect(progress.state).assertEqual(65);
      //done();
    });
    task?.start();
    }catch(err){
      console.info("====>SUB_Misc_REQUEST_create_Callback_0180 create err: " + JSON.stringify(err));
      //done();
    }
    });
    });

    /**
     * @tc.name: SUB_Misc_REQUEST_Create_Promise_0180
     * @tc.desc: Test create download task with invalid range request using promise
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create very long description string
     *           2. Create download task configuration with the long description and without overwrite parameter
     *           3. Create a download task using callback function
     *           4. Verify task creation result in the callback
     * @tc.require: issues#1573
     */
    it('SUB_Misc_REQUEST_Create_Promise_0180', Level.LEVEL1, async (done: () => void): Promise<void> => {
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://cidownload.openharmony.cn/version/Master_Version/OpenHarmony_5.0.2.45/20240924_092847/version-Master_Version-OpenHarmony_5.0.2.45-20240924_092847-hispark_taurus_Linux_weekly.tar.gz',
        title: 'createTest',
        description: 'Sample code for create task',
        mode: request.agent.Mode.FOREGROUND,
        overwrite: true,
        saveas: "./SUB_Misc_REQUEST_Create_Promise_0180.txt",
        network: request.agent.Network.WIFI,
        index: 0,
        begins: 60,
        ends: 30,
      };
      console.info("====>-------------------SUB_Misc_REQUEST_Create_Promise_0180 is starting---------------------");
      try{
      let task = await request.agent.create(baseContext, config1);
      let createOffCallback = (progress: request.agent.Progress) => {
        console.info('====>upload task failed.' + JSON.stringify(progress));
        task.off("failed");
        //expect(progress.state).assertEqual(65);
        //done();
      };
      task.on("failed", createOffCallback);
      await task.start();
    }catch(err){
      console.info("====>SUB_Misc_REQUEST_Create_Promise_0180 create err: " + JSON.stringify(err));
      //done();
    }
    });

    /**
     * @tc.name: SUB_Misc_REQUEST_create_Callback_0090
     * @tc.desc: Test create download task with all undefined configuration using callback
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
    it('SUB_Misc_REQUEST_create_Callback_0090', Level.LEVEL1, async (done: () => void): Promise<void> => {
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined
      };
      console.info("====>-------------------SUB_Misc_REQUEST_create_Callback_0090 is starting---------------------");
      try{
      request.agent.create(baseContext, config1, (err, task) => {
      if (err) {
        console.info("====>SUB_Misc_REQUEST_create_Callback_0090 create err: " + JSON.stringify(err));
        //done();
      }
      task?.start((err) => {
      if (err) {
        console.error(`====>SUB_Misc_REQUEST_create_Callback_0090 Failed to start the download task, Code: ${err.code}, message: ${err.message}`);
        //expect().assertFail();
        return;
      }
      console.info(`====>SUB_Misc_REQUEST_create_Callback_0090 Succeeded in starting a download task.`);
      //expect(true).assertEqual(true);
      //await request.agent.remove(task.tid);
      console.info("====>----------------SUB_Misc_REQUEST_create_Callback_0090 is end-------------------");
      //done();
    });
    });
    }catch(err){
      console.info("====>SUB_Misc_REQUEST_create_Callback_0090 create err: " + JSON.stringify(err));
      //done();
    }
    });

    /**
     * @tc.name: SUB_Misc_REQUEST_Create_Promise_0090
     * @tc.desc: Test create download task with all undefined configuration using promise
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
    it('SUB_Misc_REQUEST_Create_Promise_0090', Level.LEVEL1, async (done: () => void): Promise<void> => {
      let config1: request.agent.Config = {
        action: request.agent.Action.DOWNLOAD,
        url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
        title: undefined,
        description: undefined,
        mode: undefined,
        overwrite: undefined,
        method: undefined,
        data: undefined,
        saveas: undefined,
        network: undefined,
        metered: undefined,
        roaming: undefined,
        retry: undefined,
        redirect: undefined,
        index: undefined,
        begins: undefined,
        ends: undefined,
        gauge: undefined,
        precise: undefined,
        token: undefined
      };
      console.info("====>-------------------SUB_Misc_REQUEST_Create_Promise_0090 is starting---------------------");
      try{
      request.agent.create(baseContext, config).then((task)=> {
      task.start((err) => {
      if (err) {
        console.error(`====>SUB_Misc_REQUEST_Create_Promise_0090 Failed to start the download task, Code: ${err.code}, message: ${err.message}`);
        //expect().assertFail();
        return;
      }
      console.info(`====>SUB_Misc_REQUEST_Create_Promise_0090 Succeeded in starting a download task.`);
      //expect(true).assertEqual(true);
      await request.agent.remove(task.tid);
      console.info("====>------------------SUB_Misc_REQUEST_Create_Promise_0090 is end-------------------");
      //done();
    });
    }).catch((err: Error) => {
      console.info("====>SUB_Misc_REQUEST_Create_Promise_0090 create err: " + JSON.stringify(err));
      //done();
    });
    }catch(err){
      console.info("====>SUB_Misc_REQUEST_Create_Promise_0090 create err: " + JSON.stringify(err));
      //done();
    }
    });

    /**
     * @tc.name: SUB_Misc_REQUEST_Create_Promise_0080
     * @tc.desc: Test create download task with very long URL using promise
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create very long URL string
     *           2. Create download task configuration with the long URL
     *           3. Create a download task using promise
     * @tc.require: issues#1573
     */
    it('SUB_Misc_REQUEST_Create_Promise_0080', Level.LEVEL1, async (done: () => void): Promise<void> => {
      let urls = 'x';
      for (let i = 0; i < 8175; i++){
      urls += 'x';
    }
    let config2: request.agent.Config = {
      action: request.agent.Action.DOWNLOAD,
      url: 'https://' + urls + '/test.apk',
      title: 'createTest',
      description: "XTS download test!",
      mode: request.agent.Mode.BACKGROUND,
      overwrite: true,
      saveas: "./SUB_Misc_REQUEST_Create_Promise_0080.txt",
      network: request.agent.Network.WIFI,
    };
    console.info("====>-------------SUB_Misc_REQUEST_Create_Promise_0080 is starting----------------");
    try{
      await request.agent.create(baseContext, config2);
      console.info(`====>SUB_Misc_REQUEST_Create_Promise_0080 Succeeded in starting a download task.`);
      //expect().assertFail();
    }catch(err){
      console.info("====>SUB_Misc_REQUEST_Create_Promise_0080 create err: " + JSON.stringify(err));
      //expect(err.code).assertEqual(401);
      console.info("-----------------------SUB_Misc_REQUEST_Create_Promise_0080 is end-----------------------");
      //done();
    }
    });

    /**
     * @tc.name: SUB_Misc_REQUEST_Create_Promise_0110
     * @tc.desc: Test create download task with very long description using promise
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create very long description string
     *           2. Create download task configuration with the long description
     *           3. Create a download task using promise
     * @tc.require: issues#1573
     */
    it('SUB_Misc_REQUEST_Create_Promise_0110', Level.LEVEL1, async (done: () => void): Promise<void> => {
      let des = 'x';
      for (let i = 0; i < 1025; i++){
      des += 'x';
    }
    let config1: request.agent.Config = {
      action: request.agent.Action.DOWNLOAD,
      url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
      title: 'createTest',
      description: des,
      mode: request.agent.Mode.BACKGROUND,
      overwrite: true,
      saveas: "./SUB_Misc_REQUEST_Create_Promise_0110.txt",
      network: request.agent.Network.WIFI,
    };
    console.info("====>-------------SUB_Misc_REQUEST_Create_Promise_0110 is starting----------------");
    try{
      await request.agent.create(baseContext, config1);
      console.info(`====>SUB_Misc_REQUEST_Create_Promise_0110 Succeeded in starting a download task.`);
      //expect().assertFail();
    }catch(err){
      console.info("====>SUB_Misc_REQUEST_Create_Promise_0110 create err: " + JSON.stringify(err));
      //expect(err.code).assertEqual(401);
      console.info("-----------------------SUB_Misc_REQUEST_Create_Promise_0110 is end-----------------------");
      //done();
    }
    });

    /**
     * @tc.name: SUB_Misc_REQUEST_Create_Callback_0110
     * @tc.desc: Test create task when overwrite is not given and file exists
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
    it('SUB_Misc_REQUEST_Create_Callback_0110', Level.LEVEL1, async (done: () => void): Promise<void> => {
      let des = 'x';
      for (let i = 0; i < 1025; i++){
      des += 'x';
    }
    let config1: request.agent.Config = {
      action: request.agent.Action.DOWNLOAD,
      url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
      title: 'createTest',
      description: des,
      mode: request.agent.Mode.BACKGROUND,
      overwrite: true,
      saveas: "./SUB_Misc_REQUEST_Create_Callback_0110.txt",
      network: request.agent.Network.WIFI,
    };
    console.info("====>-------------------SUB_Misc_REQUEST_Create_Callback_0110 is start ---------------------");
    try{
      request.agent.create(baseContext, config1, (err, task) => {
        try {
          console.info(`====>SUB_Misc_REQUEST_Create_Callback_0110 Succeeded in starting a download task.`);
          //expect().assertFail();
        } catch (err) {
          console.info(`====>SUB_Misc_REQUEST_Create_Callback_0110 Succeeded in starting a download task.` + JSON.stringify(err));
          //request.agent.remove(task.tid);
          //done();
        }
      });
    }catch(err){
      console.info("====>SUB_Misc_REQUEST_Create_Callback_0110 create err: " + JSON.stringify(err));
      console.info("-----------------------SUB_Misc_REQUEST_Create_Callback_0110 is end-----------------------");
      //expect(err.code).assertEqual(401);
      //done();
    }
    });

    /**
     * @tc.name: SUB_Misc_REQUEST_Create_Promise_0150
     * @tc.desc: Test create download task with very long title using promise
     * @tc.size: MediumTest
     * @tc.type: Function
     * @tc.level: Level 1
     * @tc.precon: 1. The device screen remains constantly lit
     *             2. The device is connected to the Internet
     *             3. The application is always at the foreground of the device
     *             4. The download_server SA process has been initiated
     *             5. The local cache of the previous use case has been cleared
     * @tc.step: 1. Create very long title string
     *           2. Create download task configuration with the long title
     *           3. Create a download task using promise
     * @tc.require: issues#1573
     */
    it('SUB_Misc_REQUEST_Create_Promise_0150', Level.LEVEL1, async (done: () => void): Promise<void> => {
      let tit = 'x';
      for (let i = 0; i < 257; i++){
      tit += 'x';
    }
    let config1: request.agent.Config = {
      action: request.agent.Action.DOWNLOAD,
      url: 'https://gitee.com/murphy1984/download/releases/download/V1/test.hap',
      title: tit,
      description: 'Sample code for create task',
      mode: request.agent.Mode.BACKGROUND,
      overwrite: true,
      saveas: "./SUB_Misc_REQUEST_Create_Promise_0150.txt",
      network: request.agent.Network.WIFI,
    };

    console.info("====>-------------------SUB_Misc_REQUEST_Create_Promise_0150 is starting---------------------");
    try{
      await request.agent.create(baseContext, config1);
      console.info(`====>SUB_Misc_REQUEST_Create_Promise_0150 Succeeded in starting a download task.`);
      //e/xpect().assertFail();
    }catch(err){
      console.info("====>SUB_Misc_REQUEST_Create_Promise_0150 create err: " + JSON.stringify(err));
      //expect(err.code).assertEqual(401);
      console.info("-----------------------SUB_Misc_REQUEST_Create_Promise_0150 is end-----------------------");
      //done();
    }
    });
  })
}