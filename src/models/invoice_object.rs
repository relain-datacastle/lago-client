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
pub struct InvoiceObject {
    /// Unique identifier assigned to the fee within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the fee's record within the Lago system.
    #[serde(rename = "lago_id")]
    pub lago_id: uuid::Uuid,
    /// This ID helps in uniquely identifying and organizing the invoices associated with a specific customer. It provides a sequential numbering system specific to the customer, allowing for easy tracking and management of invoices within the customer's context.
    #[serde(rename = "sequential_id", skip_serializing_if = "Option::is_none")]
    pub sequential_id: Option<i32>,
    /// The unique number assigned to the invoice. This number serves as a distinct identifier for the invoice and helps in differentiating it from other invoices in the system.
    #[serde(rename = "number")]
    pub number: String,
    /// The date when the invoice was issued. It is provided in the ISO 8601 date format.
    #[serde(rename = "issuing_date")]
    pub issuing_date: String,
    /// The date when the payment dispute was lost. It is expressed in Coordinated Universal Time (UTC).
    #[serde(rename = "payment_dispute_lost_at", skip_serializing_if = "Option::is_none")]
    pub payment_dispute_lost_at: Option<String>,
    /// The payment due date for the invoice, specified in the ISO 8601 date format.
    #[serde(rename = "payment_due_date", skip_serializing_if = "Option::is_none")]
    pub payment_due_date: Option<String>,
    /// Specifies if the payment is considered as overdue.
    #[serde(rename = "payment_overdue", skip_serializing_if = "Option::is_none")]
    pub payment_overdue: Option<bool>,
    /// The net payment term, expressed in days, specifies the duration within which a customer is expected to remit payment after the invoice is finalized.
    #[serde(rename = "net_payment_term", skip_serializing_if = "Option::is_none")]
    pub net_payment_term: Option<i32>,
    /// The type of invoice issued. Possible values are `subscription`, `one-off`, `credit` or `progressive_billing`.
    #[serde(rename = "invoice_type")]
    pub invoice_type: InvoiceType,
    /// The status of the invoice. It indicates the current state of the invoice and can have following values: - `draft`: the invoice is in the draft state, waiting for the end of the grace period to be finalized. During this period, events can still be ingested and added to the invoice. - `finalized`: the invoice has been issued and finalized. In this state, events cannot be ingested or added to the invoice anymore. - `voided`: the invoice has been issued and subsequently voided. In this state, events cannot be ingested or added to the invoice anymore. - `pending`: the invoice remains pending until the taxes are fetched from the external provider. - `failed`: during an attempt of finalization of the invoice, an error happened. This invoice will have an array of error_details, explaining, in which part of the system an error happened and how it's possible to fix it. This invoice can't be edited or updated, only retried. This action will discard current error_details and will create new ones if the finalization failed again.
    #[serde(rename = "status")]
    pub status: Status,
    /// The status of the payment associated with the invoice. It can have one of the following values: - `pending`: the payment is pending, waiting for payment processing in Stripe or when the invoice is emitted but users have not updated the payment status through the endpoint. - `succeeded`: the payment of the invoice has been successfully processed. - `failed`: the payment of the invoice has failed or encountered an error during processing.
    #[serde(rename = "payment_status")]
    pub payment_status: PaymentStatus,
    /// The currency of the invoice issued.
    #[serde(rename = "currency")]
    pub currency: models::Currency,
    /// The total sum of fees amount in cents. It calculates the cumulative amount of all the fees associated with the invoice, providing a consolidated value.
    #[serde(rename = "fees_amount_cents")]
    pub fees_amount_cents: i32,
    /// The total sum of all coupons discounted on the invoice. It calculates the cumulative discount amount applied by coupons, expressed in cents.
    #[serde(rename = "coupons_amount_cents")]
    pub coupons_amount_cents: i32,
    /// The total sum of all credit notes discounted on the invoice. It calculates the cumulative discount amount applied by credit notes, expressed in cents.
    #[serde(rename = "credit_notes_amount_cents")]
    pub credit_notes_amount_cents: i32,
    /// Subtotal amount, excluding taxes, expressed in cents. This field depends on the version number. Here are the definitions based on the version: - Version 1: is equal to the sum of `fees_amount_cents`, minus `coupons_amount_cents`, and minus `prepaid_credit_amount_cents`. - Version 2: is equal to the `fees_amount_cents`. - Version 3 & 4: is equal to the `fees_amount_cents`, minus `coupons_amount_cents`
    #[serde(rename = "sub_total_excluding_taxes_amount_cents")]
    pub sub_total_excluding_taxes_amount_cents: i32,
    /// The sum of tax amount associated with the invoice, expressed in cents.
    #[serde(rename = "taxes_amount_cents")]
    pub taxes_amount_cents: i32,
    /// Subtotal amount, including taxes, expressed in cents. This field depends on the version number. Here are the definitions based on the version: - Version 1: is equal to the `total_amount_cents`. - Version 2: is equal to the sum of `fees_amount_cents` and `taxes_amount_cents`. - Version 3 & 4: is equal to the sum `sub_total_excluding_taxes_amount_cents` and `taxes_amount_cents`
    #[serde(rename = "sub_total_including_taxes_amount_cents")]
    pub sub_total_including_taxes_amount_cents: i32,
    /// The total sum of all prepaid credits discounted on the invoice. It calculates the cumulative discount amount applied by prepaid credits, expressed in cents.
    #[serde(rename = "prepaid_credit_amount_cents")]
    pub prepaid_credit_amount_cents: i32,
    /// The usage already billed in previous invoices. Only apply to `progressive_billing` and `subscription` invoices.
    #[serde(rename = "progressive_billing_credit_amount_cents")]
    pub progressive_billing_credit_amount_cents: i32,
    /// The sum of the amount and taxes amount on the invoice, expressed in cents. It calculates the total financial value of the invoice, including both the original amount and any applicable taxes.
    #[serde(rename = "total_amount_cents")]
    pub total_amount_cents: i32,
    #[serde(rename = "version_number")]
    pub version_number: i32,
    /// Contains the URL that provides direct access to the invoice PDF file. You can use this URL to download or view the PDF document of the invoice
    #[serde(rename = "file_url", skip_serializing_if = "Option::is_none")]
    pub file_url: Option<String>,
    /// The date of the invoice creation, represented in ISO 8601 datetime format and expressed in Coordinated Universal Time (UTC). The creation_date provides a standardized and internationally recognized timestamp for when the invoice object was created
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// The date of the invoice update, represented in ISO 8601 datetime format and expressed in Coordinated Universal Time (UTC). The update_date provides a standardized and internationally recognized timestamp for when the invoice object was updated
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    /// The customer on which the invoice applies. It refers to the customer account or entity associated with the invoice.
    #[serde(rename = "customer", skip_serializing_if = "Option::is_none")]
    pub customer: Option<Box<models::CustomerObject>>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Vec<models::InvoiceMetadataObject>>,
    #[serde(rename = "applied_taxes", skip_serializing_if = "Option::is_none")]
    pub applied_taxes: Option<Vec<models::InvoiceAppliedTaxObject>>,
    #[serde(rename = "applied_usage_thresholds", skip_serializing_if = "Option::is_none")]
    pub applied_usage_thresholds: Option<Vec<models::AppliedUsageThresholdObject>>,
}

