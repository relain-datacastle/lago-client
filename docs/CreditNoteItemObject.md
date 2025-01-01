# CreditNoteItemObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lago_id** | [**uuid::Uuid**](uuid::Uuid.md) | The credit note's item unique identifier, created by Lago. | 
**amount_cents** | **i32** | The credit note's item amount, expressed in cents. | 
**amount_currency** | [**models::Currency**](Currency.md) | The credit note's item currency. | 
**fee** | [**models::FeeObject**](FeeObject.md) | The fee object related to the credit note item. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


