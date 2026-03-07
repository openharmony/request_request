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
mod ut_macros {
    use super::*;

    // @tc.name: ut_cfg_ylong_basic
    // @tc.desc: Test cfg_ylong macro expands correctly
    // @tc.precon: NA
    // @tc.step: 1. Use cfg_ylong macro to define a function
    //           2. Verify the function is conditionally compiled
    // @tc.expect: Macro expands without error
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 0
    #[test]
    fn ut_cfg_ylong_basic() {
        cfg_ylong! {
            fn test_ylong_function() -> i32 {
                42
            }
        }

        #[cfg(feature = "ylong")]
        {
            assert_eq!(test_ylong_function(), 42);
        }
    }

    // @tc.name: ut_cfg_ylong_struct
    // @tc.desc: Test cfg_ylong macro with struct definition
    // @tc.precon: NA
    // @tc.step: 1. Use cfg_ylong macro to define a struct
    //           2. Verify the struct is conditionally compiled
    // @tc.expect: Macro expands without error
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_cfg_ylong_struct() {
        cfg_ylong! {
            struct TestYlongStruct {
                value: i32,
            }
        }

        #[cfg(feature = "ylong")]
        {
            let instance = TestYlongStruct { value: 100 };
            assert_eq!(instance.value, 100);
        }
    }

    // @tc.name: ut_cfg_ylong_multiple_items
    // @tc.desc: Test cfg_ylong macro with multiple items
    // @tc.precon: NA
    // @tc.step: 1. Use cfg_ylong macro to define multiple items
    //           2. Verify all items are conditionally compiled
    // @tc.expect: Macro expands without error
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_cfg_ylong_multiple_items() {
        cfg_ylong! {
            fn test_ylong_fn_1() -> i32 { 1 }
            fn test_ylong_fn_2() -> i32 { 2 }
            struct TestYlongMulti { a: i32, b: i32 }
        }

        #[cfg(feature = "ylong")]
        {
            assert_eq!(test_ylong_fn_1(), 1);
            assert_eq!(test_ylong_fn_2(), 2);
            let multi = TestYlongMulti { a: 1, b: 2 };
            assert_eq!(multi.a + multi.b, 3);
        }
    }

    // @tc.name: ut_cfg_netstack_basic
    // @tc.desc: Test cfg_netstack macro expands correctly
    // @tc.precon: NA
    // @tc.step: 1. Use cfg_netstack macro to define a function
    //           2. Verify the function is conditionally compiled
    // @tc.expect: Macro expands without error
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 0
    #[test]
    fn ut_cfg_netstack_basic() {
        cfg_netstack! {
            fn test_netstack_function() -> i32 {
                24
            }
        }

        #[cfg(feature = "netstack")]
        {
            assert_eq!(test_netstack_function(), 24);
        }
    }

    // @tc.name: ut_cfg_netstack_struct
    // @tc.desc: Test cfg_netstack macro with struct definition
    // @tc.precon: NA
    // @tc.step: 1. Use cfg_netstack macro to define a struct
    //           2. Verify the struct is conditionally compiled
    // @tc.expect: Macro expands without error
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_cfg_netstack_struct() {
        cfg_netstack! {
            struct TestNetstackStruct {
                data: String,
            }
        }

        #[cfg(feature = "netstack")]
        {
            let instance = TestNetstackStruct {
                data: "test".to_string(),
            };
            assert_eq!(instance.data, "test");
        }
    }

    // @tc.name: ut_cfg_netstack_multiple_items
    // @tc.desc: Test cfg_netstack macro with multiple items
    // @tc.precon: NA
    // @tc.step: 1. Use cfg_netstack macro to define multiple items
    //           2. Verify all items are conditionally compiled
    // @tc.expect: Macro expands without error
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_cfg_netstack_multiple_items() {
        cfg_netstack! {
            fn test_netstack_fn_1() -> &'static str { "hello" }
            fn test_netstack_fn_2() -> &'static str { "world" }
            struct TestNetstackMulti { x: bool, y: bool }
        }

        #[cfg(feature = "netstack")]
        {
            assert_eq!(test_netstack_fn_1(), "hello");
            assert_eq!(test_netstack_fn_2(), "world");
            let multi = TestNetstackMulti { x: true, y: false };
            assert!(multi.x);
            assert!(!multi.y);
        }
    }

    // @tc.name: ut_cfg_macros_empty
    // @tc.desc: Test macros with empty input
    // @tc.precon: NA
    // @tc.step: 1. Use cfg_ylong and cfg_netstack macros with no items
    //           2. Verify they compile without error
    // @tc.expect: Empty macros expand without error
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 2
    #[test]
    fn ut_cfg_macros_empty() {
        cfg_ylong! {}
        cfg_netstack! {}
    }
}
