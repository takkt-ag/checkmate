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
pub struct ShowHostResponse {
    pub id: Option<String>,
    pub title: String,
    // pub members: ?,
    pub extensions: HostOutputExtensions,
}

#[derive(Clone, Debug, Serialize)]
pub struct CreateHostRequest<'a> {
    pub folder: String,
    pub host_name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: &'a Option<HostAttributes>,
}

#[derive(Clone, Debug, Serialize)]
pub struct UpdateHostRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: &'a Option<HostAttributes>,
}

#[derive(Clone, Debug, Serialize)]
pub struct MoveToFolderRequest<'a> {
    pub target_folder: &'a str,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HostOutputExtensions {
    pub folder: String,
    pub attributes: HostAttributes,
    pub is_cluster: bool,
    pub is_offline: bool,
    pub cluster_nodes: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HostAttributes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<String>,
    #[serde(rename = "contactgroups", skip_serializing_if = "Option::is_none")]
    pub contact_groups: Option<ContactGroups>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parents: Option<Vec<String>>,
    #[serde(rename = "ipaddress", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "ipv6address", skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<String>,
    #[serde(
        rename = "additional_ipv4addresses",
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_ipv4_addresses: Option<Vec<String>>,
    #[serde(
        rename = "additional_ipv6addresses",
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_ipv6_addresses: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snmp_community: Option<SnmpCommunity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub network_scan: Option<NetworkScan>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub network_scan_result: Option<NetworkScanResult>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub management_protocol: Option<ManagementProtocol>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub management_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub management_snmp_community: Option<SnmpCommunity>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub management_ipmi_credentials: Option<IpmiCredentials>,
    // #[serde(rename = "meta_data", skip_serializing_if = "Option::is_none")]
    // pub metadata: Option<Metadata>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub locked_by: Option<LockedBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked_attributes: Option<Vec<String>>,
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

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SnmpCommunity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<SnmpCommunityType>,
    pub community: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SnmpCommunityType {
    #[serde(rename = "v1_v2_community")]
    V1V2Community,
    #[serde(rename = "v3_auth_no_privacy")]
    V3AuthNoPrivacy,
    #[serde(rename = "v3_auth_privacy")]
    V3AuthPrivacy,
    #[serde(rename = "v3_no_auth_no_privacy")]
    V3NoAuthNoPrivacy,
}

pub trait HostsApi {
    fn hosts(&self) -> HostsClient;
}

pub struct HostsClient<'a>(&'a Client);

impl HostsApi for Client {
    fn hosts(&self) -> HostsClient {
        HostsClient(self)
    }
}

impl<'a> HostsClient<'a> {
    pub fn create_host(&self, host: &CreateHostRequest) -> Result<(ShowHostResponse, ETag)> {
        self.0
            .post_with_etag("/domain-types/host_config/collections/all", host)
    }

    pub fn update_host(
        &self,
        host_name: &str,
        etag: ETag,
        host: &UpdateHostRequest,
    ) -> Result<(ShowHostResponse, ETag)> {
        self.0
            .put_if_match_with_etag(format!("/objects/host_config/{}", host_name), etag, host)
    }

    pub fn move_to_folder(
        &self,
        host_name: &str,
        etag: ETag,
        target_folder: &str,
    ) -> Result<(ShowHostResponse, ETag)> {
        self.0.post_if_match_with_etag(
            format!("/objects/host_config/{}/actions/move/invoke", host_name),
            etag,
            &MoveToFolderRequest { target_folder },
        )
    }

    pub fn show_host(&self, host_name: &str) -> Result<(ShowHostResponse, ETag)> {
        self.0
            .get_with_etag(format!("/objects/host_config/{}", host_name))
    }
}
