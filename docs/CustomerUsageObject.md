# CustomerUsageObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**from_datetime** | **String** | The lower bound of the billing period, expressed in the ISO 8601 datetime format in Coordinated Universal Time (UTC). | 
**to_datetime** | **String** | The upper bound of the billing period, expressed in the ISO 8601 datetime format in Coordinated Universal Time (UTC). | 
**issuing_date** | [**String**](string.md) | The date of creation of the invoice. | 
**lago_invoice_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | A unique identifier associated with the invoice related to this particular usage record. | [optional]
**currency** | Option<[**models::Currency**](Currency.md)> | The currency of the customer's current usage. | [optional]
**amount_cents** | **i32** | The amount in cents, tax excluded. | 
**taxes_amount_cents** | **i32** | The tax amount in cents. | 
**total_amount_cents** | **i32** | The total amount in cents, tax included. | 
**charges_usage** | [**Vec<models::CustomerChargeUsageObject>**](CustomerChargeUsageObject.md) | Array of charges that comprise the current usage. It contains detailed information about individual charge items associated with the usage. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


