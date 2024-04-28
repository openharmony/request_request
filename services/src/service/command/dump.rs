// Copyright (C) 2023 Huawei Device Co., Ltd.
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

use std::fs::File;
use std::io::Write;

use ipc::IpcResult;

use crate::manage::events::TaskManagerEvent;
use crate::service::RequestServiceStub;

const HELP_MSG: &str = "usage:\n\
                         -h                    help text for the tool\n\
                         -t [taskid]           without taskid: display all task summary info; \
                         taskid: display one task detail info\n";
impl RequestServiceStub {
    // Ignores all the file error.
    pub(crate) fn dump(&self, mut file: File, args: Vec<String>) -> IpcResult<()> {
        info!("Service dump");

        let len = args.len();
        if len == 0 || args[0] == "-h" {
            let _ = file.write(HELP_MSG.as_bytes());
            return Ok(());
        }

        if args[0] != "-t" {
            let _ = file.write("invalid args".as_bytes());
            return Ok(());
        }

        match len {
            1 => self.dump_all_task_info(file),
            2 => {
                let task_id = args[1].parse::<u32>();
                match task_id {
                    Ok(id) => self.dump_one_task_info(file, id),
                    Err(_) => {
                        let _ = file.write("-t accept a number".as_bytes());
                    }
                }
            }
            _ => {
                let _ = file.write("too many args, -t accept no arg or one arg".as_bytes());
            }
        }
        Ok(())
    }

    fn dump_all_task_info(&self, mut file: File) {
        info!("Service dump: dump all task info");

        let (event, rx) = TaskManagerEvent::dump_all();
        if !self.task_manager.send_event(event) {
            return;
        }

        let infos = match rx.get() {
            Some(infos) => infos,
            None => {
                error!("Service dump: receives infos failed");
                return;
            }
        };
        let len = infos.vec.len();
        let _ = file.write(format!("task num: {}\n", len).as_bytes());
        if len > 0 {
            let _ = file.write(
                format!(
                    "{:<20}{:<12}{:<12}{:<12}\n",
                    "id", "action", "state", "reason"
                )
                .as_bytes(),
            );
            for info in infos.vec.iter() {
                let _ = file.write(
                    format!(
                        "{:<20}{:<12}{:<12}{:<12}\n",
                        info.task_id, info.action as u8, info.state as u8, info.reason as u8
                    )
                    .as_bytes(),
                );
            }
        }
    }

    fn dump_one_task_info(&self, mut file: File, task_id: u32) {
        info!("Service dump: dump one task info");

        let (event, rx) = TaskManagerEvent::dump_one(task_id);
        if !self.task_manager.send_event(event) {
            return;
        }
        let task = match rx.get() {
            Some(task) => task,
            None => {
                error!("Service dump: receives task failed");
                return;
            }
        };

        if let Some(task) = task {
            let _ = file.write(
                format!(
                    "{:<20}{:<12}{:<12}{:<12}{:<12}{:<12}{}\n",
                    "id", "action", "state", "reason", "total_size", "tran_size", "url"
                )
                .as_bytes(),
            );
            let _ = file.write(
                format!(
                    "{:<20}{:<12}{:<12}{:<12}{:<12}{:<12}{}\n",
                    task.task_id,
                    task.action as u8,
                    task.state as u8,
                    task.reason as u8,
                    task.total_size,
                    task.tran_size,
                    task.url
                )
                .as_bytes(),
            );
        } else {
            let _ = file.write(format!("invalid task id {}", task_id).as_bytes());
        }
    }
}