impl InvoiceObject {
    pub fn new(lago_id: uuid::Uuid, number: String, issuing_date: String, invoice_type: InvoiceType, status: Status, payment_status: PaymentStatus, currency: models::Currency, fees_amount_cents: i32, coupons_amount_cents: i32, credit_notes_amount_cents: i32, sub_total_excluding_taxes_amount_cents: i32, taxes_amount_cents: i32, sub_total_including_taxes_amount_cents: i32, prepaid_credit_amount_cents: i32, progressive_billing_credit_amount_cents: i32, total_amount_cents: i32, version_number: i32, created_at: String, updated_at: String) -> InvoiceObject {
        InvoiceObject {
            lago_id,
            sequential_id: None,
            number,
            issuing_date,
            payment_dispute_lost_at: None,
            payment_due_date: None,
            payment_overdue: None,
            net_payment_term: None,
            invoice_type,
            status,
            payment_status,
            currency,
            fees_amount_cents,
            coupons_amount_cents,
            credit_notes_amount_cents,
            sub_total_excluding_taxes_amount_cents,
            taxes_amount_cents,
            sub_total_including_taxes_amount_cents,
            prepaid_credit_amount_cents,
            progressive_billing_credit_amount_cents,
            total_amount_cents,
            version_number,
            file_url: None,
            created_at,
            updated_at,
            customer: None,
            metadata: None,
            applied_taxes: None,
            applied_usage_thresholds: None,
        }
    }
}
/// The type of invoice issued. Possible values are `subscription`, `one-off`, `credit` or `progressive_billing`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InvoiceType {
    #[serde(rename = "subscription")]
    Subscription,
    #[serde(rename = "add_on")]
    AddOn,
    #[serde(rename = "credit")]
    Credit,
    #[serde(rename = "one_off")]
    OneOff,
    #[serde(rename = "progressive_billing")]
    ProgressiveBilling,
}

impl Default for InvoiceType {
    fn default() -> InvoiceType {
        Self::Subscription
    }
}
/// The status of the invoice. It indicates the current state of the invoice and can have following values: - `draft`: the invoice is in the draft state, waiting for the end of the grace period to be finalized. During this period, events can still be ingested and added to the invoice. - `finalized`: the invoice has been issued and finalized. In this state, events cannot be ingested or added to the invoice anymore. - `voided`: the invoice has been issued and subsequently voided. In this state, events cannot be ingested or added to the invoice anymore. - `pending`: the invoice remains pending until the taxes are fetched from the external provider. - `failed`: during an attempt of finalization of the invoice, an error happened. This invoice will have an array of error_details, explaining, in which part of the system an error happened and how it's possible to fix it. This invoice can't be edited or updated, only retried. This action will discard current error_details and will create new ones if the finalization failed again.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "finalized")]
    Finalized,
    #[serde(rename = "voided")]
    Voided,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "pending")]
    Pending,
}

impl Default for Status {
    fn default() -> Status {
        Self::Draft
    }
}
/// The status of the payment associated with the invoice. It can have one of the following values: - `pending`: the payment is pending, waiting for payment processing in Stripe or when the invoice is emitted but users have not updated the payment status through the endpoint. - `succeeded`: the payment of the invoice has been successfully processed. - `failed`: the payment of the invoice has failed or encountered an error during processing.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PaymentStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "failed")]
    Failed,
}

impl Default for PaymentStatus {
    fn default() -> PaymentStatus {
        Self::Pending
    }
}

