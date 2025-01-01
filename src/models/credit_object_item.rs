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

/// CreditObjectItem : The item attached to the credit.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreditObjectItem {
    /// Unique identifier assigned to the credit item within the Lago application.
    #[serde(rename = "lago_item_id")]
    pub lago_item_id: uuid::Uuid,
    /// The type of credit applied. Possible values are `coupon`, `credit_note` or `invoice` (for `progressive_billing` invoice).
    #[serde(rename = "type")]
    pub r#type: Type,
    /// The code of the credit applied. It can be the code of the coupon attached to the credit, the credit note's number or the `progressive_billing` invoice number.
    #[serde(rename = "code")]
    pub code: String,
    /// The name of the credit applied. It can be the name of the coupon attached to the credit, the initial invoice's number linked to the credit note or the `progressive_billing` invoice number.
    #[serde(rename = "name")]
    pub name: String,
}

impl CreditObjectItem {
    /// The item attached to the credit.
    pub fn new(lago_item_id: uuid::Uuid, r#type: Type, code: String, name: String) -> CreditObjectItem {
        CreditObjectItem {
            lago_item_id,
            r#type,
            code,
            name,
        }
    }
}
/// The type of credit applied. Possible values are `coupon`, `credit_note` or `invoice` (for `progressive_billing` invoice).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "coupon")]
    Coupon,
    #[serde(rename = "credit_note")]
    CreditNote,
    #[serde(rename = "invoice")]
    Invoice,
}

impl Default for Type {
    fn default() -> Type {
        Self::Coupon
    }
}
