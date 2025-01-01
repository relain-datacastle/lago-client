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
pub struct InvoicedUsageObject {
    /// Identifies the month to analyze revenue.
    #[serde(rename = "month")]
    pub month: String,
    /// The code of the usage-based billable metrics.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// The total amount of revenue for a period, expressed in cents.
    #[serde(rename = "amount_cents")]
    pub amount_cents: i32,
    /// The currency of revenue analytics. Format must be ISO 4217.
    #[serde(rename = "currency")]
    pub currency: models::Currency,
}

impl InvoicedUsageObject {
    pub fn new(month: String, amount_cents: i32, currency: models::Currency) -> InvoicedUsageObject {
        InvoicedUsageObject {
            month,
            code: None,
            amount_cents,
            currency,
        }
    }
}
