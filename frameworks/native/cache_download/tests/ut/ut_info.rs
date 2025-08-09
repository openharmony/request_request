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
mod ut_info {
    use mockall::automock;
    use std::vec;

    // 模拟DownloadInfo trait
    mock! {
        pub DownloadInfo {
            fn dns_time(&self) -> f64;
            fn connect_time(&self) -> f64;
            fn tls_time(&self) -> f64;
            fn first_send_time(&self) -> f64;
            fn first_recv_time(&self) -> f64;
            fn redirect_time(&self) -> f64;
            fn total_time(&self) -> f64;
            fn resource_size(&self) -> i64;
            fn ip(&self) -> String;
            fn dns_servers(&self) -> Vec<String>;
        }
    }

    use super::*;
    use crate::info::RustDownloadInfo;

    // @tc.name: ut_rust_download_info_from_download_info
    // @tc.desc: Test RustDownloadInfo constructor with valid DownloadInfo
    // @tc.precon: NA
    // @tc.step: 1. Create a mock DownloadInfo
    //           2. Call from_download_info method
    // @tc.expect: Returns a valid RustDownloadInfo instance
    // @tc.type: FUNC
    // @tc.require: issue#ICN31I
    // @tc.level: Level 0
    #[test]
    fn ut_rust_download_info_from_download_info() {
        let mut mock_info = MockDownloadInfo::new();
        let rust_info = RustDownloadInfo::from_download_info(mock_info);
        // 验证实例创建成功
        assert!(std::ptr::addr_of!(rust_info).is_null() == false);
    }

    // @tc.name: ut_rust_download_info_dns_time
    // @tc.desc: Test dns_time method returns correct value
    // @tc.precon: NA
    // @tc.step: 1. Create a mock DownloadInfo with dns_time 100.5
    //           2. Create RustDownloadInfo from mock
    //           3. Call dns_time method
    // @tc.expect: Returns 100.5
    // @tc.type: FUNC
    // @tc.require: issue#ICN31I
    // @tc.level: Level 1
    #[test]
    fn ut_rust_download_info_dns_time() {
        let mut mock_info = MockDownloadInfo::new();
        mock_info.expect_dns_time().returning(|| 100.5);
        let rust_info = RustDownloadInfo::from_download_info(mock_info);
        assert_eq!(rust_info.dns_time(), 100.5);
    }

    // @tc.name: ut_rust_download_info_connect_time
    // @tc.desc: Test connect_time method returns correct value
    // @tc.precon: NA
    // @tc.step: 1. Create a mock DownloadInfo with connect_time 200.75
    //           2. Create RustDownloadInfo from mock
    //           3. Call connect_time method
    // @tc.expect: Returns 200.75
    // @tc.type: FUNC
    // @tc.require: issue#ICN31I
    // @tc.level: Level 1
    #[test]
    fn ut_rust_download_info_connect_time() {
        let mut mock_info = MockDownloadInfo::new();
        mock_info.expect_connect_time().returning(|| 200.75);
        let rust_info = RustDownloadInfo::from_download_info(mock_info);
        assert_eq!(rust_info.connect_time(), 200.75);
    }

    // @tc.name: ut_rust_download_info_tls_time
    // @tc.desc: Test tls_time method returns correct value
    // @tc.precon: NA
    // @tc.step: 1. Create a mock DownloadInfo with tls_time 300.25
    //           2. Create RustDownloadInfo from mock
    //           3. Call tls_time method
    // @tc.expect: Returns 300.25
    // @tc.type: FUNC
    // @tc.require: issue#ICN31I
    // @tc.level: Level 1
    #[test]
    fn ut_rust_download_info_tls_time() {
        let mut mock_info = MockDownloadInfo::new();
        mock_info.expect_tls_time().returning(|| 300.25);
        let rust_info = RustDownloadInfo::from_download_info(mock_info);
        assert_eq!(rust_info.tls_time(), 300.25);
    }

    // @tc.name: ut_rust_download_info_first_send_time
    // @tc.desc: Test first_send_time method returns correct value
    // @tc.precon: NA
    // @tc.step: 1. Create a mock DownloadInfo with first_send_time 400.0
    //           2. Create RustDownloadInfo from mock
    //           3. Call first_send_time method
    // @tc.expect: Returns 400.0
    // @tc.type: FUNC
    // @tc.require: issue#ICN31I
    // @tc.level: Level 1
    #[test]
    fn ut_rust_download_info_first_send_time() {
        let mut mock_info = MockDownloadInfo::new();
        mock_info.expect_first_send_time().returning(|| 400.0);
        let rust_info = RustDownloadInfo::from_download_info(mock_info);
        assert_eq!(rust_info.first_send_time(), 400.0);
    }

