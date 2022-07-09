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

use std::io::Write;

use clap::{Args, Subcommand};
use clap::CommandFactory;
use clap_complete::{generate, shells};

use super::Cli;

#[derive(Debug, Args)]
pub struct GenerateCompletionsCommand {
    #[clap(subcommand)]
    pub shell: GenerateCompletionsShell,
}

#[derive(Debug, Subcommand)]
pub enum GenerateCompletionsShell {
    #[clap(name = "bash")]
    /// Generates a bash-completion script for the cli
    Bash,
}

pub fn generate_completions<T>(shell: GenerateCompletionsShell, output: &mut T) -> Result<(), Box<dyn std::error::Error>> where T: Write {
    match shell {
        GenerateCompletionsShell::Bash => {
            generate(
                shells::Bash,
                &mut Cli::command(),
                "mcpack",
                output,
            );
        }
    }
    Ok(())
}
