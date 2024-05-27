# \EconomyApi

All URIs are relative to *https://vrchat.com/api/1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_current_subscriptions**](EconomyApi.md#get_current_subscriptions) | **GET** /auth/user/subscription | Get Current Subscriptions
[**get_license_group**](EconomyApi.md#get_license_group) | **GET** /licenseGroups/{licenseGroupId} | Get License Group
[**get_steam_transaction**](EconomyApi.md#get_steam_transaction) | **GET** /Steam/transactions/{transactionId} | Get Steam Transaction
[**get_steam_transactions**](EconomyApi.md#get_steam_transactions) | **GET** /Steam/transactions | List Steam Transactions
[**get_subscriptions**](EconomyApi.md#get_subscriptions) | **GET** /subscriptions | List Subscriptions



## get_current_subscriptions

> Vec<crate::models::UserSubscription> get_current_subscriptions()
Get Current Subscriptions

Get a list of all current user subscriptions.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::UserSubscription>**](UserSubscription.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_license_group

> crate::models::LicenseGroup get_license_group(license_group_id)
Get License Group

Get a single License Group by given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**license_group_id** | **String** | Must be a valid license group ID. | [required] |

### Return type

[**crate::models::LicenseGroup**](LicenseGroup.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_steam_transaction

> crate::models::Transaction get_steam_transaction(transaction_id)
Get Steam Transaction

Get a single Steam transactions by ID. This returns the exact same information as `getSteamTransactions`, so no point in using this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_id** | **String** | Must be a valid transaction ID. | [required] |

### Return type

[**crate::models::Transaction**](Transaction.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_steam_transactions

> Vec<crate::models::Transaction> get_steam_transactions()
List Steam Transactions

Get all own Steam transactions.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Transaction>**](Transaction.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subscriptions

> Vec<crate::models::Subscription> get_subscriptions()
List Subscriptions

List all existing Subscriptions. For example, \"vrchatplus-monthly\" and \"vrchatplus-yearly\".

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Subscription>**](Subscription.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