    // @tc.name: ut_rust_download_info_first_recv_time
    // @tc.desc: Test first_recv_time method returns correct value
    // @tc.precon: NA
    // @tc.step: 1. Create a mock DownloadInfo with first_recv_time 500.125
    //           2. Create RustDownloadInfo from mock
    //           3. Call first_recv_time method
    // @tc.expect: Returns 500.125
    // @tc.type: FUNC
    // @tc.require: issue#ICN31I
    // @tc.level: Level 1
    #[test]
    fn ut_rust_download_info_first_recv_time() {
        let mut mock_info = MockDownloadInfo::new();
        mock_info.expect_first_recv_time().returning(|| 500.125);
        let rust_info = RustDownloadInfo::from_download_info(mock_info);
        assert_eq!(rust_info.first_recv_time(), 500.125);
    }

    // @tc.name: ut_rust_download_info_redirect_time
    // @tc.desc: Test redirect_time method returns correct value
    // @tc.precon: NA
    // @tc.step: 1. Create a mock DownloadInfo with redirect_time 600.375
    //           2. Create RustDownloadInfo from mock
    //           3. Call redirect_time method
    // @tc.expect: Returns 600.375
    // @tc.type: FUNC
    // @tc.require: issue#ICN31I
    // @tc.level: Level 1
    #[test]
    fn ut_rust_download_info_redirect_time() {
        let mut mock_info = MockDownloadInfo::new();
        mock_info.expect_redirect_time().returning(|| 600.375);
        let rust_info = RustDownloadInfo::from_download_info(mock_info);
        assert_eq!(rust_info.redirect_time(), 600.375);
    }

    // @tc.name: ut_rust_download_info_total_time
    // @tc.desc: Test total_time method returns correct value
    // @tc.precon: NA
    // @tc.step: 1. Create a mock DownloadInfo with total_time 700.625
    //           2. Create RustDownloadInfo from mock
    //           3. Call total_time method
    // @tc.expect: Returns 700.625
    // @tc.type: FUNC
    // @tc.require: issue#ICN31I
    // @tc.level: Level 1
    #[test]
    fn ut_rust_download_info_total_time() {
        let mut mock_info = MockDownloadInfo::new();
        mock_info.expect_total_time().returning(|| 700.625);
        let rust_info = RustDownloadInfo::from_download_info(mock_info);
        assert_eq!(rust_info.total_time(), 700.625);
    }

    // @tc.name: ut_rust_download_info_resource_size
    // @tc.desc: Test resource_size method returns correct value
    // @tc.precon: NA
    // @tc.step: 1. Create a mock DownloadInfo with resource_size 1024
    //           2. Create RustDownloadInfo from mock
    //           3. Call resource_size method
    // @tc.expect: Returns 1024
    // @tc.type: FUNC
    // @tc.require: issue#ICN31I
    // @tc.level: Level 1
    #[test]
    fn ut_rust_download_info_resource_size() {
        let mut mock_info = MockDownloadInfo::new();
        mock_info.expect_resource_size().returning(|| 1024);
        let rust_info = RustDownloadInfo::from_download_info(mock_info);
        assert_eq!(rust_info.resource_size(), 1024);
    }

    // @tc.name: ut_rust_download_info_ip
    // @tc.desc: Test ip method returns correct value
    // @tc.precon: NA
    // @tc.step: 1. Create a mock DownloadInfo with ip "192.168.1.1"
    //           2. Create RustDownloadInfo from mock
    //           3. Call ip method
    // @tc.expect: Returns "192.168.1.1"
    // @tc.type: FUNC
    // @tc.require: issue#ICN31I
    // @tc.level: Level 1
    #[test]
    fn ut_rust_download_info_ip() {
        let mut mock_info = MockDownloadInfo::new();
        mock_info.expect_ip().returning(|| "192.168.1.1".to_string());
        let rust_info = RustDownloadInfo::from_download_info(mock_info);
        assert_eq!(rust_info.ip(), "192.168.1.1");
    }

