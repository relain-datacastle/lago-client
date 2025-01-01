# CustomerChargeUsageObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**units** | **String** | The number of units consumed by the customer for a specific charge item. | 
**events_count** | **i32** | The quantity of usage events that have been recorded for a particular charge during the specified time period. These events may also be referred to as the number of transactions in some contexts. | 
**amount_cents** | **i32** | The amount in cents, tax excluded, consumed by the customer for a specific charge item. | 
**amount_currency** | [**models::Currency**](Currency.md) | The currency of a usage item consumed by the customer. | 
**charge** | [**models::CustomerChargeUsageObjectCharge**](CustomerChargeUsageObject_charge.md) |  | 
**billable_metric** | [**models::CustomerChargeUsageObjectBillableMetric**](CustomerChargeUsageObject_billable_metric.md) |  | 
**filters** | Option<[**Vec<models::CustomerChargeFiltersUsageObjectInner>**](CustomerChargeFiltersUsageObject_inner.md)> | Array of filter object, representing multiple dimensions for a charge item. | [optional]
**grouped_usage** | Option<[**Vec<models::CustomerChargeGroupedUsageObjectInner>**](CustomerChargeGroupedUsageObject_inner.md)> | Array of aggregated fees, grouped by the event properties defined in a `standard` charge model. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


