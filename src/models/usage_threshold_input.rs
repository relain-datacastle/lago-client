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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageThresholdInput {
    /// Unique identifier of the usage threshold created by Lago.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// The display name of the usage threshold.
    #[serde(rename = "threshold_display_name", skip_serializing_if = "Option::is_none")]
    pub threshold_display_name: Option<String>,
    /// The amount to reach to trigger a `progressive_billing` invoice.
    #[serde(rename = "amount_cents")]
    pub amount_cents: i32,
    /// This field when set to `true` indicates that a `progressive_billing` invoice will be created every time the lifetime usage increases by the specified amount.
    #[serde(rename = "recurring")]
    pub recurring: bool,
}

impl UsageThresholdInput {
    pub fn new(amount_cents: i32, recurring: bool) -> UsageThresholdInput {
        UsageThresholdInput {
            id: None,
            threshold_display_name: None,
            amount_cents,
            recurring,
        }
    }
}

