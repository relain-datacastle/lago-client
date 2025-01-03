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

/// OrganizationBillingConfiguration : The custom billing settings for your organization.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationBillingConfiguration {
    /// The customer invoice message that appears at the bottom of each billing documents.
    #[serde(rename = "invoice_footer", skip_serializing_if = "Option::is_none")]
    pub invoice_footer: Option<String>,
    /// The grace period, expressed in days, for finalizing the invoice. This period refers to the additional time granted to your customers beyond the invoice due date to adjust usage and line items. Can be overwritten by the customer's grace period.
    #[serde(rename = "invoice_grace_period", skip_serializing_if = "Option::is_none")]
    pub invoice_grace_period: Option<i32>,
    /// The locale of the billing documents, expressed in the ISO 639-1 format. This field indicates the language or regional variant used for the documents content issued or the embeddable customer portal.
    #[serde(rename = "document_locale", skip_serializing_if = "Option::is_none")]
    pub document_locale: Option<String>,
}

impl OrganizationBillingConfiguration {
    /// The custom billing settings for your organization.
    pub fn new() -> OrganizationBillingConfiguration {
        OrganizationBillingConfiguration {
            invoice_footer: None,
            invoice_grace_period: None,
            document_locale: None,
        }
    }
}

