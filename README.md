# Request

##  Introduction

### Overall Introduction

The Request service provides third-party applications with the download and upload capabilities. With it, you can conveniently and efficiently use and manage download and upload services, including creating, removing, suspending, resuming, and querying download and upload tasks.

### Architecture Diagram

**Figure 1** Architecture of the download subsystem 
![](figures/download_architecture_en.png "Download Subsystem")

System applications: dependent system applications.

Third-party applications: dependent third-party applications.

Download API: provides the download function.

IDL: enables NAPIs to communicate with each other.

Download service: creates, processes, and completes a download task upon request by a client.

Data download capability: saves download records.

**Figure 2** Architecture of the upload subsystem 
![](figures/subsystem_architecture_en.png "Upload Subsystem")

Extended/Third-party application: a client that initiates an upload request.

JS APIs: provide the upload function for external systems.

Local APIs: provide the upload function for local files.

Upload task: created upon request by a client for processing the upload request and forwarding the upload response from the server.

Curl adaptation: encapsulates the third-party library libcurl.

File acquisition: obtains the file to be uploaded based on the file path specified in the upload request.

libcurl: functions as a third-party library.

DataAbility capability: used to obtain files in the DataAbility file protocol path.

## Directory Structure

```
/base/request/request
├── figures                                # Architecture diagrams
├── download/ability                       # Database management module of the download service
├── download/etc 	                       # Process configuration files of the download service
	 └──interfaces/kits/js/napi            # APIs that the download service provides for external systems
		 └──download_single                # NAPIs of the download service
├── download/sa_profile                    # System service configuration files of the download service
├── download/services                      # Implementation of the download service
├── download/utils                         # Constants defined for log printing and common events of the download service
├── upload/frameworks                      # Implementation of the upload service
├── upload/interfaces/kits                 # APIs that the upload service provides for external systems
│   ├── js                                 # JS API definition
│   └── napi                               # NAPIs of the upload service
└── upload/unitest                         # Unit test of the upload service
```

##  Description

###  Available APIs

**Table 1** Major APIs in Request

| API                                                          | Description                                                  |
| ------------------------------------------------------------ | ------------------------------------------------------------ |
| download(config: DownloadConfig, callback: AsyncCallback\<DownloadTask>): void | Downloads files. This API uses an asynchronous callback to return the **DownloadTask** instance. You can then use the instance to perform operations on the download task, for example, adding listeners for the **progress**, **complete**, and **fail** events, or suspending, resuming, removing, and querying the download task. |
| download(config: DownloadConfig): Promise\<DownloadTask>     | Downloads files. This API uses a promise to return the **DownloadTask** instance. You can then use the instance to perform operations on the download task, for example, adding listeners for the **progress**, **complete**, and **fail** events, or suspending, resuming, removing, and querying the download task. |
| upload(config: UploadConfig, callback: AsyncCallback\<UploadTask>): void | Uploads files. This API uses an asynchronous callback to return the **UploadTask** instance. You can then use the instance to perform operations on the upload task, for example, adding listeners for the **progress**, **headerReceive**, and **fail** events, or removing the upload task. |
| upload(config: UploadConfig): Promise\<UploadTask>           | Uploads files. This API uses a promise to return the **UploadTask** instance. You can then use the instance to perform operations on the upload task, for example, adding listeners for the **progress**, **headerReceive**, and **fail** events, or removing the upload task. |
| download(context: BaseContext,config: DownloadConfig, callback: AsyncCallback\<DownloadTask>): void | Downloads files. This API uses an asynchronous callback to return the **DownloadTask** instance. You can then use the instance to perform operations on the download task, for example, adding listeners for the **progress**, **complete**, and **fail** events, or suspending, resuming, removing, and querying the download task. |
| download(context: BaseContext,config: DownloadConfig): Promise\<DownloadTask> | Downloads files. This API uses a promise to return the **DownloadTask** instance. You can then use the instance to perform operations on the download task, for example, adding listeners for the **progress**, **complete**, and **fail** events, or suspending, resuming, removing, and querying the download task. |
| upload(context: BaseContext,config: UploadConfig, callback: AsyncCallback\<UploadTask>): void | Uploads files. This API uses an asynchronous callback to return the **UploadTask** instance. You can then use the instance to perform operations on the upload task, for example, adding listeners for the **progress**, **headerReceive**, and **fail** events, or removing the upload task. |
| upload(context: BaseContext,config: UploadConfig): Promise\<UploadTask> | Uploads files. This API uses a promise to return the **UploadTask** instance. You can then use the instance to perform operations on the upload task, for example, adding listeners for the **progress**, **headerReceive**, and **fail** events, or removing the upload task. |

