// Copyright (C) 2025 Huawei Device Co., Ltd.
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[cfg(test)]
mod ut_server {
    use super::*;
    use std::net::TcpStream;
    use std::io::{Read, Write};
    use std::sync::Arc;
    use std::sync::Barrier;

    // @tc.name: ut_test_server_basic_creation
    // @tc.desc: Test basic server creation and port allocation
    // @tc.precon: NA
    // @tc.step: 1. Call test_server with dummy handler
    // 2. Verify returned URL format
    // 3. Attempt connection to returned address
    // @tc.expect: Server starts successfully on available port
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 0
    #[test]
    fn ut_test_server_basic_creation() {
        let url = test_server(|_lines| {});
        assert!(url.starts_with("http://127.0.0.1:"));
        assert!(url.split(':').nth(2).unwrap().parse::<u16>().is_ok());

        // Verify we can connect
        let addr = url.strip_prefix("http://").unwrap();
        TcpStream::connect(addr).expect("Failed to connect to test server");
    }

    // @tc.name: ut_test_server_port_allocation
    // @tc.desc: Test server correctly allocates new port when initial port is occupied
    // @tc.precon: NA
    // @tc.step: 1. Create first server on default port
    // 2. Create second server expecting port increment
    // 3. Verify different ports are assigned
    // @tc.expect: Second server gets port 7879 when 7878 is occupied
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_test_server_port_allocation() {
        let url1 = test_server(|_lines| {});
        let port1: u16 = url1.split(':').nth(2).unwrap().parse().unwrap();

        let url2 = test_server(|_lines| {});
        let port2: u16 = url2.split(':').nth(2).unwrap().parse().unwrap();

        assert_ne!(port1, port2);
        // Verify port incremented (could be +1 or more if multiple ports are occupied)
        assert!(port2 > port1);
    }

    // @tc.name: ut_test_server_request_handling
    // @tc.desc: Test server correctly handles and processes requests
    // @tc.precon: NA
    // @tc.step: 1. Start server with handler that checks request lines
    // 2. Send HTTP request to server
    // 3. Verify request was processed
    // @tc.expect: Server receives and processes request correctly
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_test_server_request_handling() {
        let (tx, rx) = std::sync::mpsc::channel();

        let url = test_server(move |lines| {
            let request_lines: Vec<_> = lines.collect();
            tx.send(request_lines).unwrap();
        });

        // Send test request
        let addr = url.strip_prefix("http://").unwrap();
        let mut stream = TcpStream::connect(addr).unwrap();
        stream.write_all(b"GET /test HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n").unwrap();

        // Verify request was received
        let request_lines = rx.recv_timeout(std::time::Duration::from_secs(1)).unwrap();
        assert!(!request_lines.is_empty());
        assert!(request_lines[0].as_ref().unwrap().starts_with("GET /test"));
    }

    // @tc.name: ut_test_server_concurrent_connections
    // @tc.desc: Test server handles multiple concurrent connections
    // @tc.precon: NA
    // @tc.step: 1. Start server with handler
    // 2. Create 5 concurrent connections
    // 3. Verify all connections are handled
    // @tc.expect: All concurrent connections are processed successfully
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 3
    #[test]
    fn ut_test_server_concurrent_connections() {
        let url = test_server(|_lines| {});
        let addr = url.strip_prefix("http://").unwrap();
        let barrier = Arc::new(Barrier::new(5));
        let mut handles = vec![];

        for _ in 0..5 {
            let addr = addr.to_string();
            let barrier = Arc::clone(&barrier);

            handles.push(std::thread::spawn(move || {
                barrier.wait(); // Start all connections at once
                let mut stream = TcpStream::connect(&addr).unwrap();
                stream.write_all(b"GET / HTTP/1.1\r\n\r\n").unwrap();
                let mut buffer = [0; 1024];
                let bytes_read = stream.read(&mut buffer).unwrap();
                assert!(bytes_read > 0);
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    // @tc.name: ut_test_server_response_format
    // @tc.desc: Test server returns correctly formatted HTTP response
    // @tc.precon: NA
    // @tc.step: 1. Start server with basic handler
    // 2. Send request and read response
    // 3. Verify response format
    // @tc.expect: Server returns properly formatted 200 OK response
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_test_server_response_format() {
        let url = test_server(|_lines| {});
        let addr = url.strip_prefix("http://").unwrap();
        let mut stream = TcpStream::connect(addr).unwrap();
        stream.write_all(b"GET / HTTP/1.1\r\n\r\n").unwrap();

        let mut buffer = String::new();
        stream.read_to_string(&mut buffer).unwrap();

        assert!(buffer.starts_with("HTTP/1.1 200 OK"));
        assert!(buffer.contains("\r\n\r\n"));
    }
}