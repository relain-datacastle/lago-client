# ChargeObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lago_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier of charge, created by Lago. | 
**lago_billable_metric_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier of the billable metric created by Lago. | 
**billable_metric_code** | **String** | Unique code identifying a billable metric. | 
**invoice_display_name** | Option<**String**> | Specifies the name that will be displayed on an invoice. If no value is set for this field, the name of the actual charge will be used as the default display name. | [optional]
**created_at** | **String** | The date and time when the charge was created. It is expressed in UTC format according to the ISO 8601 datetime standard. | 
**charge_model** | **String** | Specifies the pricing model used for the calculation of the final fee. It can be `standard`, `graduated`, `graduated_percentage`, `package`, `percentage`, `volume` or `dynamic`. | 
**pay_in_advance** | Option<**bool**> | This field determines the billing timing for this specific usage-based charge. When set to `true`, the charge is due and invoiced immediately. Conversely, when set to `false`, the charge is due and invoiced at the end of each billing period. | [optional]
**invoiceable** | Option<**bool**> | This field specifies whether the charge should be included in a proper invoice. If set to `false`, no invoice will be issued for this charge. You can only set it to `false` when `pay_in_advance` is `true`. | [optional]
**regroup_paid_fees** | Option<**String**> | This setting can only be configured if `pay_in_advance` is `true` and `invoiceable` is `false`. This field determines whether and when the charge fee should be included in the invoice. If `null`, no invoice will be issued for this charge fee. If `invoice`, an invoice will be generated at the end of the period, consolidating all charge fees with a succeeded payment status. | [optional]
**prorated** | Option<**bool**> | Specifies whether a charge is prorated based on the remaining number of days in the billing period or billed fully.  - If set to `true`, the charge is prorated based on the remaining days in the current billing period. - If set to `false`, the charge is billed in full. - If not defined in the request, default value is `false`. | [optional]
**min_amount_cents** | Option<**i32**> | The minimum spending amount required for the charge, measured in cents and excluding any applicable taxes. It indicates the minimum amount that needs to be charged for each billing period. | [optional]
**properties** | Option<[**models::ChargeProperties**](ChargeProperties.md)> | List of all thresholds utilized for calculating the charge. | [optional]
**filters** | Option<[**Vec<models::ChargeFilterObject>**](ChargeFilterObject.md)> | List of filters used to apply differentiated pricing based on additional event properties. | [optional]
**taxes** | Option<[**Vec<models::TaxObject>**](TaxObject.md)> | All taxes applied to the charge. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

