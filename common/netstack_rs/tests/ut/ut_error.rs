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

    // @tc.name: ut_http_error_code_variants
    // @tc.desc: Test HttpErrorCode variants have correct numeric values
    // @tc.precon: NA
    // @tc.step: 1. Verify numeric values of selected HttpErrorCode variants
    // @tc.expect: Each variant has the expected numeric value
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_http_error_code_variants_001() {
        assert_eq!(HttpErrorCode::HttpNoneErr as i32, 0);
        assert_eq!(HttpErrorCode::HttpPermissionDeniedCode as i32, 201);
        assert_eq!(HttpErrorCode::HttpParseErrorCode as i32, 401);
        assert_eq!(HttpErrorCode::HttpErrorCodeBase as i32, 2300000);
        assert_eq!(HttpErrorCode::HttpCouldntResolveProxy as i32, 2300005);
        assert_eq!(HttpErrorCode::HttpUnknownOtherError as i32, 2300999);
    }

    // @tc.name: ut_http_error_code_equality
    // @tc.desc: Test HttpErrorCode equality comparisons
    // @tc.precon: NA
    // @tc.step: 1. Compare equivalent and non-equivalent error codes
    // @tc.expect: Equal codes return true, different codes return false
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_http_error_code_equality_001() {
        let code1 = HttpErrorCode::HttpPermissionDeniedCode;
        let code2 = HttpErrorCode::HttpPermissionDeniedCode;
        let code3 = HttpErrorCode::HttpParseErrorCode;

        assert_eq!(code1, code2);
        assert_ne!(code1, code3);
    }

    // @tc.name: ut_http_error_code_default
    // @tc.desc: Test HttpErrorCode default value
    // @tc.precon: NA
    // @tc.step: 1. Get default value of HttpErrorCode
    // @tc.expect: Default is HttpUnknownOtherError
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 0
    #[test]
    fn ut_http_error_code_default_001() {
        let default_code: HttpErrorCode = Default::default();
        assert_eq!(default_code, HttpErrorCode::HttpUnknownOtherError);
    }

    // @tc.name: ut_http_client_error_creation
    // @tc.desc: Test HttpClientError creation
    // @tc.precon: NA
    // @tc.step: 1. Create HttpClientError with specific code and message
    // 2. Verify code and message are set correctly
    // @tc.expect: Error contains the specified code and message
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_http_client_error_creation_001() {
        let code = HttpErrorCode::HttpPermissionDeniedCode;
        let msg = String::from("Test error message");
        let error = HttpClientError::new(code.clone(), msg.clone());

        assert_eq!(error.code(), &code);
        assert_eq!(error.msg(), &msg);
    }

    // @tc.name: ut_http_client_error_clone
    // @tc.desc: Test HttpClientError cloning
    // @tc.precon: NA
    // @tc.step: 1. Create error and clone it
    // 2. Verify clone has same code and message
    // @tc.expect: Cloned error is identical to original
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_http_client_error_clone_001() {
        let original = HttpClientError::new(
            HttpErrorCode::HttpParseErrorCode,
            String::from("Original message")
        );
        let cloned = original.clone();

        assert_eq!(original.code(), cloned.code());
        assert_eq!(original.msg(), cloned.msg());
    }

    // @tc.name: ut_http_error_code_edge_cases
    // @tc.desc: Test HttpErrorCode edge cases
    // @tc.precon: NA
    // @tc.step: 1. Test boundary values and special cases
    // @tc.expect: All edge cases handled correctly
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 2
    #[test]
    fn ut_http_error_code_edge_cases_001() {
        // Test minimum and maximum variants
        assert_eq!(HttpErrorCode::HttpNoneErr as i32, 0);
        assert_eq!(HttpErrorCode::HttpUnknownOtherError as i32, 2300999);

        // Test variants with and without explicit values
        assert_eq!(HttpErrorCode::HttpUnsupportedProtocol as i32, 2300001);
        assert_eq!(HttpErrorCode::HttpFailedInit as i32, 2300002);
    }
}
