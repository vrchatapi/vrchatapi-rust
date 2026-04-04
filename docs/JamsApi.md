# \JamsApi

All URIs are relative to *https://api.vrchat.cloud/api/1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_jam_submission**](JamsApi.md#delete_jam_submission) | **DELETE** /jams/{jamId}/submissions/{jamSubmissionId} | Delete Jam Submission
[**get_jam**](JamsApi.md#get_jam) | **GET** /jams/{jamId} | Show jam information
[**get_jam_submissions**](JamsApi.md#get_jam_submissions) | **GET** /jams/{jamId}/submissions | Show jam submissions
[**get_jams**](JamsApi.md#get_jams) | **GET** /jams | Show jams list
[**submit_jam_content**](JamsApi.md#submit_jam_content) | **POST** /jams/{jamId}/submissions | Submit Jam Content



## delete_jam_submission

> models::Success delete_jam_submission(jam_id, jam_submission_id)
Delete Jam Submission

Withdraws a content submission from a jam.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jam_id** | **String** | Must be a valid jam ID. | [required] |
**jam_submission_id** | **String** | Must be a valid jam submission ID. | [required] |

### Return type

[**models::Success**](Success.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_jam

> models::Jam get_jam(jam_id)
Show jam information

Returns a jam.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jam_id** | **String** | Must be a valid jam ID. | [required] |

### Return type

[**models::Jam**](Jam.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_jam_submissions

> Vec<models::JamSubmission> get_jam_submissions(jam_id, content_id, submitter_id)
Show jam submissions

Returns all submissions of a jam. Can filter by contentId (for world or avatar jams) or submitterId (for a participant).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jam_id** | **String** | Must be a valid jam ID. | [required] |
**content_id** | Option<**String**> | Filter for particular content submitted, e.g., a groupId, userId, avatarId, etc. |  |
**submitter_id** | Option<**String**> | Must be a valid user ID. |  |

### Return type

[**Vec<models::JamSubmission>**](JamSubmission.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_jams

> Vec<models::Jam> get_jams(r#type)
Show jams list

Lists World Jams or Avatar Jams, both currently running and ones that have ended.  `isActive` is used to select only active or already ended jams.  `type` is used to select only world or avatar jams, and can only take `world` or `avatar`. ``

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | Option<**String**> | Only show jams of this type (`avatar` or `world`). |  |

### Return type

[**Vec<models::Jam>**](Jam.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_jam_content

> models::JamSubmission submit_jam_content(jam_id, create_jam_submission_request)
Submit Jam Content

Submits content to a jam. The content must have been uploaded by the submitter, and both the content upload and jam submission must be made within the jam's designated times.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jam_id** | **String** | Must be a valid jam ID. | [required] |
**create_jam_submission_request** | Option<[**CreateJamSubmissionRequest**](CreateJamSubmissionRequest.md)> |  |  |

### Return type

[**models::JamSubmission**](JamSubmission.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

