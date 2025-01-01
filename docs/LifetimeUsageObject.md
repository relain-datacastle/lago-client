# LifetimeUsageObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lago_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier assigned to the lifetime usage record within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the lifetime usage record within the Lago system | 
**lago_subscription_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier assigned to the subscription record within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the subscription record within the Lago system | 
**external_subscription_id** | **String** | The subscription external unique identifier (provided by your own application). | 
**external_historical_usage_amount_cents** | **i32** | The historical usage amount in cents for the subscription (provided by your own application). | 
**invoiced_usage_amount_cents** | **i32** | The total invoiced usage amount in cents for the subscription. | 
**current_usage_amount_cents** | **i32** | The current usage amount in cents for the subscription on the current billing period. | 
**from_datetime** | **String** | The recording start date and time of the subscription lifetime usage. The date and time must be in ISO 8601 format. | 
**to_datetime** | **String** | The recording end date and time of the subscription lifetime usage. The date and time must be in ISO 8601 format. | 
**usage_thresholds** | Option<[**Vec<models::LifetimeUsageThresholdObject>**](LifetimeUsageThresholdObject.md)> | Array of usage thresholds attached to the subscription's plan. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


