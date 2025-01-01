# CreditObjectItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lago_item_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier assigned to the credit item within the Lago application. | 
**r#type** | **String** | The type of credit applied. Possible values are `coupon`, `credit_note` or `invoice` (for `progressive_billing` invoice). | 
**code** | **String** | The code of the credit applied. It can be the code of the coupon attached to the credit, the credit note's number or the `progressive_billing` invoice number. | 
**name** | **String** | The name of the credit applied. It can be the name of the coupon attached to the credit, the initial invoice's number linked to the credit note or the `progressive_billing` invoice number. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


