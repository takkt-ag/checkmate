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

use clap::Parser;
use std::path::PathBuf;

/// Configure checkmk declaratively using checkmate by providing a configuration file.
#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Args {
    /// URL to the checkmk server.
    ///
    /// If checkmk is not running at the root-path, please include the required prefix here.
    #[arg(long, env = "CHECKMATE_CHECKMK_SERVER_URL")]
    pub server_url: String,
    /// The checkmk site to configure.
    #[arg(long, env = "CHECKMATE_CHECKMK_SITE")]
    pub site: String,
    /// The username to use for authentication.
    #[arg(long, default_value = "automation", env = "CHECKMATE_CHECKMK_USERNAME")]
    pub username: String,
    /// The secret to use for authentication.
    ///
    /// You should preferably provide this through the environment variable `CHECKMATE_SECRET`.
    #[arg(long, env = "CHECKMATE_CHECKMK_SECRET")]
    pub secret: String,
    /// The configuration file to use.
    #[arg(long, default_value = "checkmate.yaml", env = "CHECKMATE_CONFIG_FILE")]
    pub config_file: PathBuf,
}
