# PlanOverridesObjectChargesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique identifier of the charge created by Lago. | [optional]
**billable_metric_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique identifier of the billable metric created by Lago. | [optional]
**invoice_display_name** | Option<**String**> | Specifies the name that will be displayed on an invoice. If no value is set for this field, the name of the actual charge will be used as the default display name. | [optional]
**min_amount_cents** | Option<**i32**> | The minimum spending amount required for the charge, measured in cents and excluding any applicable taxes. It indicates the minimum amount that needs to be charged for each billing period. | [optional]
**properties** | Option<[**models::ChargeProperties**](ChargeProperties.md)> | List of all thresholds utilized for calculating the charge. | [optional]
**filters** | Option<[**Vec<models::ChargeFilterInput>**](ChargeFilterInput.md)> | List of filters used to apply differentiated pricing based on additional event properties. | [optional]
**tax_codes** | Option<**Vec<String>**> | List of unique code used to identify the taxes. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


