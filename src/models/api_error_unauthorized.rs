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
pub struct ApiErrorUnauthorized {
    #[serde(rename = "status")]
    pub status: i32,
    #[serde(rename = "error")]
    pub error: String,
}

impl ApiErrorUnauthorized {
    pub fn new(status: i32, error: String) -> ApiErrorUnauthorized {
        ApiErrorUnauthorized {
            status,
            error,
        }
    }
}

