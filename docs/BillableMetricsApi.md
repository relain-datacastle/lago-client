# \BillableMetricsApi

All URIs are relative to *https://api.getlago.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_billable_metric**](BillableMetricsApi.md#create_billable_metric) | **POST** /billable_metrics | Create a billable metric
[**destroy_billable_metric**](BillableMetricsApi.md#destroy_billable_metric) | **DELETE** /billable_metrics/{code} | Delete a billable metric
[**evaluate_billable_metric_expression**](BillableMetricsApi.md#evaluate_billable_metric_expression) | **POST** /billable_metrics/evaluate_expression | Evaluate an expression for a billable metric
[**find_all_billable_metrics**](BillableMetricsApi.md#find_all_billable_metrics) | **GET** /billable_metrics | List all billable metrics
[**find_billable_metric**](BillableMetricsApi.md#find_billable_metric) | **GET** /billable_metrics/{code} | Retrieve a billable metric
[**update_billable_metric**](BillableMetricsApi.md#update_billable_metric) | **PUT** /billable_metrics/{code} | Update a billable metric



## create_billable_metric

> models::BillableMetric create_billable_metric(billable_metric_create_input)
Create a billable metric

This endpoint creates a new billable metric representing a pricing component of your application.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**billable_metric_create_input** | [**BillableMetricCreateInput**](BillableMetricCreateInput.md) | Billable metric payload | [required] |

### Return type

[**models::BillableMetric**](BillableMetric.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_billable_metric

> models::BillableMetric destroy_billable_metric(code)
Delete a billable metric

This endpoint deletes an existing billable metric representing a pricing component of your application.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | Code of the existing billable metric. | [required] |

### Return type

[**models::BillableMetric**](BillableMetric.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## evaluate_billable_metric_expression

> models::BillableMetricEvaluateExpressionResult evaluate_billable_metric_expression(billable_metric_evaluate_expression_input)
Evaluate an expression for a billable metric

Evaluate an expression for a billable metric creation by providing the expression and test data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**billable_metric_evaluate_expression_input** | [**BillableMetricEvaluateExpressionInput**](BillableMetricEvaluateExpressionInput.md) | Billable metric expression evaluation payload | [required] |

### Return type

[**models::BillableMetricEvaluateExpressionResult**](BillableMetricEvaluateExpressionResult.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_all_billable_metrics

> models::BillableMetricsPaginated find_all_billable_metrics(page, per_page)
List all billable metrics

This endpoint retrieves all existing billable metrics that represent pricing components of your application.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number. |  |
**per_page** | Option<**i32**> | Number of records per page. |  |

### Return type

[**models::BillableMetricsPaginated**](BillableMetricsPaginated.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_billable_metric

> models::BillableMetric find_billable_metric(code)
Retrieve a billable metric

This endpoint retrieves an existing billable metric that represents a pricing component of your application. The billable metric is identified by its unique code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | Code of the existing billable metric. | [required] |

### Return type

[**models::BillableMetric**](BillableMetric.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_billable_metric

> models::BillableMetric update_billable_metric(code, billable_metric_update_input)
Update a billable metric

This endpoint updates an existing billable metric representing a pricing component of your application.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | Code of the existing billable metric. | [required] |
**billable_metric_update_input** | [**BillableMetricUpdateInput**](BillableMetricUpdateInput.md) | Billable metric payload | [required] |

### Return type

[**models::BillableMetric**](BillableMetric.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

