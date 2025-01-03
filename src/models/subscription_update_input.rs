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
pub struct SubscriptionUpdateInput {
    /// If the field is not defined and multiple `active` and `pending` subscriptions exists, Lago will update the `active` subscription. However, if you wish to update a `pending` subscription, please ensure that you include the `status` attribute with the `pending` value in your request body.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "subscription")]
    pub subscription: Box<models::SubscriptionUpdateInputSubscription>,
}

impl SubscriptionUpdateInput {
    pub fn new(subscription: models::SubscriptionUpdateInputSubscription) -> SubscriptionUpdateInput {
        SubscriptionUpdateInput {
            status: None,
            subscription: Box::new(subscription),
        }
    }
}
/// If the field is not defined and multiple `active` and `pending` subscriptions exists, Lago will update the `active` subscription. However, if you wish to update a `pending` subscription, please ensure that you include the `status` attribute with the `pending` value in your request body.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "pending")]
    Pending,
}

impl Default for Status {
    fn default() -> Status {
        Self::Active
    }
}

