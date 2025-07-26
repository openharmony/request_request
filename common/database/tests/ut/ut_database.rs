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

use std::fs;

use ffi::SecurityLevel;

use super::*;

fn get_rdb() -> RdbStore<'static> {
    let _ = fs::create_dir_all("/data/test");

    let mut config = OpenConfig::new("/data/test/request_database_test.db");
    config.encrypt_status(false);
    config.security_level(SecurityLevel::S1);
    config.bundle_name("test");

    RdbStore::open(config).unwrap()
}

// @tc.name: ut_database_query
// @tc.desc: Test database query function with insert and select operations
// @tc.precon: NA
// @tc.step: 1. Create test database and table
//           2. Insert multiple test records
//           3. Query all records and verify count
//           4. Check each record's id and name
// @tc.expect: Query returns 10 records with correct id and name
// @tc.type: FUNC
// @tc.require: issues#ICN31I
#[test]
fn ut_database_query() {
    let rdb = get_rdb();
    rdb.execute("DROP TABLE IF EXISTS test_table_001", ())
        .unwrap();
    rdb.execute(
        "CREATE TABLE IF NOT EXISTS test_table_001 (id INTEGER PRIMARY KEY, name TEXT)",
        (),
    )
        .unwrap();
    for i in 0..10 {
        rdb.execute(
            "INSERT OR REPLACE INTO test_table_001 (id, name) VALUES (?, ?)",
            (i, "test"),
        )
            .unwrap();
    }
    let mut set = rdb
        .query::<(i32, String)>("SELECT * from test_table_001", ())
        .unwrap();
    assert_eq!(set.row_count(), 10);
    assert_eq!(set.column_count(), 2);
    for row in set.enumerate() {
        let (index, (id, name)) = row;
        assert_eq!(index as i32, id);
        assert_eq!("test", name);
    }
}

// @tc.name: ut_database_option
// @tc.desc: Test database operations with optional values
// @tc.precon: NA
// @tc.step: 1. Create test database and table
//           2. Insert record with None value
//           3. Verify None value retrieval
//           4. Update record with Some value
//           5. Verify Some value retrieval
// @tc.expect: None and Some values are correctly stored and retrieved
// @tc.type: FUNC
// @tc.require: issues#ICN31I
#[test]
fn ut_database_option() {
    const TEST_STRING: &str = "TEST";

    let rdb = get_rdb();
    rdb.execute("DROP TABLE IF EXISTS test_table_002", ())
        .unwrap();
    rdb.execute(
        "CREATE TABLE IF NOT EXISTS test_table_002 (id INTEGER PRIMARY KEY, name TEXT)",
        (),
    )
        .unwrap();
    let _ = rdb.execute(
        "INSERT OR REPLACE INTO test_table_002 (id, name) VALUES (?, ?)",
        (0, Option::<String>::None),
    );
    let mut set = rdb
        .query::<Option<String>>("SELECT name from test_table_002 WHERE id=0", ())
        .unwrap();
    assert_eq!(set.next().unwrap(), None);

    let _ = rdb.execute(
        "INSERT OR REPLACE INTO test_table_002 (id, name) VALUES (?, ?)",
        (0, Some(TEST_STRING)),
    );
    let mut set = rdb
        .query::<Option<String>>("SELECT name from test_table_002 WHERE id=0", ())
        .unwrap();
    assert_eq!(set.next().unwrap(), Some(TEST_STRING.to_string()));

    let _ = rdb.execute(
        "INSERT OR REPLACE INTO test_table_002 (id, name) VALUES (?, ?)",
        (0, TEST_STRING),
    );
    let mut set = rdb
        .query::<Option<String>>("SELECT name from test_table_002 WHERE id=0", ())
        .unwrap();
    assert_eq!(set.next().unwrap(), Some(TEST_STRING.to_string()));
}
