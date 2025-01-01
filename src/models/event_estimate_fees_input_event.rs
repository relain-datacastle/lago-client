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
pub struct EventEstimateFeesInputEvent {
    /// The code that identifies a targeted billable metric. It is essential that this code matches the `code` property of one of your active billable metrics. If the provided code does not correspond to any active billable metric, it will be ignored during the process.
    #[serde(rename = "code")]
    pub code: String,
    /// The unique identifier of the subscription within your application.
    #[serde(rename = "external_subscription_id")]
    pub external_subscription_id: String,
    /// This field represents additional properties associated with the event, which are utilized in the calculation of the final fee. This object becomes mandatory when the targeted billable metric employs a `sum_agg`, `max_agg`, or `unique_count_agg` aggregation method. However, when using a simple `count_agg`, this object is not required.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

impl EventEstimateFeesInputEvent {
    pub fn new(code: String, external_subscription_id: String) -> EventEstimateFeesInputEvent {
        EventEstimateFeesInputEvent {
            code,
            external_subscription_id,
            properties: None,
        }
    }
}

