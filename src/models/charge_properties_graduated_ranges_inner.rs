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
pub struct ChargePropertiesGraduatedRangesInner {
    /// Specifies the lower value of a tier for a `graduated` charge model. It must be either 0 or the previous range's `to_value + 1` to maintain the proper sequence of values.
    #[serde(rename = "from_value")]
    pub from_value: i32,
    /// Specifies the highest value of a tier for a `graduated` charge model. - This value must be higher than the from_value of the same tier. - This value must be null for the last tier.
    #[serde(rename = "to_value")]
    pub to_value: i32,
    /// The flat amount for a whole tier, excluding tax, for a `graduated` charge model. It is expressed as a decimal value.
    #[serde(rename = "flat_amount")]
    pub flat_amount: String,
    /// The unit price, excluding tax, for a specific tier of a `graduated` charge model. It is expressed as a decimal value.
    #[serde(rename = "per_unit_amount")]
    pub per_unit_amount: String,
}

impl ChargePropertiesGraduatedRangesInner {
    pub fn new(from_value: i32, to_value: i32, flat_amount: String, per_unit_amount: String) -> ChargePropertiesGraduatedRangesInner {
        ChargePropertiesGraduatedRangesInner {
            from_value,
            to_value,
            flat_amount,
            per_unit_amount,
        }
    }
}

