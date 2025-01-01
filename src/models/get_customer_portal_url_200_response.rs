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
pub struct GetCustomerPortalUrl200Response {
    #[serde(rename = "customer")]
    pub customer: Box<models::GetCustomerPortalUrl200ResponseCustomer>,
}

impl GetCustomerPortalUrl200Response {
    pub fn new(customer: models::GetCustomerPortalUrl200ResponseCustomer) -> GetCustomerPortalUrl200Response {
        GetCustomerPortalUrl200Response {
            customer: Box::new(customer),
        }
    }
}

