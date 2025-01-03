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
pub struct SubscriptionObjectExtended {
    /// Unique identifier assigned to the subscription within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the subscription's record within the Lago system
    #[serde(rename = "lago_id")]
    pub lago_id: uuid::Uuid,
    /// The subscription external unique identifier (provided by your own application).
    #[serde(rename = "external_id")]
    pub external_id: String,
    /// Unique identifier assigned to the customer within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the customer's record within the Lago system
    #[serde(rename = "lago_customer_id")]
    pub lago_customer_id: uuid::Uuid,
    /// The customer external unique identifier (provided by your own application).
    #[serde(rename = "external_customer_id")]
    pub external_customer_id: String,
    /// The billing time for the subscription, which can be set as either `anniversary` or `calendar`. If not explicitly provided, it will default to `calendar`. The billing time determines the timing of recurring billing cycles for the subscription. By specifying `anniversary`, the billing cycle will be based on the specific date the subscription started (billed fully), while `calendar` sets the billing cycle at the first day of the week/month/year (billed with proration).
    #[serde(rename = "billing_time")]
    pub billing_time: BillingTime,
    /// The display name of the subscription on an invoice. This field allows for customization of the subscription's name for billing purposes, especially useful when a single customer has multiple subscriptions using the same plan.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The unique code representing the plan to be attached to the customer. This code must correspond to the `code` property of one of the active plans.
    #[serde(rename = "plan_code")]
    pub plan_code: String,
    /// The status of the subscription, which can have the following values: - `pending`: a previous subscription has been downgraded, and the current one is awaiting automatic activation at the end of the billing period. - `active`: the subscription is currently active and applied to the customer. - `terminated`: the subscription is no longer active. - `canceled`: the subscription has been stopped before its activation. This can occur when two consecutive downgrades have been applied to a customer or when a subscription with a pending status is terminated.
    #[serde(rename = "status")]
    pub status: Status,
    /// The creation date of the subscription, represented in ISO 8601 datetime format and expressed in Coordinated Universal Time (UTC). This date provides a timestamp indicating when the subscription was initially created.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// The cancellation date of the subscription. This field is not null when the subscription is `canceled`. This date should be provided in ISO 8601 datetime format and expressed in Coordinated Universal Time (UTC).
    #[serde(rename = "canceled_at", skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<String>,
    /// The effective start date of the subscription. This field can be null if the subscription is `pending` or `canceled`. This date should be provided in ISO 8601 datetime format and expressed in Coordinated Universal Time (UTC).
    #[serde(rename = "started_at", skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    /// The effective end date of the subscription. If this field is set to null, the subscription will automatically renew. This date should be provided in ISO 8601 datetime format, and use Coordinated Universal Time (UTC).
    #[serde(rename = "ending_at", skip_serializing_if = "Option::is_none")]
    pub ending_at: Option<String>,
    /// The anniversary date and time of the initial subscription. This date serves as the basis for billing subscriptions with `anniversary` billing time. The `anniversary_date` should be provided in ISO 8601 datetime format and expressed in Coordinated Universal Time (UTC).
    #[serde(rename = "subscription_at")]
    pub subscription_at: String,
    /// The termination date of the subscription. This field is not null when the subscription is `terminated`. This date should be provided in ISO 8601 datetime format and expressed in Coordinated Universal Time (UTC)
    #[serde(rename = "terminated_at", skip_serializing_if = "Option::is_none")]
    pub terminated_at: Option<String>,
    /// The code identifying the previous plan associated with this subscription.
    #[serde(rename = "previous_plan_code", skip_serializing_if = "Option::is_none")]
    pub previous_plan_code: Option<String>,
    /// The code identifying the next plan in the case of a downgrade.
    #[serde(rename = "next_plan_code", skip_serializing_if = "Option::is_none")]
    pub next_plan_code: Option<String>,
    /// The date when the plan will be downgraded, represented in ISO 8601 date format.
    #[serde(rename = "downgrade_plan_date", skip_serializing_if = "Option::is_none")]
    pub downgrade_plan_date: Option<String>,
    /// The date when the free trial is ended, represented in ISO 8601 date format.
    #[serde(rename = "trial_ended_at", skip_serializing_if = "Option::is_none")]
    pub trial_ended_at: Option<String>,
    /// The date and time when the current billing period started, represented in ISO 8601 date format.
    #[serde(rename = "current_billing_period_started_at", skip_serializing_if = "Option::is_none")]
    pub current_billing_period_started_at: Option<String>,
    /// The date and time when the current billing period ends, represented in ISO 8601 date format.
    #[serde(rename = "current_billing_period_ending_at", skip_serializing_if = "Option::is_none")]
    pub current_billing_period_ending_at: Option<String>,
    #[serde(rename = "plan", skip_serializing_if = "Option::is_none")]
    pub plan: Option<Box<models::PlanObject>>,
}

impl SubscriptionObjectExtended {
    pub fn new(lago_id: uuid::Uuid, external_id: String, lago_customer_id: uuid::Uuid, external_customer_id: String, billing_time: BillingTime, plan_code: String, status: Status, created_at: String, subscription_at: String) -> SubscriptionObjectExtended {
        SubscriptionObjectExtended {
            lago_id,
            external_id,
            lago_customer_id,
            external_customer_id,
            billing_time,
            name: None,
            plan_code,
            status,
            created_at,
            canceled_at: None,
            started_at: None,
            ending_at: None,
            subscription_at,
            terminated_at: None,
            previous_plan_code: None,
            next_plan_code: None,
            downgrade_plan_date: None,
            trial_ended_at: None,
            current_billing_period_started_at: None,
            current_billing_period_ending_at: None,
            plan: None,
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
/// The status of the subscription, which can have the following values: - `pending`: a previous subscription has been downgraded, and the current one is awaiting automatic activation at the end of the billing period. - `active`: the subscription is currently active and applied to the customer. - `terminated`: the subscription is no longer active. - `canceled`: the subscription has been stopped before its activation. This can occur when two consecutive downgrades have been applied to a customer or when a subscription with a pending status is terminated.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "terminated")]
    Terminated,
    #[serde(rename = "canceled")]
    Canceled,
}

impl Default for Status {
    fn default() -> Status {
        Self::Active
    }
}

