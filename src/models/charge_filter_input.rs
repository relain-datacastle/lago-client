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

/// ChargeFilterInput : Values used to apply differentiated pricing based on additional event properties.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChargeFilterInput {
    /// Specifies the name that will be displayed on an invoice. If no value is set for this field, the values of the filter will be used as the default display name.
    #[serde(rename = "invoice_display_name", skip_serializing_if = "Option::is_none")]
    pub invoice_display_name: Option<String>,
    /// List of all thresholds utilized for calculating the charge.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Box<models::ChargeProperties>>,
    /// List of possible filter values. The key and values must match one of the billable metric filters.
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<std::collections::HashMap<String, Vec<String>>>,
}

impl ChargeFilterInput {
    /// Values used to apply differentiated pricing based on additional event properties.
    pub fn new() -> ChargeFilterInput {
        ChargeFilterInput {
            invoice_display_name: None,
            properties: None,
            values: None,
        }
    }
}

