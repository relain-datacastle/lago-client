# SubscriptionUpdateInputSubscription

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The display name of the subscription on an invoice. This field allows for customization of the subscription's name for billing purposes, especially useful when a single customer has multiple subscriptions using the same plan. | [optional]
**ending_at** | **String** | The effective end date of the subscription. If this field is set to null, the subscription will automatically renew. This date should be provided in ISO 8601 datetime format, and use Coordinated Universal Time (UTC). | 
**subscription_at** | Option<**String**> | The start date and time of the subscription. This field can only be modified for pending subscriptions that have not yet started. This date should be provided in ISO 8601 datetime format and expressed in Coordinated Universal Time (UTC). | [optional]
**plan_overrides** | Option<[**models::PlanOverridesObject**](PlanOverridesObject.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


