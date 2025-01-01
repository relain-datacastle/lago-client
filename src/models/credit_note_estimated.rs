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
pub struct CreditNoteEstimated {
    #[serde(rename = "estimated_credit_note")]
    pub estimated_credit_note: Box<models::CreditNoteEstimatedEstimatedCreditNote>,
}

impl CreditNoteEstimated {
    pub fn new(estimated_credit_note: models::CreditNoteEstimatedEstimatedCreditNote) -> CreditNoteEstimated {
        CreditNoteEstimated {
            estimated_credit_note: Box::new(estimated_credit_note),
        }
    }
}

