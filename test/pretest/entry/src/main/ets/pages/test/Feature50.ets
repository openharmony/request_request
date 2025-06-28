import common from '@ohos.app.ability.common';
import wifiManager from '@ohos.wifiManager';
import commonEvent from '@ohos.commonEventManager';
import { BusinessError, request, systemDateTime } from '@kit.BasicServicesKit';
import abilityAccessCtrl, { PermissionRequestResult, Permissions } from '@ohos.abilityAccessCtrl';
import bundleManager from '@ohos.bundle.bundleManager';
import fs from '@ohos.file.fs';
import photoAccessHelper from '@ohos.file.photoAccessHelper';
import picker from '@ohos.file.picker';
import dataShare from '@ohos.data.dataShare';

@Entry
@Component
struct feature50 {
  @State downloadNum: number = 0;
  @State message: string = "";
  @State url: string = '';
  @State seq: number = 0;
  @State proxyUrl: string = '';
  controller: TextAreaController = new TextAreaController()
  @State Url_400k: string = 'https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test400K.txt';
  @State taskLists: Array<request.agent.Task> = [];
  @State cost_time: number = 0;
  @State timeCard: string = "0";


  build() {
    Row() {
      Column() {
        Row() {
          TextInput({ text: this.url, placeholder: 'input your word...', controller: this.controller })
            .placeholderColor(Color.Grey)
            .placeholderFont({ size: 14, weight: 400 })
            .caretColor(Color.Blue)
            .width('95%')
            .height(40)
            .fontSize(14)
            .fontColor(Color.Black)
            .onChange((value: string) => {
              this.url = value
              console.info(`====>${this.url}`)
            })
        }

        Flex({ direction: FlexDirection.Column, alignItems: ItemAlign.Center }) {

          Row() {
            Button('检查权限')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                this.checkPermissions();
              })
            Button('显示权限窗口')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                this.showPermissions();
              })

            Button('传相册(选的第1张)')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                this.getImgPaths(1);
              })
          }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

          Row() {
            Button('传相册(多张)')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                this.getImgPaths(2);
              })
            Button('传相册(多张 + 用户沙箱文件)')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                this.getImgPaths(3);
              })
          }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

          TextInput({ text: this.proxyUrl, placeholder: 'input your word...', controller: this.controller })
            .placeholderColor(Color.Grey)
            .placeholderFont({ size: 14, weight: 400 })
            .caretColor(Color.Blue)
            .width('95%')
            .height(40)
            .fontSize(14)
            .fontColor(Color.Black)
            .onChange((value: string) => {
              this.proxyUrl = value
              console.info(`====>${this.proxyUrl}`)
            })

          Row() {
            Button('代理上传')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                this.proxyUpload();
              })
            Button('代理下载')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                this.proxyDownload();
              })
            Button('无代理下载')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                this.noProxyDownload();
              })
          }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

          Row() {
            Button('下载')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                this.DownloadFore();
              })
            Button('上传验证')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                await this.UploadLocalRange();
              })
          }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

          Row() {
            Button('证书锁定上传')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                this.pinsUpload();
              })
            Button('证书锁定下载')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                this.pinsDownload();
              })
          }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

          Row() {
            Button('断开网络')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                this.disableWifi();
              })

            Button('恢复网络')
              .fontSize(20)
              .fontWeight(FontWeight.Bold)
              .onClick(async () => {
                this.enableWifi();
              })

            Text(this.timeCard).fontSize(20)
          }.margin({ top: '1%' }).width('100%').align(Alignment.Center)

          TextArea({
            text: this.message,
            placeholder: 'console',
            controller: this.controller
          })
            .placeholderColor(Color.Grey)
            .placeholderFont({ size: 14, weight: 400 })
            .caretColor(Color.Blue)
            .width(400)
            .height('40%')
            .margin(20)
            .fontSize(14)
            .focusable(false)
            .fontColor(Color.Black)
            .onChange((value: string) => {
              this.message = value;
              this.controller.caretPosition(this.message.length)
            })
            .margin({ top: '5%' })
        }
      }
      .width('100%')
    }
    .height('100%')
  }

  completedCallback = (progress: request.agent.Progress) => {
    console.info('====>on_COMPLETE_callback:' + JSON.stringify(progress))
    this.message += `====>上传完成且状态码为${progress.state}\n`

  }
  failedCallback = (progress: request.agent.Progress) => {
    console.info('====>on_Failed_callback:' + JSON.stringify(progress))
    this.message += `====>失败failed progress.state: ${progress.state} \n`
  }
  createOnCallback = (progress: request.agent.Progress) => {
    console.info('已上传： ' + ((progress.processed / 1024)));
    this.message += `已上传： ${(progress.processed / 1024)} \n `;
  };

  async download() {
    console.info("===Download test start!");
    console.info("===Build download config...");

    let config: request.agent.Config = {
      action: request.agent.Action.DOWNLOAD,
      url: this.url,
      overwrite: true,
      network: request.agent.Network.WIFI,
      gauge: true,
    }

    console.info("Build download config finished!");

    try {
      let task: request.agent.Task = await request.agent.create(getContext(), config);

      task.on('completed', async (progress) => {
        this.downloadNum++;
        console.info(`Download status: ${progress.state}, download percent 100%`);
      })

      task.on('progress', async (progress) => {
        console.info(`Download status: ${progress.state}, downloaded ${progress.processed}`);
      })

      await task.start();
      console.info("Downloading started...");
    } catch (err) {
      console.error(`Download err: ${err.code}, ${err.message}`);
    }
  }

  async checkPermissions() {
    console.info("===/Request : 开始检查权限");
    let bundleFlags = bundleManager.BundleFlag.GET_BUNDLE_INFO_WITH_APPLICATION;
    let id = -1;
    try {
      let data: bundleManager.BundleInfo = await bundleManager.getBundleInfoForSelf(bundleFlags);
      id = data.appInfo.accessTokenId;
    } catch (e) {
      console.info("===/Request : 获取应用信息失败" + e);
    }
    console.info("===/Request : 应用 token id: " + id);
    let atManager: abilityAccessCtrl.AtManager = abilityAccessCtrl.createAtManager();
    let res: abilityAccessCtrl.GrantStatus = await atManager.checkAccessToken(id, "ohos.permission.READ_MEDIA");
    if (res == abilityAccessCtrl.GrantStatus.PERMISSION_GRANTED) {
      console.info("===/Request : 有权限: ");
      this.message = "有权限\n";
    } else {
      console.info("===/Request : 没有权限: ");
      this.message = "没有权限\n";
    }
  }

  // 这个接口对用户只能拉起一次，首次选择后后续不会拉起
  async showPermissions() {
    console.info("===/Request : 开始显示权限");
    let atManager: abilityAccessCtrl.AtManager = abilityAccessCtrl.createAtManager();
    // "ohos.permission.READ_DOCUMENT"
    let data: PermissionRequestResult =
      await atManager.requestPermissionsFromUser(getContext(this), ["ohos.permission.READ_MEDIA"]);
    let pers = data.permissions;
    let res = data.authResults;
    this.message = "Request : hasPermissions pers: " + pers + "\n";
    console.info("===/Request : hasPermissions pers: " + pers);
    this.message = "Request : hasPermissions res: " + res + "\n";
    console.info("===/Request : hasPermissions res: " + res);
  }

  async getImgPaths(type: number) {

    let arr: Array<string> = [];
    const photoSelectOptions = new picker.PhotoSelectOptions();
    photoSelectOptions.MIMEType = picker.PhotoViewMIMETypes.IMAGE_VIDEO_TYPE; // 过滤选择媒体文件类型为 IMAGE 和 VIDEO
    photoSelectOptions.maxSelectNumber = 10; // 选择媒体文件的最大数目

    const photoViewPicker = new picker.PhotoViewPicker();
    let photoSelectResult = await photoViewPicker.select(photoSelectOptions);
    for (let index = 0; index < photoSelectResult.photoUris.length; index++) {
      arr[index] = photoSelectResult.photoUris[index];
      console.info('===/Request : photoViewPicker.select to file succeed and URI is: ' + arr[index]);
    }

    if (type == 1) {
      await this.uploadApi10([arr[0]]);
    } else if (type == 2) {
      await this.uploadApi10(arr);
    } else if (type == 3) {
      // 获取应用文件路径
      let context = getContext(this) as common.UIAbilityContext;
      let cacheDir = context.cacheDir;

      // 新建一个本地应用文件
      let file = fs.openSync(cacheDir + '/app_file.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      fs.writeSync(file.fd, 'upload file test');

      arr.push("./app_file.txt");
      await this.uploadApi10(arr);
    }
    return;
  }

  async uploadApi10(paths: Array<string>) {
    let data: Array<request.agent.FormItem> = [];
    for (let i = 0; i < paths.length; i++) {
      const path = paths[i];
      let form: request.agent.FormItem = {
        name: "t" + i,
        value: {
          path: path,
          filename: "test" + i,
        }
      }
      data.push(form)
    }

    let config: request.agent.Config = {
      action: request.agent.Action.UPLOAD,
      url: this.url,
      mode: request.agent.Mode.FOREGROUND,
      data: data,
    };

    try {
      let task = await request.agent.create(getContext(this), config);

      console.error("===/Request : ---create task:" + JSON.stringify(config));

      task.on('failed', (progress: request.agent.Progress) => {
        console.info('====>on_failed_callback:' + JSON.stringify(progress))
        this.message += `====>failed progress.state: ${progress.state}\n`
      }
      );

      task.on('progress', (p) => {
        console.error("===/Request : ---progress:  " + p.state);
      }
      );

      task.on('completed', (p) => {
        console.error("===/Request : ---completed:  " + JSON.stringify(p));
      });
      await task.start();
    } catch (e) {
      console.error("===/Request : ---catch err:  " + e);
    }
  }

  async DownloadFore() {
    let config: request.agent.Config = {
      action: request.agent.Action.DOWNLOAD,
      url: this.url,
      mode: request.agent.Mode.FOREGROUND,
      gauge: true,
      overwrite: true,
      network: request.agent.Network.ANY,
    };

    let task: request.agent.Task = await request.agent.create(getContext(), config);
    console.info('===>config:' + JSON.stringify(task.config))
    task.on('progress', async (progress) => {
      console.info(`===/Request download status ${progress.state}, downloaded ${progress.processed}`);
    })
    task.on('completed', this.completedCallback)
    task.on('failed', this.failedCallback)
    try {
      await task.start();
      this.message = `====>start task\n`
    } catch (err) {
      this.message = `====>start task err: ${err.message}\n`
    }
    console.info("Downloading started");
  }

  async UploadLocalRange() {
    let data: Array<request.agent.FormItem> = [
      {
        name: "down10",
        value: [
          {
            filename: "1.tar.gz",
            path: "./on_response.txt"
          }]
      }
    ];

    let config: request.agent.Config = {
      action: request.agent.Action.UPLOAD,
      url: this.url,
      data: data,
      method: "POST",
      overwrite: false,
    }
    console.info('====>task.config:' + JSON.stringify(config));

    request.agent.create(getContext(), config).then((task: request.agent.Task) => {
      task.on('completed', this.completedCallback)
      task.on('failed', this.failedCallback)
      task.start((err: BusinessError) => {
        if (err) {
          console.error(`Failed to start the upload task, Code: ${err.code}  message: ${err.message}`);
          return;
        }
      });
      task.on('progress', this.createOnCallback);
      console.info(`Succeeded in creating a upload task. result: ${task.tid}`);
    }).catch((err: BusinessError) => {
      console.error(`Failed to create a upload task, Code: ${err.code}, message: ${err.message}`);
    });
  }

  async noProxyDownload() {
    let config: request.agent.Config = {
      action: request.agent.Action.DOWNLOAD,
      url: this.url,
      gauge: true,
      overwrite: true,
      network: request.agent.Network.WIFI,
    };

    let task: request.agent.Task = await request.agent.create(getContext(), config);
    task.on('progress', async (progress) => {
      console.info(`===/Request download status ${progress.state}, downloaded ${progress.processed}`);
    })
    task.on('completed', this.completedCallback)
    task.on('completed', () => {
      request.agent.remove(task.tid)
    })
    task.on('failed', this.failedCallback)
    try {
      await task.start();
    } catch (err) {
      this.message = `====>start task err: ${err.message}\n`
    }
    console.info("Downloading started");
  }

  async proxyDownload() {
    let config: request.agent.Config = {
      action: request.agent.Action.DOWNLOAD,
      url: this.url,
      gauge: true,
      overwrite: true,
      proxy: this.proxyUrl,
      network: request.agent.Network.WIFI,
    };

    let task: request.agent.Task = await request.agent.create(getContext(), config);
    task.on('progress', async (progress) => {
      console.info(`===/Request download status ${progress.state}, downloaded ${progress.processed}`);
    })
    task.on('completed', async (progress) => {
      console.info('====>on_COMPLETE_callback:' + JSON.stringify(progress))
      this.message += `====>下载完成且状态码为: ${progress.state}\n`
      request.agent.remove(task.tid)
    })
    task.on('failed', this.failedCallback)
    try {
      await task.start();
    } catch (err) {
      this.message = `====>start task err: ${err.message}\n`
    }
    console.info("Downloading started");
  }

  async proxyUpload() {
    let attachments: Array<request.agent.FormItem> = [{
      name: "test0",
      value: [
        {
          filename: "test0.txt",
          path: "./test0.txt",
        },
      ]
    }];
    let config: request.agent.Config = {
      action: request.agent.Action.UPLOAD,
      url: this.url,
      title: 'taskOnTest',
      mode: request.agent.Mode.FOREGROUND,
      overwrite: false,
      method: "POST",
      data: attachments,
      proxy: this.proxyUrl,
      saveas: "./"
    };
    console.info('====>task.config:' + JSON.stringify(config));
    request.agent.create(getContext(), config).then((task: request.agent.Task) => {
      task.on('completed', this.completedCallback)
      task.start((err: BusinessError) => {
        if (err) {
          console.error(`Failed to start the upload task, Code: ${err.code}  message: ${err.message}`);
          return;
        }
      });
      task.on('progress', this.createOnCallback);
      task.on('failed', this.failedCallback)
      console.info(`Succeeded in creating a upload task. result: ${task.tid}`);
    }).catch((err: BusinessError) => {
      console.error(`Failed to create a upload task, Code: ${err.code}, message: ${err.message}`);
    });
  }

  async pinsDownload() {
    let config: request.agent.Config = {
      action: request.agent.Action.DOWNLOAD,
      url: this.url + '/image.jpg',
      gauge: true,
      overwrite: true,
      network: request.agent.Network.WIFI,
    };

    let task: request.agent.Task = await request.agent.create(getContext(), config);
    this.message += `task create sucess;tid${task.tid} \n`  ;

    console.info('====>task.config:' + JSON.stringify(config));
    task.on('progress', async (progress) => {
      console.info(`===/Request download status ${progress.state}, downloaded ${progress.processed}`);
    })
    task.on('completed', async (progress) => {
      console.info('====>on_COMPLETE_callback:' + JSON.stringify(progress))
      this.message += `====>下载完成且状态码为: ${progress.state}\n`
      request.agent.remove(task.tid)
    })
    task.on('failed', this.failedCallback)
    // task.off('response', responsef);
    console.info("===>Downloading before");
    await task.start();
    console.info("===>Downloading started");
  }

  async pinsUpload() {
    let attachments: Array<request.agent.FormItem> = [{
      name: "test",
      value: [
        {
          filename: "image.jpg",
          path: "./image.jpg",
        }
      ]
    }];
    let config: request.agent.Config = {
      action: request.agent.Action.UPLOAD,
      url: this.url,
      title: 'taskOnTest',
      mode: request.agent.Mode.FOREGROUND,
      overwrite: false,
      method: "POST",
      data: attachments,
      saveas: "./"
    };
    console.info('====>task.config:' + JSON.stringify(config));
    request.agent.create(getContext(), config).then((task: request.agent.Task) => {
      task.on('completed', this.completedCallback)
      task.on('failed', this.failedCallback)
      task.start((err: BusinessError) => {
        if (err) {
          console.error(`Failed to start the upload task, Code: ${err.code}  message: ${err.message}`);
          return;
        }
      });
      task.on('progress', this.createOnCallback);
      console.info(`Succeeded in creating a upload task. result: ${task.tid}`);
      this.message += `task create sucess;tid${task.tid} \n`  ;

    }).catch((err: BusinessError) => {
      console.error(`Failed to create a upload task, Code: ${err.code}, message: ${err.message}`);
    });
  }

  async queryTask(taskId: string) {
    try {
      let task_info = await request.agent.query(taskId)
      console.info('====>searchTaskList:' + JSON.stringify(task_info));
      this.message += `====>query task uid: ${JSON.stringify(task_info.uid)}\n`
      this.message += `====>query task bundle: ${JSON.stringify(task_info.bundle)}\n`
    } catch (err) {
      this.message += `====>search task err: ${err.message}\n`
    }
  }

  async disableWifi() {
    try {
      wifiManager.disableWifi();
      let count = 0;
      let t = setInterval(() => {
        count += 1
        this.timeCard = count.toString()
        if (count >= 65) {
          this.timeCard = '服务是否退出'
          clearInterval(t)
        }
      }, 1000)
    } catch (err) {
      this.message += `====>disableWifi err: ${err.message}\n`;
    }
  }

  async enableWifi() {
    try {
      wifiManager.enableWifi();
      this.timeCard = '服务是否启动';
    } catch (err) {
      this.message += `====>enableWifi err: ${err.message}\n`;
    }
  }
}