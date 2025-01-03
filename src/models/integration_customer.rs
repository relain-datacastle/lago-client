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

/// IntegrationCustomer : Configuration specific to the accounting and tax integrations. This object contains settings and parameters necessary for syncing documents and payments.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationCustomer {
    /// A unique identifier for the integration customer object in the Lago application.
    #[serde(rename = "lago_id", skip_serializing_if = "Option::is_none")]
    pub lago_id: Option<uuid::Uuid>,
    /// The integration type used for accounting and tax syncs. Accepted values: `netsuite, anrok`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// Unique code used to identify an integration connection.
    #[serde(rename = "integration_code", skip_serializing_if = "Option::is_none")]
    pub integration_code: Option<String>,
    /// The customer ID within the integration's system. If this field is not provided, Lago has the option to create a new customer record within the integration's system on behalf of the customer.
    #[serde(rename = "external_customer_id", skip_serializing_if = "Option::is_none")]
    pub external_customer_id: Option<String>,
    /// Set this field to `true` if you want to create a customer record in the integration's system. This option is applicable only when the `external_customer_id` is null and the `sync_with_provider` field is set to `true`. By default, the value is set to `false`
    #[serde(rename = "sync_with_provider", skip_serializing_if = "Option::is_none")]
    pub sync_with_provider: Option<bool>,
    /// This optional field is needed only when working with `netsuite` connection.
    #[serde(rename = "subsidiary_id", skip_serializing_if = "Option::is_none")]
    pub subsidiary_id: Option<String>,
}

impl IntegrationCustomer {
    /// Configuration specific to the accounting and tax integrations. This object contains settings and parameters necessary for syncing documents and payments.
    pub fn new() -> IntegrationCustomer {
        IntegrationCustomer {
            lago_id: None,
            r#type: None,
            integration_code: None,
            external_customer_id: None,
            sync_with_provider: None,
            subsidiary_id: None,
        }
    }
}
/// The integration type used for accounting and tax syncs. Accepted values: `netsuite, anrok`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "netsuite")]
    Netsuite,
    #[serde(rename = "anrok")]
    Anrok,
}

impl Default for Type {
    fn default() -> Type {
        Self::Netsuite
    }
}

