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

use self_update::cargo_crate_version;

pub fn upgrade(quiet: bool) -> Result<(), Box<dyn std::error::Error>> {
    self_update::backends::github::Update::configure()
        .repo_owner("philippewarren")
        .repo_name("mcpack")
        .bin_path_in_archive("bin/mcpack")
        .bin_name("mcpack")
        .show_download_progress(!quiet)
        .show_output(!quiet)
        .no_confirm(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    Ok(())
}
