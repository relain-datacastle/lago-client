# PlanUpdateInputPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the plan. | [optional]
**invoice_display_name** | Option<**String**> | Specifies the name that will be displayed on an invoice. If no value is set for this field, the name of the plan will be used as the default display name. | [optional]
**code** | Option<**String**> | The code of the plan. It serves as a unique identifier associated with a particular plan. The code is typically used for internal or system-level identification purposes, like assigning a subscription, for instance. | [optional]
**interval** | Option<**String**> | The interval used for recurring billing. It represents the frequency at which subscription billing occurs. The interval can be one of the following values: `yearly`, `quarterly`, `monthly`, or `weekly`. | [optional]
**description** | Option<**String**> | The description on the plan. | [optional]
**amount_cents** | Option<**i32**> | The base cost of the plan, excluding any applicable taxes, that is billed on a recurring basis. This value is defined at 0 if your plan is a pay-as-you-go plan. | [optional]
**amount_currency** | Option<[**models::Currency**](Currency.md)> | The currency of the plan. It indicates the monetary unit in which the plan's cost, including taxes and usage-based charges, is expressed. | [optional]
**trial_period** | Option<**f64**> | The duration in days during which the base cost of the plan is offered for free. | [optional]
**pay_in_advance** | Option<**bool**> | This field determines the billing timing for the plan. When set to `true`, the base cost of the plan is due at the beginning of each billing period. Conversely, when set to `false`, the base cost of the plan is due at the end of each billing period. | [optional]
**bill_charges_monthly** | Option<**bool**> | This field, when set to `true`, enables to invoice usage-based charges on monthly basis, even if the cadence of the plan is yearly. This allows customers to pay charges overage on a monthly basis. This can be set to true only if the plan's interval is `yearly`. | [optional]
**tax_codes** | Option<**Vec<String>**> | List of unique code used to identify the taxes. | [optional]
**minimum_commitment** | Option<[**models::MinimumCommitmentInput**](MinimumCommitmentInput.md)> |  | [optional]
**charges** | Option<[**Vec<models::PlanUpdateInputPlanChargesInner>**](PlanUpdateInput_plan_charges_inner.md)> | Additional usage-based charges for this plan. | [optional]
**usage_thresholds** | Option<[**Vec<models::UsageThresholdInput>**](UsageThresholdInput.md)> | List of usage thresholds to apply to the plan. | [optional]
**cascade_updates** | Option<**bool**> | This field determines whether the changes made to the plan should be cascaded to the children plans. When set to `true`, the changes will be cascaded into children. Conversely, when set to `false`, the changes will only be applied to the plan itself. If not defined in the request, default value is `false`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