**Table 2** Major APIs in DownloadTask

| API                                                          | Description                                                  |
| ------------------------------------------------------------ | ------------------------------------------------------------ |
| on(type: 'progress', callback:(receivedSize: number, totalSize: number) => void): void | Enables listening for download task progress changes. This API uses a callback to return the download progress. If this API is called multiple times, the callback passed in the last call is used. |
| on(type: 'complete' l 'pause' l 'remove', callback:() => void): void | Enables listening for download states: **complete**, **pause**, and **remove**. This API uses a callback to return the result. If this API is called multiple times, the callback passed in the last call is used. |
| on(type: 'fail', callback:(error: number) => void): void     | Enables listening for the download failure event. This API uses a callback to return the error code. If this API is called multiple times, the callback passed in the last call is used. |
| off(type: 'progress', callback?:(receivedSize: number, totalSize: number) => void): void | Disables listening for download task progress changes. This API uses a callback to return the result. |
| off(type: 'complete' l 'pause' l 'remove', callback?:() => void): void | Disables listening for download states: **complete**, **pause**, and **remove**. This API uses a callback to return the result. |
| off(type: 'fail', callback?:(error: number) => void): void   | Disables listening for the download failure event. This API uses a callback to return the result. |
| remove(): Promise\<boolean>                                  | Removes this download task. This API uses a promise to return the result. |
| remove(callback: AsyncCallback\<boolean>): void              | Removes this download task. This API uses an asynchronous callback to return the result. |
| pause(): Promise\<boolean>                                   | Pauses this download task. This API uses a promise to return the result. |
| pause(callback: AsyncCallback\<boolean>): void               | Pauses this download task. This API uses an asynchronous callback to return the result. |
| resume(): Promise\<boolean>                                  | Resumes this download task. This API uses a promise to return the result. |
| resume(callback: AsyncCallback\<boolean>): void              | Resumes this download task. This API uses an asynchronous callback to return the result. |
| query(): Promise\<DownloadInfo>                              | Queries download tasks. This API uses a promise to return the **DownloadInfo** object. |
| query(callback: AsyncCallback\<DownloadInfo>): void          | Queries download tasks. This API uses an asynchronous callback to return the **DownloadInfo** object. |
| queryMimeType(): Promise\<string>                            | Queries the MIME type of this download task. This API uses a promise to return the MIME type. |
| queryMimeType(callback: AsyncCallback\<string>): void        | Queries the MIME type of this download task. This API uses an asynchronous callback to return the MIME type. |

**Table 3** DownloadConfig

| Name          | Type    | Description                                        |
| ------------- | ------- | -------------------------------------------------- |
| url           | string  | Target URL.                                        |
| header        | Object  | Request header.                                    |
| enableMetered | boolean | Whether to allow downloads over a metered network. |
| enableRoaming | boolean | Whether to allow downloads during network roaming. |
| description   | string  | Description of the download task.                  |
| networkType   | number  | Network type under which downloads are allowed.    |
| filePath      | string  | File save path.                                    |
| title         | string  | Title of the download task.                        |
| background    | boolean | Download background.                               |

**Table 4** DownloadInfo

