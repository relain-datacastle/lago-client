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
pub struct WalletsPaginated {
    #[serde(rename = "wallets")]
    pub wallets: Vec<models::WalletObject>,
    #[serde(rename = "meta")]
    pub meta: Box<models::PaginationMeta>,
}

impl WalletsPaginated {
    pub fn new(wallets: Vec<models::WalletObject>, meta: models::PaginationMeta) -> WalletsPaginated {
        WalletsPaginated {
            wallets,
            meta: Box::new(meta),
        }
    }
}

