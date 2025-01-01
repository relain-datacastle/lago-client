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
pub struct CouponCreateInput {
    #[serde(rename = "coupon")]
    pub coupon: Box<models::CouponBaseInput>,
}

impl CouponCreateInput {
    pub fn new(coupon: models::CouponBaseInput) -> CouponCreateInput {
        CouponCreateInput {
            coupon: Box::new(coupon),
        }
    }
}

