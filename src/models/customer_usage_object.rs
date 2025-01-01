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
pub struct CustomerUsageObject {
    /// The lower bound of the billing period, expressed in the ISO 8601 datetime format in Coordinated Universal Time (UTC).
    #[serde(rename = "from_datetime")]
    pub from_datetime: String,
    /// The upper bound of the billing period, expressed in the ISO 8601 datetime format in Coordinated Universal Time (UTC).
    #[serde(rename = "to_datetime")]
    pub to_datetime: String,
    /// The date of creation of the invoice.
    #[serde(rename = "issuing_date")]
    pub issuing_date: String,
    /// A unique identifier associated with the invoice related to this particular usage record.
    #[serde(rename = "lago_invoice_id", skip_serializing_if = "Option::is_none")]
    pub lago_invoice_id: Option<uuid::Uuid>,
    /// The currency of the customer's current usage.
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<models::Currency>,
    /// The amount in cents, tax excluded.
    #[serde(rename = "amount_cents")]
    pub amount_cents: i32,
    /// The tax amount in cents.
    #[serde(rename = "taxes_amount_cents")]
    pub taxes_amount_cents: i32,
    /// The total amount in cents, tax included.
    #[serde(rename = "total_amount_cents")]
    pub total_amount_cents: i32,
    /// Array of charges that comprise the current usage. It contains detailed information about individual charge items associated with the usage.
    #[serde(rename = "charges_usage")]
    pub charges_usage: Vec<models::CustomerChargeUsageObject>,
}

impl CustomerUsageObject {
    pub fn new(from_datetime: String, to_datetime: String, issuing_date: String, amount_cents: i32, taxes_amount_cents: i32, total_amount_cents: i32, charges_usage: Vec<models::CustomerChargeUsageObject>) -> CustomerUsageObject {
        CustomerUsageObject {
            from_datetime,
            to_datetime,
            issuing_date,
            lago_invoice_id: None,
            currency: None,
            amount_cents,
            taxes_amount_cents,
            total_amount_cents,
            charges_usage,
        }
    }
}

