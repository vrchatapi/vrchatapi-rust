# \EconomyApi

All URIs are relative to *https://api.vrchat.cloud/api/1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_balance**](EconomyApi.md#get_balance) | **GET** /user/{userId}/balance | Get Balance
[**get_current_subscriptions**](EconomyApi.md#get_current_subscriptions) | **GET** /auth/user/subscription | Get Current Subscriptions
[**get_license_group**](EconomyApi.md#get_license_group) | **GET** /licenseGroups/{licenseGroupId} | Get License Group
[**get_product_listing**](EconomyApi.md#get_product_listing) | **GET** /listing/{productId} | Get Product Listing
[**get_product_listings**](EconomyApi.md#get_product_listings) | **GET** /user/{userId}/listings | Get User Product Listings
[**get_steam_transaction**](EconomyApi.md#get_steam_transaction) | **GET** /Steam/transactions/{transactionId} | Get Steam Transaction
[**get_steam_transactions**](EconomyApi.md#get_steam_transactions) | **GET** /Steam/transactions | List Steam Transactions
[**get_subscriptions**](EconomyApi.md#get_subscriptions) | **GET** /subscriptions | List Subscriptions
[**get_tilia_status**](EconomyApi.md#get_tilia_status) | **GET** /tilia/status | Get Tilia Status
[**get_tilia_tos**](EconomyApi.md#get_tilia_tos) | **GET** /user/{userId}/tilia/tos | Get Tilia TOS Agreement Status
[**get_token_bundles**](EconomyApi.md#get_token_bundles) | **GET** /tokenBundles | List Token Bundles



## get_balance

> models::Balance get_balance(user_id)
Get Balance

Gets the balance of a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |

### Return type

[**models::Balance**](Balance.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_subscriptions

> Vec<models::UserSubscription> get_current_subscriptions()
Get Current Subscriptions

Get a list of all current user subscriptions.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::UserSubscription>**](UserSubscription.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_license_group

> models::LicenseGroup get_license_group(license_group_id)
Get License Group

Get a single License Group by given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**license_group_id** | **String** | Must be a valid license group ID. | [required] |

### Return type

[**models::LicenseGroup**](LicenseGroup.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_product_listing

> models::ProductListing get_product_listing(product_id, hydrate)
Get Product Listing

Gets a product listing

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** | Must be a valid product ID. | [required] |
**hydrate** | Option<**bool**> | Populates some fields and changes types of others for certain objects. |  |

### Return type

[**models::ProductListing**](ProductListing.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_product_listings

> Vec<models::ProductListing> get_product_listings(user_id, n, offset, hydrate, group_id, active)
Get User Product Listings

Gets the product listings of a given user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |
**hydrate** | Option<**bool**> | Populates some fields and changes types of others for certain objects. |  |
**group_id** | Option<**String**> | Must be a valid group ID. |  |
**active** | Option<**bool**> | Filter for users' listings and inventory bundles. |  |

### Return type

[**Vec<models::ProductListing>**](ProductListing.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_steam_transaction

> models::Transaction get_steam_transaction(transaction_id)
Get Steam Transaction

Get a single Steam transactions by ID. This returns the exact same information as `getSteamTransactions`, so no point in using this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_id** | **String** | Must be a valid transaction ID. | [required] |

### Return type

[**models::Transaction**](Transaction.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_steam_transactions

> Vec<models::Transaction> get_steam_transactions()
List Steam Transactions

Get all own Steam transactions.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Transaction>**](Transaction.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subscriptions

> Vec<models::Subscription> get_subscriptions()
List Subscriptions

List all existing Subscriptions. For example, \"vrchatplus-monthly\" and \"vrchatplus-yearly\".

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Subscription>**](Subscription.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tilia_status

> models::TiliaStatus get_tilia_status()
Get Tilia Status

Gets the status of Tilia integration

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::TiliaStatus**](TiliaStatus.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tilia_tos

> models::TiliaTos get_tilia_tos(user_id)
Get Tilia TOS Agreement Status

Gets the status of the agreement of a user to the Tilia TOS

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |

### Return type

[**models::TiliaTos**](TiliaTOS.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_token_bundles

> Vec<models::TokenBundle> get_token_bundles()
List Token Bundles

Gets the list of token bundles

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::TokenBundle>**](TokenBundle.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

