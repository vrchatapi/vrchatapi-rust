# \AuthenticationApi

All URIs are relative to *https://api.vrchat.cloud/api/1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_pending2_fa**](AuthenticationApi.md#cancel_pending2_fa) | **DELETE** /auth/twofactorauth/totp/pending | Cancel pending enabling of time-based 2FA codes
[**check_user_exists**](AuthenticationApi.md#check_user_exists) | **GET** /auth/exists | Check User Exists
[**confirm_email**](AuthenticationApi.md#confirm_email) | **GET** /auth/confirmEmail | Confirm Email
[**create_global_avatar_moderation**](AuthenticationApi.md#create_global_avatar_moderation) | **POST** /auth/user/avatarmoderations | Create Global Avatar Moderation
[**delete_global_avatar_moderation**](AuthenticationApi.md#delete_global_avatar_moderation) | **DELETE** /auth/user/avatarmoderations | Delete Global Avatar Moderation
[**delete_moderation_report**](AuthenticationApi.md#delete_moderation_report) | **DELETE** /moderationReports/{moderationReportId} | Delete Moderation Report
[**delete_user**](AuthenticationApi.md#delete_user) | **PUT** /users/{userId}/delete | Delete User
[**disable2_fa**](AuthenticationApi.md#disable2_fa) | **DELETE** /auth/twofactorauth | Disable 2FA
[**enable2_fa**](AuthenticationApi.md#enable2_fa) | **POST** /auth/twofactorauth/totp/pending | Enable time-based 2FA codes
[**get_current_user**](AuthenticationApi.md#get_current_user) | **GET** /auth/user | Login and/or Get Current User Info
[**get_global_avatar_moderations**](AuthenticationApi.md#get_global_avatar_moderations) | **GET** /auth/user/avatarmoderations | Get Global Avatar Moderations
[**get_moderation_reports**](AuthenticationApi.md#get_moderation_reports) | **GET** /moderationReports | Get Moderation Reports
[**get_recovery_codes**](AuthenticationApi.md#get_recovery_codes) | **GET** /auth/user/twofactorauth/otp | Get 2FA Recovery codes
[**logout**](AuthenticationApi.md#logout) | **PUT** /logout | Logout
[**register_user_account**](AuthenticationApi.md#register_user_account) | **POST** /auth/register | Register User Account
[**resend_email_confirmation**](AuthenticationApi.md#resend_email_confirmation) | **POST** /auth/user/resendEmail | Resend Email Confirmation
[**submit_moderation_report**](AuthenticationApi.md#submit_moderation_report) | **POST** /moderationReports | Submit Moderation Report
[**verify2_fa**](AuthenticationApi.md#verify2_fa) | **POST** /auth/twofactorauth/totp/verify | Verify 2FA code
[**verify2_fa_email_code**](AuthenticationApi.md#verify2_fa_email_code) | **POST** /auth/twofactorauth/emailotp/verify | Verify 2FA email code
[**verify_auth_token**](AuthenticationApi.md#verify_auth_token) | **GET** /auth | Verify Auth Token
[**verify_login_place**](AuthenticationApi.md#verify_login_place) | **GET** /auth/verifyLoginPlace | Verify Login Place
[**verify_pending2_fa**](AuthenticationApi.md#verify_pending2_fa) | **POST** /auth/twofactorauth/totp/pending/verify | Verify Pending 2FA code
[**verify_recovery_code**](AuthenticationApi.md#verify_recovery_code) | **POST** /auth/twofactorauth/otp/verify | Verify 2FA code with Recovery code



## cancel_pending2_fa

> models::Disable2FaResult cancel_pending2_fa()
Cancel pending enabling of time-based 2FA codes

Cancels the sequence for enabling time-based 2FA.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Disable2FaResult**](Disable2FAResult.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_user_exists

> models::UserExists check_user_exists(email, display_name, username, exclude_user_id)
Check User Exists

Checks if a user by a given `username`, `displayName` or `email` exist. This is used during registration to check if a username has already been taken, during change of displayName to check if a displayName is available, and during change of email to check if the email is already used. In the later two cases the `excludeUserId` is used to exclude oneself, otherwise the result would always be true.  It is **REQUIRED** to include **AT LEAST** `username`, `displayName` **or** `email` query parameter. Although they can be combined - in addition with `excludeUserId` (generally to exclude yourself) - to further fine-tune the search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | Option<**String**> | Filter by email. |  |
**display_name** | Option<**String**> | Filter by displayName. |  |
**username** | Option<**String**> | Filter by Username. |  |
**exclude_user_id** | Option<**String**> | Exclude by UserID. |  |

### Return type

[**models::UserExists**](UserExists.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## confirm_email

> confirm_email(id, verify_email)
Confirm Email

Confirms the email address for a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Target user for which to verify email. | [required] |
**verify_email** | **String** | Token to verify email. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_global_avatar_moderation

> models::AvatarModerationCreated create_global_avatar_moderation(create_avatar_moderation_request)
Create Global Avatar Moderation

Globally moderates an avatar.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_avatar_moderation_request** | [**CreateAvatarModerationRequest**](CreateAvatarModerationRequest.md) |  | [required] |

### Return type

[**models::AvatarModerationCreated**](AvatarModerationCreated.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_global_avatar_moderation

> models::OkStatus2 delete_global_avatar_moderation(target_avatar_id, avatar_moderation_type)
Delete Global Avatar Moderation

Globally unmoderates an avatar.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target_avatar_id** | **String** | Must be a valid avatar ID. | [required] |
**avatar_moderation_type** | [**AvatarModerationType**](.md) | The avatar moderation type associated with the avatar. | [required] |

### Return type

[**models::OkStatus2**](OkStatus2.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_moderation_report

> models::SuccessFlag delete_moderation_report(moderation_report_id)
Delete Moderation Report

Delete a moderation report

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**moderation_report_id** | **String** | The moderation report id. | [required] |

### Return type

[**models::SuccessFlag**](SuccessFlag.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user

> models::CurrentUser delete_user(user_id)
Delete User

Deletes the account with given ID. Normal users only have permission to delete their own account. Account deletion is 14 days from this request, and will be cancelled if you do an authenticated request with the account afterwards.  **VRC+ NOTE:** Despite the 14-days cooldown, any VRC+ subscription will be cancelled **immediately**.  **METHOD NOTE:** Despite this being a Delete action, the method type required is PUT.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |

### Return type

[**models::CurrentUser**](CurrentUser.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable2_fa

> models::Disable2FaResult disable2_fa()
Disable 2FA

Disables 2FA for the currently logged in account

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Disable2FaResult**](Disable2FAResult.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable2_fa

> models::Pending2FaResult enable2_fa()
Enable time-based 2FA codes

Begins the sequence for enabling time-based 2FA.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Pending2FaResult**](Pending2FAResult.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_user

> models::CurrentUser get_current_user()
Login and/or Get Current User Info

This endpoint does the following two operations:   1) Checks if you are already logged in by looking for a valid `auth` cookie. If you are have a valid auth cookie then no additional auth-related actions are taken. If you are **not** logged in then it will log you in with the `Authorization` header and set the `auth` cookie. The `auth` cookie will only be sent once.   2) If logged in, this function will also return the CurrentUser object containing detailed information about the currently logged in user.  The auth string after `Authorization: Basic {string}` is a base64-encoded string of the username and password, both individually url-encoded, and then joined with a colon.  > base64(urlencode(username):urlencode(password))  **WARNING: Session Limit:** Each authentication with login credentials counts as a separate session, out of which you have a limited amount. Make sure to save and reuse the `auth` cookie if you are often restarting the program. The provided API libraries automatically save cookies during runtime, but does not persist during restart. While it can be fine to use username/password during development, expect in production to very fast run into the rate-limit and be temporarily blocked from making new sessions until older ones expire. The exact number of simultaneous sessions is unknown/undisclosed.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::CurrentUser**](CurrentUser.md)

### Authorization

[authHeader](../README.md#authHeader), [twoFactorAuthCookie](../README.md#twoFactorAuthCookie), [authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_global_avatar_moderations

> Vec<models::AvatarModeration> get_global_avatar_moderations()
Get Global Avatar Moderations

Returns list of globally moderated avatars.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::AvatarModeration>**](AvatarModeration.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_moderation_reports

> models::PaginatedModerationReportList get_moderation_reports(offset, n, reporting_user_id, status, r#type)
Get Moderation Reports

Get submitted moderation reports

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**reporting_user_id** | Option<**String**> | Filter for moderation reports. |  |
**status** | Option<**String**> | Filter for moderation reports. One of: `closed`... |  |
**r#type** | Option<**String**> | Filter for moderation reports. One of: `avatar`, `group`, `user`, `world`... |  |

### Return type

[**models::PaginatedModerationReportList**](PaginatedModerationReportList.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recovery_codes

> models::TwoFactorRecoveryCodes get_recovery_codes()
Get 2FA Recovery codes

Gets the OTP (One Time Password) recovery codes for accounts with 2FA-protection enabled.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::TwoFactorRecoveryCodes**](TwoFactorRecoveryCodes.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logout

> models::Success logout()
Logout

Invalidates the login session.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Success**](Success.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_user_account

> models::CurrentUser register_user_account(register_user_account_request)
Register User Account

~~Register a new user account.~~  **DEPRECATED:** Automated creation of accounts has no legitimate public third-party use case, and would be in violation of ToS §13.2: *By using the Platform, you agree not to: i. [...] use the Platform in a manner inconsistent with individual human usage* This endpoint is documented in the interest of completeness

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**register_user_account_request** | [**RegisterUserAccountRequest**](RegisterUserAccountRequest.md) |  | [required] |

### Return type

[**models::CurrentUser**](CurrentUser.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resend_email_confirmation

> models::Success resend_email_confirmation()
Resend Email Confirmation

Requests a resend of pending email address confirmation email

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Success**](Success.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_moderation_report

> models::ModerationReport submit_moderation_report(submit_moderation_report_request)
Submit Moderation Report

Submit a moderation report

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**submit_moderation_report_request** | [**SubmitModerationReportRequest**](SubmitModerationReportRequest.md) |  | [required] |

### Return type

[**models::ModerationReport**](ModerationReport.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify2_fa

> models::Verify2FaResult verify2_fa(two_factor_auth_code)
Verify 2FA code

Finishes the login sequence with a normal 2FA-generated code for accounts with 2FA-protection enabled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**two_factor_auth_code** | [**TwoFactorAuthCode**](TwoFactorAuthCode.md) |  | [required] |

### Return type

[**models::Verify2FaResult**](Verify2FAResult.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify2_fa_email_code

> models::Verify2FaEmailCodeResult verify2_fa_email_code(two_factor_email_code)
Verify 2FA email code

Finishes the login sequence with an 2FA email code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**two_factor_email_code** | [**TwoFactorEmailCode**](TwoFactorEmailCode.md) |  | [required] |

### Return type

[**models::Verify2FaEmailCodeResult**](Verify2FAEmailCodeResult.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_auth_token

> models::VerifyAuthTokenResult verify_auth_token()
Verify Auth Token

Verify whether the currently provided Auth Token is valid.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::VerifyAuthTokenResult**](VerifyAuthTokenResult.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_login_place

> verify_login_place(token, user_id)
Verify Login Place

Verifies a login attempt for a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Token to verify login attempt. | [required] |
**user_id** | Option<**String**> | Filter by UserID. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_pending2_fa

> models::Verify2FaResult verify_pending2_fa(two_factor_auth_code)
Verify Pending 2FA code

Finishes sequence for enabling time-based 2FA.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**two_factor_auth_code** | [**TwoFactorAuthCode**](TwoFactorAuthCode.md) |  | [required] |

### Return type

[**models::Verify2FaResult**](Verify2FAResult.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_recovery_code

> models::Verify2FaResult verify_recovery_code(two_factor_auth_code)
Verify 2FA code with Recovery code

Finishes the login sequence with an OTP (One Time Password) recovery code for accounts with 2FA-protection enabled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**two_factor_auth_code** | [**TwoFactorAuthCode**](TwoFactorAuthCode.md) |  | [required] |

### Return type

[**models::Verify2FaResult**](Verify2FAResult.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

