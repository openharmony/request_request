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

use std::path::Path;

static AREA1: &str = "/data/storage/el1/base";
static AREA2: &str = "/data/storage/el2/base";
static AREA5: &str = "/data/storage/el5/base";

pub fn path_exists<P: AsRef<Path>>(path: P) -> bool {
    Path::new(path.as_ref()).exists()
}

pub fn belong_app_base(path: &str) -> bool {
    path.starts_with(AREA1) || path.starts_with(AREA2) || path.starts_with(AREA5)
}

pub fn check_standardized_path(path: &str) -> bool {
    if path.is_empty() || !path.starts_with('/') || path.ends_with('/') || path.contains("//") {
        return false;
    }
    // The application side has been standardized and should not receive
    // unstandardized paths.
    static NOT_ALLOWED: [&str; 11] = [
        r".", r".\", r"\.", r"..", r"\..", r"\.\.", r"\.\.\", r"\..\", r".\.", r".\.\", r"..\",
    ];
    for segment in path.split('/').filter(|s| !s.is_empty()) {
        if NOT_ALLOWED.contains(&segment) {
            return false;
        }
    }
    true
}

pub fn delete_base_for_list(v: &mut Vec<&str>) {
    v.retain(|s| s.len() > AREA1.len());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_split_whole_path() {
        assert!(check_standardized_path("/A/B/C"));
        assert!(!check_standardized_path("/A/B/C/../D"));
        assert!(!check_standardized_path("/A/B/../C/../D"));
        assert!(!check_standardized_path("/A/B/C/../../D"));
        assert!(!check_standardized_path("/A/B/C/../.."));
        assert!(!check_standardized_path("/A/B/../../C"));
        assert!(!check_standardized_path("/A/B/../../../C"));
        assert!(!check_standardized_path("/../B/C/D"));
        assert!(!check_standardized_path("/A/B/./C"));
        assert!(!check_standardized_path("/A/B/C/"));
        assert!(!check_standardized_path("A/B/C/"));
        assert!(!check_standardized_path("A/B/C"));
        assert!(!check_standardized_path("//A//B//C"));
        assert!(!check_standardized_path("/A/B//C"));
        assert!(!check_standardized_path("/"));
        assert!(!check_standardized_path(""));
        assert!(!check_standardized_path(r"/A/B/../C"));
        assert!(!check_standardized_path(r"/A/B/\.\./C"));
        assert!(!check_standardized_path(r"/A/B/\.\.\/C"));
        assert!(!check_standardized_path(r"/A/B/..\/C"));
        assert!(!check_standardized_path(r"/A/B/.\./C"));
        assert!(!check_standardized_path(r"/A/B/\../C"));
    }

    #[test]
    fn ut_delete_base() {
        let mut v = vec![
            "/data",
            "/data/storage",
            "/data/storage/el1",
            "/data/storage/el1/base",
            "/data/storage/el1/base/A",
            "/data/storage/el1/base/A/B",
        ];
        delete_base_for_list(&mut v);
        let v2 = vec!["/data/storage/el1/base/A", "/data/storage/el1/base/A/B"];
        assert_eq!(v, v2);
    }
}
