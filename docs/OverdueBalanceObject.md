# OverdueBalanceObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**month** | **String** | Identifies the month to analyze revenue. | 
**amount_cents** | **i32** | The total amount of revenue for a period, expressed in cents. | 
**currency** | [**models::Currency**](Currency.md) | The currency of revenue analytics. Format must be ISO 4217. | 
**lago_invoice_ids** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | The Lago invoice IDs associated with the revenue. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

