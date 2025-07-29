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

#[cfg(test)]
mod ut_error {
    use super::*;
    use std::io;

    // Mock struct implementing CommonError for testing
    struct MockCommonError {
        code: u16,
        msg: String,
    }

    impl CommonError for MockCommonError {
        fn code(&self) -> u16 {
            self.code
        }

        fn msg(&self) -> &str {
            &self.msg
        }
    }

    // @tc.name: ut_cache_download_error_new
    // @tc.desc: Test CacheDownloadError creation with proper initialization
    // @tc.precon: NA
    // @tc.step: 1. Create CacheDownloadError with sample values
    // 2. Verify all fields are initialized correctly
    // @tc.expect: CacheDownloadError instance created with correct code, message and kind
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 0
    #[test]
    fn ut_cache_download_error_new() {
        let error = CacheDownloadError {
            code: Some(404),
            message: "Not Found".to_string(),
            kind: ErrorKind::Http,
        };

        assert_eq!(error.code(), 404);
        assert_eq!(error.message(), "Not Found");
        assert_eq!(error.ffi_kind(), ErrorKind::Http as i32);
    }

    // @tc.name: ut_cache_download_error_code
    // @tc.desc: Test code() method returns correct value
    // @tc.precon: NA
    // @tc.step: 1. Create CacheDownloadError with Some code
    // 2. Call code() method
    // 3. Verify returned value matches expected code
    // @tc.expect: code() returns the stored error code
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_cache_download_error_code() {
        let error = CacheDownloadError {
            code: Some(500),
            message: "Internal Server Error".to_string(),
            kind: ErrorKind::Http,
        };

        assert_eq!(error.code(), 500);
    }

    // @tc.name: ut_cache_download_error_code_default
    // @tc.desc: Test code() method returns 0 when code is None
    // @tc.precon: NA
    // @tc.step: 1. Create CacheDownloadError with None code
    // 2. Call code() method
    // 3. Verify returned value is 0
    // @tc.expect: code() returns 0 for None code
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 2
    #[test]
    fn ut_cache_download_error_code_default() {
        let error = CacheDownloadError {
            code: None,
            message: "Unknown Error".to_string(),
            kind: ErrorKind::Http,
        };

        assert_eq!(error.code(), 0);
    }

    // @tc.name: ut_cache_download_error_message
    // @tc.desc: Test message() method returns correct string
    // @tc.precon: NA
    // @tc.step: 1. Create CacheDownloadError with sample message
    // 2. Call message() method
    // 3. Verify returned string matches expected message
    // @tc.expect: message() returns the stored error message
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_cache_download_error_message() {
        let error = CacheDownloadError {
            code: Some(400),
            message: "Bad Request".to_string(),
            kind: ErrorKind::Http,
        };

        assert_eq!(error.message(), "Bad Request");
    }

    // @tc.name: ut_cache_download_error_ffi_kind
    // @tc.desc: Test ffi_kind() method returns correct i32 value
    // @tc.precon: NA
    // @tc.step: 1. Create CacheDownloadError with Http kind
    // 2. Call ffi_kind() method
    // 3. Create CacheDownloadError with Io kind
    // 4. Call ffi_kind() method
    // 5. Verify returned values match expected i32 representations
    // @tc.expect: ffi_kind() returns 0 for Http and 1 for Io
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_cache_download_error_ffi_kind() {
        let http_error = CacheDownloadError {
            code: Some(404),
            message: "Not Found".to_string(),
            kind: ErrorKind::Http,
        };

        let io_error = CacheDownloadError {
            code: Some(1),
            message: "IO Error".to_string(),
            kind: ErrorKind::Io,
        };

        assert_eq!(http_error.ffi_kind(), 0);
        assert_eq!(io_error.ffi_kind(), 1);
    }

    // @tc.name: ut_cache_download_error_from_io_error
    // @tc.desc: Test From<io::Error> conversion
    // @tc.precon: NA
    // @tc.step: 1. Create io::Error with known code and message
    // 2. Convert to CacheDownloadError using From trait
    // 3. Verify all fields are correctly converted
    // @tc.expect: CacheDownloadError created with Io kind, matching code and message
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_cache_download_error_from_io_error() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "File not found");
        let cache_err: CacheDownloadError = io_err.into();

        assert_eq!(cache_err.kind, ErrorKind::Io);
        assert_eq!(cache_err.message(), "File not found");
        assert_eq!(
            cache_err.code(),
            io::Error::new(io::ErrorKind::NotFound, "")
                .raw_os_error()
                .unwrap_or(0)
        );
    }

    // @tc.name: ut_cache_download_error_from_common_error
    // @tc.desc: Test From<&E> conversion where E: CommonError
    // @tc.precon: NA
    // @tc.step: 1. Create MockCommonError with sample code and message
    // 2. Convert to CacheDownloadError using From trait
    // 3. Verify all fields are correctly converted
    // @tc.expect: CacheDownloadError created with Http kind, matching code and message
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_cache_download_error_from_common_error() {
        let common_err = MockCommonError {
            code: 403,
            msg: "Forbidden".to_string(),
        };

        let cache_err = CacheDownloadError::from(&common_err);

        assert_eq!(cache_err.kind, ErrorKind::Http);
        assert_eq!(cache_err.code(), 403);
        assert_eq!(cache_err.message(), "Forbidden");
    }
}
