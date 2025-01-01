# PaymentRequestObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lago_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier of the payment request, created by Lago. | 
**email** | **String** | The customer's email address used for sending dunning notifications | 
**amount_cents** | **i32** | The sum of the total amounts of all the invoices included in the payment request, expressed in cents. | 
**amount_currency** | [**models::Currency**](Currency.md) | The currency of the payment request. | 
**payment_status** | **String** | The status of the payment associated with the payment request. It can have one of the following values: - `pending`: the payment is pending, waiting for payment processing in the payment provider or when the invoice is emitted but users have not updated the payment status through the endpoint. - `succeeded`: the payment of the payment request has been successfully processed. - `failed`: the payment of the payment request has failed or encountered an error during processing. | 
**created_at** | **String** | The date and time when the payment request was created. It is expressed in UTC format according to the ISO 8601 datetime standard. This field provides the timestamp for the exact moment when the payment request was initially created. | 
**customer** | [**models::Model0**](0.md) | The customer on which the payment request applies. It refers to the customer account or entity associated with the payment request. | 
**invoices** | [**Vec<models::Model0>**](0.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


