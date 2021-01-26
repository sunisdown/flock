// Copyright (c) 2020-2021, UMD Database Group. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A Query API to associate front-end CLI with back-end function generation and
//! continuous deployment.

#![warn(missing_docs)]
// Clippy lints, some should be disabled incrementally
#![allow(
    clippy::float_cmp,
    clippy::module_inception,
    clippy::new_without_default,
    clippy::ptr_arg,
    clippy::type_complexity,
    clippy::wrong_self_convention
)]

use arrow::datatypes::SchemaRef;
use datafusion::physical_plan::ExecutionPlan;
use std::sync::Arc;

/// A `Query` trait to decouple CLI and back-end cloud function generation.
pub trait Query {
    /// Returns a SQL query.
    fn sql(&self) -> &String;
    /// Returns the data schema for a given query.
    fn schema(&self) -> &Option<SchemaRef>;
    /// Returns the entire physical plan for a given query.
    fn plan(&self) -> Arc<dyn ExecutionPlan>;
}

pub mod batch;
pub mod stream;

pub use batch::BatchQuery;
pub use stream::StreamQuery;
