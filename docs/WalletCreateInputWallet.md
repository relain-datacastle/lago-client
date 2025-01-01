# WalletCreateInputWallet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the wallet. | [optional]
**rate_amount** | **String** | The rate of conversion between credits and the amount in the specified currency. It indicates the ratio or factor used to convert credits into the corresponding monetary value in the currency of the transaction. | 
**currency** | [**models::Currency**](Currency.md) | The currency of the wallet. | 
**paid_credits** | Option<**String**> | The number of paid credits. Required only if there is no granted credits. | [optional]
**granted_credits** | Option<**String**> | The number of free granted credits. Required only if there is no paid credits. | [optional]
**external_customer_id** | **String** | The customer external unique identifier (provided by your own application) | 
**expiration_at** | Option<**String**> | The date and time that determines when the wallet will expire. It follows the ISO 8601 datetime format and is expressed in Coordinated Universal Time (UTC). | [optional]
**invoice_requires_successful_payment** | Option<**bool**> | A boolean setting that, when set to true, delays issuing an invoice for a wallet top-up until a successful payment is made; if false, the invoice is issued immediately upon wallet top-up, regardless of the payment status. Default value of false. | [optional]
**transaction_metadata** | Option<[**Vec<models::WalletCreateInputWalletTransactionMetadataInner>**](WalletCreateInput_wallet_transaction_metadata_inner.md)> | This optional field allows you to store a list of key-value pairs that provide additional information or custom attributes. These key-value pairs will be included in the metadata of wallet transactions generated during the wallet creation process. | [optional]
**recurring_transaction_rules** | Option<[**Vec<models::WalletCreateInputWalletRecurringTransactionRulesInner>**](WalletCreateInput_wallet_recurring_transaction_rules_inner.md)> | List of recurring transaction rules. Currently, we only allow one recurring rule per wallet. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


