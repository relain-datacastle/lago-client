# WebhookEndpointObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lago_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier assigned to the wallet within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the webhook endpoint's record within the Lago system. | 
**lago_organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier assigned to the organization attached to the webhook endpoint within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the organization's record within the Lago system. | 
**webhook_url** | **String** | The name of the wallet. | 
**signature_algo** | Option<**String**> | The signature algo for the webhook. | [optional]
**created_at** | **String** | The date of the webhook endpoint creation, represented in ISO 8601 datetime format and expressed in Coordinated Universal Time (UTC). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


