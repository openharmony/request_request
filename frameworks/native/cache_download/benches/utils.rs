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

//! Utility functions for benchmarking the cache download service.
//! 
//! This module provides helper functions for setting up test environments, including
//! logging initialization and mock HTTP server creation for performance benchmarking.

use std::io::{BufRead, BufReader, Lines, Write};
use std::net::{TcpListener, TcpStream};
use std::{fs, thread};

/// Initializes the logging system for benchmark tests.
///
/// Sets up the environment logger to write to a test log file with millisecond precision
/// timestamps. Configures the logger in test mode to avoid interference with benchmarking.
///
/// # Panics
///
/// Panics if the log file cannot be created or opened.
pub fn init() {
    // Create or truncate the test log file
    let file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("test.log")
        .unwrap();
    
    // Configure and initialize the logger
    let _ = env_logger::builder()
        .is_test(true)  // Set test mode to reduce logging overhead
        .format_timestamp_millis()  // Use millisecond precision for timestamps
        .target(env_logger::Target::Pipe(Box::new(file)))  // Redirect logs to file
        .try_init();
}

/// Creates a test HTTP server for benchmarking.
///
/// Starts a new TCP listener on localhost, automatically finding an available port,
/// and spawns a thread to handle incoming connections. The provided callback function
/// processes the request lines, and the server responds with a simple 200 OK response.
///
/// # Type Parameters
/// - `F`: Function to process the request lines from the client
///
/// # Parameters
/// - `f`: Callback function that processes the request lines
///
/// # Returns
/// The URL of the running test server
///
/// # Panics
///
/// Panics if the server cannot accept connections or process them.
pub fn test_server<F>(f: F) -> String
where
    F: FnOnce(Lines<BufReader<&mut TcpStream>>) + Send + 'static,
{
    let server = "127.0.0.1";
    let mut port = 7878;
    
    // Find an available port by incrementing from 7878
    let listener = loop {
        match TcpListener::bind((server, port)) {
            Ok(listener) => break listener,
            Err(_) => port += 1,
        }
    };
    
    // Spawn a thread to handle the connection
    thread::spawn(move || {
        let stream = listener.incoming().next().unwrap().unwrap();
        handle_connection(stream, f);
    });
    
    // Return the URL of the test server
    format!("http://{}:{}", server, port)
}

/// Handles an incoming TCP connection for the test server.
///
/// Creates a buffered reader from the TCP stream, extracts the request lines,
/// and passes them to the provided callback function. After processing, sends
/// a simple HTTP 200 OK response back to the client.
///
/// # Type Parameters
/// - `F`: Function to process the request lines
///
/// # Parameters
/// - `stream`: TCP stream representing the client connection
/// - `task_f`: Callback function that processes the request lines
///
/// # Panics
///
/// Panics if there is an error writing the response to the stream.
fn handle_connection<F>(mut stream: TcpStream, task_f: F)
where
    F: FnOnce(Lines<BufReader<&mut TcpStream>>),
{
    // Create a buffered reader for the request
    let buf_reader = BufReader::new(&mut stream);
    let lines = buf_reader.lines();
    
    // Process the request lines with the provided callback
    task_f(lines);
    
    // Send a simple HTTP 200 OK response
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
}
