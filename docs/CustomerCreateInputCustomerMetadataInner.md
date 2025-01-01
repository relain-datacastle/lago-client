# CustomerCreateInputCustomerMetadataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Identifier for the metadata object, only required when updating a key-value pair | [optional]
**key** | **String** | The metadata object key | 
**value** | **String** | The metadata object value | 
**display_in_invoice** | **bool** | Determines whether the item or information should be displayed in the invoice. If set to true, the item or information will be included and visible in the generated invoice. If set to false, the item or information will be excluded and not displayed in the invoice. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


