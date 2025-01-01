# \SubscriptionsApi

All URIs are relative to *https://api.getlago.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_subscription**](SubscriptionsApi.md#create_subscription) | **POST** /subscriptions | Assign a plan to a customer
[**destroy_subscription**](SubscriptionsApi.md#destroy_subscription) | **DELETE** /subscriptions/{external_id} | Terminate a subscription
[**find_all_subscriptions**](SubscriptionsApi.md#find_all_subscriptions) | **GET** /subscriptions | List all subscriptions
[**find_subscription**](SubscriptionsApi.md#find_subscription) | **GET** /subscriptions/{external_id} | Retrieve a subscription
[**get_subscription_lifetime_usage**](SubscriptionsApi.md#get_subscription_lifetime_usage) | **GET** /subscriptions/{external_id}/lifetime_usage | Retrive subscription lifetime usage
[**update_subscription**](SubscriptionsApi.md#update_subscription) | **PUT** /subscriptions/{external_id} | Update a subscription
[**update_subscription_lifetime_usage**](SubscriptionsApi.md#update_subscription_lifetime_usage) | **PUT** /subscriptions/{external_id}/lifetime_usage | Update a subscription lifetime usage



## create_subscription

> models::Subscription create_subscription(subscription_create_input)
Assign a plan to a customer

This endpoint assigns a plan to a customer, creating or modifying a subscription. Ideal for initial subscriptions or plan changes (upgrades/downgrades).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_create_input** | [**SubscriptionCreateInput**](SubscriptionCreateInput.md) | Subscription payload | [required] |

### Return type

[**models::Subscription**](Subscription.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_subscription

> models::Subscription destroy_subscription(external_id, status)
Terminate a subscription

This endpoint allows you to terminate a subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_id** | **String** | External ID of the existing subscription | [required] |
**status** | Option<**String**> | If the field is not defined, Lago will terminate only `active` subscriptions. However, if you wish to cancel a `pending` subscription, please ensure that you include `status=pending` in your request. |  |

### Return type

[**models::Subscription**](Subscription.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_all_subscriptions

> models::SubscriptionsPaginated find_all_subscriptions(page, per_page, external_customer_id, plan_code, status_left_square_bracket_right_square_bracket)
List all subscriptions

This endpoint retrieves all active subscriptions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number. |  |
**per_page** | Option<**i32**> | Number of records per page. |  |
**external_customer_id** | Option<**String**> | The customer external unique identifier (provided by your own application) |  |
**plan_code** | Option<**String**> | The unique code representing the plan to be attached to the customer. This code must correspond to the code property of one of the active plans. |  |
**status_left_square_bracket_right_square_bracket** | Option<[**Vec<String>**](String.md)> | If the field is not defined, Lago will return only `active` subscriptions. However, if you wish to fetch subscriptions by different status you can define them in a status[] query param. Available filter values: `pending`, `canceled`, `terminated`, `active`. |  |

### Return type

[**models::SubscriptionsPaginated**](SubscriptionsPaginated.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_subscription

> models::Subscription find_subscription(external_id)
Retrieve a subscription

This endpoint retrieves a specific subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_id** | **String** | External ID of the existing subscription | [required] |

### Return type

[**models::Subscription**](Subscription.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subscription_lifetime_usage

> models::LifetimeUsage get_subscription_lifetime_usage(external_id)
Retrive subscription lifetime usage

This endpoint enables the retrieval of the lifetime usage of a subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_id** | **String** | External ID of the existing subscription | [required] |

### Return type

[**models::LifetimeUsage**](LifetimeUsage.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_subscription

> models::Subscription update_subscription(external_id, subscription_update_input)
Update a subscription

This endpoint allows you to update a subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_id** | **String** | External ID of the existing subscription | [required] |
**subscription_update_input** | [**SubscriptionUpdateInput**](SubscriptionUpdateInput.md) | Update an existing subscription | [required] |

### Return type

[**models::Subscription**](Subscription.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_subscription_lifetime_usage

> models::LifetimeUsage update_subscription_lifetime_usage(external_id, lifetime_usage_input)
Update a subscription lifetime usage

This endpoint allows you to update the lifetime usage of a subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_id** | **String** | External ID of the existing subscription | [required] |
**lifetime_usage_input** | [**LifetimeUsageInput**](LifetimeUsageInput.md) | Update the lifetime usage of a subscription | [required] |

### Return type

[**models::LifetimeUsage**](LifetimeUsage.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

