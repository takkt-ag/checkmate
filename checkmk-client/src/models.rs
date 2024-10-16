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

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ContactGroups {
    pub groups: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurse_perms: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurse_use: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#use: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_for_services: Option<bool>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TagCriticality {
    #[serde(rename = "prod")]
    Prod,
    #[serde(rename = "critical")]
    Critical,
    #[serde(rename = "test")]
    Test,
    #[serde(rename = "offline")]
    Offline,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TagNetworking {
    #[serde(rename = "lan")]
    Lan,
    #[serde(rename = "wan")]
    Wan,
    #[serde(rename = "dmz")]
    Dmz,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TagAgent {
    #[serde(rename = "cmk-agent")]
    CmkAgent,
    #[serde(rename = "all-agents")]
    AllAgents,
    #[serde(rename = "special-agents")]
    SpecialAgents,
    #[serde(rename = "no-agent")]
    NoAgent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TagPiggyback {
    #[serde(rename = "auto-piggyback")]
    AutoPiggyback,
    #[serde(rename = "piggyback")]
    Piggyback,
    #[serde(rename = "no-piggyback")]
    NoPiggyback,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TagSnmpDs {
    #[serde(rename = "no-snmp")]
    NoSnmp,
    #[serde(rename = "snmp-v2")]
    SnmpV2,
    #[serde(rename = "snmp-v1")]
    SnmpV1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TagAddressFamily {
    #[serde(rename = "ip-v4-only")]
    IpV4Only,
    #[serde(rename = "ip-v6-only")]
    IpV6Only,
    #[serde(rename = "ip-v4v6")]
    IpV4v6,
    #[serde(rename = "no-ip")]
    NoIp,
}
