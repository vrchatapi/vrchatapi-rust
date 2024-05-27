# \FilesApi

All URIs are relative to *https://vrchat.com/api/1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_file**](FilesApi.md#create_file) | **POST** /file | Create File
[**create_file_version**](FilesApi.md#create_file_version) | **POST** /file/{fileId} | Create File Version
[**delete_file**](FilesApi.md#delete_file) | **DELETE** /file/{fileId} | Delete File
[**delete_file_version**](FilesApi.md#delete_file_version) | **DELETE** /file/{fileId}/{versionId} | Delete File Version
[**download_file_version**](FilesApi.md#download_file_version) | **GET** /file/{fileId}/{versionId} | Download File Version
[**finish_file_data_upload**](FilesApi.md#finish_file_data_upload) | **PUT** /file/{fileId}/{versionId}/{fileType}/finish | Finish FileData Upload
[**get_file**](FilesApi.md#get_file) | **GET** /file/{fileId} | Show File
[**get_file_data_upload_status**](FilesApi.md#get_file_data_upload_status) | **GET** /file/{fileId}/{versionId}/{fileType}/status | Check FileData Upload Status
[**get_files**](FilesApi.md#get_files) | **GET** /files | List Files
[**start_file_data_upload**](FilesApi.md#start_file_data_upload) | **PUT** /file/{fileId}/{versionId}/{fileType}/start | Start FileData Upload



## create_file

> crate::models::File create_file(create_file_request)
Create File

Creates a new File object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_file_request** | Option<[**CreateFileRequest**](CreateFileRequest.md)> |  |  |

### Return type

[**crate::models::File**](File.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_file_version

> crate::models::File create_file_version(file_id, create_file_version_request)
Create File Version

Creates a new FileVersion. Once a Version has been created, proceed to the `/file/{fileId}/{versionId}/file/start` endpoint to start a file upload.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | Must be a valid file ID. | [required] |
**create_file_version_request** | Option<[**CreateFileVersionRequest**](CreateFileVersionRequest.md)> |  |  |

### Return type

[**crate::models::File**](File.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_file

> crate::models::Success delete_file(file_id)
Delete File

Deletes a File object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | Must be a valid file ID. | [required] |

### Return type

[**crate::models::Success**](Success.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_file_version

> crate::models::File delete_file_version(file_id, version_id)
Delete File Version

Delete a specific version of a file. You can only delete the latest version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | Must be a valid file ID. | [required] |
**version_id** | **i32** | Version ID of the asset. | [required] |

### Return type

[**crate::models::File**](File.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_file_version

> download_file_version(file_id, version_id)
Download File Version

Downloads the file with the provided version number.  **Version Note:** Version 0 is always when the file was created. The real data is usually always located in version 1 and up.  **Extension Note:** Files are not guaranteed to have a file extensions. UnityPackage files tends to have it, images through this endpoint do not. You are responsible for appending file extension from the `extension` field when neccesary.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | Must be a valid file ID. | [required] |
**version_id** | **i32** | Version ID of the asset. | [required] |

### Return type

 (empty response body)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## finish_file_data_upload

> crate::models::File finish_file_data_upload(file_id, version_id, file_type, finish_file_data_upload_request)
Finish FileData Upload

Finish an upload of a FileData. This will mark it as \"complete\". After uploading the `file` for Avatars and Worlds you then have to upload a `signature` file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | Must be a valid file ID. | [required] |
**version_id** | **i32** | Version ID of the asset. | [required] |
**file_type** | **String** | Type of file. | [required] |
**finish_file_data_upload_request** | Option<[**FinishFileDataUploadRequest**](FinishFileDataUploadRequest.md)> | Please see documentation on ETag's: [https://teppen.io/2018/06/23/aws_s3_etags/](https://teppen.io/2018/06/23/aws_s3_etags/)  ETag's should NOT be present when uploading a `signature`. |  |

### Return type

[**crate::models::File**](File.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_file

> crate::models::File get_file(file_id)
Show File

Shows general information about the \"File\" object. Each File can have several \"Version\"'s, and each Version can have multiple real files or \"Data\" blobs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | Must be a valid file ID. | [required] |

### Return type

[**crate::models::File**](File.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_file_data_upload_status

> crate::models::FileVersionUploadStatus get_file_data_upload_status(file_id, version_id, file_type)
Check FileData Upload Status

Retrieves the upload status for file upload. Can currently only be accessed when `status` is `waiting`. Trying to access it on a file version already uploaded currently times out.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | Must be a valid file ID. | [required] |
**version_id** | **i32** | Version ID of the asset. | [required] |
**file_type** | **String** | Type of file. | [required] |

### Return type

[**crate::models::FileVersionUploadStatus**](FileVersionUploadStatus.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_files

> Vec<crate::models::File> get_files(tag, user_id, n, offset)
List Files

Returns a list of files

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag** | Option<**String**> | Tag, for example \"icon\" or \"gallery\", not included by default. |  |
**user_id** | Option<**String**> | UserID, will always generate a 500 permission error. |  |
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |

### Return type

[**Vec<crate::models::File>**](File.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_file_data_upload

> crate::models::FileUploadUrl start_file_data_upload(file_id, version_id, file_type, part_number)
Start FileData Upload

Starts an upload of a specific FilePart. This endpoint will return an AWS URL which you can PUT data to. You need to call this and receive a new AWS API URL for each `partNumber`. Please see AWS's REST documentation on \"PUT Object to S3\" on how to upload. Once all parts has been uploaded, proceed to `/finish` endpoint.  **Note:** `nextPartNumber` seems like it is always ignored. Despite it returning 0, first partNumber is always 1.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | Must be a valid file ID. | [required] |
**version_id** | **i32** | Version ID of the asset. | [required] |
**file_type** | **String** | Type of file. | [required] |
**part_number** | Option<**i32**> | The part number to start uploading. If not provided, the first part will be started. |  |

### Return type

[**crate::models::FileUploadUrl**](FileUploadURL.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

