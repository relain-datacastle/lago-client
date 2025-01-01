# FeeObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lago_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique identifier assigned to the fee within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the fee's record within the Lago system. | [optional]
**lago_charge_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique identifier assigned to the charge that the fee belongs to | [optional]
**lago_charge_filter_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique identifier assigned to the charge filter that the fee belongs to | [optional]
**lago_invoice_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique identifier assigned to the invoice that the fee belongs to | [optional]
**lago_true_up_fee_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique identifier assigned to the true-up fee when a minimum has been set to the charge. This identifier helps to distinguish and manage the true-up fee associated with the charge, which may be applicable when a minimum threshold or limit is set for the charge amount. | [optional]
**lago_true_up_parent_fee_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique identifier assigned to the parent fee on which the true-up fee is assigned. This identifier establishes the relationship between the parent fee and the associated true-up fee. | [optional]
**lago_subscription_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique identifier assigned to the subscription, created by Lago. This field is specifically displayed when the fee type is charge or subscription. | [optional]
**lago_customer_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique identifier assigned to the customer, created by Lago. This field is specifically displayed when the fee type is charge or subscription. | [optional]
**external_customer_id** | Option<**String**> | Unique identifier assigned to the customer in your application. This field is specifically displayed when the fee type is charge or subscription. | [optional]
**external_subscription_id** | Option<**String**> | Unique identifier assigned to the subscription in your application. This field is specifically displayed when the fee type is charge or subscription. | [optional]
**invoice_display_name** | Option<**String**> | Specifies the name that will be displayed on an invoice. If no value is set for this field, the name of the actual charge will be used as the default display name. | [optional]
**amount_cents** | **i32** | The cost of this specific fee, excluding any applicable taxes. | 
**precise_amount** | Option<**String**> | The cost of this specific fee, excluding any applicable taxes, with precision. | [optional]
**precise_total_amount** | Option<**String**> | The cost of this specific fee, including any applicable taxes, with precision. | [optional]
**amount_currency** | [**models::Currency**](Currency.md) | The currency of this specific fee. It indicates the monetary unit in which the fee's cost is expressed. | 
**taxes_amount_cents** | **i32** | The cost of the tax associated with this specific fee. | 
**taxes_precise_amount** | Option<**String**> | The cost of the tax associated with this specific fee, with precision. | [optional]
**taxes_rate** | **f64** | The tax rate associated with this specific fee. | 
**units** | **String** | The number of units used to charge the customer. This field indicates the quantity or count of units consumed or utilized in the context of the charge. It helps in determining the basis for calculating the fee or cost associated with the usage of the service or product provided to the customer. | 
**precise_unit_amount** | **String** | The unit amount of the fee per unit, with precision. | 
**total_amount_cents** | **i32** | The cost of this specific fee, including any applicable taxes. | 
**total_amount_currency** | [**models::Currency**](Currency.md) | The currency of this specific fee, including any applicable taxes. | 
**events_count** | Option<**i32**> | The number of events that have been sent and used to charge the customer. This field indicates the count or quantity of events that have been processed and considered in the charging process. | [optional]
**pay_in_advance** | **bool** | Flag that indicates whether the fee was paid in advance. It serves as a boolean value, where `true` represents that the fee was paid in advance (straightaway), and `false` indicates that the fee was not paid in arrears (at the end of the period). | 
**invoiceable** | **bool** | Flag that indicates whether the fee was included on the invoice. It serves as a boolean value, where `true` represents that the fee was included on the invoice, and `false` indicates that the fee was not included on the invoice. | 
**from_date** | Option<**String**> | The beginning date of the period that the fee covers. It is applicable only to `subscription` and `charge` fees. This field indicates the start date of the billing period or subscription period associated with the fee. | [optional]
**to_date** | Option<**String**> | The ending date of the period that the fee covers. It is applicable only to `subscription` and `charge` fees. This field indicates the end date of the billing period or subscription period associated with the fee. | [optional]
**payment_status** | **String** | Indicates the payment status of the fee. It represents the current status of the payment associated with the fee. The possible values for this field are `pending`, `succeeded`, `failed` and `refunded`. | 
**created_at** | Option<**String**> | The date and time when the fee was created. It is provided in Coordinated Universal Time (UTC) format. | [optional]
**succeeded_at** | Option<**String**> | The date and time when the payment for the fee was successfully processed. It is provided in Coordinated Universal Time (UTC) format. | [optional]
**failed_at** | Option<**String**> | The date and time when the payment for the fee failed to process. It is provided in Coordinated Universal Time (UTC) format. | [optional]
**refunded_at** | Option<**String**> | The date and time when the payment for the fee was refunded. It is provided in Coordinated Universal Time (UTC) format | [optional]
**event_transaction_id** | Option<**String**> | Unique identifier assigned to the transaction. This field is specifically displayed when the fee type is `charge` and the payment for the fee is made in advance (`pay_in_advance` is set to `true`). | [optional]
**amount_details** | Option<[**models::FeeObjectAmountDetails**](FeeObject_amount_details.md)> |  | [optional]
**item** | [**models::FeeObjectItem**](FeeObject_item.md) |  | 
**applied_taxes** | Option<[**Vec<models::FeeAppliedTaxObject>**](FeeAppliedTaxObject.md)> | List of fee applied taxes | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


