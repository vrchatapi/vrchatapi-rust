# \AuthenticationApi

All URIs are relative to *https://vrchat.com/api/1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_user_exists**](AuthenticationApi.md#check_user_exists) | **GET** /auth/exists | Check User Exists
[**delete_user**](AuthenticationApi.md#delete_user) | **PUT** /users/{userId}/delete | Delete User
[**get_current_user**](AuthenticationApi.md#get_current_user) | **GET** /auth/user | Login and/or Get Current User Info
[**logout**](AuthenticationApi.md#logout) | **PUT** /logout | Logout
[**verify2_fa**](AuthenticationApi.md#verify2_fa) | **POST** /auth/twofactorauth/totp/verify | Verify 2FA code
[**verify2_fa_email_code**](AuthenticationApi.md#verify2_fa_email_code) | **POST** /auth/twofactorauth/emailotp/verify | Verify 2FA email code
[**verify_auth_token**](AuthenticationApi.md#verify_auth_token) | **GET** /auth | Verify Auth Token
[**verify_recovery_code**](AuthenticationApi.md#verify_recovery_code) | **POST** /auth/twofactorauth/otp/verify | Verify 2FA code with Recovery code



## check_user_exists

> crate::models::UserExists check_user_exists(email, display_name, user_id, exclude_user_id)
Check User Exists

Checks if a user by a given `username`, `displayName` or `email` exist. This is used during registration to check if a username has already been taken, during change of displayName to check if a displayName is available, and during change of email to check if the email is already used. In the later two cases the `excludeUserId` is used to exclude oneself, otherwise the result would always be true.  It is **REQUIRED** to include **AT LEAST** `username`, `displayName` **or** `email` query parameter. Although they can be combined - in addition with `excludeUserId` (generally to exclude yourself) - to further fine-tune the search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | Option<**String**> | Filter by email. |  |
**display_name** | Option<**String**> | Filter by displayName. |  |
**user_id** | Option<**String**> | Filter by UserID. |  |
**exclude_user_id** | Option<**String**> | Exclude by UserID. |  |

### Return type

[**crate::models::UserExists**](UserExists.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user

> crate::models::CurrentUser delete_user(user_id)
Delete User

Deletes the account with given ID. Normal users only have permission to delete their own account. Account deletion is 14 days from this request, and will be cancelled if you do an authenticated request with the account afterwards.  **VRC+ NOTE:** Despite the 14-days cooldown, any VRC+ subscription will be cancelled **immediately**.  **METHOD NOTE:** Despite this being a Delete action, the method type required is PUT.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |

### Return type

[**crate::models::CurrentUser**](CurrentUser.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_user

> crate::models::CurrentUser get_current_user()
Login and/or Get Current User Info

This endpoint does the following two operations:   1) Checks if you are already logged in by looking for a valid `auth` cookie. If you are have a valid auth cookie then no additional auth-related actions are taken. If you are **not** logged in then it will log you in with the `Authorization` header and set the `auth` cookie. The `auth` cookie will only be sent once.   2) If logged in, this function will also return the CurrentUser object containing detailed information about the currently logged in user.  The auth string after `Authorization: Basic {string}` is a base64-encoded string of the username and password, both individually url-encoded, and then joined with a colon.    > base64(urlencode(username):urlencode(password))  **WARNING: Session Limit:** Each authentication with login credentials counts as a separate session, out of which you have a limited amount. Make sure to save and reuse the `auth` cookie if you are often restarting the program. The provided API libraries automatically save cookies during runtime, but does not persist during restart. While it can be fine to use username/password during development, expect in production to very fast run into the rate-limit and be temporarily blocked from making new sessions until older ones expire. The exact number of simultaneous sessions is unknown/undisclosed.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CurrentUser**](CurrentUser.md)

### Authorization

[authCookie](../README.md#authCookie), [authHeader](../README.md#authHeader), [twoFactorAuthCookie](../README.md#twoFactorAuthCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logout

> crate::models::Success logout()
Logout

Invalidates the login session.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Success**](Success.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify2_fa

> crate::models::Verify2FaResult verify2_fa(two_factor_auth_code)
Verify 2FA code

Finishes the login sequence with a normal 2FA-generated code for accounts with 2FA-protection enabled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**two_factor_auth_code** | [**TwoFactorAuthCode**](TwoFactorAuthCode.md) |  | [required] |

### Return type

[**crate::models::Verify2FaResult**](Verify2FAResult.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify2_fa_email_code

> crate::models::Verify2FaEmailCodeResult verify2_fa_email_code(two_factor_email_code)
Verify 2FA email code

Finishes the login sequence with an 2FA email code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**two_factor_email_code** | [**TwoFactorEmailCode**](TwoFactorEmailCode.md) |  | [required] |

### Return type

[**crate::models::Verify2FaEmailCodeResult**](Verify2FAEmailCodeResult.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_auth_token

> crate::models::VerifyAuthTokenResult verify_auth_token()
Verify Auth Token

Verify whether the currently provided Auth Token is valid.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::VerifyAuthTokenResult**](VerifyAuthTokenResult.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_recovery_code

> crate::models::Verify2FaResult verify_recovery_code(two_factor_auth_code)
Verify 2FA code with Recovery code

Finishes the login sequence with an OTP (One Time Password) recovery code for accounts with 2FA-protection enabled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**two_factor_auth_code** | [**TwoFactorAuthCode**](TwoFactorAuthCode.md) |  | [required] |

### Return type

[**crate::models::Verify2FaResult**](Verify2FAResult.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

