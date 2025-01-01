# UsageThresholdInput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique identifier of the usage threshold created by Lago. | [optional]
**threshold_display_name** | Option<**String**> | The display name of the usage threshold. | [optional]
**amount_cents** | **i32** | The amount to reach to trigger a `progressive_billing` invoice. | 
**recurring** | **bool** | This field when set to `true` indicates that a `progressive_billing` invoice will be created every time the lifetime usage increases by the specified amount. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


