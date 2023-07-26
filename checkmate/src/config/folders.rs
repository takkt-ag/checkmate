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

use super::{
    Host,
    Ruleset,
};
use crate::Result;

use checkmk_client::folders::{
    CreateFolderRequest,
    FolderAttributes,
    FoldersApi,
    ShowFolderResponse,
    UpdateFolderRequest,
};
use serde::{
    Deserialize,
    Deserializer,
};
use std::{
    collections::HashMap,
    ffi::OsStr,
    path::{
        Path,
        PathBuf,
    },
    rc::Rc,
};

#[derive(Debug, Deserialize)]
pub struct Folders {
    #[serde(rename = "/", deserialize_with = "deserialize_root_folder")]
    pub root_folder: Rc<Folder>,
}

fn deserialize_root_folder<'de, D>(deserializer: D) -> std::result::Result<Rc<Folder>, D::Error>
where
    D: Deserializer<'de>,
{
    let root_folder = Folder::deserialize(deserializer)?;
    Ok(populate_folder(PathBuf::from("/"), root_folder))
}

fn populate_folder(path: PathBuf, mut folder: Folder) -> Rc<Folder> {
    folder.path = path;
    Rc::new_cyclic(move |folder_weak| {
        if let Some(hosts) = &mut folder.hosts {
            hosts.iter_mut().for_each(|host| {
                host.folder = folder_weak.clone();
            });
        }
        if let Some(rulesets) = &mut folder.rulesets {
            rulesets.iter_mut().for_each(|(_, ruleset)| {
                let mut r = (**ruleset).clone();
                r.folder = folder_weak.clone();
                *ruleset = Rc::new_cyclic(move |ruleset_weak| {
                    for rule in r.rules.iter_mut() {
                        rule.ruleset = ruleset_weak.clone();
                    }
                    r
                });
            });
        }

        folder.folders = folder
            .folders
            .into_iter()
            .map(|(child_name, child_folder)| {
                let mut child_path = folder.path.clone();
                child_path.push(child_name.trim_start_matches('/'));
                (
                    child_name,
                    populate_folder(
                        child_path,
                        Rc::try_unwrap(child_folder).expect("folder is not unique"),
                    ),
                )
            })
            .collect();

        folder
    })
}

#[derive(Debug, Default, Deserialize)]
pub struct Folder {
    #[serde(default)]
    pub path: PathBuf,
    pub title: String,
    pub attributes: Option<FolderAttributes>,
    #[serde(default, deserialize_with = "deserialize_rulesets")]
    pub rulesets: Option<HashMap<String, Rc<Ruleset>>>,
    pub hosts: Option<Vec<Host>>,
    #[serde(flatten, deserialize_with = "crate::de::deserialize_map_values_as_rc")]
    pub folders: HashMap<String, Rc<Folder>>,
}

fn deserialize_rulesets<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<HashMap<String, Rc<Ruleset>>>, D::Error>
where
    D: Deserializer<'de>,
{
    let rulesets: HashMap<String, Ruleset> = HashMap::deserialize(deserializer)?;
    Ok(Some(
        rulesets
            .into_iter()
            .map(|(name, mut ruleset)| {
                let ruleset = Rc::new_cyclic({
                    let name = name.clone();
                    move |ruleset_weak| {
                        for rule in ruleset.rules.iter_mut() {
                            rule.ruleset = ruleset_weak.clone();
                            rule.normalize_properties(&name);
                        }
                        ruleset.name = name;
                        ruleset
                    }
                });
                (name, ruleset)
            })
            .collect(),
    ))
}

impl Folder {
    pub fn id(&self) -> String {
        self.path
            .to_string_lossy()
            .replace(std::path::MAIN_SEPARATOR, "~")
    }

    pub fn name(&self) -> &str {
        self.path
            .file_name()
            .and_then(OsStr::to_str)
            .unwrap_or_default()
    }

    pub fn parent(&self) -> String {
        self.path
            .parent()
            .map(Path::to_string_lossy)
            .unwrap_or_default()
            .replace(std::path::MAIN_SEPARATOR, "~")
    }

    fn needs_update(&self, folder_api: &ShowFolderResponse) -> bool {
        let mut equal = self.title == folder_api.title;
        equal &=
            self.attributes.clone().unwrap_or_default() == folder_api.extensions.clone().attributes;
        !equal
    }

    pub fn apply_to_site(&self, cmk: &checkmk_client::Client) -> Result<()> {
        let id = self.id();
        match cmk.folders().show_folder(&id) {
            Ok((folder_api, etag)) if self.needs_update(&folder_api) => {
                println!("{}: updating existing folder", id);
                cmk.folders()
                    .update_folder(&id, etag, &self.into())
                    .map(|_| ())
                    .map_err(Into::into)
            }
            Ok(_) => {
                println!("{}: folder exists with correct attributes", id);
                Ok(())
            }
            Err(error @ checkmk_client::ClientError::HttpRequestError(_))
                if error.is_status(404) =>
            {
                println!("{}: creating missing folder", id);
                cmk.folders()
                    .create_folder(&self.into())
                    .map(|_| ())
                    .map_err(Into::into)
            }
            Err(e) => Err(e.into()),
        }
    }
}

impl<'a> From<&'a Folder> for CreateFolderRequest<'a> {
    fn from(folder: &'a Folder) -> Self {
        Self {
            name: folder.name(),
            title: &folder.title,
            parent: folder.parent(),
            attributes: &folder.attributes,
        }
    }
}

impl<'a> From<&'a Folder> for UpdateFolderRequest<'a> {
    fn from(folder: &'a Folder) -> Self {
        Self {
            title: &folder.title,
            attributes: &folder.attributes,
        }
    }
}
