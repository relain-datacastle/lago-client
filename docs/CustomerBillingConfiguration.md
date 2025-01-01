# CustomerBillingConfiguration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**invoice_grace_period** | Option<**i32**> | The grace period, expressed in days, for the invoice. This period refers to the additional time granted to the customer beyond the invoice due date to adjust usage and line items | [optional]
**payment_provider** | Option<**String**> | The payment provider utilized to initiate payments for invoices issued by Lago. Accepted values: `stripe`, `adyen`, `gocardless` or null. This field is required if you intend to assign a `provider_customer_id`. | [optional]
**payment_provider_code** | Option<**String**> | Unique code used to identify a payment provider connection. | [optional]
**provider_customer_id** | Option<**String**> | The customer ID within the payment provider's system. If this field is not provided, Lago has the option to create a new customer record within the payment provider's system on behalf of the customer | [optional]
**sync** | Option<**bool**> | Set this field to `true` if you want to create the customer in the payment provider synchronously with the customer creation process in Lago. This option is applicable only when the `provider_customer_id` is `null` and the customer is automatically created in the payment provider through Lago. By default, the value is set to `false` | [optional]
**sync_with_provider** | Option<**bool**> | Set this field to `true` if you want to create a customer record in the payment provider's system. This option is applicable only when the `provider_customer_id` is null and the `sync_with_provider` field is set to `true`. By default, the value is set to `false` | [optional]
**document_locale** | Option<**String**> | The document locale, specified in the ISO 639-1 format. This field represents the language or locale used for the documents issued by Lago | [optional]
**provider_payment_methods** | Option<**Vec<String>**> | Specifies the available payment methods that can be used for this customer when `payment_provider` is set to `stripe`. The `provider_payment_methods` field is an array that allows multiple payment options to be defined. If this field is not explicitly set, the payment methods will be set to `card`. For now, possible values are `card`, `sepa_debit`, `us_bank_account`, `bacs_debit` and `link`. Note that when `link` is selected, `card` should also be provided in the array. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