| Name               | Type   | Description                                   |
| ------------------ | ------ | --------------------------------------------- |
| description        | string | Description of the download task.             |
| downloadedBytes    | number | Number of downloaded bytes.                   |
| downloadId         | number | ID of the download task.                      |
| failedReason       | number | Reason why the download task failed.          |
| fileName           | string | Name of the file to save.                     |
| filePath           | string | File save path.                               |
| pausedReason       | number | Reason for suspending the download task.      |
| status             | number | Status of the download task.                  |
| targetURI          | string | URL of the download task.                     |
| downloadTitle      | string | Title of the download task.                   |
| downloadTotalBytes | number | Total number of bytes in the downloaded file. |

**Table 5** Major APIs in UploadTask

| API                                                          | Description                                                  |
| ------------------------------------------------------------ | ------------------------------------------------------------ |
| on(type: 'progress', callback:AsyncCallback <uploadedSize: number, totalSize: number> => void): void | Enables listening for upload task progress changes. This API uses an asynchronous callback to return the upload progress. If this API is called multiple times, the callback passed in the last call is used. |
| on(type: 'headerReceive', callback: AsyncCallback\<object> => void): void | Enables listening for the header receive event. This API uses an asynchronous callback to return the HTTP header response. If this API is called multiple times, the callback passed in the last call is used. |
| off(type: 'progress', callback:AsyncCallback<uploadedSize: number, totalSize: number>=> void): void | Disables listening for upload task progress changes. This API uses an asynchronous callback to return the upload progress. |
| off(type: 'headerReceive', callback:AsyncCallback\<object> => void): void | Disables listening for the header receive event. This API uses an asynchronous callback to return the HTTP header response. |
| remove(): Promise\<boolean>                                  | Removes this upload task. This API uses a promise to return the result. |
| remove(callback: AsyncCallback\<boolean>): void              | Removes this upload task. This API uses an asynchronous callback to return the result. |
| on(type: 'fail', callback: (err: number, result?: Array) => void): void | Enables listening for the upload failure event. This API uses a callback to return the result. |
| off(type: 'fail', callback?: (err: number, result?: Array) => void): void | Disables listening for the upload failure event. This API uses a callback to return the result. |
| on(type: 'complete', callback: (code: Array, result?: Array) => void): void | Enables listening for the upload completion event. This API uses a callback to return the result. |
| off(type: 'complete', callback?: (code: Array, result?: Array) => void): void | Disables listening for the upload completion event. This API uses a callback to return the result. |

**Table 6** UploadConfig

| Name       | Type                | Description                   |
| ---------- | ------------------- | ----------------------------- |
| url        | string              | Target URL.                   |
| header     | Object              | Request header.               |
| header     | Object              | Request header.               |
| files      | Array\<File>        | List of local files.          |
| data       | Array\<RequestData> | List of upload request forms. |
| background | boolean             | Upload background.            |

**Table 7** File

| Name     | Type   | Description                                                  |
| -------- | ------ | ------------------------------------------------------------ |
| filename | string | File name in the request header when multiple parts are submitted. |
| name     | string | Name of a form item when multiple parts are submitted. The default value is **file**. |
| uri      | string | Local path for storing files. Protocol types **dataability** and **internal** are supported. **internal** supports only temporary directories. Example: **dataability:///com.domainname.dataability.persondata/person/10/file.txt internal://cache/path/to/file.txt**. |
| type     | string | Type of the file content. By default, the type is obtained based on the extension of the file name or URI. |

**Table 8** RequestData

| Name  | Type   | Description              |
| ----- | ------ | ------------------------ |
| name  | string | Name of a form element.  |
| value | string | Value of a form element. |

###  How to Use

```js
// Import the module.
import request from '@ohos.request';
```

```js
// Download service API usage description
let downloadConfig = {
  url: 'http://mirror.bjtu.edu.cn/kernel/linux/libs/libc5/libc5.cvs.tar.gz',
  header: {},
  enableMetered: true,
  enableRoaming: true,
  description: 'download libc from mirror site',
  networkType: 1,
  filePath: '/data/libc5.cvs.tgz',
  title: 'download libc',
}
let downloadTask;

```

