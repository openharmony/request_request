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

//! Query interface for download tasks.
//! 
//! This module extends the `RequestProxy` with functionality for querying and searching
//! download tasks. It provides methods to retrieve task information, mime types, and
//! search for tasks based on various criteria.

// External dependencies
use ipc::parcel::MsgParcel;
use ipc::remote;

// Download core dependencies
use request_core::config::{Action,TaskConfig};
use request_core::filter::SearchFilter;
use request_core::info::{State, TaskInfo};
use request_core::interface;
use std::time::{SystemTime, UNIX_EPOCH};

// Local dependencies
use crate::proxy::{RequestProxy, SERVICE_TOKEN};

impl RequestProxy {
    /// Queries basic information about a specific download task.
    ///
    /// # Parameters
    /// - `task_id`: Unique identifier of the task to query
    ///
    /// # Returns
    /// - `Ok(())` on success
    /// - `Err(i32)` with an error code on failure
    ///
    /// # Notes
    /// This method is currently not fully implemented and contains a `todo!()` placeholder.
    pub(crate) fn query(&self, task_id: i64) -> Result<TaskInfo, i32> {
        let remote = self.remote()?;

        let mut data = MsgParcel::new();
        data.write_interface_token(SERVICE_TOKEN).unwrap();

        data.write(&1u32).unwrap();
        data.write(&task_id.to_string()).unwrap();

        let mut reply = remote.send_request(interface::QUERY, &mut data).map_err(|_| 13400003)?;

        let code = reply.read::<i32>().unwrap(); // error code

        if code != 0 {
            return Err(code);
        }
        let task_info = reply.read::<TaskInfo>().unwrap(); // task info
        Ok(task_info)
    }

    /// Queries the MIME type of a specific download task.
    ///
    /// # Parameters
    /// - `task_id`: Unique identifier of the task to query
    ///
    /// # Returns
    /// A `Result` containing either:
    /// - `Ok(String)` with the MIME type of the download task
    /// - `Err(i32)` with an error code if the task doesn't exist or cannot be accessed
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use request_next::proxy::RequestProxy;
    ///
    /// fn example() -> Result<(), i32> {
    ///     let proxy = RequestProxy::get_instance();
    ///     let task_id = 12345;
    ///     
    ///     match proxy.query_mime_type(task_id) {
    ///         Ok(mime_type) => println!("Task MIME type: {}", mime_type),
    ///         Err(error) => println!("Failed to get MIME type: {}", error),
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub(crate) fn query_mime_type(&self, task_id: i64) -> Result<String, i32> {
        let remote = self.remote()?;

        let mut data = MsgParcel::new();
        data.write_interface_token(SERVICE_TOKEN).unwrap();

        data.write(&task_id.to_string()).unwrap();

        let mut reply = remote
            .send_request(interface::QUERY_MIME_TYPE, &mut data)
            .map_err(|_| 13400003)?;

        let code = reply.read::<i32>().unwrap(); // error code
        if code != 0 {
            return Err(code);
        }

        let mime_type = reply.read::<String>().unwrap();
        Ok(mime_type)
    }

    /// Retrieves detailed information about a specific download task.
    ///
    /// # Parameters
    /// - `task_id`: Unique identifier of the task to retrieve
    ///
    /// # Returns
    /// A `Result` containing either:
    /// - `Ok(TaskInfo)` with detailed information about the task
    /// - `Err(i32)` with an error code if the task doesn't exist or cannot be accessed
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use request_next::proxy::RequestProxy;
    ///
    /// fn example() -> Result<(), i32> {
    ///     let proxy = RequestProxy::get_instance();
    ///     let task_id = 12345;
    ///     
    ///     match proxy.show(task_id) {
    ///         Ok(task_info) => println!("Task URL: {}", task_info.url),
    ///         Err(error) => println!("Failed to retrieve task info: {}", error),
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub(crate) fn show(&self, task_id: i64) -> Result<TaskInfo, i32> {
        let remote = self.remote()?;

        let mut data = MsgParcel::new();
        data.write_interface_token(SERVICE_TOKEN).unwrap();

        data.write(&1u32).unwrap();
        data.write(&task_id.to_string()).unwrap();

        let mut reply = remote.send_request(interface::SHOW, &mut data).map_err(|_| 13400003)?;

        let code = reply.read::<i32>().unwrap(); // error code
        if code != 0 {
            return Err(code);
        }

        let code = reply.read::<i32>().unwrap(); // error code
        if code != 0 {
            return Err(code);
        }
        let task_info = reply.read::<TaskInfo>().unwrap(); // task info
        Ok(task_info)
    }

