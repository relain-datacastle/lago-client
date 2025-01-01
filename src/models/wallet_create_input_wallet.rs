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
pub struct WalletCreateInputWallet {
    /// The name of the wallet.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The rate of conversion between credits and the amount in the specified currency. It indicates the ratio or factor used to convert credits into the corresponding monetary value in the currency of the transaction.
    #[serde(rename = "rate_amount")]
    pub rate_amount: String,
    /// The currency of the wallet.
    #[serde(rename = "currency")]
    pub currency: models::Currency,
    /// The number of paid credits. Required only if there is no granted credits.
    #[serde(rename = "paid_credits", skip_serializing_if = "Option::is_none")]
    pub paid_credits: Option<String>,
    /// The number of free granted credits. Required only if there is no paid credits.
    #[serde(rename = "granted_credits", skip_serializing_if = "Option::is_none")]
    pub granted_credits: Option<String>,
    /// The customer external unique identifier (provided by your own application)
    #[serde(rename = "external_customer_id")]
    pub external_customer_id: String,
    /// The date and time that determines when the wallet will expire. It follows the ISO 8601 datetime format and is expressed in Coordinated Universal Time (UTC).
    #[serde(rename = "expiration_at", skip_serializing_if = "Option::is_none")]
    pub expiration_at: Option<String>,
    /// A boolean setting that, when set to true, delays issuing an invoice for a wallet top-up until a successful payment is made; if false, the invoice is issued immediately upon wallet top-up, regardless of the payment status. Default value of false.
    #[serde(rename = "invoice_requires_successful_payment", skip_serializing_if = "Option::is_none")]
    pub invoice_requires_successful_payment: Option<bool>,
    /// This optional field allows you to store a list of key-value pairs that provide additional information or custom attributes. These key-value pairs will be included in the metadata of wallet transactions generated during the wallet creation process.
    #[serde(rename = "transaction_metadata", skip_serializing_if = "Option::is_none")]
    pub transaction_metadata: Option<Vec<models::WalletCreateInputWalletTransactionMetadataInner>>,
    /// List of recurring transaction rules. Currently, we only allow one recurring rule per wallet.
    #[serde(rename = "recurring_transaction_rules", skip_serializing_if = "Option::is_none")]
    pub recurring_transaction_rules: Option<Vec<models::WalletCreateInputWalletRecurringTransactionRulesInner>>,
}

impl WalletCreateInputWallet {
    pub fn new(rate_amount: String, currency: models::Currency, external_customer_id: String) -> WalletCreateInputWallet {
        WalletCreateInputWallet {
            name: None,
            rate_amount,
            currency,
            paid_credits: None,
            granted_credits: None,
            external_customer_id,
            expiration_at: None,
            invoice_requires_successful_payment: None,
            transaction_metadata: None,
            recurring_transaction_rules: None,
        }
    }
}
