# SubscriptionCreateInputSubscription

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**external_customer_id** | **String** | The customer external unique identifier (provided by your own application) | 
**plan_code** | **String** | The unique code representing the plan to be attached to the customer. This code must correspond to the `code` property of one of the active plans. | 
**name** | Option<**String**> | The display name of the subscription on an invoice. This field allows for customization of the subscription's name for billing purposes, especially useful when a single customer has multiple subscriptions using the same plan. | [optional]
**external_id** | **String** | The unique external identifier for the subscription. This identifier serves as an idempotency key, ensuring that each subscription is unique. | 
**billing_time** | Option<**String**> | The billing time for the subscription, which can be set as either `anniversary` or `calendar`. If not explicitly provided, it will default to `calendar`. The billing time determines the timing of recurring billing cycles for the subscription. By specifying `anniversary`, the billing cycle will be based on the specific date the subscription started (billed fully), while `calendar` sets the billing cycle at the first day of the week/month/year (billed with proration). | [optional]
**ending_at** | Option<**String**> | The effective end date of the subscription. If this field is set to null, the subscription will automatically renew. This date should be provided in ISO 8601 datetime format, and use Coordinated Universal Time (UTC). | [optional]
**subscription_at** | Option<**String**> | The start date for the subscription, allowing for the creation of subscriptions that can begin in the past or future. Please note that it cannot be used to update the start date of a pending subscription or schedule an upgrade/downgrade. The start_date should be provided in ISO 8601 datetime format and expressed in Coordinated Universal Time (UTC). | [optional]
**plan_overrides** | Option<[**models::PlanOverridesObject**](PlanOverridesObject.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


