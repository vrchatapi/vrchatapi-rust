# World

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asset_url** | **String** |  | 
**asset_url_object** | [**serde_json::Value**](.md) |  | 
**author_id** | **String** |  | 
**author_name** | **String** |  | 
**capacity** | **i32** |  | 
**created_at** | **String** |  | 
**description** | **String** |  | 
**favorites** | Option<**i32**> |  | [optional][default to 0]
**featured** | **bool** |  | [default to false]
**heat** | **i32** |  | [default to 0]
**id** | **String** |  | 
**image_url** | **String** |  | 
**instances** | Option<[**Vec<Vec<serde_json::Value>>**](array.md)> |  | [optional]
**labs_publication_date** | **String** |  | 
**name** | **String** |  | 
**namespace** | **String** |  | 
**occupants** | Option<**i32**> |  | [optional][default to 0]
**organization** | **String** |  | [default to vrchat]
**plugin_url_object** | [**serde_json::Value**](.md) |  | 
**popularity** | **i32** |  | [default to 0]
**preview_youtube_id** | Option<**String**> |  | [optional]
**private_occupants** | Option<**i32**> |  | [optional][default to 0]
**public_occupants** | Option<**i32**> |  | [optional][default to 0]
**publication_date** | **String** |  | 
**release_status** | [**crate::models::ReleaseStatus**](ReleaseStatus.md) |  | 
**tags** | **Vec<String>** |  | 
**thumbnail_image_url** | **String** |  | 
**unity_package_url_object** | [**serde_json::Value**](.md) |  | 
**unity_packages** | [**Vec<crate::models::UnityPackage>**](UnityPackage.md) |  | 
**updated_at** | **String** |  | 
**version** | **i32** |  | [default to 0]
**visits** | **i32** |  | [default to 0]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


