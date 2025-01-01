# SubscriptionObjectExtended

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lago_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier assigned to the subscription within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the subscription's record within the Lago system | 
**external_id** | **String** | The subscription external unique identifier (provided by your own application). | 
**lago_customer_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier assigned to the customer within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the customer's record within the Lago system | 
**external_customer_id** | **String** | The customer external unique identifier (provided by your own application). | 
**billing_time** | **String** | The billing time for the subscription, which can be set as either `anniversary` or `calendar`. If not explicitly provided, it will default to `calendar`. The billing time determines the timing of recurring billing cycles for the subscription. By specifying `anniversary`, the billing cycle will be based on the specific date the subscription started (billed fully), while `calendar` sets the billing cycle at the first day of the week/month/year (billed with proration). | 
**name** | Option<**String**> | The display name of the subscription on an invoice. This field allows for customization of the subscription's name for billing purposes, especially useful when a single customer has multiple subscriptions using the same plan. | [optional]
**plan_code** | **String** | The unique code representing the plan to be attached to the customer. This code must correspond to the `code` property of one of the active plans. | 
**status** | **String** | The status of the subscription, which can have the following values: - `pending`: a previous subscription has been downgraded, and the current one is awaiting automatic activation at the end of the billing period. - `active`: the subscription is currently active and applied to the customer. - `terminated`: the subscription is no longer active. - `canceled`: the subscription has been stopped before its activation. This can occur when two consecutive downgrades have been applied to a customer or when a subscription with a pending status is terminated. | 
**created_at** | **String** | The creation date of the subscription, represented in ISO 8601 datetime format and expressed in Coordinated Universal Time (UTC). This date provides a timestamp indicating when the subscription was initially created. | 
**canceled_at** | Option<**String**> | The cancellation date of the subscription. This field is not null when the subscription is `canceled`. This date should be provided in ISO 8601 datetime format and expressed in Coordinated Universal Time (UTC). | [optional]
**started_at** | Option<**String**> | The effective start date of the subscription. This field can be null if the subscription is `pending` or `canceled`. This date should be provided in ISO 8601 datetime format and expressed in Coordinated Universal Time (UTC). | [optional]
**ending_at** | Option<**String**> | The effective end date of the subscription. If this field is set to null, the subscription will automatically renew. This date should be provided in ISO 8601 datetime format, and use Coordinated Universal Time (UTC). | [optional]
**subscription_at** | **String** | The anniversary date and time of the initial subscription. This date serves as the basis for billing subscriptions with `anniversary` billing time. The `anniversary_date` should be provided in ISO 8601 datetime format and expressed in Coordinated Universal Time (UTC). | 
**terminated_at** | Option<**String**> | The termination date of the subscription. This field is not null when the subscription is `terminated`. This date should be provided in ISO 8601 datetime format and expressed in Coordinated Universal Time (UTC) | [optional]
**previous_plan_code** | Option<**String**> | The code identifying the previous plan associated with this subscription. | [optional]
**next_plan_code** | Option<**String**> | The code identifying the next plan in the case of a downgrade. | [optional]
**downgrade_plan_date** | Option<[**String**](string.md)> | The date when the plan will be downgraded, represented in ISO 8601 date format. | [optional]
**trial_ended_at** | Option<**String**> | The date when the free trial is ended, represented in ISO 8601 date format. | [optional]
**current_billing_period_started_at** | Option<**String**> | The date and time when the current billing period started, represented in ISO 8601 date format. | [optional]
**current_billing_period_ending_at** | Option<**String**> | The date and time when the current billing period ends, represented in ISO 8601 date format. | [optional]
**plan** | Option<[**models::PlanObject**](PlanObject.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


