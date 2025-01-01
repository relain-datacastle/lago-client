# CouponObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lago_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier of the coupon, created by Lago. | 
**name** | **String** | The name of the coupon. | 
**code** | **String** | Unique code used to identify the coupon. | 
**description** | Option<**String**> | Description of the coupon. | [optional]
**coupon_type** | **String** | The type of the coupon. It can have two possible values: `fixed_amount` or `percentage`.  - If set to `fixed_amount`, the coupon represents a fixed amount discount. - If set to `percentage`, the coupon represents a percentage-based discount. | 
**amount_cents** | Option<**i32**> | The amount of the coupon in cents. This field is required only for coupon with `fixed_amount` type. | [optional]
**amount_currency** | Option<[**models::Currency**](Currency.md)> | The currency of the coupon. This field is required only for coupon with `fixed_amount` type. | [optional]
**reusable** | **bool** | Indicates whether the coupon can be reused or not. If set to `true`, the coupon is reusable, meaning it can be applied multiple times to the same customer. If set to `false`, the coupon can only be used once and is not reusable. If not specified, this field is set to `true` by default. | 
**limited_plans** | **bool** | The coupon is limited to specific plans. The possible values can be `true` or `false`. | 
**plan_codes** | Option<**Vec<String>**> | An array of plan codes to which the coupon is applicable. By specifying the plan codes in this field, you can restrict the coupon's usage to specific plans only. | [optional]
**limited_billable_metrics** | **bool** | The coupon is limited to specific billable metrics. The possible values can be `true` or `false`. | 
**billable_metric_codes** | Option<**Vec<String>**> | An array of billable metric codes to which the coupon is applicable. By specifying the billable metric codes in this field, you can restrict the coupon's usage to specific metrics only. | [optional]
**percentage_rate** | Option<**String**> | The percentage rate of the coupon. This field is required only for coupons with a `percentage` coupon type. | [optional]
**frequency** | **String** | The type of frequency for the coupon. It can have three possible values: `once`, `recurring`, or `forever`.  - If set to `once`, the coupon is applicable only for a single use. - If set to `recurring`, the coupon can be used multiple times for recurring billing periods. - If set to `forever`, the coupon has unlimited usage and can be applied indefinitely. | 
**frequency_duration** | Option<**i32**> | Specifies the number of billing periods to which the coupon applies. This field is required only for coupons with a `recurring` frequency type | [optional]
**expiration** | **String** | Specifies the type of expiration for the coupon. It can have two possible values: `time_limit` or `no_expiration`.  - If set to `time_limit`, the coupon has an expiration based on a specified time limit. - If set to `no_expiration`, the coupon does not have an expiration date and remains valid indefinitely. | 
**expiration_at** | Option<**String**> | The expiration date and time of the coupon. This field is required only for coupons with `expiration` set to `time_limit`. The expiration date and time should be specified in UTC format according to the ISO 8601 datetime standard. It indicates the exact moment when the coupon will expire and is no longer valid. | [optional]
**created_at** | **String** | The date and time when the coupon was created. It is expressed in UTC format according to the ISO 8601 datetime standard. This field provides the timestamp for the exact moment when the coupon was initially created. | 
**terminated_at** | Option<**String**> | This field indicates if the coupon has been terminated and is no longer usable. If it's not null, it won't be removed for existing customers using it, but it prevents the coupon from being applied in the future. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


