# WalletUpdateInputWallet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the wallet. | [optional]
**expiration_at** | Option<**String**> | The date and time that determines when the wallet will expire. It follows the ISO 8601 datetime format and is expressed in Coordinated Universal Time (UTC). | [optional]
**invoice_requires_successful_payment** | Option<**bool**> | A boolean setting that, when set to true, delays issuing an invoice for a wallet top-up until a successful payment is made; if false, the invoice is issued immediately upon wallet top-up, regardless of the payment status. Default value of false. | [optional]
**recurring_transaction_rules** | Option<[**Vec<models::WalletUpdateInputWalletRecurringTransactionRulesInner>**](WalletUpdateInput_wallet_recurring_transaction_rules_inner.md)> | List of recurring transaction rules. Currently, we only allow one recurring rule per wallet. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


