# CustomerChargeFiltersUsageObjectInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**units** | Option<**String**> | The number of units consumed for a specific charge filter related to a charge item. | [optional]
**amount_cents** | Option<**i32**> | The amount in cents, tax excluded, consumed for a specific charge filter related to a charge item. | [optional]
**events_count** | Option<**i32**> | The quantity of usage events that have been recorded for a particular charge filter during the specified time period. These events may also be referred to as the number of transactions in some contexts. | [optional]
**invoice_display_name** | Option<**String**> | Specifies the name that will be displayed on an invoice. | [optional]
**values** | Option<**std::collections::HashMap<String, String>**> | List of filter values applied to the usage. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


