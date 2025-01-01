# CustomerChargeUsageObjectBillableMetric

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lago_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier assigned to the billable metric within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the billable metric's record within the Lago system. | 
**name** | **String** | The name of the billable metric used for this charge. | 
**code** | **String** | The code of the billable metric used for this charge. | 
**aggregation_type** | **String** | The aggregation type of the billable metric used for this charge. Possible values are `count_agg`, `sum_agg`, `max_agg` or `unique_count_agg`. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


