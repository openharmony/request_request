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

// Test file content for download tests
const TEST_FILE_CONTENT = 'This is a test file for download testing.\nLine 2\nLine 3\n';
const LARGE_TEST_CONTENT = 'x'.repeat(1042003); // 1MB for large file tests

function parseHttpRequest(message: string): { method: string, path: string, headers: Map<string, string> } {
  const lines = message.split('\r\n');
  const requestLine = lines[0];
  const parts = requestLine.split(' ');
  const method = parts[0] || 'GET';
  const path = parts[1] || '/';
  
  const headers = new Map<string, string>();
  for (let i = 1; i < lines.length; i++) {
    const line = lines[i];
    if (line === '') break;
    const colonIndex = line.indexOf(':');
    if (colonIndex > 0) {
      const key = line.substring(0, colonIndex).trim().toLowerCase();
      const value = line.substring(colonIndex + 1).trim();
      headers.set(key, value);
    }
  }
  
  return { method, path, headers };
}

function buildHttpResponse(statusCode: number, statusText: string, contentType: string, body: string, extraHeaders: string = ''): string {
  return `HTTP/1.1 ${statusCode} ${statusText}\r\n` +
         `Content-Type: ${contentType}\r\n` +
         `Content-Length: ${body.length}\r\n` +
         extraHeaders +
         `Connection: close\r\n` +
         `\r\n` +
         body;
}

function handleRequest(request: { method: string, path: string, headers: Map<string, string> }): string {
  const path = request.path;
  const method = request.method;
  
  // 1. Handle 404 paths (error testing)
  if (path.includes('test123.txt') || path.includes('test1.txt') || 
      path.includes('test.jpg') || path.includes('bird123.png') || 
      path.includes('.gif') || path.endsWith('.pdf') ||
      path.includes('/unsupported')) {
    return buildHttpResponse(404, 'Not Found', 'text/plain', 'File not found');
  }
  
  // 2. Handle redirect paths
  if (path.includes('/redirect')) {
    return `HTTP/1.1 302 Found\r\n` +
           `Location: http://127.0.0.1:8078/test.txt\r\n` +
           `Content-Length: 0\r\n` +
           `Connection: close\r\n` +
           `\r\n`;
  }
  
  // 3. Handle large file paths with Range support
  if (path.includes('large_test') || path.includes('largefile.bin') || 
      path.includes('large-file.zip') || path.endsWith('.bin') || 
      path.includes('.zip') || path.includes('.mp4')) {
    const rangeHeader = request.headers.get('range');
    if (rangeHeader && rangeHeader.startsWith('bytes=')) {
      // Parse Range header (e.g., "bytes=0-1023")
      const range = rangeHeader.replace('bytes=', '').split('-');
      const start = parseInt(range[0]) || 0;
      const end = range[1] ? parseInt(range[1]) : LARGE_TEST_CONTENT.length - 1;
      const chunk = LARGE_TEST_CONTENT.substring(start, end + 1);
      
      return `HTTP/1.1 206 Partial Content\r\n` +
             `Content-Type: application/octet-stream\r\n` +
             `Content-Range: bytes ${start}-${end}/${LARGE_TEST_CONTENT.length}\r\n` +
             `Content-Length: ${chunk.length}\r\n` +
             `Connection: close\r\n` +
             `\r\n` +
             chunk;
    }
    // Return full content for large files
    return buildHttpResponse(200, 'OK', 'application/octet-stream', LARGE_TEST_CONTENT);
  }
  
  // 4. Handle mp3 files (test.mp3 for error testing should return 404)
  if (path.includes('test.mp3')) {
    return buildHttpResponse(404, 'Not Found', 'text/plain', 'File not found');
  }
  if (path.includes('.mp3')) {
    return buildHttpResponse(200, 'OK', 'audio/mpeg', 'MP3 test content data');
  }
  
  // 5. Handle JSON files
  if (path.includes('.json')) {
    const jsonContent = '{"status":"success","count":10}';
    return buildHttpResponse(200, 'OK', 'application/json', jsonContent);
  }
  
  // 6. Handle upload endpoints
  if (method === 'POST' || method === 'PUT') {
    if (path.includes('/upload') || path.includes('/backup') || path.includes('/api/')) {
      return buildHttpResponse(200, 'OK', 'application/json', '{"status":"success","id":"upload_123"}');
    }
    return buildHttpResponse(200, 'OK', 'text/plain', 'upload ok!');
  }
  
  // 7. Handle HEAD requests
  if (method === 'HEAD') {
    return `HTTP/1.1 200 OK\r\n` +
           `Content-Type: text/plain\r\n` +
           `Content-Length: ${TEST_FILE_CONTENT.length}\r\n` +
           `Connection: close\r\n` +
           `\r\n`;
  }
  
  // 8. Handle secure/protected paths
  if (path.includes('/secure/') || path.includes('/protected/')) {
    const authHeader = request.headers.get('authorization');
    if (!authHeader) {
      return buildHttpResponse(401, 'Unauthorized', 'text/plain', 'Authentication required');
    }
    return buildHttpResponse(200, 'OK', 'application/pdf', 'PDF content');
  }
  
  // 9. Default: return standard test file content
  return buildHttpResponse(200, 'OK', 'text/plain', TEST_FILE_CONTENT);
}

