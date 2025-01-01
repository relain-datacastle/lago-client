# WalletObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lago_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier assigned to the wallet within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the wallet's record within the Lago system. | 
**lago_customer_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier assigned to the customer within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the customer's record within the Lago system. | 
**external_customer_id** | **String** | The customer external unique identifier (provided by your own application) | 
**status** | **String** | The status of the wallet. Possible values are `active` or `terminated`. | 
**currency** | [**models::Currency**](Currency.md) | The currency of the wallet. | 
**name** | Option<**String**> | The name of the wallet. | [optional]
**rate_amount** | **String** | The rate of conversion between credits and the amount in the specified currency. It indicates the ratio or factor used to convert credits into the corresponding monetary value in the currency of the transaction. | 
**credits_balance** | **String** | The current wallet balance expressed in credits. This reflects the available balance after all transactions are settled. | 
**balance_cents** | **i32** | The current wallet balance expressed in cents. This reflects the available balance after all transactions are settled. | 
**consumed_credits** | **String** | The number of consumed credits. | 
**created_at** | **String** | The date of the wallet creation, represented in ISO 8601 datetime format and expressed in Coordinated Universal Time (UTC). | 
**expiration_at** | Option<**String**> | The date and time that determines when the wallet will expire. It follows the ISO 8601 datetime format and is expressed in Coordinated Universal Time (UTC). | [optional]
**last_balance_sync_at** | Option<**String**> | The date and time of the last balance top-up. It follows the ISO 8601 datetime format and is expressed in Coordinated Universal Time (UTC). | [optional]
**last_consumed_credit_at** | Option<**String**> | The date and time of the last credits consumption. It follows the ISO 8601 datetime format and is expressed in Coordinated Universal Time (UTC). | [optional]
**terminated_at** | Option<**String**> | The date of terminaison of the wallet. It follows the ISO 8601 datetime format and is expressed in Coordinated Universal Time (UTC). | [optional]
**invoice_requires_successful_payment** | Option<**bool**> | A boolean setting that, when set to true, delays issuing an invoice for a wallet top-up until a successful payment is made; if false, the invoice is issued immediately upon wallet top-up, regardless of the payment status. Default value of false. | [optional]
**recurring_transaction_rules** | Option<[**Vec<models::WalletObjectRecurringTransactionRulesInner>**](WalletObject_recurring_transaction_rules_inner.md)> | List of recurring transaction rules. Currently, we only allow one recurring rule per wallet. | [optional]
**ongoing_balance_cents** | **i32** | The ongoing wallet balance expressed in cents. This represents the `balance_cents` minus the `ongoing_usage_balance_cents`, showing the real time balance after accounting for current usage including taxes. | 
**ongoing_usage_balance_cents** | **i32** | The ongoing usage balance of the wallet, expressed in cents. This reflects all current usage and draft invoices including taxes. | 
**credits_ongoing_balance** | **String** | The ongoing wallet balance expressed in credits. This represents the `credits_balance` minus the `credits_ongoing_usage_balance`, showing the real time balance after accounting for current usage including taxes. | 
**credits_ongoing_usage_balance** | **String** | The ongoing usage balance of the wallet, expressed in credits. This reflects all current usage and draft invoices including taxes. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


