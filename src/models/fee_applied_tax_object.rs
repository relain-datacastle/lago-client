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
pub struct FeeAppliedTaxObject {
    /// Unique identifier of the fee, created by Lago.
    #[serde(rename = "lago_fee_id", skip_serializing_if = "Option::is_none")]
    pub lago_fee_id: Option<uuid::Uuid>,
    /// Unique identifier of the applied tax, created by Lago.
    #[serde(rename = "lago_id", skip_serializing_if = "Option::is_none")]
    pub lago_id: Option<uuid::Uuid>,
    /// Unique identifier of the tax, created by Lago.
    #[serde(rename = "lago_tax_id", skip_serializing_if = "Option::is_none")]
    pub lago_tax_id: Option<uuid::Uuid>,
    /// Name of the tax.
    #[serde(rename = "tax_name", skip_serializing_if = "Option::is_none")]
    pub tax_name: Option<String>,
    /// Unique code used to identify the tax associated with the API request.
    #[serde(rename = "tax_code", skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    /// The percentage rate of the tax
    #[serde(rename = "tax_rate", skip_serializing_if = "Option::is_none")]
    pub tax_rate: Option<f64>,
    /// Internal description of the taxe
    #[serde(rename = "tax_description", skip_serializing_if = "Option::is_none")]
    pub tax_description: Option<String>,
    /// Amount of the tax
    #[serde(rename = "amount_cents", skip_serializing_if = "Option::is_none")]
    pub amount_cents: Option<i32>,
    /// Currency of the tax
    #[serde(rename = "amount_currency", skip_serializing_if = "Option::is_none")]
    pub amount_currency: Option<models::Currency>,
    /// The date and time when the applied tax was created. It is expressed in UTC format according to the ISO 8601 datetime standard. This field provides the timestamp for the exact moment when the applied tax was initially created.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

impl FeeAppliedTaxObject {
    pub fn new() -> FeeAppliedTaxObject {
        FeeAppliedTaxObject {
            lago_fee_id: None,
            lago_id: None,
            lago_tax_id: None,
            tax_name: None,
            tax_code: None,
            tax_rate: None,
            tax_description: None,
            amount_cents: None,
            amount_currency: None,
            created_at: None,
        }
    }
}
