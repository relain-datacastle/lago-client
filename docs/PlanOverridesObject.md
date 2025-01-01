# PlanOverridesObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount_cents** | Option<**i32**> | The base cost of the plan, excluding any applicable taxes, that is billed on a recurring basis. This value is defined at 0 if your plan is a pay-as-you-go plan. | [optional]
**amount_currency** | Option<[**models::Currency**](Currency.md)> | The currency of the plan. It indicates the monetary unit in which the plan's cost, including taxes and usage-based charges, is expressed. | [optional]
**description** | Option<**String**> | The description on the plan. | [optional]
**invoice_display_name** | Option<**String**> | Specifies the name that will be displayed on an invoice. If no value is set for this field, the name of the plan will be used as the default display name. | [optional]
**name** | Option<**String**> | The name of the plan. | [optional]
**tax_codes** | Option<**Vec<String>**> | List of unique code used to identify the taxes. | [optional]
**trial_period** | Option<**f64**> | The duration in days during which the base cost of the plan is offered for free. | [optional]
**minimum_commitment** | Option<[**models::MinimumCommitmentObject**](MinimumCommitmentObject.md)> |  | [optional]
**charges** | Option<[**Vec<models::PlanOverridesObjectChargesInner>**](PlanOverridesObject_charges_inner.md)> | Additional usage-based charges for this plan. | [optional]
**usage_thresholds** | Option<[**Vec<models::UsageThresholdObject>**](UsageThresholdObject.md)> | List of usage thresholds applied to the subscription. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


