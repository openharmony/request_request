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

use request_client::listen::ser::{Serialize, UdsSer};

// @tc.name: ut_uds_ser_new
// @tc.desc: Test UdsSer creation
// @tc.precon: NA
// @tc.step: 1. Create UdsSer with byte buffer
//           2. Verify creation succeeds
// @tc.expect: UdsSer is created successfully
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_uds_ser_new() {
    let data = [0u8; 8];
    let _ser = UdsSer::new(&data);
    assert!(true);
}

// @tc.name: ut_uds_ser_read_i64
// @tc.desc: Test UdsSer read i64
// @tc.precon: NA
// @tc.step: 1. Create UdsSer with i64 bytes
//           2. Read i64 value
//           3. Verify correct value is read
// @tc.expect: i64 value is read correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_uds_ser_read_i64() {
    let value: i64 = 1234567890123456789;
    let bytes = value.to_ne_bytes();
    let mut ser = UdsSer::new(&bytes);
    
    let result: i64 = ser.read();
    assert_eq!(result, value);
}

// @tc.name: ut_uds_ser_read_u64
// @tc.desc: Test UdsSer read u64
// @tc.precon: NA
// @tc.step: 1. Create UdsSer with u64 bytes
//           2. Read u64 value
//           3. Verify correct value is read
// @tc.expect: u64 value is read correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_uds_ser_read_u64() {
    let value: u64 = 9876543210987654321;
    let bytes = value.to_ne_bytes();
    let mut ser = UdsSer::new(&bytes);
    
    let result: u64 = ser.read();
    assert_eq!(result, value);
}

// @tc.name: ut_uds_ser_read_i32
// @tc.desc: Test UdsSer read i32
// @tc.precon: NA
// @tc.step: 1. Create UdsSer with i32 bytes
//           2. Read i32 value
//           3. Verify correct value is read
// @tc.expect: i32 value is read correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_uds_ser_read_i32() {
    let value: i32 = 123456789;
    let bytes = value.to_ne_bytes();
    let mut ser = UdsSer::new(&bytes);
    
    let result: i32 = ser.read();
    assert_eq!(result, value);
}

// @tc.name: ut_uds_ser_read_u32
// @tc.desc: Test UdsSer read u32
// @tc.precon: NA
// @tc.step: 1. Create UdsSer with u32 bytes
//           2. Read u32 value
//           3. Verify correct value is read
// @tc.expect: u32 value is read correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_uds_ser_read_u32() {
    let value: u32 = 987654321;
    let bytes = value.to_ne_bytes();
    let mut ser = UdsSer::new(&bytes);
    
    let result: u32 = ser.read();
    assert_eq!(result, value);
}

// @tc.name: ut_uds_ser_read_i16
// @tc.desc: Test UdsSer read i16
// @tc.precon: NA
// @tc.step: 1. Create UdsSer with i16 bytes
//           2. Read i16 value
//           3. Verify correct value is read
// @tc.expect: i16 value is read correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_uds_ser_read_i16() {
    let value: i16 = 12345;
    let bytes = value.to_ne_bytes();
    let mut ser = UdsSer::new(&bytes);
    
    let result: i16 = ser.read();
    assert_eq!(result, value);
}

// @tc.name: ut_uds_ser_read_string
// @tc.desc: Test UdsSer read String
// @tc.precon: NA
// @tc.step: 1. Create UdsSer with null-terminated string bytes
//           2. Read String value
//           3. Verify correct string is read
// @tc.expect: String value is read correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_uds_ser_read_string() {
    let value = "hello world";
    let mut bytes = value.as_bytes().to_vec();
    bytes.push(0);
    
    let mut ser = UdsSer::new(&bytes);
    let result: String = ser.read();
    
    assert_eq!(result, value);
}

// @tc.name: ut_uds_ser_read_string_empty
// @tc.desc: Test UdsSer read empty String
// @tc.precon: NA
// @tc.step: 1. Create UdsSer with only null terminator
//           2. Read String value
//           3. Verify empty string is read
// @tc.expect: Empty string is read correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_uds_ser_read_string_empty() {
    let bytes = [0u8];
    let mut ser = UdsSer::new(&bytes);
    
    let result: String = ser.read();
    assert_eq!(result, "");
}

// @tc.name: ut_uds_ser_read_string_unicode
// @tc.desc: Test UdsSer read unicode String
// @tc.precon: NA
// @tc.step: 1. Create UdsSer with unicode string bytes
//           2. Read String value
//           3. Verify unicode string is read correctly
// @tc.expect: Unicode string is read correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_uds_ser_read_string_unicode() {
    let value = "你好世界";
    let mut bytes = value.as_bytes().to_vec();
    bytes.push(0);
    
    let mut ser = UdsSer::new(&bytes);
    let result: String = ser.read();
    
    assert_eq!(result, value);
}

// @tc.name: ut_uds_ser_read_multiple_values
// @tc.desc: Test UdsSer read multiple values sequentially
// @tc.precon: NA
// @tc.step: 1. Create UdsSer with multiple values
//           2. Read values sequentially
//           3. Verify all values are read correctly
// @tc.expect: All values are read correctly in order
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_uds_ser_read_multiple_values() {
    let i32_val: i32 = 123;
    let u64_val: u64 = 456789;
    let i16_val: i16 = 78;
    
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&i32_val.to_ne_bytes());
    bytes.extend_from_slice(&u64_val.to_ne_bytes());
    bytes.extend_from_slice(&i16_val.to_ne_bytes());
    
    let mut ser = UdsSer::new(&bytes);
    
    let r1: i32 = ser.read();
    let r2: u64 = ser.read();
    let r3: i16 = ser.read();
    
    assert_eq!(r1, i32_val);
    assert_eq!(r2, u64_val);
    assert_eq!(r3, i16_val);
}

