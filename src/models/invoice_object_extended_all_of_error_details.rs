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
pub struct InvoiceObjectExtendedAllOfErrorDetails {
    /// Unique identifier assigned to the error_detail within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the error's record within the Lago system.
    #[serde(rename = "lago_id")]
    pub lago_id: uuid::Uuid,
    /// Code that specifies part of the application / connection, where the error originally happened
    #[serde(rename = "error_code")]
    pub error_code: String,
    /// Key value list of more elaborated error detail, where by the key of error_code an external service error details are stored
    #[serde(rename = "details")]
    pub details: serde_json::Value,
}

impl InvoiceObjectExtendedAllOfErrorDetails {
    pub fn new(lago_id: uuid::Uuid, error_code: String, details: serde_json::Value) -> InvoiceObjectExtendedAllOfErrorDetails {
        InvoiceObjectExtendedAllOfErrorDetails {
            lago_id,
            error_code,
            details,
        }
    }
}
