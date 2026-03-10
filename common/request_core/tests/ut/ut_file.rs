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

use request_core::file::FileSpec;

// @tc.name: ut_file_spec_new
// @tc.desc: Test FileSpec creation with new()
// @tc.precon: NA
// @tc.step: 1. Create FileSpec using new()
//           2. Verify all fields have default values
// @tc.expect: All fields have empty/default values
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_file_spec_new() {
    let file_spec = FileSpec::new();
    
    assert_eq!(file_spec.name, "");
    assert_eq!(file_spec.path, "");
    assert_eq!(file_spec.file_name, "");
    assert_eq!(file_spec.mime_type, "");
    assert_eq!(file_spec.is_user_file, false);
    assert!(file_spec.fd.is_none());
}

// @tc.name: ut_file_spec_set_name
// @tc.desc: Test setting name field
// @tc.precon: NA
// @tc.step: 1. Create FileSpec
//           2. Set name field
//           3. Verify field is set correctly
// @tc.expect: name is set correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_file_spec_set_name() {
    let mut file_spec = FileSpec::new();
    file_spec.name = "profile_picture".to_string();
    
    assert_eq!(file_spec.name, "profile_picture");
}

// @tc.name: ut_file_spec_set_path
// @tc.desc: Test setting path field
// @tc.precon: NA
// @tc.step: 1. Create FileSpec
//           2. Set path field
//           3. Verify field is set correctly
// @tc.expect: path is set correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_file_spec_set_path() {
    let mut file_spec = FileSpec::new();
    file_spec.path = "/data/profile.jpg".to_string();
    
    assert_eq!(file_spec.path, "/data/profile.jpg");
}

// @tc.name: ut_file_spec_set_file_name
// @tc.desc: Test setting file_name field
// @tc.precon: NA
// @tc.step: 1. Create FileSpec
//           2. Set file_name field
//           3. Verify field is set correctly
// @tc.expect: file_name is set correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_file_spec_set_file_name() {
    let mut file_spec = FileSpec::new();
    file_spec.file_name = "profile.jpg".to_string();
    
    assert_eq!(file_spec.file_name, "profile.jpg");
}

// @tc.name: ut_file_spec_set_mime_type
// @tc.desc: Test setting mime_type field
// @tc.precon: NA
// @tc.step: 1. Create FileSpec
//           2. Set mime_type field
//           3. Verify field is set correctly
// @tc.expect: mime_type is set correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_file_spec_set_mime_type() {
    let mut file_spec = FileSpec::new();
    file_spec.mime_type = "image/jpeg".to_string();
    
    assert_eq!(file_spec.mime_type, "image/jpeg");
}

// @tc.name: ut_file_spec_set_is_user_file
// @tc.desc: Test setting is_user_file field
// @tc.precon: NA
// @tc.step: 1. Create FileSpec
//           2. Set is_user_file field
//           3. Verify field is set correctly
// @tc.expect: is_user_file is set correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_file_spec_set_is_user_file() {
    let mut file_spec = FileSpec::new();
    file_spec.is_user_file = true;
    
    assert_eq!(file_spec.is_user_file, true);
}

// @tc.name: ut_file_spec_set_fd
// @tc.desc: Test setting fd field
// @tc.precon: NA
// @tc.step: 1. Create FileSpec
//           2. Set fd field
//           3. Verify field is set correctly
// @tc.expect: fd is set correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_file_spec_set_fd() {
    let mut file_spec = FileSpec::new();
    file_spec.fd = Some(42);
    
    assert_eq!(file_spec.fd, Some(42));
}

// @tc.name: ut_file_spec_set_all_fields
// @tc.desc: Test setting all fields
// @tc.precon: NA
// @tc.step: 1. Create FileSpec
//           2. Set all fields
//           3. Verify all fields are set correctly
// @tc.expect: All fields are set correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_file_spec_set_all_fields() {
    let mut file_spec = FileSpec::new();
    file_spec.name = "document".to_string();
    file_spec.path = "/data/docs/report.pdf".to_string();
    file_spec.file_name = "report.pdf".to_string();
    file_spec.mime_type = "application/pdf".to_string();
    file_spec.is_user_file = true;
    file_spec.fd = Some(10);
    
    assert_eq!(file_spec.name, "document");
    assert_eq!(file_spec.path, "/data/docs/report.pdf");
    assert_eq!(file_spec.file_name, "report.pdf");
    assert_eq!(file_spec.mime_type, "application/pdf");
    assert_eq!(file_spec.is_user_file, true);
    assert_eq!(file_spec.fd, Some(10));
}

// @tc.name: ut_file_spec_clone
// @tc.desc: Test FileSpec clone
// @tc.precon: NA
// @tc.step: 1. Create FileSpec with values
//           2. Clone the FileSpec
//           3. Verify clone has same values
// @tc.expect: Clone has identical values
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_file_spec_clone() {
    let mut file_spec = FileSpec::new();
    file_spec.name = "test".to_string();
    file_spec.path = "/test/path".to_string();
    
    let cloned = file_spec.clone();
    
    assert_eq!(cloned.name, "test");
    assert_eq!(cloned.path, "/test/path");
}

// @tc.name: ut_file_spec_debug
// @tc.desc: Test FileSpec Debug trait
// @tc.precon: NA
// @tc.step: 1. Create FileSpec
//           2. Format with Debug trait
//           3. Verify output contains field names
// @tc.expect: Debug output is correct
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_file_spec_debug() {
    let file_spec = FileSpec::new();
    let debug_str = format!("{:?}", file_spec);
    
    assert!(debug_str.contains("name"));
    assert!(debug_str.contains("path"));
    assert!(debug_str.contains("file_name"));
    assert!(debug_str.contains("mime_type"));
}

// @tc.name: ut_file_spec_different_mime_types
// @tc.desc: Test FileSpec with different MIME types
// @tc.precon: NA
// @tc.step: 1. Create FileSpec with various MIME types
//           2. Verify each is set correctly
// @tc.expect: All MIME types work correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_file_spec_different_mime_types() {
    let mime_types = [
        "image/jpeg",
        "image/png",
        "application/pdf",
        "text/plain",
        "application/json",
        "video/mp4",
        "audio/mpeg",
    ];
    
    for mime_type in mime_types {
        let mut file_spec = FileSpec::new();
        file_spec.mime_type = mime_type.to_string();
        assert_eq!(file_spec.mime_type, mime_type);
    }
}

// @tc.name: ut_file_spec_unicode_values
// @tc.desc: Test FileSpec with unicode values
// @tc.precon: NA
// @tc.step: 1. Create FileSpec with unicode strings
//           2. Verify unicode is handled correctly
// @tc.expect: Unicode values are stored correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_file_spec_unicode_values() {
    let mut file_spec = FileSpec::new();
    file_spec.name = "文件名".to_string();
    file_spec.path = "/数据/文件.txt".to_string();
    file_spec.file_name = "测试文件.txt".to_string();
    
    assert_eq!(file_spec.name, "文件名");
    assert_eq!(file_spec.path, "/数据/文件.txt");
    assert_eq!(file_spec.file_name, "测试文件.txt");
}
