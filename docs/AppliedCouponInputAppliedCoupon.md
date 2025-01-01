# AppliedCouponInputAppliedCoupon

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**external_customer_id** | **String** | The customer external unique identifier (provided by your own application) | 
**coupon_code** | **String** | Unique code used to identify the coupon. | 
**frequency** | Option<**String**> | The type of frequency for the coupon. It can have three possible values: `once`, `recurring` or `forever`.  - If set to `once`, the coupon is applicable only for a single use. - If set to `recurring`, the coupon can be used multiple times for recurring billing periods. - If set to `forever`, the coupon has unlimited usage and can be applied indefinitely. | [optional]
**frequency_duration** | Option<**i32**> | Specifies the number of billing periods to which the coupon applies. This field is required only for coupons with a `recurring` frequency type | [optional]
**amount_cents** | Option<**i32**> | The amount of the coupon in cents. This field is required only for coupon with `fixed_amount` type. | [optional]
**amount_currency** | Option<[**models::Currency**](Currency.md)> | The currency of the coupon. This field is required only for coupon with `fixed_amount` type. | [optional]
**percentage_rate** | Option<**String**> | The percentage rate of the coupon. This field is required only for coupons with a `percentage` coupon type. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


