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
pub struct WebhookEndpointObject {
    /// Unique identifier assigned to the wallet within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the webhook endpoint's record within the Lago system.
    #[serde(rename = "lago_id")]
    pub lago_id: uuid::Uuid,
    /// Unique identifier assigned to the organization attached to the webhook endpoint within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the organization's record within the Lago system.
    #[serde(rename = "lago_organization_id")]
    pub lago_organization_id: uuid::Uuid,
    /// The name of the wallet.
    #[serde(rename = "webhook_url")]
    pub webhook_url: String,
    /// The signature algo for the webhook.
    #[serde(rename = "signature_algo", skip_serializing_if = "Option::is_none")]
    pub signature_algo: Option<SignatureAlgo>,
    /// The date of the webhook endpoint creation, represented in ISO 8601 datetime format and expressed in Coordinated Universal Time (UTC).
    #[serde(rename = "created_at")]
    pub created_at: String,
}

impl WebhookEndpointObject {
    pub fn new(lago_id: uuid::Uuid, lago_organization_id: uuid::Uuid, webhook_url: String, created_at: String) -> WebhookEndpointObject {
        WebhookEndpointObject {
            lago_id,
            lago_organization_id,
            webhook_url,
            signature_algo: None,
            created_at,
        }
    }
}
/// The signature algo for the webhook.
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

