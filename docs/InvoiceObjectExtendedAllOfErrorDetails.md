# InvoiceObjectExtendedAllOfErrorDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lago_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier assigned to the error_detail within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the error's record within the Lago system. | 
**error_code** | **String** | Code that specifies part of the application / connection, where the error originally happened | 
**details** | [**serde_json::Value**](.md) | Key value list of more elaborated error detail, where by the key of error_code an external service error details are stored | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


