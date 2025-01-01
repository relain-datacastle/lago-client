# EventInputEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transaction_id** | **String** | This field represents a unique identifier for the event. It is crucial for ensuring idempotency, meaning that each event can be uniquely identified and processed without causing any unintended side effects. | 
**external_subscription_id** | **String** | The unique identifier of the subscription in your application. This field is mandatory in order to link events to the correct customer subscription. | 
**code** | **String** | The code that identifies a targeted billable metric. It is essential that this code matches the `code` property of one of your active billable metrics. If the provided code does not correspond to any active billable metric, it will be ignored during the process. | 
**timestamp** | Option<[**models::EventInputEventTimestamp**](EventInput_event_timestamp.md)> |  | [optional]
**precise_total_amount_cents** | Option<**String**> | The precise total amount in cents with precision used by the `dynamic` pricing model to compute the usage amount. | [optional]
**properties** | Option<[**std::collections::HashMap<String, models::BillableMetricEvaluateExpressionInputEventPropertiesValue>**](BillableMetricEvaluateExpressionInput_event_properties_value.md)> | This field represents additional properties associated with the event, which are utilized in the calculation of the final fee. This object becomes mandatory when the targeted billable metric employs a `sum_agg`, `max_agg`, or `unique_count_agg` aggregation method. However, when using a simple `count_agg`, this object is not required. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