    /// Updates the last access time of a download task.
    ///
    /// # Parameters
    /// - `task_id`: Unique identifier of the task to update
    /// - `token`: Authentication token for accessing the task
    ///
    /// # Returns
    /// - `Ok(())` on success
    /// - `Err(i32)` with an error code on failure
    ///
    /// # Notes
    /// This method is currently not fully implemented and contains a `todo!()` placeholder.
    pub(crate) fn touch(&self, task_id: i64, token: String) -> Result<TaskInfo, i32> {
        let remote = self.remote()?;

        let mut data = MsgParcel::new();
        data.write_interface_token(SERVICE_TOKEN).unwrap();

        data.write(&1u32).unwrap();
        data.write(&task_id.to_string()).unwrap();
        data.write(&token).unwrap(); // authentication token

        let mut reply = remote.send_request(interface::TOUCH, &mut data).map_err(|_| 13400003)?;

        let code = reply.read::<i32>().unwrap(); // error code
        if code != 0 {
            return Err(code);
        }
        let code = reply.read::<i32>().unwrap(); // error code
        if code != 0 {
            return Err(code);
        }
        let task_info = reply.read::<TaskInfo>().unwrap(); // task info
        Ok(task_info)
    }

    /// Searches for download tasks based on specified filter criteria.
    ///
    /// # Parameters
    /// - `filter`: Search criteria to filter tasks by bundle name, time range, state, action, and mode
    ///
    /// # Returns
    /// A `Result` containing either:
    /// - `Ok(Vec<String>)` with a list of matching task IDs
    /// - `Err(i32)` with an error code if the search fails
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use request_next::proxy::RequestProxy;
    /// use request_core::filter::SearchFilter;
    /// use request_core::info::State;
    ///
    /// fn example() -> Result<(), i32> {
    ///     let proxy = RequestProxy::get_instance();
    ///     
    ///     // Search for completed tasks
    ///     let filter = SearchFilter {
    ///         state: Some(State::Completed),
    ///         bundle_name: None,
    ///         before: None,
    ///         after: None,
    ///         action: None,
    ///         mode: None,
    ///     };
    ///     
    ///     match proxy.search(filter) {
    ///         Ok(task_ids) => println!("Found {} completed tasks", task_ids.len()),
    ///         Err(error) => println!("Search failed: {}", error),
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub(crate) fn search(&self, filter: SearchFilter) -> Result<Vec<String>, i32> {
        let remote = self.remote()?;

        let mut data = MsgParcel::new();
        data.write_interface_token(SERVICE_TOKEN).unwrap();

        // Serialize bundle name filter, use "*" as wildcard for None
        match filter.bundle_name {
            Some(ref bundle) => data.write(bundle).unwrap(),
            None => data.write(&"*".to_string()).unwrap(),
        }

        // Serialize the filter parameters into the parcel
        match filter.before {
            Some(before) => data.write(&before).unwrap(),
            None => match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(n) => data.write(&(n.as_millis() as i64)).unwrap(),
                Err(_) => data.write(&(0i64)).unwrap(),
            },
        }

        match filter.after {
            Some(after) => data.write(&after).unwrap(),
            None => match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(n) => data
                    .write(&(n.as_millis() as i64 - 24 * 60 * 60 * 1000))
                    .unwrap(),
                Err(_) => data.write(&(0i64)).unwrap(),
            },
        }

        match filter.state {
            Some(state) => data.write(&(state as u32)).unwrap(),
            None => data.write(&(State::Any as u32)).unwrap(),
        }

        match filter.action {
            Some(action) => data.write(&(action as u32)).unwrap(),
            None => data.write(&(2u32)).unwrap(),
        }

        match filter.mode {
            Some(mode) => data.write(&(mode as u32)).unwrap(),
            None => data.write(&02u32).unwrap(), // Default mode value
        }

        let mut reply = remote.send_request(interface::SEARCH, &mut data).map_err(|_| 13400003)?;

        // First value in reply is the number of results
        let len = reply.read::<u32>().unwrap();
        let mut ids = Vec::with_capacity(len as usize);
        
        // Read each task ID from the reply
        for _ in 0..len {
            let id = reply.read::<String>().unwrap();
            ids.push(id);
        }
        Ok(ids)
    }

    /// Retrieves a download task with authentication.
    ///
    /// # Parameters
    /// - `task_id`: Unique identifier of the task to retrieve
    /// - `token`: Authentication token for accessing the task
    ///
    /// # Returns
    /// - `Ok(())` on success
    /// - `Err(i32)` with an error code on failure
    ///
    /// # Notes
    /// This method is currently not fully implemented and contains a `todo!()` placeholder.
    pub(crate) fn get_task(&self, task_id: i64, token: Option<String>) -> Result<TaskConfig, i32> {
        let remote = self.remote()?;

        let mut data = MsgParcel::new();
        data.write_interface_token(SERVICE_TOKEN).unwrap();

        data.write(&task_id.to_string()).unwrap();
        match token {
            Some(t) => data.write(&t).unwrap(),
            None => data.write(&"".to_string()).unwrap(),
        } // authentication token

        let mut reply = remote.send_request(interface::GET_TASK, &mut data).map_err(|_| 13400003)?;

        let code = reply.read::<i32>().unwrap(); // error code
        if code != 0 && code != 5 {
            return Err(code);
        }

        //Deserialize
        let task_config = reply.read::<TaskConfig>().unwrap();
        Ok(task_config)
    }
}
