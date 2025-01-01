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
pub struct SubscriptionCreateInputSubscription {
    /// The customer external unique identifier (provided by your own application)
    #[serde(rename = "external_customer_id")]
    pub external_customer_id: String,
    /// The unique code representing the plan to be attached to the customer. This code must correspond to the `code` property of one of the active plans.
    #[serde(rename = "plan_code")]
    pub plan_code: String,
    /// The display name of the subscription on an invoice. This field allows for customization of the subscription's name for billing purposes, especially useful when a single customer has multiple subscriptions using the same plan.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The unique external identifier for the subscription. This identifier serves as an idempotency key, ensuring that each subscription is unique.
    #[serde(rename = "external_id")]
    pub external_id: String,
    /// The billing time for the subscription, which can be set as either `anniversary` or `calendar`. If not explicitly provided, it will default to `calendar`. The billing time determines the timing of recurring billing cycles for the subscription. By specifying `anniversary`, the billing cycle will be based on the specific date the subscription started (billed fully), while `calendar` sets the billing cycle at the first day of the week/month/year (billed with proration).
    #[serde(rename = "billing_time", skip_serializing_if = "Option::is_none")]
    pub billing_time: Option<BillingTime>,
    /// The effective end date of the subscription. If this field is set to null, the subscription will automatically renew. This date should be provided in ISO 8601 datetime format, and use Coordinated Universal Time (UTC).
    #[serde(rename = "ending_at", skip_serializing_if = "Option::is_none")]
    pub ending_at: Option<String>,
    /// The start date for the subscription, allowing for the creation of subscriptions that can begin in the past or future. Please note that it cannot be used to update the start date of a pending subscription or schedule an upgrade/downgrade. The start_date should be provided in ISO 8601 datetime format and expressed in Coordinated Universal Time (UTC).
    #[serde(rename = "subscription_at", skip_serializing_if = "Option::is_none")]
    pub subscription_at: Option<String>,
    #[serde(rename = "plan_overrides", skip_serializing_if = "Option::is_none")]
    pub plan_overrides: Option<Box<models::PlanOverridesObject>>,
}

impl SubscriptionCreateInputSubscription {
    pub fn new(external_customer_id: String, plan_code: String, external_id: String) -> SubscriptionCreateInputSubscription {
        SubscriptionCreateInputSubscription {
            external_customer_id,
            plan_code,
            name: None,
            external_id,
            billing_time: None,
            ending_at: None,
            subscription_at: None,
            plan_overrides: None,
        }
    }
}
/// The billing time for the subscription, which can be set as either `anniversary` or `calendar`. If not explicitly provided, it will default to `calendar`. The billing time determines the timing of recurring billing cycles for the subscription. By specifying `anniversary`, the billing cycle will be based on the specific date the subscription started (billed fully), while `calendar` sets the billing cycle at the first day of the week/month/year (billed with proration).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BillingTime {
    #[serde(rename = "calendar")]
    Calendar,
    #[serde(rename = "anniversary")]
    Anniversary,
}

impl Default for BillingTime {
    fn default() -> BillingTime {
        Self::Calendar
    }
}
