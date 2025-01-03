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
pub struct AppliedCouponObject {
    /// Unique identifier of the applied coupon, created by Lago.
    #[serde(rename = "lago_id")]
    pub lago_id: uuid::Uuid,
    /// Unique identifier of the coupon, created by Lago.
    #[serde(rename = "lago_coupon_id")]
    pub lago_coupon_id: uuid::Uuid,
    /// Unique code used to identify the coupon.
    #[serde(rename = "coupon_code")]
    pub coupon_code: String,
    /// The name of the coupon.
    #[serde(rename = "coupon_name")]
    pub coupon_name: String,
    /// Unique identifier of the customer, created by Lago.
    #[serde(rename = "lago_customer_id")]
    pub lago_customer_id: uuid::Uuid,
    /// The customer external unique identifier (provided by your own application)
    #[serde(rename = "external_customer_id")]
    pub external_customer_id: String,
    /// The status of the coupon. Can be either `active` or `terminated`.
    #[serde(rename = "status")]
    pub status: Status,
    /// The amount of the coupon in cents. This field is required only for coupon with `fixed_amount` type.
    #[serde(rename = "amount_cents", skip_serializing_if = "Option::is_none")]
    pub amount_cents: Option<i32>,
    /// The remaining amount in cents for a `fixed_amount` coupon with a frequency set to `once`. This field indicates the remaining balance or value that can still be utilized from the coupon.
    #[serde(rename = "amount_cents_remaining", skip_serializing_if = "Option::is_none")]
    pub amount_cents_remaining: Option<i32>,
    /// The currency of the coupon. This field is required only for coupon with `fixed_amount` type.
    #[serde(rename = "amount_currency", skip_serializing_if = "Option::is_none")]
    pub amount_currency: Option<models::Currency>,
    /// The percentage rate of the coupon. This field is required only for coupons with a `percentage` coupon type.
    #[serde(rename = "percentage_rate", skip_serializing_if = "Option::is_none")]
    pub percentage_rate: Option<String>,
    /// The type of frequency for the coupon. It can have three possible values: `once`, `recurring` or `forever`.  - If set to `once`, the coupon is applicable only for a single use. - If set to `recurring`, the coupon can be used multiple times for recurring billing periods. - If set to `forever`, the coupon has unlimited usage and can be applied indefinitely.
    #[serde(rename = "frequency")]
    pub frequency: Frequency,
    /// Specifies the number of billing periods to which the coupon applies. This field is required only for coupons with a `recurring` frequency type
    #[serde(rename = "frequency_duration", skip_serializing_if = "Option::is_none")]
    pub frequency_duration: Option<i32>,
    /// The remaining number of billing periods to which the coupon is applicable. This field determines the remaining usage or availability of the coupon based on the remaining billing periods.
    #[serde(rename = "frequency_duration_remaining", skip_serializing_if = "Option::is_none")]
    pub frequency_duration_remaining: Option<i32>,
    /// The date and time after which the coupon will stop applying to customer's invoices. Once the expiration date is reached, the coupon will no longer be applicable, and any further invoices generated for the customer will not include the coupon discount.
    #[serde(rename = "expiration_at", skip_serializing_if = "Option::is_none")]
    pub expiration_at: Option<String>,
    /// The date and time when the coupon was assigned to a customer. It is expressed in UTC format according to the ISO 8601 datetime standard.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// This field indicates the specific moment when the coupon amount is fully utilized or when the coupon is removed from the customer's coupon list. It is expressed in UTC format according to the ISO 8601 datetime standard.
    #[serde(rename = "terminated_at", skip_serializing_if = "Option::is_none")]
    pub terminated_at: Option<String>,
}

impl AppliedCouponObject {
    pub fn new(lago_id: uuid::Uuid, lago_coupon_id: uuid::Uuid, coupon_code: String, coupon_name: String, lago_customer_id: uuid::Uuid, external_customer_id: String, status: Status, frequency: Frequency, created_at: String) -> AppliedCouponObject {
        AppliedCouponObject {
            lago_id,
            lago_coupon_id,
            coupon_code,
            coupon_name,
            lago_customer_id,
            external_customer_id,
            status,
            amount_cents: None,
            amount_cents_remaining: None,
            amount_currency: None,
            percentage_rate: None,
            frequency,
            frequency_duration: None,
            frequency_duration_remaining: None,
            expiration_at: None,
            created_at,
            terminated_at: None,
        }
    }
}
/// The status of the coupon. Can be either `active` or `terminated`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "terminated")]
    Terminated,
}

impl Default for Status {
    fn default() -> Status {
        Self::Active
    }
}
/// The type of frequency for the coupon. It can have three possible values: `once`, `recurring` or `forever`.  - If set to `once`, the coupon is applicable only for a single use. - If set to `recurring`, the coupon can be used multiple times for recurring billing periods. - If set to `forever`, the coupon has unlimited usage and can be applied indefinitely.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Frequency {
    #[serde(rename = "once")]
    Once,
    #[serde(rename = "recurring")]
    Recurring,
    #[serde(rename = "forever")]
    Forever,
}

impl Default for Frequency {
    fn default() -> Frequency {
        Self::Once
    }
}

