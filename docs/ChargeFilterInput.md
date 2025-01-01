# ChargeFilterInput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**invoice_display_name** | Option<**String**> | Specifies the name that will be displayed on an invoice. If no value is set for this field, the values of the filter will be used as the default display name. | [optional]
**properties** | Option<[**models::ChargeProperties**](ChargeProperties.md)> | List of all thresholds utilized for calculating the charge. | [optional]
**values** | Option<[**std::collections::HashMap<String, Vec<String>>**](Vec.md)> | List of possible filter values. The key and values must match one of the billable metric filters. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


