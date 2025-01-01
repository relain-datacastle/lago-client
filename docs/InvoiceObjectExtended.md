# InvoiceObjectExtended

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lago_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier assigned to the fee within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the fee's record within the Lago system. | 
**sequential_id** | Option<**i32**> | This ID helps in uniquely identifying and organizing the invoices associated with a specific customer. It provides a sequential numbering system specific to the customer, allowing for easy tracking and management of invoices within the customer's context. | [optional]
**number** | **String** | The unique number assigned to the invoice. This number serves as a distinct identifier for the invoice and helps in differentiating it from other invoices in the system. | 
**issuing_date** | [**String**](string.md) | The date when the invoice was issued. It is provided in the ISO 8601 date format. | 
**payment_dispute_lost_at** | Option<**String**> | The date when the payment dispute was lost. It is expressed in Coordinated Universal Time (UTC). | [optional]
**payment_due_date** | Option<[**String**](string.md)> | The payment due date for the invoice, specified in the ISO 8601 date format. | [optional]
**payment_overdue** | Option<**bool**> | Specifies if the payment is considered as overdue. | [optional]
**net_payment_term** | Option<**i32**> | The net payment term, expressed in days, specifies the duration within which a customer is expected to remit payment after the invoice is finalized. | [optional]
**invoice_type** | **String** | The type of invoice issued. Possible values are `subscription`, `one-off`, `credit` or `progressive_billing`. | 
**status** | **String** | The status of the invoice. It indicates the current state of the invoice and can have following values: - `draft`: the invoice is in the draft state, waiting for the end of the grace period to be finalized. During this period, events can still be ingested and added to the invoice. - `finalized`: the invoice has been issued and finalized. In this state, events cannot be ingested or added to the invoice anymore. - `voided`: the invoice has been issued and subsequently voided. In this state, events cannot be ingested or added to the invoice anymore. - `pending`: the invoice remains pending until the taxes are fetched from the external provider. - `failed`: during an attempt of finalization of the invoice, an error happened. This invoice will have an array of error_details, explaining, in which part of the system an error happened and how it's possible to fix it. This invoice can't be edited or updated, only retried. This action will discard current error_details and will create new ones if the finalization failed again. | 
**payment_status** | **String** | The status of the payment associated with the invoice. It can have one of the following values: - `pending`: the payment is pending, waiting for payment processing in Stripe or when the invoice is emitted but users have not updated the payment status through the endpoint. - `succeeded`: the payment of the invoice has been successfully processed. - `failed`: the payment of the invoice has failed or encountered an error during processing. | 
**currency** | [**models::Currency**](Currency.md) | The currency of the invoice issued. | 
**fees_amount_cents** | **i32** | The total sum of fees amount in cents. It calculates the cumulative amount of all the fees associated with the invoice, providing a consolidated value. | 
**coupons_amount_cents** | **i32** | The total sum of all coupons discounted on the invoice. It calculates the cumulative discount amount applied by coupons, expressed in cents. | 
**credit_notes_amount_cents** | **i32** | The total sum of all credit notes discounted on the invoice. It calculates the cumulative discount amount applied by credit notes, expressed in cents. | 
**sub_total_excluding_taxes_amount_cents** | **i32** | Subtotal amount, excluding taxes, expressed in cents. This field depends on the version number. Here are the definitions based on the version: - Version 1: is equal to the sum of `fees_amount_cents`, minus `coupons_amount_cents`, and minus `prepaid_credit_amount_cents`. - Version 2: is equal to the `fees_amount_cents`. - Version 3 & 4: is equal to the `fees_amount_cents`, minus `coupons_amount_cents` | 
**taxes_amount_cents** | **i32** | The sum of tax amount associated with the invoice, expressed in cents. | 
**sub_total_including_taxes_amount_cents** | **i32** | Subtotal amount, including taxes, expressed in cents. This field depends on the version number. Here are the definitions based on the version: - Version 1: is equal to the `total_amount_cents`. - Version 2: is equal to the sum of `fees_amount_cents` and `taxes_amount_cents`. - Version 3 & 4: is equal to the sum `sub_total_excluding_taxes_amount_cents` and `taxes_amount_cents` | 
**prepaid_credit_amount_cents** | **i32** | The total sum of all prepaid credits discounted on the invoice. It calculates the cumulative discount amount applied by prepaid credits, expressed in cents. | 
**progressive_billing_credit_amount_cents** | **i32** | The usage already billed in previous invoices. Only apply to `progressive_billing` and `subscription` invoices. | 
**total_amount_cents** | **i32** | The sum of the amount and taxes amount on the invoice, expressed in cents. It calculates the total financial value of the invoice, including both the original amount and any applicable taxes. | 
**version_number** | **i32** |  | 
**file_url** | Option<**String**> | Contains the URL that provides direct access to the invoice PDF file. You can use this URL to download or view the PDF document of the invoice | [optional]
**created_at** | **String** | The date of the invoice creation, represented in ISO 8601 datetime format and expressed in Coordinated Universal Time (UTC). The creation_date provides a standardized and internationally recognized timestamp for when the invoice object was created | 
**updated_at** | **String** | The date of the invoice update, represented in ISO 8601 datetime format and expressed in Coordinated Universal Time (UTC). The update_date provides a standardized and internationally recognized timestamp for when the invoice object was updated | 
**customer** | Option<[**models::CustomerObject**](CustomerObject.md)> | The customer on which the invoice applies. It refers to the customer account or entity associated with the invoice. | [optional]
**metadata** | Option<[**Vec<models::InvoiceMetadataObject>**](InvoiceMetadataObject.md)> |  | [optional]
**applied_taxes** | Option<[**Vec<models::InvoiceAppliedTaxObject>**](InvoiceAppliedTaxObject.md)> |  | [optional]
**applied_usage_thresholds** | Option<[**Vec<models::AppliedUsageThresholdObject>**](AppliedUsageThresholdObject.md)> |  | [optional]
**credits** | Option<[**Vec<models::CreditObject>**](CreditObject.md)> |  | [optional]
**fees** | Option<[**Vec<models::FeeObject>**](FeeObject.md)> |  | [optional]
**subscriptions** | Option<[**Vec<models::SubscriptionObject>**](SubscriptionObject.md)> |  | [optional]
**error_details** | Option<[**Vec<models::InvoiceObjectExtendedAllOfErrorDetails>**](InvoiceObjectExtended_allOf_error_details.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


