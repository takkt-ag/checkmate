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
    Result,
};
use crate::ETag;
use serde::{
    Deserialize,
    Serialize,
};

pub trait ChangesApi {
    fn changes(&self) -> ChangesClient;
}

pub struct ChangesClient<'a>(&'a Client);

impl ChangesApi for Client {
    fn changes(&self) -> ChangesClient {
        ChangesClient(self)
    }
}

impl<'a> ChangesClient<'a> {
    pub fn show_all_pending_changes(&self) -> Result<(ShowAllPendingChangesResponse, ETag)> {
        self.0
            .get_with_etag("/domain-types/activation_run/collections/pending_changes")
    }

    pub fn wait_for_activation_completion(&self, id: &str) -> Result<()> {
        self.0
            .http_client
            .get(self.0.url_for_endpoint(format!(
                "/objects/activation_run/{id}/actions/wait-for-completion/invoke"
            )))
            .send()
            .map(|_| ())
            .map_err(Into::into)
    }

    pub fn activate_pending_changes(&self, etag: ETag) -> Result<ActivatePendingChangesResponse> {
        self.0.post_if_match(
            "/domain-types/activation_run/actions/activate-changes/invoke",
            etag,
            &ActivatePendingChangesRequest {
                redirect: false,
                sites: vec![self.0.site.clone()],
                force_foreign_changes: false,
            },
        )
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ShowAllPendingChangesResponse {
    #[serde(rename = "value")]
    pub pending_changes: Vec<PendingChange>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PendingChange {
    pub id: String,
    pub user_id: String,
    pub action_name: String,
    pub text: String,
    pub time: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct ActivatePendingChangesRequest {
    redirect: bool,
    sites: Vec<String>,
    force_foreign_changes: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ActivatePendingChangesResponse {
    pub id: String,
}