1. Obtain the **DownloadTask** instance.

```js
// Obtain the DownloadTask instance using a callback.
request.download(downloadConfig, (err, data) => {
    if (err) {
        console.error('Failed to request the download. Cause: ' + JSON.stringify(err));
        return;
    }
    console.info('Success to request the download.);
    downloadTask = data;
});

// Obtain the DownloadTask instance using a promise.
request.download(downloadConfig).then((data) => {
    console.info('Success to request the download.);
    downloadTask = data;
}).catch((err) => {
    console.error('Failed to request the download. Cause: ' + JSON.stringify(err));
})
// Obtain the DownloadTask instance using a callback.
request.download(BaseContext, downloadConfig, (err, data) => {
    if (err) {
        console.error('Failed to request the download. Cause: ' + JSON.stringify(err));
        return;
    }
    console.info('Success to request the download.);
    downloadTask = data;
});

// Obtain the DownloadTask instance using a promise.
request.download(BaseContext, downloadConfig).then((data) => {
    console.info('Success to request the download.);
    downloadTask = data;
}).catch((err) => {
    console.error('Failed to request the download. Cause: ' + JSON.stringify(err));
})
```

2. Enable listening for download task progress changes.

```js
// Enable listening for download task progress changes and obtain the download progress using a callback.
downloadTask.on('progress', (receivedSize, totalSize) => {
    console.info("download receivedSize :" + receivedSize + " totalSize :" + totalSize);
});
```

3. Enable listening for the complete event.

```js
// Enable listening for the complete event and obtain the result using a callback.
downloadTask.on('complete', () => {
    console.info("download task has been completed");
});
```

4. Enable listening for the pause event.

```js
// Enable listening for the pause event and obtain the result using a callback.
downloadTask.on('pause', () => {
    console.info("download task has been paused");
});
```

5. Enable listening for the remove event.

```js
// Enable listening for the remove event and obtain the result using a callback.
downloadTask.on('remove', () => {
    console.info("download task has been removed");
});
```

6. Enable listening for the download failure event.

```js
// Enable listening for the download failure event and obtain the error code using a callback.
downloadTask.on('fail', (error) => {
    console.info("download fail error:" + error);
});

```

7. Disable listening for download task progress changes.

```js
// Disable listening for download task progress changes and obtain the download progress using a callback.
downloadTask.off('progress', (receivedSize, totalSize) => {
    console.info("download receivedSize :" + receivedSize + " totalSize :" + totalSize);
});

```

8. Disable listening for the complete event.

```js
// Disable listening for the complete event and obtain the result using a callback.
downloadTask.off('complete', () => {
    console.info("delete complete notification");
});

```

9. Disable listening for the pause event.

```js
// Disable listening for the pause event and obtain the result using a callback.
downloadTask.off('pause', () => {
    console.info("delete pause notification");
});

```

10. Disable listening for the pause event.

```js
// Disable listening for the remove event and obtain the result using a callback.
downloadTask.off('remove', () => {
    console.info("delete remove notification");
});

```

11. Disable listening for the download failure event.

```js
// Disable listening for the download failure event and obtain the error code using a callback.
downloadTask.off('fail', (error) => {
    console.info("remove fail notification error:" + error);
});

```

12. Remove the download task.

```js
// Remove the download task and obtain the result using a promise.
downloadTask.remove().then((result) => {
    if (result) {
        console.info('Success to remove the download task.(promise) ');
    } else {
        console.error('Failed to remove the download task.(promise) ');
    }
}).catch((err) => {
    console.error('Failed to remove the download task.(promise) Cause: ' + JSON.stringify(err));
});

// Remove the download task and obtain the result using a callback.
downloadTask.remove((err, result) => {
    if (err) {
        console.error('Failed to remove the download task.(callback) Cause: ' + JSON.stringify(err));
        return;
    }
    if (result) {
        console.info('Success to remove the download task.(callback) ');
    } else {
        console.error('Failed to remove the download task.(callback) ');
    }
});

```

