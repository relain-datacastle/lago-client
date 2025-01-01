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
pub struct FindAllEvents200Response {
    #[serde(rename = "events")]
    pub events: Vec<models::EventObject>,
    #[serde(rename = "meta")]
    pub meta: Box<models::PaginationMeta>,
}

impl FindAllEvents200Response {
    pub fn new(events: Vec<models::EventObject>, meta: models::PaginationMeta) -> FindAllEvents200Response {
        FindAllEvents200Response {
            events,
            meta: Box::new(meta),
        }
    }
}

