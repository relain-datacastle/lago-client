# TaxObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lago_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier of the tax, created by Lago. | 
**name** | **String** | Name of the tax. | 
**code** | **String** | Unique code used to identify the tax associated with the API request. | 
**description** | Option<**String**> | Internal description of the taxe | [optional]
**rate** | **f64** | The percentage rate of the tax | 
**applied_to_organization** | **bool** | Set to `true` if the tax is used as one of the organization's default | 
**created_at** | **String** | Creation date of the tax. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


