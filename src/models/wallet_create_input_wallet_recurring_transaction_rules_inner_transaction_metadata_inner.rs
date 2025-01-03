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
pub struct WalletCreateInputWalletRecurringTransactionRulesInnerTransactionMetadataInner {
    /// The unique identifier for the attribute.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The value associated with the key.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl WalletCreateInputWalletRecurringTransactionRulesInnerTransactionMetadataInner {
    pub fn new() -> WalletCreateInputWalletRecurringTransactionRulesInnerTransactionMetadataInner {
        WalletCreateInputWalletRecurringTransactionRulesInnerTransactionMetadataInner {
            key: None,
            value: None,
        }
    }
}

