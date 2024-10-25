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

use super::Folder;
use crate::Result;

use checkmk_client::hosts::{
    CreateHostRequest,
    HostAttributes,
    HostsApi,
    ShowHostResponse,
    UpdateHostRequest,
};
use serde::Deserialize;
use std::rc::Weak;

#[derive(Debug, Deserialize)]
pub struct Host {
    pub host_name: String,
    #[serde(default, deserialize_with = "crate::de::deserialize_to_empty_weak")]
    pub folder: Weak<Folder>,
    pub attributes: Option<HostAttributes>,
}

impl Host {
    fn new_path(&self, host_api: &ShowHostResponse) -> Option<String> {
        let folder = self.folder.upgrade().expect("folder weak ref is broken");
        let potentially_new_path = folder.path.to_string_lossy();
        if potentially_new_path != host_api.extensions.folder {
            Some(potentially_new_path.into_owned())
        } else {
            None
        }
    }

    fn needs_update(&self, host_api: &ShowHostResponse) -> bool {
        self.attributes.clone().unwrap_or_default() != host_api.extensions.attributes
    }

    pub fn apply_to_site(&self, cmk: &checkmk_client::Client) -> Result<()> {
        let id = self.host_name.clone();
        match cmk.hosts().show_host(&id) {
            Ok((mut host_api, mut etag)) => {
                if let Some(new_path) = self.new_path(&host_api) {
                    println!("{}: moving host to new folder", id);
                    (host_api, etag) = cmk.hosts().move_to_folder(&id, etag, &new_path)?;
                }
                if self.needs_update(&host_api) {
                    println!("{}: updating existing host", id);
                    cmk.hosts().update_host(&id, etag, &self.into())?;
                }
                Ok(())
            }
            Err(error @ checkmk_client::ClientError::HttpRequestError(_))
                if error.is_status(404) =>
            {
                println!("{}: creating missing host", id);
                cmk.hosts()
                    .create_host(&self.into())
                    .map(|_| ())
                    .map_err(Into::into)
            }
            Err(e) => Err(e.into()),
        }
    }
}

impl<'a> From<&'a Host> for CreateHostRequest<'a> {
    fn from(host: &'a Host) -> Self {
        Self {
            folder: host
                .folder
                .upgrade()
                .expect("folder weak ref is broken")
                .id(),
            host_name: &host.host_name,
            attributes: &host.attributes,
        }
    }
}

impl<'a> From<&'a Host> for UpdateHostRequest<'a> {
    fn from(host: &'a Host) -> Self {
        Self {
            attributes: &host.attributes,
        }
    }
}