function sleep(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms));
}

export default class Server {
  private iServer: socket.TCPSocketServer | null = null;
  private isRunning: boolean = false;
  private serverPort: number = 0;
  
  private handleClient = (client: socket.TCPSocketConnection) => {
    console.log('===>client connected');
    let socketInfo = async (value: socket.SocketMessageInfo) => {
      let messageRecv = '';
      for (let i: number = 0; i < value.message.byteLength; i++) {
        let uint8Array = new Uint8Array(value.message)
        let messages = uint8Array[i]
        let message = String.fromCharCode(messages);
        messageRecv += message;
      }
      console.log('===>on message request: ' + messageRecv.substring(0, 200));

      const request = parseHttpRequest(messageRecv);
      
      // Check if this is testTimeout006 (large_test.zip for timeout testing)
      // It uses totalTimeout: 2, so we need to delay > 2 seconds
      const isTimeout006 = request.path.includes('large_test.zip') && 
                           !request.headers.get('range');
      
      if (isTimeout006) {
        // Delay 3 seconds for testTimeout006 to trigger totalTimeout fault
        console.log('===>delaying response 3s for testTimeout006');
        await sleep(3000);
      }
      
      const response = handleRequest(request);

      let option: socket.TCPSendOptions = {
        data: response
      }
      await client.send(option);
      client.close();
    };
    client.on('message', socketInfo);
  }
  
  /**
   * 启动服务器，尝试使用指定的起始端口，如果失败则自动递增端口号
   * @param startPort 起始端口号（默认 8078）
   * @returns 实际监听的端口号
   */
  async serverStart(startPort: number = 8078): Promise<number> {
    // 如果服务器已经在运行，返回当前端口
    if (this.isRunning && this.iServer) {
      console.log('===>server already running on port ' + this.serverPort);
      return this.serverPort;
    }
    
    // 尝试端口范围：从 startPort 到 startPort + 100
    for (let port = startPort; port < startPort + 100; port++) {
      // 每次尝试都创建新的服务器实例
      this.iServer = socket.constructTCPSocketServerInstance();
      
      let addr: socket.NetAddress = {
        address: '127.0.0.1', 
        port: port
      }
      
      try {
        console.log('===>trying to start server on 127.0.0.1:' + port);
        await this.iServer.listen(addr);
        console.log('===>server listening on port ' + port);
        this.iServer.on('connect', this.handleClient);
        this.isRunning = true;
        this.serverPort = port;
        return port;
      } catch (err) {
        console.log('===>port ' + port + ' failed: ' + JSON.stringify(err));
        // 端口失败，清理实例并继续尝试下一个
        this.iServer = null;
        continue;
      }
    }
    
    // 所有端口都失败了，清理并返回0（让测试用例决定如何处理）
    console.log('===>all ports from ' + startPort + ' to ' + (startPort + 99) + ' failed');
    this.iServer = null;
    this.isRunning = false;
    this.serverPort = 0;
    // 不抛出异常，返回0让调用者处理
    return 0;
  }
  
  /**
   * 获取当前服务器监听的端口
   * @returns 端口号，如果服务器未运行返回 0
   */
  getPort(): number {
    return this.serverPort;
  }
  
  /**
   * 获取服务器的基础 URL
   * @returns 如 "http://127.0.0.1:8080"，如果服务器未运行返回空字符串
   */
  getBaseUrl(): string {
    if (!this.isRunning || this.serverPort === 0) {
      return '';
    }
    return `http://127.0.0.1:${this.serverPort}`;
  }

  async serverClose() {
    if (this.isRunning && this.iServer) {
      console.log('===>close server on port ' + this.serverPort);
      try {
        await this.iServer.close();
      } catch (e) {
        console.log('===>close server error (ignored)');
      }
      this.isRunning = false;
      this.iServer = null;
      this.serverPort = 0;
    }
  }
}
