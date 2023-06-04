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

pub mod changes;
pub mod folders;
pub mod hosts;
pub mod models;

use reqwest::header;
use serde::{
    de::DeserializeOwned,
    Serialize,
};
use thiserror::Error;

type ETag = String;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("HTTP request failed: {0:?}")]
    HttpRequestError(#[from] reqwest::Error),
    #[error("HTTP response did not contain an ETag or it was invalid")]
    MissingOrInvalidETagError,
    #[error("HTTP request could not be enriched by header: {0}")]
    HttpRequestInvalidHeaderValue(String),
}

impl ClientError {
    pub fn is_status(&self, status_code: u16) -> bool {
        match self {
            Self::HttpRequestError(error) => {
                error.is_status() && error.status().unwrap() == status_code
            }
            _ => false,
        }
    }
}

impl From<header::InvalidHeaderValue> for ClientError {
    fn from(value: header::InvalidHeaderValue) -> Self {
        Self::HttpRequestInvalidHeaderValue(format!("{}", value))
    }
}

pub type Result<T> = std::result::Result<T, ClientError>;

pub struct Client {
    pub http_client: reqwest::blocking::Client,
    pub server_url: String,
    pub site: String,
}

impl Client {
    pub fn new<S: AsRef<str>>(server_url: S, site: S, username: S, secret: S) -> Result<Self> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!(
                "Bearer {} {}",
                username.as_ref(),
                secret.as_ref()
            ))?,
        );
        let http_client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self {
            http_client,
            server_url: server_url.as_ref().to_owned(),
            site: site.as_ref().to_owned(),
        })
    }

    fn url_for_endpoint<S: AsRef<str>>(&self, endpoint: S) -> String {
        format!(
            "{}/{}/check_mk/api/1.0{}{}",
            self.server_url,
            self.site,
            if endpoint.as_ref().starts_with('/') {
                ""
            } else {
                "/"
            },
            endpoint.as_ref()
        )
    }

    fn get_with_etag<O: DeserializeOwned, S: AsRef<str>>(&self, endpoint: S) -> Result<(O, ETag)> {
        self.http_client
            .get(self.url_for_endpoint(endpoint))
            .send()?
            .error_for_status()?
            .json_with_etag()
    }

    fn post<I: Serialize, O: DeserializeOwned, S: AsRef<str>>(
        &self,
        endpoint: S,
        body: &I,
    ) -> Result<O> {
        self.http_client
            .post(self.url_for_endpoint(endpoint))
            .json(body)
            .send()?
            .error_for_status()?
            .json()
            .map_err(Into::into)
    }

    fn post_if_match<I: Serialize, O: DeserializeOwned, S: AsRef<str>>(
        &self,
        endpoint: S,
        if_match: ETag,
        body: &I,
    ) -> Result<O> {
        self.http_client
            .post(self.url_for_endpoint(endpoint))
            .header(header::IF_MATCH, header::HeaderValue::from_str(&if_match)?)
            .json(body)
            .send()?
            .error_for_status()?
            .json()
            .map_err(Into::into)
    }

    fn post_with_etag<I: Serialize, O: DeserializeOwned, S: AsRef<str>>(
        &self,
        endpoint: S,
        body: &I,
    ) -> Result<(O, ETag)> {
        self.http_client
            .post(self.url_for_endpoint(endpoint))
            .json(body)
            .send()?
            .error_for_status()?
            .json_with_etag()
    }

    fn post_if_match_with_etag<I: Serialize, O: DeserializeOwned, S: AsRef<str>>(
        &self,
        endpoint: S,
        if_match: ETag,
        body: &I,
    ) -> Result<(O, ETag)> {
        self.http_client
            .post(self.url_for_endpoint(endpoint))
            .header(header::IF_MATCH, header::HeaderValue::from_str(&if_match)?)
            .json(body)
            .send()?
            .error_for_status()?
            .json_with_etag()
    }

    fn put_if_match_with_etag<I: Serialize, O: DeserializeOwned, S: AsRef<str>>(
        &self,
        endpoint: S,
        if_match: ETag,
        body: &I,
    ) -> Result<(O, ETag)> {
        self.http_client
            .put(self.url_for_endpoint(endpoint))
            .header(header::IF_MATCH, header::HeaderValue::from_str(&if_match)?)
            .json(body)
            .send()?
            .error_for_status()?
            .json_with_etag()
    }
}

trait ResponseExt<T> {
    fn json_with_etag(self) -> Result<(T, ETag)>;
}

impl<T> ResponseExt<T> for reqwest::blocking::Response
where
    T: DeserializeOwned,
{
    fn json_with_etag(self) -> Result<(T, ETag)> {
        let etag = self
            .headers()
            .get(header::ETAG)
            .ok_or(ClientError::MissingOrInvalidETagError)
            .and_then(|etag_header_value| {
                etag_header_value
                    .to_str()
                    .map_err(|_| ClientError::MissingOrInvalidETagError)
            })?
            .to_owned();
        Ok((self.json()?, etag))
    }
}
