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
pub struct Fee {
    #[serde(rename = "fee")]
    pub fee: Box<models::FeeObject>,
}

impl Fee {
    pub fn new(fee: models::FeeObject) -> Fee {
        Fee {
            fee: Box::new(fee),
        }
    }
}

