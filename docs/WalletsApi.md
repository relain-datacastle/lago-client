# \WalletsApi

All URIs are relative to *https://api.getlago.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_wallet**](WalletsApi.md#create_wallet) | **POST** /wallets | Create a wallet
[**create_wallet_transaction**](WalletsApi.md#create_wallet_transaction) | **POST** /wallet_transactions | Top up a wallet
[**destroy_wallet**](WalletsApi.md#destroy_wallet) | **DELETE** /wallets/{lago_id} | Terminate a wallet
[**find_all_wallet_transactions**](WalletsApi.md#find_all_wallet_transactions) | **GET** /wallets/{lago_id}/wallet_transactions | List all wallet transactions
[**find_all_wallets**](WalletsApi.md#find_all_wallets) | **GET** /wallets | List all wallets
[**find_wallet**](WalletsApi.md#find_wallet) | **GET** /wallets/{lago_id} | Retrieve a wallet
[**update_wallet**](WalletsApi.md#update_wallet) | **PUT** /wallets/{lago_id} | Update a wallet



## create_wallet

> models::Wallet create_wallet(wallet_create_input)
Create a wallet

This endpoint is used to create a wallet with prepaid credits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_create_input** | [**WalletCreateInput**](WalletCreateInput.md) | Wallet payload | [required] |

### Return type

[**models::Wallet**](Wallet.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_wallet_transaction

> models::WalletTransactions create_wallet_transaction(wallet_transaction_create_input)
Top up a wallet

This endpoint is used to top-up an active wallet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_transaction_create_input** | [**WalletTransactionCreateInput**](WalletTransactionCreateInput.md) | Wallet transaction payload | [required] |

### Return type

[**models::WalletTransactions**](WalletTransactions.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_wallet

> models::Wallet destroy_wallet(lago_id)
Terminate a wallet

This endpoint is used to terminate an existing wallet with prepaid credits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lago_id** | **uuid::Uuid** | Unique identifier assigned to the wallet within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the wallet's record within the Lago system. | [required] |

### Return type

[**models::Wallet**](Wallet.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_all_wallet_transactions

> models::WalletTransactionsPaginated find_all_wallet_transactions(lago_id, page, per_page, status, transaction_status, transaction_type)
List all wallet transactions

This endpoint is used to list all wallet transactions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lago_id** | **uuid::Uuid** | Unique identifier assigned to the wallet within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the wallet's record within the Lago system. | [required] |
**page** | Option<**i32**> | Page number. |  |
**per_page** | Option<**i32**> | Number of records per page. |  |
**status** | Option<**String**> | The status of the wallet transaction. Possible values are `pending` or `settled`. |  |
**transaction_status** | Option<**String**> | The transaction status of the wallet transaction. Possible values are `purchased` (with pending or settled status), `granted` (without invoice_id), `voided` or `invoiced`. |  |
**transaction_type** | Option<**String**> | The transaction type of the wallet transaction. Possible values are `inbound` (increasing the wallet balance) or `outbound` (decreasing the wallet balance). |  |

### Return type

[**models::WalletTransactionsPaginated**](WalletTransactionsPaginated.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_all_wallets

> models::WalletsPaginated find_all_wallets(external_customer_id, page, per_page)
List all wallets

This endpoint is used to list all wallets with prepaid credits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_customer_id** | **String** | The customer external unique identifier (provided by your own application). | [required] |
**page** | Option<**i32**> | Page number. |  |
**per_page** | Option<**i32**> | Number of records per page. |  |

### Return type

[**models::WalletsPaginated**](WalletsPaginated.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_wallet

> models::Wallet find_wallet(lago_id)
Retrieve a wallet

This endpoint is used to retrieve an existing wallet with prepaid credits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lago_id** | **uuid::Uuid** | Unique identifier assigned to the wallet within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the wallet's record within the Lago system. | [required] |

### Return type

[**models::Wallet**](Wallet.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_wallet

> models::Wallet update_wallet(lago_id, wallet_update_input)
Update a wallet

This endpoint is used to update an existing wallet with prepaid credits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lago_id** | **uuid::Uuid** | Unique identifier assigned to the wallet within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the wallet's record within the Lago system. | [required] |
**wallet_update_input** | [**WalletUpdateInput**](WalletUpdateInput.md) | Wallet update payload | [required] |

### Return type

[**models::Wallet**](Wallet.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

