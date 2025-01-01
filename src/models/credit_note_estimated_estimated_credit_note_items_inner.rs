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
pub struct CreditNoteEstimatedEstimatedCreditNoteItemsInner {
    /// The credit note's item amount, expressed in cents.
    #[serde(rename = "amount_cents")]
    pub amount_cents: i32,
    /// Unique identifier assigned to the fee within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the fee's record within the Lago system.
    #[serde(rename = "lago_fee_id")]
    pub lago_fee_id: uuid::Uuid,
}

impl CreditNoteEstimatedEstimatedCreditNoteItemsInner {
    pub fn new(amount_cents: i32, lago_fee_id: uuid::Uuid) -> CreditNoteEstimatedEstimatedCreditNoteItemsInner {
        CreditNoteEstimatedEstimatedCreditNoteItemsInner {
            amount_cents,
            lago_fee_id,
        }
    }
}

