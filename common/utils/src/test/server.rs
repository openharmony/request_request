// Copyright (C) 2024 Huawei Device Co., Ltd.
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

use std::io::{BufRead, BufReader, Lines, Write};
use std::net::{TcpListener, TcpStream};

pub fn test_server<F>(f: F) -> String
where
    F: FnOnce(Lines<BufReader<&mut TcpStream>>) + Send + 'static,
{
    let server = "127.0.0.1";
    let mut port = 7878;
    let listener = loop {
        match TcpListener::bind((server, port)) {
            Ok(listener) => break listener,
            Err(_) => port += 1,
        }
    };
    std::thread::spawn(move || {
        let stream = listener.incoming().next().unwrap().unwrap();
        handle_connection(stream, f);
    });
    format!("http://{}:{}", server, port)
}

fn handle_connection<F>(mut stream: TcpStream, task_f: F)
where
    F: FnOnce(Lines<BufReader<&mut TcpStream>>),
{
    let buf_reader = BufReader::new(&mut stream);
    let lines = buf_reader.lines();
    task_f(lines);
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
}
