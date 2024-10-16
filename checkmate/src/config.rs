// Copyright 2024 TAKKT Industrial & Packaging GmbH
//
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
//
// SPDX-License-Identifier: Apache-2.0

mod folders;
mod hosts;
mod rulesets;

pub use folders::{
    Folder,
    Folders,
};
pub use hosts::Host;
pub use rulesets::Ruleset;

use color_eyre::eyre::{
    Result,
    WrapErr,
};
use serde::Deserialize;
use std::{
    collections::HashMap,
    path::Path,
};

/// The central type describing the declarative configuration which will be applied to the desired
/// check_mk site.
#[derive(Debug, Deserialize)]
pub struct DeclarativeConfig {
    pub folders: Folders,
}

impl DeclarativeConfig {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let config: DeclarativeConfig = serde_yaml::from_reader(std::fs::File::open(path)?)
            .wrap_err("Failed to parse declarative config")?;
        config.verify_constraints()?;
        Ok(config)
    }

    fn verify_constraints(&self) -> Result<()> {
        let mut discovered_hosts: HashMap<&str, &Path> = HashMap::new();
        let mut folders_to_visit = vec![&self.folders.root_folder];
        while let Some(folder) = folders_to_visit.pop() {
            for host in folder.hosts.iter().flatten() {
                if let Some(existing_path) = discovered_hosts.insert(&host.host_name, &folder.path)
                {
                    color_eyre::eyre::bail!(
                        "Host {} is defined both in folder {} and folder {}",
                        host.host_name,
                        existing_path.display(),
                        folder.path.display()
                    );
                }
            }
            folders_to_visit.extend(folder.folders.values());
        }

        Ok(())
    }
}
