# BillableMetricEvaluateExpressionInput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**expression** | **String** | Expression used to calculate the event units. The expression is evalutated for each event and the result is then used to calculate the total aggregated units. Accepted function are `ceil`, `concat` and `round` as well as `+`, `-`, `\\` and `*` operations. Round is accepting an optional second parameter to specify the number of decimal.  | 
**event** | [**models::BillableMetricEvaluateExpressionInputEvent**](BillableMetricEvaluateExpressionInput_event.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


