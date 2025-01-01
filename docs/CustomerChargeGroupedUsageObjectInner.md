# CustomerChargeGroupedUsageObjectInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount_cents** | Option<**i32**> | The amount in cents, tax excluded, consumed for a specific group related to a charge item. | [optional]
**events_count** | Option<**i32**> | The quantity of usage events that have been recorded for a particular charge during the specified time period. These events may also be referred to as the number of transactions in some contexts. | [optional]
**units** | Option<**String**> | The number of units consumed for a specific group related to a charge item. | [optional]
**grouped_by** | Option<**std::collections::HashMap<String, String>**> | Key value list of event properties aggregated by the charge model | [optional]
**filters** | Option<[**Vec<models::CustomerChargeFiltersUsageObjectInner>**](CustomerChargeFiltersUsageObject_inner.md)> | Array of filter object, representing multiple dimensions for a charge item. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


