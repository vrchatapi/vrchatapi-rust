# \DeprecatedApi

All URIs are relative to *https://api.vrchat.cloud/api/1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_balance_earnings**](DeprecatedApi.md#get_balance_earnings) | **GET** /user/{userId}/balance/earnings | Get Balance Earnings
[**get_prop_publish_status**](DeprecatedApi.md#get_prop_publish_status) | **GET** /props/{propId}/publish | Get Prop Publish Status
[**get_seller_eligibility**](DeprecatedApi.md#get_seller_eligibility) | **GET** /economy/seller/eligibility | Get Seller Eligibility
[**get_tilia_status**](DeprecatedApi.md#get_tilia_status) | **GET** /tilia/status | Get Tilia Status
[**get_tilia_tos**](DeprecatedApi.md#get_tilia_tos) | **GET** /user/{userId}/tilia/tos | Get Tilia TOS Agreement Status
[**get_user_credits_eligible**](DeprecatedApi.md#get_user_credits_eligible) | **GET** /users/{userId}/credits/eligible | Get User Credits Eligibility
[**publish_prop**](DeprecatedApi.md#publish_prop) | **PUT** /props/{propId}/publish | Publish Prop
[**unpublish_prop**](DeprecatedApi.md#unpublish_prop) | **DELETE** /props/{propId}/publish | Unpublish Prop
[**update_tilia_tos**](DeprecatedApi.md#update_tilia_tos) | **PUT** /user/{userId}/tilia/tos | Update Tilia TOS Agreement Status



## get_balance_earnings

> models::Balance get_balance_earnings(user_id)
Get Balance Earnings

Return the user's balance from earnings.

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


## get_prop_publish_status

> models::PropPublishStatus get_prop_publish_status(prop_id)
Get Prop Publish Status

Return the PropPublishStatus object. `/props/{propId}` is still served.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prop_id** | **String** | Prop ID. | [required] |

### Return type

[**models::PropPublishStatus**](PropPublishStatus.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_seller_eligibility

> models::SellerEligibility get_seller_eligibility()
Get Seller Eligibility

Return the current user's eligibility to become a seller.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SellerEligibility**](SellerEligibility.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tilia_status

> models::TiliaStatus get_tilia_status()
Get Tilia Status

Return the Tilia integration status.

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

Return the user's Tilia TOS agreement status.

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


## get_user_credits_eligible

> models::UserCreditsEligible get_user_credits_eligible(user_id, subscription_id)
Get User Credits Eligibility

Return the user's subscription credit eligibility.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |
**subscription_id** | **String** |  | [required] |

### Return type

[**models::UserCreditsEligible**](UserCreditsEligible.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## publish_prop

> models::PropPublishStatus publish_prop(prop_id)
Publish Prop

Publish a prop and return the updated PropPublishStatus object. `/props/{propId}` is still served.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prop_id** | **String** | Prop ID. | [required] |

### Return type

[**models::PropPublishStatus**](PropPublishStatus.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unpublish_prop

> models::PropPublishStatus unpublish_prop(prop_id)
Unpublish Prop

Unpublish a prop and return the updated PropPublishStatus object. `/props/{propId}` is still served.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prop_id** | **String** | Prop ID. | [required] |

### Return type

[**models::PropPublishStatus**](PropPublishStatus.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_tilia_tos

> serde_json::Value update_tilia_tos(user_id, update_tilia_tos_request)
Update Tilia TOS Agreement Status

Update the user's Tilia TOS agreement status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |
**update_tilia_tos_request** | Option<[**UpdateTiliaTosRequest**](UpdateTiliaTosRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

