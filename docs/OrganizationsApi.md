# \OrganizationsApi

All URIs are relative to *https://api.getlago.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**update_organization**](OrganizationsApi.md#update_organization) | **PUT** /organizations | Update your organization



## update_organization

> models::Organization update_organization(organization_update_input)
Update your organization

This endpoint is used to update your own organization's settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_update_input** | [**OrganizationUpdateInput**](OrganizationUpdateInput.md) | Update an existing organization | [required] |

### Return type

[**models::Organization**](Organization.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