13. Pause the download task.

```js
// Pause the download task and obtain the result using a promise.
downloadTask.pause().then(() => {
	console.info('Success to pause the download task.(promise) ');
}).catch((err) => {
    console.error('Failed to pause the download task.(promise) Cause: ' + JSON.stringify(err));
});

// Pause the download task and obtain the result using a callback.
downloadTask.pause((err) => {
    if (err) {
        console.error('Failed to pause the download task.(callback) Cause: ' + JSON.stringify(err));
		return;
    }
	console.info('Success to pause the download task.(callback) ');
});

```

14. Resume the download task.

```js
// Resume the download task and obtain the result using a promise.
downloadTask.resume().then(() => {
	console.info('Success to resume the download task.(promise) ');
}).catch((err) => {
    console.error('Failed to resume the download task.(promise) Cause: ' + JSON.stringify(err));
});

// Resume the download task and obtain the result using a callback.
downloadTask.resume((err) => {
    if (err) {
        console.error('Failed to resume the download task.(callback) Cause: ' + JSON.stringify(err));
		return;
    }
	console.info('Success to resume the download task.(callback) ');
});

```

15. Query the download task.

```js
// Query the download task and obtain the result using a promise.
downloadTask.query().then((downloadInfo) => {
	console.info('Success to query the download task.(promise) ');
}).catch((err) => {
    console.error('Failed to query the download task.(promise) Cause: ' + JSON.stringify(err));
});

// Query the download task and obtain the result using a callback.
downloadTask.query((err, downloadInfo) => {
    if (err) {
        console.error('Failed to query the download task.(callback) Cause: ' + JSON.stringify(err));
		return;
    }
	console.info('Success to query the download task.(callback) ');
});

```

16. Query the MIME type of the download task.

```js
// Query the MIME type of the download task and obtain the result using a promise.
downloadTask.queryMimeType().then((mime) => {
	console.info('Success to queryMimeType the download task.(promise) MimeType ' + JSON.stringify(mime));
}).catch((err) => {
    console.error('Failed to queryMimeType the download task.(promise) Cause: ' + JSON.stringify(err));
});

// Query the MIME type of the download task and obtain the result using a callback.
downloadTask.queryMimeType((err, mime) => {
    if (err) {
        console.error('Failed to queryMimeType the download task.(callback) Cause: ' + JSON.stringify(err));
		return;
    }
	console.info('Success to queryMimeType the download task.(promise) MimeType ' + JSON.stringify(mime));
});

```

```js
// Upload service API usage description

```

1. Upload files.

```js
// Upload a group of files.
let url = 'http://192.168.2.211/files/';
let file1 = { filename: "test", name: "test", uri: "internal://cache/test.jpg", type: "jpg" };
let file2 = { filename: "test", name: "test", uri: "internal://cache/test.zip", type: "zip" };
let file3 = { filename: "test", name: "test", uri: "internal://cache/test.mp4", type: "mp4" };
let file4 = { filename: "test", name: "test", uri: "internal://cache/test.exe", type: "exe" };
let file5 = { filename: "test", name: "test", uri: "internal://cache/test.pdf", type: "pdf" };
let file6 = { filename: "test", name: "test", uri: "internal://cache/test.txt", type: "txt" };
let largeFile = { filename: "test", name: "test", uri: "internal://cache/testLarge.txt", type: "txt" };
let dataabilityFile = { filename: "test", name: "test",
  uri: "dataability://com.test.testApp/person/test.txt", type: "txt" };
let files = [file1, file2, file3, file4, file5, file6, largeFile, dataabilityFile];
let data = [{ name: "name123", value: "123" }];
let uploadTask;

```

2. Obtain the **UploadTask** instance.

