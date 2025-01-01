# \PlansApi

All URIs are relative to *https://api.getlago.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_plan**](PlansApi.md#create_plan) | **POST** /plans | Create a plan
[**destroy_plan**](PlansApi.md#destroy_plan) | **DELETE** /plans/{code} | Delete a plan
[**find_all_plans**](PlansApi.md#find_all_plans) | **GET** /plans | List all plans
[**find_plan**](PlansApi.md#find_plan) | **GET** /plans/{code} | Retrieve a plan
[**update_plan**](PlansApi.md#update_plan) | **PUT** /plans/{code} | Update a plan



## create_plan

> models::Plan create_plan(plan_create_input)
Create a plan

This endpoint creates a plan with subscription and usage-based charges. It supports flexible billing cadence (in-advance or in-arrears) and allows for both recurring and metered charges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plan_create_input** | [**PlanCreateInput**](PlanCreateInput.md) | Plan payload | [required] |

### Return type

[**models::Plan**](Plan.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_plan

> models::Plan destroy_plan(code)
Delete a plan

This endpoint deletes a specific plan. Note that this plan could be associated with active subscriptions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | The code of the plan. It serves as a unique identifier associated with a particular plan. The code is typically used for internal or system-level identification purposes, like assigning a subscription, for instance. | [required] |

### Return type

[**models::Plan**](Plan.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_all_plans

> models::PlansPaginated find_all_plans(page, per_page)
List all plans

This endpoint retrieves all existing plans.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number. |  |
**per_page** | Option<**i32**> | Number of records per page. |  |

### Return type

[**models::PlansPaginated**](PlansPaginated.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_plan

> models::Plan find_plan(code)
Retrieve a plan

This endpoint retrieves a specific plan.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | The code of the plan. It serves as a unique identifier associated with a particular plan. The code is typically used for internal or system-level identification purposes, like assigning a subscription, for instance. | [required] |

### Return type

[**models::Plan**](Plan.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_plan

> models::Plan update_plan(code, plan_update_input)
Update a plan

This endpoint updates a specific plan with subscription and usage-based charges. It supports flexible billing cadence (in-advance or in-arrears) and allows for both recurring and metered charges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | The code of the plan. It serves as a unique identifier associated with a particular plan. The code is typically used for internal or system-level identification purposes, like assigning a subscription, for instance. | [required] |
**plan_update_input** | [**PlanUpdateInput**](PlanUpdateInput.md) | Plan payload | [required] |

### Return type

[**models::Plan**](Plan.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

