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
pub struct InvoiceUpdateInput {
    #[serde(rename = "invoice")]
    pub invoice: Box<models::InvoiceUpdateInputInvoice>,
}

impl InvoiceUpdateInput {
    pub fn new(invoice: models::InvoiceUpdateInputInvoice) -> InvoiceUpdateInput {
        InvoiceUpdateInput {
            invoice: Box::new(invoice),
        }
    }
}

