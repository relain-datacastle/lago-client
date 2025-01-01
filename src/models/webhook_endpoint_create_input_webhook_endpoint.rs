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
pub struct WebhookEndpointCreateInputWebhookEndpoint {
    /// The URL of the webhook endpoint.
    #[serde(rename = "webhook_url")]
    pub webhook_url: String,
    /// The signature used for the webhook. If no value is passed,
    #[serde(rename = "signature_algo", skip_serializing_if = "Option::is_none")]
    pub signature_algo: Option<SignatureAlgo>,
}

impl WebhookEndpointCreateInputWebhookEndpoint {
    pub fn new(webhook_url: String) -> WebhookEndpointCreateInputWebhookEndpoint {
        WebhookEndpointCreateInputWebhookEndpoint {
            webhook_url,
            signature_algo: None,
        }
    }
}
/// The signature used for the webhook. If no value is passed,
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SignatureAlgo {
    #[serde(rename = "jwt")]
    Jwt,
    #[serde(rename = "hmac")]
    Hmac,
}

impl Default for SignatureAlgo {
    fn default() -> SignatureAlgo {
        Self::Jwt
    }
}
