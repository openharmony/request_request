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
    }
    task.on('completed', on_completedCallback);
    await task.start();
    hilog.info(domain, tag, "====>SUB_Misc_REQUEST_Download_overwrite_Static_0300 task.start");
  } catch (err: BusinessError) {
    hilog.info(domain, tag,
      "====>SUB_Misc_REQUEST_Download_overwrite_Static_0300 create catch err: " + JSON.stringify(err));
    expect().assertFail();
  }
});

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

  })
}