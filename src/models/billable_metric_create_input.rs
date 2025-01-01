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
pub struct BillableMetricCreateInput {
    #[serde(rename = "billable_metric")]
    pub billable_metric: Box<models::BillableMetricBaseInput>,
}

impl BillableMetricCreateInput {
    pub fn new(billable_metric: models::BillableMetricBaseInput) -> BillableMetricCreateInput {
        BillableMetricCreateInput {
            billable_metric: Box::new(billable_metric),
        }
    }
}
