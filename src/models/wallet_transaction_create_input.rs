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
pub struct WalletTransactionCreateInput {
    #[serde(rename = "wallet_transaction")]
    pub wallet_transaction: Box<models::WalletTransactionCreateInputWalletTransaction>,
}

impl WalletTransactionCreateInput {
    pub fn new(wallet_transaction: models::WalletTransactionCreateInputWalletTransaction) -> WalletTransactionCreateInput {
        WalletTransactionCreateInput {
            wallet_transaction: Box::new(wallet_transaction),
        }
    }
}

