# BillableMetricObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lago_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier of the billable metric created by Lago. | 
**name** | **String** | Name of the billable metric. | 
**code** | **String** | Unique code used to identify the billable metric associated with the API request. This code associates each event with the correct metric. | 
**description** | Option<**String**> | Internal description of the billable metric. | [optional]
**recurring** | **bool** | Defines if the billable metric is persisted billing period over billing period.  - If set to `true`: the accumulated number of units calculated from the previous billing period is persisted to the next billing period. - If set to `false`: the accumulated number of units is reset to 0 at the end of the billing period. - If not defined in the request, default value is `false`. | 
**rounding_function** | Option<**String**> | Refers to the numeric value or mathematical expression that will be rounded based on the calculated number of billing units. Possible values are `round`, `ceil` and `floor`. | [optional]
**rounding_precision** | Option<**i32**> | Specifies the number of decimal places to which the `rounding_function` will be rounded. It can be a positive or negative value. | [optional]
**created_at** | **String** | Creation date of the billable metric. | 
**expression** | Option<**String**> | Expression used to calculate the event units. The expression is evalutated for each event and the result is then used to calculate the total aggregated units. | [optional]
**field_name** | Option<**String**> | Property of the billable metric used for aggregating usage data. This field is not required for `count_agg`. | [optional]
**aggregation_type** | **String** | Aggregation method used to compute usage for this billable metric. | 
**weighted_interval** | Option<**String**> | Parameter exclusively utilized in conjunction with the `weighted_sum` aggregation type. It serves to adjust the aggregation result by assigning weights and proration to the result based on time intervals. When this field is not provided, the default time interval is assumed to be in `seconds`. | [optional]
**filters** | Option<[**Vec<models::BillableMetricFilterObject>**](BillableMetricFilterObject.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


