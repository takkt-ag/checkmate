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
    Client,
    ETag,
    Result,
};
use crate::models::{
    ContactGroups,
    TagAddressFamily,
    TagAgent,
    TagCriticality,
    TagNetworking,
    TagPiggyback,
    TagSnmpDs,
};
use serde::{
    Deserialize,
    Serialize,
};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize)]
pub struct ShowFolderResponse {
    pub id: Option<String>,
    pub title: String,
    pub extensions: FolderOutputExtensions,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateFolderRequest<'a> {
    pub name: &'a str,
    pub title: &'a str,
    pub parent: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: &'a Option<FolderAttributes>,
}

#[derive(Clone, Debug, Serialize)]
pub struct UpdateFolderRequest<'a> {
    pub title: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: &'a Option<FolderAttributes>,
}

#[derive(Clone, Debug, Serialize)]
pub struct FolderUpdate<'a> {
    pub title: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: &'a Option<FolderAttributes>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FolderOutputExtensions {
    pub path: String,
    pub attributes: FolderAttributes,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FolderAttributes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<String>,
    #[serde(rename = "contactgroups", skip_serializing_if = "Option::is_none")]
    pub contact_groups: Option<ContactGroups>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parents: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_criticality: Option<TagCriticality>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_networking: Option<TagNetworking>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_agent: Option<TagAgent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_piggyback: Option<TagPiggyback>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_snmp_ds: Option<TagSnmpDs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_address_family: Option<TagAddressFamily>,
}

pub trait FoldersApi {
    fn folders(&self) -> FoldersClient;
}

pub struct FoldersClient<'a>(&'a Client);

impl FoldersApi for Client {
    fn folders(&self) -> FoldersClient {
        FoldersClient(self)
    }
}

impl<'a> FoldersClient<'a> {
    pub fn create_folder(
        &self,
        folder: &CreateFolderRequest,
    ) -> Result<(ShowFolderResponse, ETag)> {
        self.0
            .post_with_etag("/domain-types/folder_config/collections/all", folder)
    }

    pub fn update_folder(
        &self,
        id: &str,
        etag: ETag,
        folder: &UpdateFolderRequest,
    ) -> Result<(ShowFolderResponse, ETag)> {
        self.0
            .put_if_match_with_etag(format!("/objects/folder_config/{}", id), etag, folder)
    }

    pub fn show_folder(&self, folder: &str) -> Result<(ShowFolderResponse, ETag)> {
        self.0
            .get_with_etag(format!("/objects/folder_config/{}", folder))
    }
}
