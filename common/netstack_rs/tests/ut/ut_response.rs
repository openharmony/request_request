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
mod ut_response {
    use super::*;
    use std::collections::HashMap;
    use cxx::SharedPtr;

    // Mock HttpClientResponse for testing
    struct MockHttpClientResponse {
        status_code: i32,
        headers: Vec<String>,
    }

    impl MockHttpClientResponse {
        fn new(status_code: i32, headers: Vec<String>) -> Self {
            Self {
                status_code,
                headers,
            }
        }

        fn GetResponseCode(&self) -> i32 {
            self.status_code
        }

        fn GetHeaders(&self) -> Vec<String> {
            self.headers.clone()
        }
    }

    // @tc.name: ut_response_code_from_int
    // @tc.desc: Test conversion from integer to ResponseCode
    // @tc.precon: NA
    // @tc.step: 1. Convert various integers to ResponseCode using try_into()
    // @tc.expect: Integers map to correct ResponseCode variants
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_response_code_from_int() {
        assert_eq!(ResponseCode::Ok, 200.try_into().unwrap());
        assert_eq!(ResponseCode::Created, 201.try_into().unwrap());
        assert_eq!(ResponseCode::NotFound, 404.try_into().unwrap());
        assert_eq!(ResponseCode::InternalError, 500.try_into().unwrap());
    }

    // @tc.name: ut_response_code_invalid
    // @tc.desc: Test conversion from invalid integer to ResponseCode
    // @tc.precon: NA
    // @tc.step: 1. Convert invalid integers to ResponseCode using try_into()
    // @tc.expect: Invalid integers return default ResponseCode::None
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 2
    #[test]
    fn ut_response_code_invalid() {
        assert_eq!(ResponseCode::None, 0.try_into().unwrap_or_default());
        assert_eq!(ResponseCode::None, 199.try_into().unwrap_or_default());
        assert_eq!(ResponseCode::None, 600.try_into().unwrap_or_default());
    }

    // @tc.name: ut_response_code_default
    // @tc.desc: Test default ResponseCode
    // @tc.precon: NA
    // @tc.step: 1. Get default ResponseCode
    // @tc.expect: Default is ResponseCode::None
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 0
    #[test]
    fn ut_response_code_default() {
        let default_code: ResponseCode = Default::default();
        assert_eq!(default_code, ResponseCode::None);
    }

    // @tc.name: ut_response_headers
    // @tc.desc: Test parsing headers from response
    // @tc.precon: NA
    // @tc.step: 1. Create mock response with headers
    // 2. Call headers() method
    // @tc.expect: Headers are parsed into HashMap with lowercase keys
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_response_headers() {
        let mock_headers = vec![
            "Content-Type".to_string(), "application/json".to_string(),
            "Server".to_string(), "TestServer".to_string(),
        ];
        let mock_response = MockHttpClientResponse::new(200, mock_headers);
        let response = Response::from_ffi(&mock_response);
        let headers = response.headers();

        assert_eq!(headers.get("content-type"), Some(&"application/json".to_string()));
        assert_eq!(headers.get("server"), Some(&"TestServer".to_string()));
        assert_eq!(headers.len(), 2);
    }

    // @tc.name: ut_response_headers_malformed
    // @tc.desc: Test parsing malformed headers
    // @tc.precon: NA
    // @tc.step: 1. Create mock response with odd number of header entries
    // 2. Call headers() method
    // @tc.expect: Headers are parsed correctly ignoring last unpaired entry
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 2
    #[test]
    fn ut_response_headers_malformed() {
        let mock_headers = vec![
            "Content-Type".to_string(), "application/json".to_string(),
            "Server".to_string(), // Unpaired header key
        ];
        let mock_response = MockHttpClientResponse::new(200, mock_headers);
        let response = Response::from_ffi(&mock_response);
        let headers = response.headers();

        assert_eq!(headers.len(), 1);
        assert_eq!(headers.get("content-type"), Some(&"application/json".to_string()));
    }

    // @tc.name: ut_response_status
    // @tc.desc: Test getting response status code
    // @tc.precon: NA
    // @tc.step: 1. Create mock responses with different status codes
    // 2. Call status() method
    // @tc.expect: Correct ResponseCode is returned
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_response_status() {
        let mock_response = MockHttpClientResponse::new(200, vec![]);
        let response = Response::from_ffi(&mock_response);
        assert_eq!(response.status(), ResponseCode::Ok);

        let mock_response = MockHttpClientResponse::new(404, vec![]);
        let response = Response::from_ffi(&mock_response);
        assert_eq!(response.status(), ResponseCode::NotFound);
    }

    // @tc.name: ut_response_status_invalid
    // @tc.desc: Test handling invalid status code
    // @tc.precon: NA
    // @tc.step: 1. Create mock response with invalid status code
    // 2. Call status() method
    // @tc.expect: Default ResponseCode::None is returned
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 2
    #[test]
    fn ut_response_status_invalid() {
        let mock_response = MockHttpClientResponse::new(999, vec![]);
        let response = Response::from_ffi(&mock_response);
        assert_eq!(response.status(), ResponseCode::None);
    }
}