```js
// Obtain the UploadTask instance using a callback.
request.upload({ url, header, "POST", files, data }, (err, data) => {
    if (err) {
        console.error('Failed to request the upload. Cause: ' + JSON.stringify(err));
        return;
    }
    console.info('Success to request the upload.);
    uploadTask = data;
});

// Obtain the UploadTask instance using a promise.
request.upload({ url, header, "POST", files, data }).then((data) => {
    console.info('Success to request the upload.);
    uploadTask = data;
}).catch((err) => {
    console.error('Failed to request the upload. Cause: ' + JSON.stringify(err));
})
// Obtain the UploadTask instance using a callback.
request.upload(BaseContext, { url, header, "POST", files, data }, (err, data) => {
    if (err) {
        console.error('Failed to request the upload. Cause: ' + JSON.stringify(err));
        return;
    }
    console.info('Success to request the upload.);
    uploadTask = data;
});

// Obtain the UploadTask instance using a promise.
request.upload(BaseContext, { url, header, "POST", files, data }).then((data) => {
    console.info('Success to request the upload.);
    uploadTask = data;
}).catch((err) => {
    console.error('Failed to request the upload. Cause: ' + JSON.stringify(err));
})

```

3. Enable listening for upload task progress changes.

```js
// Enable listening for upload task progress changes and obtain the result using a callback.
uploadTask.on('progress', (uploadedSize, totalSize) => {
    console.info("on progress upload uploadedSize :" + uploadedSize + " totalSize :" + totalSize);
});

```

4. Enable listening for the header receive event. 

```js
// Enable listening for the header receive event and obtain the result using a callback.
uploadTask.on('headerReceive', (headers) => {
    console.info("on headerReceive headers:" + JSON.stringify(headers));
});

```

5. Enable listening for the upload completion event.

```
// Enable listening for the upload completion event and obtain the result using a callback.
uploadTask.on('complete', (code,result) => {
    console.info("upload task has been completed"+code);
});

```

6. Enable listening for the upload failure event.

```js
// Enable listening for the upload failure event and obtain the result using a callback.
uploadTask.on('fail', (error,result) => {
    console.info("on fail error:" + error+"result is:"+result);
});

```

7. Disable listening for upload task progress changes.

```js
// Disable listening for upload task progress changes and obtain the result using a callback.
uploadTask.off('progress', (uploadedSize, totalSize) => {
    console.info("on progress upload uploadedSize :" + uploadedSize + " totalSize :" + totalSize);
});

```

8. Disable listening for the header receive event.

```js
// Disable listening for the header receive event and obtain the result using a callback.
uploadTask.off('headerReceive', (headers) => {
    console.info("on headerReceive headers:" + JSON.stringify(headers));
});

```

9. Disable listening for the upload completion event.

```js
// Disable listening for the upload completion event and obtain the result using a callback.
uploadTask.off('complete', (code,result) => {
    console.info("delete complete notification");
});

```

10. Disable listening for the upload failure event.

```js
// Disable listening for the upload failure event and obtain the result using a callback.
uploadTask.off('fail', (error,result) => {
    console.info("on fail error:" + error+"result is:"+result);
});

```

11. Remove the upload task.

```js
// Remove the upload task and obtain the result using a promise.
uploadTask.remove().then((result) => {
    if (result) {
        console.info('Success to remove the upload task.(promise) ');
    } else {
        console.error('Failed to remove the upload task.(promise) ');
    }
}).catch((err) => {
    console.error('Failed to remove the upload task.(promise) Cause: ' + JSON.stringify(err));
});

// Remove the upload task and obtain the result using a callback.
uploadTask.remove((err, result) => {
    if (err) {
        console.error('Failed to remove the upload task.(callback) Cause: ' + JSON.stringify(err));
        return;
    }
    if (result) {
        console.info('Success to remove the upload task.(callback) ');
    } else {
        console.error('Failed to remove the upload task.(callback) ');
    }
});

```

##  Repositories Involved

[request_request](https://gitee.com/openharmony/request_request)