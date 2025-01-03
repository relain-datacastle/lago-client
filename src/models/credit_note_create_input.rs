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
pub struct CreditNoteCreateInput {
    #[serde(rename = "credit_note")]
    pub credit_note: Box<models::CreditNoteCreateInputCreditNote>,
}

impl CreditNoteCreateInput {
    pub fn new(credit_note: models::CreditNoteCreateInputCreditNote) -> CreditNoteCreateInput {
        CreditNoteCreateInput {
            credit_note: Box::new(credit_note),
        }
    }
}

