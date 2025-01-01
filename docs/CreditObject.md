# CreditObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lago_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier assigned to the credit within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the credit's item record within the Lago system. | 
**amount_cents** | **i32** | The amount of credit associated with the invoice, expressed in cents. | 
**amount_currency** | [**models::Currency**](Currency.md) | The currency of the credit. | 
**before_taxes** | **bool** | Indicates whether the credit is applied on the amount before taxes (coupons) or after taxes (credit notes). This flag helps determine the order in which credits are applied to the invoice calculation | 
**item** | [**models::CreditObjectItem**](CreditObject_item.md) |  | 
**invoice** | [**models::CreditObjectInvoice**](CreditObject_invoice.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


