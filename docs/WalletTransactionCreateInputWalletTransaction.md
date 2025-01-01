# WalletTransactionCreateInputWalletTransaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**wallet_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier assigned to the wallet within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the wallet's record within the Lago system. | 
**paid_credits** | Option<**String**> | The number of paid credits. | [optional]
**granted_credits** | Option<**String**> | The number of free granted credits. | [optional]
**voided_credits** | Option<**String**> | The number of voided credits. | [optional]
**invoice_requires_successful_payment** | Option<**bool**> | A boolean setting that, when set to true, delays issuing an invoice for a wallet top-up until a successful payment is made; if false, the invoice is issued immediately upon wallet top-up, regardless of the payment status. Default value of false. | [optional]
**metadata** | Option<[**Vec<models::WalletCreateInputWalletRecurringTransactionRulesInnerTransactionMetadataInner>**](WalletCreateInput_wallet_recurring_transaction_rules_inner_transaction_metadata_inner.md)> | This optional field allows you to store a list of key-value pairs that hold additional information or custom attributes related to the data. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


