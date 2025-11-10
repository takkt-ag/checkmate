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

use super::{
    Client,
    Result,
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Serialize)]
pub struct CreateRuleRequest<'a> {
    pub ruleset: String,
    pub folder: String,
    pub properties: &'a RuleProperties,
    pub value_raw: &'a String,
    pub conditions: &'a RuleConditions,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ShowRuleResponse {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub extensions: RuleOutputExtensions,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListRulesResponse {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "value")]
    pub rules: Vec<ShowRuleResponse>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "position")]
pub enum MoveToPositionRequest {
    #[serde(rename = "top_of_folder")]
    TopOfFolder { folder: String },
    #[serde(rename = "bottom_of_folder")]
    BottomOfFolder { folder: String },
    #[serde(rename = "after_specific_rule")]
    AfterSpecificRule { rule_id: String },
    #[serde(rename = "before_specific_rule")]
    BeforeSpecificRule { rule_id: String },
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RuleOutputExtensions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ruleset: Option<String>,
    pub folder: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_index: Option<i64>,
    pub properties: RuleProperties,
    pub value_raw: String,
    pub conditions: RuleConditions,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default = "default_option_false"
    )]
    pub disabled: Option<bool>,
}

fn default_option_false() -> Option<bool> {
    Some(false)
}

impl Default for RuleProperties {
    fn default() -> Self {
        Self {
            description: None,
            comment: None,
            documentation_url: None,
            disabled: Some(false),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RuleConditions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_name: Option<HostNameCondition>,
    #[serde(default)]
    pub host_tags: Vec<HostTagsCondition>,
    #[serde(default)]
    pub host_labels: Vec<HostLabelsCondition>,
    #[serde(default)]
    pub service_labels: Vec<ServiceLabelsCondition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_description: Option<ServiceDescriptionCondition>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HostNameCondition {
    #[serde(default)]
    pub match_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<HostNameConditionOperator>,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum HostNameConditionOperator {
    #[serde(rename = "one_of")]
    OneOf,
    #[serde(rename = "none_of")]
    NoneOf,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HostTagsCondition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<HostTagsConditionOperator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum HostTagsConditionOperator {
    #[serde(rename = "is")]
    Is,
    #[serde(rename = "is_not")]
    IsNot,
    #[serde(rename = "one_of")]
    OneOf,
    #[serde(rename = "none_of")]
    NoneOf,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HostLabelsCondition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<HostLabelsConditionOperator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum HostLabelsConditionOperator {
    #[serde(rename = "is")]
    Is,
    #[serde(rename = "is_not")]
    IsNot,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServiceLabelsCondition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<ServiceLabelsConditionOperator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ServiceLabelsConditionOperator {
    #[serde(rename = "is")]
    Is,
    #[serde(rename = "is_not")]
    IsNot,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServiceDescriptionCondition {
    #[serde(default)]
    pub match_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<ServiceDescriptionConditionOperator>,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ServiceDescriptionConditionOperator {
    #[serde(rename = "one_of")]
    OneOf,
    #[serde(rename = "none_of")]
    NoneOf,
}

pub trait RulesApi {
    fn rules(&self) -> RulesClient<'_>;
}

pub struct RulesClient<'a>(&'a Client);

impl RulesApi for Client {
    fn rules(&self) -> RulesClient<'_> {
        RulesClient(self)
    }
}

impl RulesClient<'_> {
    pub fn create_rule(&self, rule: &CreateRuleRequest) -> Result<ShowRuleResponse> {
        self.0.post("/domain-types/rule/collections/all", rule)
    }

    pub fn list_rules(&self, ruleset_name: &str) -> Result<ListRulesResponse> {
        self.0
            .get_with_action("/domain-types/rule/collections/all", |request_builder| {
                request_builder.query(&[("ruleset_name", ruleset_name)])
            })
    }

    pub fn delete_rule(&self, rule_id: &str) -> Result<()> {
        self.0.delete(format!("/objects/rule/{}", rule_id))
    }

    pub fn show_rule(&self, rule_id: &str) -> Result<ShowRuleResponse> {
        self.0.get(format!("/objects/rule/{}", rule_id))
    }

    pub fn move_rule_to_position(
        &self,
        rule_id: &str,
        move_to_position: &MoveToPositionRequest,
    ) -> Result<()> {
        self.0.post(
            format!("/objects/rule/{}/actions/move/invoke", rule_id),
            move_to_position,
        )
    }
}