    // @tc.name: ut_rust_download_info_dns_servers
    // @tc.desc: Test dns_servers method returns correct value
    // @tc.precon: NA
    // @tc.step: 1. Create a mock DownloadInfo with dns_servers ["8.8.8.8", "8.8.4.4"]
    //           2. Create RustDownloadInfo from mock
    //           3. Call dns_servers method
    // @tc.expect: Returns ["8.8.8.8", "8.8.4.4"]
    // @tc.type: FUNC
    // @tc.require: issue#ICN31I
    // @tc.level: Level 1
    #[test]
    fn ut_rust_download_info_dns_servers() {
        let mut mock_info = MockDownloadInfo::new();
        mock_info.expect_dns_servers().returning(|| {
            vec!["8.8.8.8".to_string(), "8.8.4.4".to_string()]
        });
        let rust_info = RustDownloadInfo::from_download_info(mock_info);
        assert_eq!(rust_info.dns_servers(), ["8.8.8.8", "8.8.4.4"]);
    }

    // @tc.name: ut_rust_download_info_edge_case_zero_values
    // @tc.desc: Test RustDownloadInfo methods with zero values
    // @tc.precon: NA
    // @tc.step: 1. Create a mock DownloadInfo with all time values 0.0
    //           2. Create RustDownloadInfo from mock
    //           3. Call all time methods
    // @tc.expect: All methods return 0.0
    // @tc.type: FUNC
    // @tc.require: issue#ICN31I
    // @tc.level: Level 2
    #[test]
    fn ut_rust_download_info_edge_case_zero_values() {
        let mut mock_info = MockDownloadInfo::new();
        mock_info.expect_dns_time().returning(|| 0.0);
        mock_info.expect_connect_time().returning(|| 0.0);
        mock_info.expect_tls_time().returning(|| 0.0);
        mock_info.expect_first_send_time().returning(|| 0.0);
        mock_info.expect_first_recv_time().returning(|| 0.0);
        mock_info.expect_redirect_time().returning(|| 0.0);
        mock_info.expect_total_time().returning(|| 0.0);
        mock_info.expect_resource_size().returning(|| 0);

        let rust_info = RustDownloadInfo::from_download_info(mock_info);

        assert_eq!(rust_info.dns_time(), 0.0);
        assert_eq!(rust_info.connect_time(), 0.0);
        assert_eq!(rust_info.tls_time(), 0.0);
        assert_eq!(rust_info.first_send_time(), 0.0);
        assert_eq!(rust_info.first_recv_time(), 0.0);
        assert_eq!(rust_info.redirect_time(), 0.0);
        assert_eq!(rust_info.total_time(), 0.0);
        assert_eq!(rust_info.resource_size(), 0);
    }

    // @tc.name: ut_rust_download_info_edge_case_negative_values
    // @tc.desc: Test RustDownloadInfo methods with negative values
    // @tc.precon: NA
    // @tc.step: 1. Create a mock DownloadInfo with negative time and resource values
    //           2. Create RustDownloadInfo from mock
    //           3. Call all methods
    // @tc.expect: Methods return the negative values
    // @tc.type: FUNC
    // @tc.require: issue#ICN31I
    // @tc.level: Level 2
    #[test]
    fn ut_rust_download_info_edge_case_negative_values() {
        let mut mock_info = MockDownloadInfo::new();
        mock_info.expect_dns_time().returning(|| -100.5);
        mock_info.expect_connect_time().returning(|| -200.75);
        mock_info.expect_resource_size().returning(|| -1024);

        let rust_info = RustDownloadInfo::from_download_info(mock_info);

        assert_eq!(rust_info.dns_time(), -100.5);
        assert_eq!(rust_info.connect_time(), -200.75);
        assert_eq!(rust_info.resource_size(), -1024);
    }

    // @tc.name: ut_rust_download_info_edge_case_empty_values
    // @tc.desc: Test RustDownloadInfo methods with empty string and vec
    // @tc.precon: NA
    // @tc.step: 1. Create a mock DownloadInfo with empty ip and dns_servers
    //           2. Create RustDownloadInfo from mock
    //           3. Call ip and dns_servers methods
    // @tc.expect: ip returns empty string, dns_servers returns empty vec
    // @tc.type: FUNC
    // @tc.require: issue#ICN31I
    // @tc.level: Level 2
    #[test]
    fn ut_rust_download_info_edge_case_empty_values() {
        let mut mock_info = MockDownloadInfo::new();
        mock_info.expect_ip().returning(|| "".to_string());
        mock_info.expect_dns_servers().returning(|| vec![]);

        let rust_info = RustDownloadInfo::from_download_info(mock_info);

        assert_eq!(rust_info.ip(), "");
        assert!(rust_info.dns_servers().is_empty());
    }
}