# EventObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lago_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier assigned to the event within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the event's record within the Lago system | 
**transaction_id** | **String** | This field represents a unique identifier for the event. It is crucial for ensuring idempotency, meaning that each event can be uniquely identified and processed without causing any unintended side effects. | 
**lago_customer_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier assigned to the customer within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the customer's record within the Lago system | 
**code** | **String** | The code that identifies a targeted billable metric. It is essential that this code matches the `code` property of one of your active billable metrics. If the provided code does not correspond to any active billable metric, it will be ignored during the process. | 
**timestamp** | **String** | This field captures the Unix timestamp in seconds indicating the occurrence of the event in Coordinated Universal Time (UTC). If this timestamp is not provided, the API will automatically set it to the time of event reception. | 
**precise_total_amount_cents** | Option<**String**> | The precise total amount that was sent in the event payload. This filed is used by the `dynamic` pricing model. | [optional]
**properties** | Option<[**models::EventObjectProperties**](EventObject_properties.md)> |  | [optional]
**lago_subscription_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier assigned to the subscription within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the subscription's record within the Lago system | 
**external_subscription_id** | **String** | The unique identifier of the subscription within your application. It is a mandatory field when the customer possesses multiple subscriptions or when the `external_customer_id` is not provided. | 
**created_at** | **String** | The creation date of the event's record in the Lago application, presented in the ISO 8601 datetime format, specifically in Coordinated Universal Time (UTC). It provides the precise timestamp of when the event's record was created within the Lago application | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


