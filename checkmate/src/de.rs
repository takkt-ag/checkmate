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

use serde::{
    Deserialize,
    Deserializer,
};
use std::{
    collections::HashMap,
    rc::{
        Rc,
        Weak,
    },
};

pub fn deserialize_map_values_as_rc<'de, D, K, V>(
    deserializer: D,
) -> Result<HashMap<K, Rc<V>>, D::Error>
where
    D: Deserializer<'de>,
    K: Deserialize<'de> + Eq + std::hash::Hash,
    V: Deserialize<'de>,
{
    let map: HashMap<K, V> = HashMap::deserialize(deserializer)?;
    Ok(map
        .into_iter()
        .map(|(key, value)| (key, Rc::new(value)))
        .collect())
}

pub fn deserialize_to_empty_weak<'de, D, T>(_: D) -> Result<Weak<T>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Weak::new())
}
