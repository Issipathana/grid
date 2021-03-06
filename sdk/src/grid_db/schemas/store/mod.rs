// Copyright 2018-2020 Cargill Incorporated
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

#[cfg(feature = "diesel")]
pub mod diesel;
pub mod memory;

use crate::grid_db::schemas::error::SchemaStoreError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Schema {
    pub name: String,
    pub description: String,
    pub owner: String,
    pub properties: Vec<PropertyDefinition>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    pub start_commit_num: i64,
    pub end_commit_num: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyDefinition {
    pub start_commit_num: i64,
    pub end_commit_num: i64,
    pub name: String,
    pub schema_name: String,
    pub data_type: String,
    pub required: bool,
    pub description: String,
    pub number_exponent: i64,
    pub enum_options: Vec<String>,
    pub struct_properties: Vec<PropertyDefinition>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
}

pub trait SchemaStore: Send + Sync {
    /// Adds a new schema to underlying storage
    ///
    /// # Arguments
    ///
    ///  * `schema` - The new schema to be added
    fn add_schema(&self, schema: &Schema) -> Result<(), SchemaStoreError>;

    /// Retrieve a schema from the underlying storage
    ///
    /// # Arguments
    ///
    ///  * `name` - Name of schema being fetched
    ///  * `service_id` - Sevice ID needed for when the source of the schema
    ///  is a splinter circuit
    fn fetch_schema(
        &self,
        name: &str,
        service_id: Option<&str>,
    ) -> Result<Option<Schema>, SchemaStoreError>;

    /// List all schemas in underlying storage
    ///
    /// # Arguments
    ///
    ///  * `service_id` - Sevice ID needed for when the source of the schema
    ///  is a splinter circuit
    fn list_schemas(&self, service_id: Option<&str>) -> Result<Vec<Schema>, SchemaStoreError>;
}
