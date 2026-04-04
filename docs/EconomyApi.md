# \EconomyApi

All URIs are relative to *https://api.vrchat.cloud/api/1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_product**](EconomyApi.md#create_product) | **POST** /products | Create Product
[**create_product_listing_direct**](EconomyApi.md#create_product_listing_direct) | **POST** /listing | Create Product Listing
[**delete_product**](EconomyApi.md#delete_product) | **DELETE** /products/{productId} | Delete Product
[**delete_product_listing_direct**](EconomyApi.md#delete_product_listing_direct) | **DELETE** /listing/{productId} | Delete Product Listing
[**get_active_licenses**](EconomyApi.md#get_active_licenses) | **GET** /economy/licenses/active | Get Active Licenses
[**get_balance**](EconomyApi.md#get_balance) | **GET** /user/{userId}/balance | Get Balance
[**get_balance_earnings**](EconomyApi.md#get_balance_earnings) | **GET** /user/{userId}/balance/earnings | Get Balance Earnings
[**get_bulk_gift_purchases**](EconomyApi.md#get_bulk_gift_purchases) | **GET** /user/bulk/gift/purchases | Get Bulk Gift Purchases
[**get_current_subscriptions**](EconomyApi.md#get_current_subscriptions) | **GET** /auth/user/subscription | Get Current Subscriptions
[**get_earnings_metrics**](EconomyApi.md#get_earnings_metrics) | **GET** /economy/metrics/earnings | Get Earnings Metrics
[**get_economy_account**](EconomyApi.md#get_economy_account) | **GET** /user/{userId}/economy/account | Get Economy Account
[**get_economy_balances**](EconomyApi.md#get_economy_balances) | **GET** /user/{userId}/economy/balances | Get Economy Balances
[**get_economy_payout_status**](EconomyApi.md#get_economy_payout_status) | **GET** /user/{userId}/economy/payouts/status | Get Economy Payout Status
[**get_economy_payouts**](EconomyApi.md#get_economy_payouts) | **GET** /user/{userId}/economy/payouts/list | Get Economy Payouts
[**get_license_group**](EconomyApi.md#get_license_group) | **GET** /licenseGroups/{licenseGroupId} | Get License Group
[**get_product_listing**](EconomyApi.md#get_product_listing) | **GET** /listing/{productId} | Get Product Listing
[**get_product_listing_alternate**](EconomyApi.md#get_product_listing_alternate) | **GET** /products/{productId} | Get Product Listing (alternate)
[**get_product_listings**](EconomyApi.md#get_product_listings) | **GET** /user/{userId}/listings | Get User Product Listings
[**get_product_purchase**](EconomyApi.md#get_product_purchase) | **GET** /economy/purchases/{productPurchaseId} | Get Product Purchase
[**get_product_purchase_history**](EconomyApi.md#get_product_purchase_history) | **GET** /user/{userId}/economy/transactions | Get Product Purchase History
[**get_product_purchase_stacks**](EconomyApi.md#get_product_purchase_stacks) | **GET** /economy/purchases/{productPurchaseId}/stacks | Get Product Purchase Stacks
[**get_product_purchases**](EconomyApi.md#get_product_purchases) | **GET** /economy/purchases | Get Product Purchases
[**get_recent_subscription**](EconomyApi.md#get_recent_subscription) | **GET** /user/subscription/recent | Get Recent Subscription
[**get_seller_eligibility**](EconomyApi.md#get_seller_eligibility) | **GET** /economy/seller/eligibility | Get Seller Eligibility
[**get_steam_transaction**](EconomyApi.md#get_steam_transaction) | **GET** /Steam/transactions/{transactionId} | Get Steam Transaction
[**get_steam_transactions**](EconomyApi.md#get_steam_transactions) | **GET** /Steam/transactions | List Steam Transactions
[**get_store**](EconomyApi.md#get_store) | **GET** /economy/store | Get Store
[**get_store_shelves**](EconomyApi.md#get_store_shelves) | **GET** /economy/store/shelves | Get Store Shelves
[**get_subscriptions**](EconomyApi.md#get_subscriptions) | **GET** /subscriptions | List Subscriptions
[**get_tilia_status**](EconomyApi.md#get_tilia_status) | **GET** /tilia/status | Get Tilia Status
[**get_tilia_tos**](EconomyApi.md#get_tilia_tos) | **GET** /user/{userId}/tilia/tos | Get Tilia TOS Agreement Status
[**get_token_bundles**](EconomyApi.md#get_token_bundles) | **GET** /tokenBundles | List Token Bundles
[**get_user_credits_eligible**](EconomyApi.md#get_user_credits_eligible) | **GET** /users/{userId}/credits/eligible | Get User Credits Eligiblity
[**get_user_subscription_eligible**](EconomyApi.md#get_user_subscription_eligible) | **GET** /users/{userId}/subscription/eligible | Get User Subscription Eligiblity
[**get_user_tilia_kyc**](EconomyApi.md#get_user_tilia_kyc) | **GET** /user/{userId}/tilia/kyc | Get User Tilia KYC
[**list_stores**](EconomyApi.md#list_stores) | **GET** /economy/stores | List Stores
[**list_user_products**](EconomyApi.md#list_user_products) | **GET** /user/{userId}/products | List User Products
[**purchase_product_listing**](EconomyApi.md#purchase_product_listing) | **POST** /economy/purchase/listing | Purchase Product Listing
[**update_product**](EconomyApi.md#update_product) | **PUT** /products/{productId} | Update Product
[**update_product_listing_direct**](EconomyApi.md#update_product_listing_direct) | **PUT** /listing/{productId} | Update Product Listing
[**update_tilia_tos**](EconomyApi.md#update_tilia_tos) | **PUT** /user/{userId}/tilia/tos | Update Tilia TOS Agreement Status



## create_product

> models::Product create_product(create_product_request)
Create Product

Creates a product and returns the new Product object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_product_request** | [**CreateProductRequest**](CreateProductRequest.md) |  | [required] |

### Return type

[**models::Product**](Product.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_product_listing_direct

> models::ProductListing create_product_listing_direct(create_listing_request)
Create Product Listing

Creates a listing and returns the new ProductListing object. The request body is based on observed fields and may be incomplete.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_listing_request** | [**CreateListingRequest**](CreateListingRequest.md) |  | [required] |

### Return type

[**models::ProductListing**](ProductListing.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_product

> models::SuccessFlag delete_product(product_id)
Delete Product

Deletes a product.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** | Must be a valid product ID. | [required] |

### Return type

[**models::SuccessFlag**](SuccessFlag.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_product_listing_direct

> models::SuccessFlag delete_product_listing_direct(product_id, hydrate)
Delete Product Listing

Deletes a listing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** | Must be a valid product ID. | [required] |
**hydrate** | Option<**bool**> | Populates some fields and changes types of others for certain objects. |  |

### Return type

[**models::SuccessFlag**](SuccessFlag.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_active_licenses

> Vec<models::License> get_active_licenses()
Get Active Licenses

Gets active licenses

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::License>**](License.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## get_balance_earnings

> models::Balance get_balance_earnings(user_id)
Get Balance Earnings

Gets the balance of a user from earnings

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


## get_bulk_gift_purchases

> Vec<serde_json::Value> get_bulk_gift_purchases(most_recent)
Get Bulk Gift Purchases

Get bulk gift purchases made by the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**most_recent** | Option<**bool**> |  |  |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

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


## get_earnings_metrics

> models::EarningsMetrics get_earnings_metrics(seller_id, metric_date_start, metric_date_end, group_by_duration)
Get Earnings Metrics

Gets earnings totals and breakdown metrics for the currently authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seller_id** | **String** | Seller to retrieve economy metrics for. | [required] |
**metric_date_start** | Option<**String**> | Lower bound for economy metrics queries. Observed formats include both date-only and full ISO timestamps. |  |
**metric_date_end** | Option<**String**> | Upper bound for economy metrics queries. Observed formats include both date-only and full ISO timestamps. |  |
**group_by_duration** | Option<**String**> | Time bucket size for economy metrics. Observed values include `days` and `years`. |  |

### Return type

[**models::EarningsMetrics**](EarningsMetrics.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_economy_account

> models::EconomyAccount get_economy_account(user_id)
Get Economy Account

Gets the economy account of a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |

### Return type

[**models::EconomyAccount**](EconomyAccount.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_economy_balances

> models::EconomyBalances get_economy_balances(user_id)
Get Economy Balances

Gets the combined balances for a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |

### Return type

[**models::EconomyBalances**](EconomyBalances.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_economy_payout_status

> models::EconomyPayoutStatus get_economy_payout_status(user_id)
Get Economy Payout Status

Gets the current payout status and eligibility information for a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |

### Return type

[**models::EconomyPayoutStatus**](EconomyPayoutStatus.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_economy_payouts

> models::EconomyPayoutList get_economy_payouts(user_id)
Get Economy Payouts

Gets the payout history for a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |

### Return type

[**models::EconomyPayoutList**](EconomyPayoutList.md)

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


## get_product_listing_alternate

> models::ProductListing get_product_listing_alternate(product_id)
Get Product Listing (alternate)

Gets a product listing

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** | Must be a valid product ID. | [required] |

### Return type

[**models::ProductListing**](ProductListing.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_product_listings

> Vec<models::ProductListing> get_product_listings(user_id, n, offset, hydrate, listing_type, group_id, active)
Get User Product Listings

Gets the product listings of a given user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |
**hydrate** | Option<**bool**> | Populates some fields and changes types of others for certain objects. |  |
**listing_type** | Option<**String**> | Filter user listings by category. Observed values include `otp` and `subscription`. |  |
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


## get_product_purchase

> models::ProductPurchase get_product_purchase(product_purchase_id)
Get Product Purchase

Gets a single product purchase

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_purchase_id** | **String** | Must be a valid purchase ID. | [required] |

### Return type

[**models::ProductPurchase**](ProductPurchase.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_product_purchase_history

> models::ProductPurchaseHistory get_product_purchase_history(user_id, n, date_min, date_max, from_user_id, to_user_id, sort, order)
Get Product Purchase History

Gets a history of product purchases

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**date_min** | Option<**String**> | The start date of the search range. |  |
**date_max** | Option<**String**> | The end date of the search range. |  |
**from_user_id** | Option<**String**> | Must be a valid user ID. |  |
**to_user_id** | Option<**String**> | Must be a valid user ID. |  |
**sort** | Option<[**SortOptionProductPurchase**](SortOptionProductPurchase.md)> | The sort order of the results. |  |
**order** | Option<[**OrderOptionShort**](OrderOptionShort.md)> | Result ordering |  |

### Return type

[**models::ProductPurchaseHistory**](ProductPurchaseHistory.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_product_purchase_stacks

> Vec<serde_json::Value> get_product_purchase_stacks(product_purchase_id)
Get Product Purchase Stacks

Gets stacks for a product purchase

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_purchase_id** | **String** | Must be a valid purchase ID. | [required] |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_product_purchases

> Vec<models::ProductPurchase> get_product_purchases(buyer_id, seller_id, n, offset, most_recent, sort, order)
Get Product Purchases

Gets product purchases

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**buyer_id** | **String** | Must be a valid user ID. | [required] |
**seller_id** | Option<**String**> | Filter results by seller. |  |
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |
**most_recent** | Option<**bool**> |  |  |
**sort** | Option<[**SortOptionProductPurchase**](SortOptionProductPurchase.md)> | The sort order of the results. |  |
**order** | Option<[**OrderOptionShort**](OrderOptionShort.md)> | Result ordering |  |

### Return type

[**Vec<models::ProductPurchase>**](ProductPurchase.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recent_subscription

> models::UserSubscription get_recent_subscription()
Get Recent Subscription

Get the most recent user subscription.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UserSubscription**](UserSubscription.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_seller_eligibility

> models::SellerEligibility get_seller_eligibility()
Get Seller Eligibility

Get the eligibility of the currently authenticated user to become a seller

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


## get_store

> models::Store get_store(store_id, hydrate_listings, hydrate_products)
Get Store

Gets a store

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** |  | [required] |
**hydrate_listings** | Option<**bool**> | Listings fields will be populated. |  |
**hydrate_products** | Option<**bool**> | Products fields will be populated. |  |

### Return type

[**models::Store**](Store.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_store_shelves

> Vec<models::StoreShelf> get_store_shelves(store_id, hydrate_listings, fetch)
Get Store Shelves

Gets the shelves for a store

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** |  | [required] |
**hydrate_listings** | Option<**bool**> | Listings fields will be populated. |  |
**fetch** | Option<[**StoreView**](StoreView.md)> |  |  |

### Return type

[**Vec<models::StoreShelf>**](StoreShelf.md)

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


## get_user_credits_eligible

> models::UserCreditsEligible get_user_credits_eligible(user_id, subscription_id)
Get User Credits Eligiblity

Get the user's eligibility status for subscriptions based on available credits.

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


## get_user_subscription_eligible

> models::UserSubscriptionEligible get_user_subscription_eligible(user_id, steam_id)
Get User Subscription Eligiblity

Get the user's eligibility status for subscriptions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |
**steam_id** | Option<**String**> | The Steam ID of the user. |  |

### Return type

[**models::UserSubscriptionEligible**](UserSubscriptionEligible.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_tilia_kyc

> models::TiliaKyc get_user_tilia_kyc(user_id)
Get User Tilia KYC

Gets KYC status details for a user's Tilia account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |

### Return type

[**models::TiliaKyc**](TiliaKyc.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_stores

> Vec<models::Store> list_stores(seller_id, management_pov, n, offset)
List Stores

Lists stores, optionally filtered to a seller and adjusted for management views.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seller_id** | Option<**String**> | Filter results by seller. |  |
**management_pov** | Option<**bool**> | Return stores from the seller management point of view. |  |
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |

### Return type

[**Vec<models::Store>**](Store.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_user_products

> Vec<models::Product> list_user_products(user_id, n, offset)
List User Products

Gets the products of a given user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |

### Return type

[**Vec<models::Product>**](Product.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## purchase_product_listing

> models::ProductPurchase purchase_product_listing(purchase_product_listing_request)
Purchase Product Listing

Purchases a product listing

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**purchase_product_listing_request** | Option<[**PurchaseProductListingRequest**](PurchaseProductListingRequest.md)> |  |  |

### Return type

[**models::ProductPurchase**](ProductPurchase.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_product

> models::Product update_product(product_id, update_product_request)
Update Product

Updates a product and returns the updated Product object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** | Must be a valid product ID. | [required] |
**update_product_request** | [**UpdateProductRequest**](UpdateProductRequest.md) |  | [required] |

### Return type

[**models::Product**](Product.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_product_listing_direct

> models::ProductListing update_product_listing_direct(product_id, update_listing_request, hydrate)
Update Product Listing

Updates the active state of a listing. Setting `active` to `true` publishes the listing, while `false` unpublishes it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** | Must be a valid product ID. | [required] |
**update_listing_request** | [**UpdateListingRequest**](UpdateListingRequest.md) |  | [required] |
**hydrate** | Option<**bool**> | Populates some fields and changes types of others for certain objects. |  |

### Return type

[**models::ProductListing**](ProductListing.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_tilia_tos

> serde_json::Value update_tilia_tos(user_id, update_tilia_tos_request)
Update Tilia TOS Agreement Status

Updates the status of the agreement of a user to the Tilia TOS

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

