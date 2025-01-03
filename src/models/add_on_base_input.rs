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
pub struct AddOnBaseInput {
    /// The name of the add-on.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specifies the name that will be displayed on an invoice. If no value is set for this field, the name of the actual charge will be used as the default display name.
    #[serde(rename = "invoice_display_name", skip_serializing_if = "Option::is_none")]
    pub invoice_display_name: Option<String>,
    /// Unique code used to identify the add-on.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// The cost of the add-on in cents, excluding any applicable taxes, that is billed to a customer. By creating a one-off invoice, you will be able to override this value.
    #[serde(rename = "amount_cents", skip_serializing_if = "Option::is_none")]
    pub amount_cents: Option<i32>,
    /// The currency of the add-on.
    #[serde(rename = "amount_currency", skip_serializing_if = "Option::is_none")]
    pub amount_currency: Option<models::Currency>,
    /// The description of the add-on.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// List of unique code used to identify the taxes.
    #[serde(rename = "tax_codes", skip_serializing_if = "Option::is_none")]
    pub tax_codes: Option<Vec<String>>,
}

impl AddOnBaseInput {
    pub fn new() -> AddOnBaseInput {
        AddOnBaseInput {
            name: None,
            invoice_display_name: None,
            code: None,
            amount_cents: None,
            amount_currency: None,
            description: None,
            tax_codes: None,
        }
    }
}

