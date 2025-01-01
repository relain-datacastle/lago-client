# AppliedCouponObjectExtended

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lago_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier of the applied coupon, created by Lago. | 
**lago_coupon_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier of the coupon, created by Lago. | 
**coupon_code** | **String** | Unique code used to identify the coupon. | 
**coupon_name** | **String** | The name of the coupon. | 
**lago_customer_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier of the customer, created by Lago. | 
**external_customer_id** | **String** | The customer external unique identifier (provided by your own application) | 
**status** | **String** | The status of the coupon. Can be either `active` or `terminated`. | 
**amount_cents** | Option<**i32**> | The amount of the coupon in cents. This field is required only for coupon with `fixed_amount` type. | [optional]
**amount_cents_remaining** | Option<**i32**> | The remaining amount in cents for a `fixed_amount` coupon with a frequency set to `once`. This field indicates the remaining balance or value that can still be utilized from the coupon. | [optional]
**amount_currency** | Option<[**models::Currency**](Currency.md)> | The currency of the coupon. This field is required only for coupon with `fixed_amount` type. | [optional]
**percentage_rate** | Option<**String**> | The percentage rate of the coupon. This field is required only for coupons with a `percentage` coupon type. | [optional]
**frequency** | **String** | The type of frequency for the coupon. It can have three possible values: `once`, `recurring` or `forever`.  - If set to `once`, the coupon is applicable only for a single use. - If set to `recurring`, the coupon can be used multiple times for recurring billing periods. - If set to `forever`, the coupon has unlimited usage and can be applied indefinitely. | 
**frequency_duration** | Option<**i32**> | Specifies the number of billing periods to which the coupon applies. This field is required only for coupons with a `recurring` frequency type | [optional]
**frequency_duration_remaining** | Option<**i32**> | The remaining number of billing periods to which the coupon is applicable. This field determines the remaining usage or availability of the coupon based on the remaining billing periods. | [optional]
**expiration_at** | Option<**String**> | The date and time after which the coupon will stop applying to customer's invoices. Once the expiration date is reached, the coupon will no longer be applicable, and any further invoices generated for the customer will not include the coupon discount. | [optional]
**created_at** | **String** | The date and time when the coupon was assigned to a customer. It is expressed in UTC format according to the ISO 8601 datetime standard. | 
**terminated_at** | Option<**String**> | This field indicates the specific moment when the coupon amount is fully utilized or when the coupon is removed from the customer's coupon list. It is expressed in UTC format according to the ISO 8601 datetime standard. | [optional]
**credits** | [**Vec<models::CreditObject>**](CreditObject.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


