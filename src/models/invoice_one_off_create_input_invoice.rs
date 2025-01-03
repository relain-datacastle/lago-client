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
pub struct InvoiceOneOffCreateInputInvoice {
    /// Unique identifier assigned to the customer in your application.
    #[serde(rename = "external_customer_id")]
    pub external_customer_id: String,
    /// The currency of the invoice.
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<models::Currency>,
    #[serde(rename = "fees")]
    pub fees: Vec<models::InvoiceOneOffCreateInputInvoiceFeesInner>,
}

impl InvoiceOneOffCreateInputInvoice {
    pub fn new(external_customer_id: String, fees: Vec<models::InvoiceOneOffCreateInputInvoiceFeesInner>) -> InvoiceOneOffCreateInputInvoice {
        InvoiceOneOffCreateInputInvoice {
            external_customer_id,
            currency: None,
            fees,
        }
    }
}

