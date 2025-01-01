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
pub struct FeeObjectAmountDetailsVolumeRangesInner {
    /// The flat amount for a whole tier, excluding tax, for a `volume` charge model.
    #[serde(rename = "per_unit_amount")]
    pub per_unit_amount: String,
    /// The unit price, excluding tax, for a specific tier of a `volume` charge model.
    #[serde(rename = "flat_unit_amount")]
    pub flat_unit_amount: String,
    /// Total amount of received units to be charged.
    #[serde(rename = "per_unit_total_amount")]
    pub per_unit_total_amount: String,
}

impl FeeObjectAmountDetailsVolumeRangesInner {
    pub fn new(per_unit_amount: String, flat_unit_amount: String, per_unit_total_amount: String) -> FeeObjectAmountDetailsVolumeRangesInner {
        FeeObjectAmountDetailsVolumeRangesInner {
            per_unit_amount,
            flat_unit_amount,
            per_unit_total_amount,
        }
    }
}
