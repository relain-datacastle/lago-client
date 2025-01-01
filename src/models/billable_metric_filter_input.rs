/*
 * Lago API documentation
 *
 * Lago API allows your application to push customer information and metrics (events) from your application to the billing application.
 *
 * The version of the OpenAPI document: 1.17.1
 * Contact: tech@getlago.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// BillableMetricFilterInput : Values used to apply differentiated pricing based on additional event properties.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BillableMetricFilterInput {
    /// Filter key to add to the event properties payload
    #[serde(rename = "key")]
    pub key: String,
    /// List of possible filter values
    #[serde(rename = "values")]
    pub values: Vec<String>,
}

impl BillableMetricFilterInput {
    /// Values used to apply differentiated pricing based on additional event properties.
    pub fn new(key: String, values: Vec<String>) -> BillableMetricFilterInput {
        BillableMetricFilterInput {
            key,
            values,
        }
    }
}

