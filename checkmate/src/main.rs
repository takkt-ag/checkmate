// Copyright 2023 KAISER+KRAFT EUROPA GmbH
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

mod cli;
mod config;
mod de;

use crate::config::{
    DeclarativeConfig,
    Folder,
};
use anyhow::Result;
use checkmk_client::changes::ChangesApi;
use clap::Parser;

fn main() -> Result<()> {
    let args = cli::Args::parse();
    let client =
        checkmk_client::Client::new(&args.server_url, &args.site, &args.username, &args.secret)?;
    let config = DeclarativeConfig::load_from_file(&args.config_file)?;

    process_folders(client, config)?;
    Ok(())
}

fn process_folders(client: checkmk_client::Client, config: DeclarativeConfig) -> Result<()> {
    let root_folder = config.folders.root_folder;
    apply_folders(&client, &root_folder)?;
    apply_pending_changes(&client)?;
    Ok(())
}

fn apply_folders(client: &checkmk_client::Client, folder: &Folder) -> Result<()> {
    folder.apply_to_site(client)?;
    for folder in folder.folders.values() {
        apply_folders(client, folder)?;
    }
    if let Some(hosts) = &folder.hosts {
        for host in hosts {
            host.apply_to_site(client)?;
        }
    }
    if let Some(rulesets) = &folder.rulesets {
        for ruleset in rulesets.values() {
            ruleset.apply_to_site(client)?;
        }
    }
    Ok(())
}

fn apply_pending_changes(client: &checkmk_client::Client) -> Result<()> {
    let ((), etag) = client.changes().show_all_pending_changes()?;
    match client.changes().activate_pending_changes(etag) {
        Ok(c) => {
            client.changes().wait_for_activation_completion(&c.id)?;
            println!("change_info: {:#?}", c);
            Ok(())
        }
        Err(error @ checkmk_client::ClientError::HttpRequestError(_)) if error.is_status(422) => {
            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}
