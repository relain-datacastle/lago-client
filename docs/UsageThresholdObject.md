# UsageThresholdObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lago_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier of the usage threshold created by Lago. | 
**threshold_display_name** | Option<**String**> | The display name of the usage threshold. | [optional]
**amount_cents** | **i32** | The amount to reach to trigger a `progressive_billing` invoice. | 
**recurring** | **bool** | This field when set to `true` indicates that a `progressive_billing` invoice will be created every time the lifetime usage increases by the specified amount. | 
**created_at** | **String** | The date and time when the usage threshold was created. It is expressed in UTC format according to the ISO 8601 datetime standard. | 
**updated_at** | **String** | The date and time when the usage threshold was last updated. It is expressed in UTC format according to the ISO 8601 datetime standard. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


