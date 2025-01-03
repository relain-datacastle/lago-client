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
pub struct OrganizationObject {
    /// Unique identifier assigned to the organization within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the organization's record within the Lago system
    #[serde(rename = "lago_id")]
    pub lago_id: uuid::Uuid,
    /// The name of your organization.
    #[serde(rename = "name")]
    pub name: String,
    /// The date of creation of your organization, represented in ISO 8601 datetime format and expressed in Coordinated Universal Time (UTC).
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// The URL of your newest updated webhook endpoint. This URL allows your organization to receive important messages, notifications, or data from the Lago system. By configuring your webhook endpoint to this URL, you can ensure that your organization stays informed and receives relevant information in a timely manner.
    #[serde(rename = "webhook_url", skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
    /// The array containing your webhooks URLs.
    #[serde(rename = "webhook_urls", skip_serializing_if = "Option::is_none")]
    pub webhook_urls: Option<Vec<String>>,
    /// The country of your organization.
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<models::Country>,
    /// The default currency of an organization.
    #[serde(rename = "default_currency", skip_serializing_if = "Option::is_none")]
    pub default_currency: Option<models::Currency>,
    /// The first line of your organization's billing address.
    #[serde(rename = "address_line1", skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<String>,
    /// The second line of your organization's billing address.
    #[serde(rename = "address_line2", skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,
    /// The state of your organization's billing address.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The zipcode of your organization's billing address.
    #[serde(rename = "zipcode", skip_serializing_if = "Option::is_none")]
    pub zipcode: Option<String>,
    /// The email address of your organization used to bill your customers.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The city of your organization's billing address.
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The legal name of your organization.
    #[serde(rename = "legal_name", skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    /// The legal number of your organization.
    #[serde(rename = "legal_number", skip_serializing_if = "Option::is_none")]
    pub legal_number: Option<String>,
    /// This parameter configures the method of incrementing invoice numbers for your customers.  - `per_customer`: Invoice numbers are incremented individually for each customer. This means every customer will have their own unique sequence of invoice numbers, separate from other customers. It ensures that each customer's invoice numbers follow a distinct and isolated numbering pattern. - `per_organization`: Invoice number incrementation is made across your entire organization. Rather than individual sequences for each customer, all invoices within the organization follow a single, unified numbering system. This creates a continuous and organization-wide sequence for all invoice numbers. Invoices are incremented per month (dynamic value used is YYYYMM), and invoice numbers are reset at the end of each month.  The default value for `document_numbering` is set to `per_customer`, meaning that, unless changed, invoice numbers will increment uniquely for each customer.
    #[serde(rename = "document_numbering")]
    pub document_numbering: DocumentNumbering,
    /// Sets the prefix for invoices and credit notes. Default is the first three letters of your organization name plus the last four digits of your organization ID. Customizable within 1-10 characters, and automatically capitalized by Lago.
    #[serde(rename = "document_number_prefix")]
    pub document_number_prefix: String,
    /// The net payment term, expressed in days, specifies the duration within which a customer is expected to remit payment after the invoice is finalized.
    #[serde(rename = "net_payment_term", skip_serializing_if = "Option::is_none")]
    pub net_payment_term: Option<i32>,
    /// The tax identification number of your organization.
    #[serde(rename = "tax_identification_number", skip_serializing_if = "Option::is_none")]
    pub tax_identification_number: Option<String>,
    /// Your organization's timezone, used for billing purposes in your own local time. Can be overwritten by the customer's timezone.
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<models::Timezone>,
    #[serde(rename = "billing_configuration")]
    pub billing_configuration: Box<models::OrganizationBillingConfiguration>,
    /// List of default organization taxes
    #[serde(rename = "taxes", skip_serializing_if = "Option::is_none")]
    pub taxes: Option<Vec<models::TaxObject>>,
    /// Indicates whether invoices with a zero total amount should be finalized. If set to true, zero amount invoices will be finalized. If set to false, zero amount invoices will not be finalized.
    #[serde(rename = "finalize_zero_amount_invoice", skip_serializing_if = "Option::is_none")]
    pub finalize_zero_amount_invoice: Option<bool>,
}

impl OrganizationObject {
    pub fn new(lago_id: uuid::Uuid, name: String, created_at: String, document_numbering: DocumentNumbering, document_number_prefix: String, billing_configuration: models::OrganizationBillingConfiguration) -> OrganizationObject {
        OrganizationObject {
            lago_id,
            name,
            created_at,
            webhook_url: None,
            webhook_urls: None,
            country: None,
            default_currency: None,
            address_line1: None,
            address_line2: None,
            state: None,
            zipcode: None,
            email: None,
            city: None,
            legal_name: None,
            legal_number: None,
            document_numbering,
            document_number_prefix,
            net_payment_term: None,
            tax_identification_number: None,
            timezone: None,
            billing_configuration: Box::new(billing_configuration),
            taxes: None,
            finalize_zero_amount_invoice: None,
        }
    }
}
/// This parameter configures the method of incrementing invoice numbers for your customers.  - `per_customer`: Invoice numbers are incremented individually for each customer. This means every customer will have their own unique sequence of invoice numbers, separate from other customers. It ensures that each customer's invoice numbers follow a distinct and isolated numbering pattern. - `per_organization`: Invoice number incrementation is made across your entire organization. Rather than individual sequences for each customer, all invoices within the organization follow a single, unified numbering system. This creates a continuous and organization-wide sequence for all invoice numbers. Invoices are incremented per month (dynamic value used is YYYYMM), and invoice numbers are reset at the end of each month.  The default value for `document_numbering` is set to `per_customer`, meaning that, unless changed, invoice numbers will increment uniquely for each customer.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DocumentNumbering {
    #[serde(rename = "per_customer")]
    Customer,
    #[serde(rename = "per_organization")]
    Organization,
}

impl Default for DocumentNumbering {
    fn default() -> DocumentNumbering {
        Self::Customer
    }
}

