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

use crate::{
    config::Folder,
    Result,
};
use checkmk_client::rules::{
    CreateRuleRequest,
    RuleConditions,
    RuleProperties,
    RulesApi,
    ShowRuleResponse,
};
use serde::Deserialize;
use std::rc::{
    Rc,
    Weak,
};

#[derive(Debug, Clone, Deserialize)]
pub struct Ruleset {
    #[serde(default)]
    pub name: String,
    pub rules: Vec<Rule>,
    #[serde(default, deserialize_with = "crate::de::deserialize_to_empty_weak")]
    pub folder: Weak<Folder>,
}

impl Ruleset {
    fn folder(&self) -> Rc<Folder> {
        self.folder.upgrade().expect("folder weak ref is broken")
    }

    fn replace_all_rules(
        &self,
        rules_api: Vec<ShowRuleResponse>,
        cmk: &checkmk_client::Client,
    ) -> Result<()> {
        let rules = cmk.rules();
        for rule_api in rules_api {
            rules.delete_rule(&rule_api.id)?;
        }
        for rule in self.rules.iter() {
            // TODO: if this fails, the earlier deletions are still dormant. We'll have to recover
            //       from this somehow.
            rule.apply_to_site(cmk)?;
        }
        Ok(())
    }

    pub fn apply_to_site(&self, cmk: &checkmk_client::Client) -> Result<()> {
        // The implementation here is relatively naive. It will identify if there is any change
        // needed, and if so, it will delete all existing rules and (re-)add all rules defined. This
        // can result in a lot of changes, but it will ensure that the rules are both configured
        // correctly, and in the correct order. Since applying the rules is an atomic operation,
        // this should not be a problem.

        let rules_api = cmk
            .rules()
            .list_rules(&self.name)?
            .rules
            .into_iter()
            .filter(|rule_api| rule_api.extensions.folder == self.folder().path.to_string_lossy())
            .collect::<Vec<_>>();

        if rules_api.len() != self.rules.len() {
            println!(
                "[RULESETS] {} (in folder {}): different amount of rules, replacing all rules",
                self.name,
                self.folder().id()
            );
            return self.replace_all_rules(rules_api, cmk);
        }

        let any_mismatch = self
            .rules
            .iter()
            .zip(rules_api.iter())
            .any(|(rule, api_rule)| !rule.is_same_rule(api_rule) || rule.needs_update(api_rule));
        if any_mismatch {
            println!("[RULESETS] {} (in folder {}): at least one rule differs in configuration, replacing all rules", self.name, self.folder().id());
            return self.replace_all_rules(rules_api, cmk);
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Rule {
    /// The user's unique identifier of the rule.
    ///
    /// This is not the rule's UUID.
    #[serde(rename = "id")]
    pub custom_id: String,
    #[serde(default)]
    pub properties: RuleProperties,
    #[serde(default)]
    pub conditions: RuleConditions,
    pub value_raw: String,
    #[serde(default, deserialize_with = "crate::de::deserialize_to_empty_weak")]
    pub ruleset: Weak<Ruleset>,
}

impl Rule {
    fn ruleset(&self) -> Rc<Ruleset> {
        self.ruleset.upgrade().expect("ruleset weak ref is broken")
    }

    // TODO: the factoring of this is not great. The normalization having to be invoked after
    //       deserialization could result in this being forgotten, or this function accidentally not
    //       being idempotent and being called multiple times. Improving this should be considered.
    pub fn normalize_properties(&mut self, ruleset: &str) {
        let marker = self.marker(ruleset);
        if let Some(comment) = &mut self.properties.comment {
            if !comment.contains(&marker) {
                *comment = format!("{}\n{}", comment, marker);
            }
        } else {
            self.properties.comment = Some(marker);
        }
    }

    fn marker(&self, ruleset: &str) -> String {
        format!("[checkmate:{}:{}]", ruleset, self.custom_id)
    }

    fn is_same_rule(&self, api_rule: &ShowRuleResponse) -> bool {
        api_rule
            .extensions
            .properties
            .comment
            .as_ref()
            .map(|comment| comment.contains(&self.marker(&self.ruleset().name)))
            .unwrap_or(false)
    }

    fn needs_update(&self, api_rule: &ShowRuleResponse) -> bool {
        self.ruleset().folder().path.to_string_lossy() != api_rule.extensions.folder
            || self.conditions != api_rule.extensions.conditions
            || self.value_raw != api_rule.extensions.value_raw
            || self.properties != api_rule.extensions.properties
    }

    pub fn apply_to_site(&self, cmk: &checkmk_client::Client) -> Result<()> {
        // Applying the rule here means just creating it, rather than potentially updating an
        // existing one. This is caused by two factors: the first is that the Checkmk REST API does
        // not allow updating rules, and the second is that we want to ensure that the rules are in
        // the correct order, and recreating them in a deterministic order is an easy way to achieve
        // this.
        cmk.rules().create_rule(&self.into())?;
        Ok(())
    }
}

impl<'a> From<&'a Rule> for CreateRuleRequest<'a> {
    fn from(value: &'a Rule) -> Self {
        Self {
            ruleset: value.ruleset().name.clone(),
            folder: value.ruleset().folder().path.to_string_lossy().into_owned(),
            properties: &value.properties,
            value_raw: &value.value_raw,
            conditions: &value.conditions,
        }
    }
}
