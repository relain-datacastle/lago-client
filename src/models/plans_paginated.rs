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
pub struct PlansPaginated {
    #[serde(rename = "plans")]
    pub plans: Vec<models::PlanObject>,
    #[serde(rename = "meta")]
    pub meta: Box<models::PaginationMeta>,
}

impl PlansPaginated {
    pub fn new(plans: Vec<models::PlanObject>, meta: models::PaginationMeta) -> PlansPaginated {
        PlansPaginated {
            plans,
            meta: Box::new(meta),
        }
    }
}
