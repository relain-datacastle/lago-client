# EventEstimateFeesInputEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **String** | The code that identifies a targeted billable metric. It is essential that this code matches the `code` property of one of your active billable metrics. If the provided code does not correspond to any active billable metric, it will be ignored during the process. | 
**external_subscription_id** | **String** | The unique identifier of the subscription within your application. | 
**properties** | Option<[**serde_json::Value**](.md)> | This field represents additional properties associated with the event, which are utilized in the calculation of the final fee. This object becomes mandatory when the targeted billable metric employs a `sum_agg`, `max_agg`, or `unique_count_agg` aggregation method. However, when using a simple `count_agg`, this object is not required. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


