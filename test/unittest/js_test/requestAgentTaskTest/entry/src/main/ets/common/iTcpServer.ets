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
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or impligetProfileByAbilityEmptyAbilityCalled.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

import socket from '@ohos.net.socket';

let addr: socket.NetAddress = {
  address: '127.0.0.1', 
  port: 8078
}
let iServer: socket.TCPSocketServer = socket.constructTCPSocketServerInstance();

let connectCallback = (client: socket.TCPSocketConnection)=> {
  console.log('===>client on message');
  let socketInfo = async (value: socket.SocketMessageInfo) => {
    let messageRecv = '';
    for (let i: number = 0; i < value.message.byteLength; i++) {
      let uint8Array = new Uint8Array(value.message)
      let messages = uint8Array[i]
      let message = String.fromCharCode(messages);
      messageRecv += message;
    }
    console.log('===>on message message: ' + JSON.stringify(messageRecv));
    console.log('===>remoteInfo: ' + JSON.stringify(value.remoteInfo));

    let resp = 'HTTP/1.1 200 OK\r\ncontent-length:10\r\n\r\nupload ok!'
    let option: socket.TCPSendOptions = {
      data: resp
    }
    await client.send(option)
  };
  client.on('message', socketInfo);
}


export default class Server {
  async serverStart() {
    console.log('===>start server');
    await iServer.listen(addr);
    console.log('===>client server on connect');
    iServer.on('connect',connectCallback)
  }
}