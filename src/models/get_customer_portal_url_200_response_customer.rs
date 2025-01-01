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
pub struct GetCustomerPortalUrl200ResponseCustomer {
    /// An embeddable link for displaying a customer portal
    #[serde(rename = "portal_url")]
    pub portal_url: String,
}

impl GetCustomerPortalUrl200ResponseCustomer {
    pub fn new(portal_url: String) -> GetCustomerPortalUrl200ResponseCustomer {
        GetCustomerPortalUrl200ResponseCustomer {
            portal_url,
        }
    }
}

