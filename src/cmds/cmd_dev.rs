/*
 * Copyright (c) 2022 Philippe Warren (philippewarren)
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

use clap::{Args, Subcommand};

use cmd_dev_init::{dev_init, DevInitCommand};

mod cmd_dev_init;

#[derive(Debug, Args)]
pub struct DevCommand {
    #[clap(subcommand)]
    pub action: DevAction,
}

#[derive(Debug, Subcommand)]
pub enum DevAction {
    #[clap(name = "init")]
    /// Initializes a modpack for development
    Init(DevInitCommand),
}

pub fn dev(cmd: DevCommand) {
    match cmd.action {
        DevAction::Init(cmd) => dev_init(cmd),
    }
}