// @tc.name: ut_uds_ser_read_negative_i64
// @tc.desc: Test UdsSer read negative i64
// @tc.precon: NA
// @tc.step: 1. Create UdsSer with negative i64 bytes
//           2. Read i64 value
//           3. Verify correct negative value is read
// @tc.expect: Negative i64 value is read correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_uds_ser_read_negative_i64() {
    let value: i64 = -1234567890123456789;
    let bytes = value.to_ne_bytes();
    let mut ser = UdsSer::new(&bytes);
    
    let result: i64 = ser.read();
    assert_eq!(result, value);
}

// @tc.name: ut_uds_ser_read_negative_i32
// @tc.desc: Test UdsSer read negative i32
// @tc.precon: NA
// @tc.step: 1. Create UdsSer with negative i32 bytes
//           2. Read i32 value
//           3. Verify correct negative value is read
// @tc.expect: Negative i32 value is read correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_uds_ser_read_negative_i32() {
    let value: i32 = -123456789;
    let bytes = value.to_ne_bytes();
    let mut ser = UdsSer::new(&bytes);
    
    let result: i32 = ser.read();
    assert_eq!(result, value);
}

// @tc.name: ut_uds_ser_read_zero_values
// @tc.desc: Test UdsSer read zero values
// @tc.precon: NA
// @tc.step: 1. Create UdsSer with zero values
//           2. Read values
//           3. Verify zero values are read correctly
// @tc.expect: Zero values are read correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_uds_ser_read_zero_values() {
    let i64_zero: i64 = 0;
    let i32_zero: i32 = 0;
    let i16_zero: i16 = 0;
    
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&i64_zero.to_ne_bytes());
    bytes.extend_from_slice(&i32_zero.to_ne_bytes());
    bytes.extend_from_slice(&i16_zero.to_ne_bytes());
    
    let mut ser = UdsSer::new(&bytes);
    
    let r1: i64 = ser.read();
    let r2: i32 = ser.read();
    let r3: i16 = ser.read();
    
    assert_eq!(r1, 0);
    assert_eq!(r2, 0);
    assert_eq!(r3, 0);
}

// @tc.name: ut_uds_ser_read_max_values
// @tc.desc: Test UdsSer read maximum values
// @tc.precon: NA
// @tc.step: 1. Create UdsSer with maximum values
//           2. Read values
//           3. Verify maximum values are read correctly
// @tc.expect: Maximum values are read correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_uds_ser_read_max_values() {
    let i64_max: i64 = i64::MAX;
    let u64_max: u64 = u64::MAX;
    let i32_max: i32 = i32::MAX;
    let u32_max: u32 = u32::MAX;
    
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&i64_max.to_ne_bytes());
    bytes.extend_from_slice(&u64_max.to_ne_bytes());
    bytes.extend_from_slice(&i32_max.to_ne_bytes());
    bytes.extend_from_slice(&u32_max.to_ne_bytes());
    
    let mut ser = UdsSer::new(&bytes);
    
    let r1: i64 = ser.read();
    let r2: u64 = ser.read();
    let r3: i32 = ser.read();
    let r4: u32 = ser.read();
    
    assert_eq!(r1, i64_max);
    assert_eq!(r2, u64_max);
    assert_eq!(r3, i32_max);
    assert_eq!(r4, u32_max);
}

// @tc.name: ut_uds_ser_read_min_values
// @tc.desc: Test UdsSer read minimum values
// @tc.precon: NA
// @tc.step: 1. Create UdsSer with minimum values
//           2. Read values
//           3. Verify minimum values are read correctly
// @tc.expect: Minimum values are read correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_uds_ser_read_min_values() {
    let i64_min: i64 = i64::MIN;
    let i32_min: i32 = i32::MIN;
    let i16_min: i16 = i16::MIN;
    
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&i64_min.to_ne_bytes());
    bytes.extend_from_slice(&i32_min.to_ne_bytes());
    bytes.extend_from_slice(&i16_min.to_ne_bytes());
    
    let mut ser = UdsSer::new(&bytes);
    
    let r1: i64 = ser.read();
    let r2: i32 = ser.read();
    let r3: i16 = ser.read();
    
    assert_eq!(r1, i64_min);
    assert_eq!(r2, i32_min);
    assert_eq!(r3, i16_min);
}

// @tc.name: ut_uds_ser_read_string_with_special_chars
// @tc.desc: Test UdsSer read string with special characters
// @tc.precon: NA
// @tc.step: 1. Create UdsSer with special character string
//           2. Read String value
//           3. Verify string with special chars is read correctly
// @tc.expect: String with special characters is read correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_uds_ser_read_string_with_special_chars() {
    let value = "hello\tworld\ntest\r\n";
    let mut bytes = value.as_bytes().to_vec();
    bytes.push(0);
    
    let mut ser = UdsSer::new(&bytes);
    let result: String = ser.read();
    
    assert_eq!(result, value);
}
