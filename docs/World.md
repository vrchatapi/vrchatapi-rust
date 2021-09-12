# World

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asset_url** | **String** |  | 
**asset_url_object** | [**serde_json::Value**](.md) |  | 
**author_id** | **String** |  | 
**author_name** | **String** |  | [readonly]
**capacity** | **f32** |  | [readonly]
**created_at** | **String** |  | [readonly]
**description** | **String** |  | 
**favorites** | Option<**f32**> |  | [optional][readonly][default to 0]
**featured** | **bool** |  | [default to false]
**heat** | **f32** |  | [readonly][default to 0]
**id** | **String** |  | 
**image_url** | **String** |  | 
**instances** | Option<[**Vec<Vec<serde_json::Value>>**](array.md)> |  | [optional]
**labs_publication_date** | **String** |  | 
**name** | **String** |  | 
**namespace** | **String** |  | 
**occupants** | Option<**f32**> |  | [optional][readonly][default to 0]
**organization** | **String** |  | [default to vrchat]
**plugin_url_object** | [**serde_json::Value**](.md) |  | 
**popularity** | **f32** |  | [readonly][default to 0]
**preview_youtube_id** | Option<**String**> |  | [optional]
**private_occupants** | Option<**f32**> |  | [optional][readonly][default to 0]
**public_occupants** | Option<**f32**> |  | [optional][readonly][default to 0]
**publication_date** | **String** |  | 
**release_status** | [**crate::models::ReleaseStatus**](ReleaseStatus.md) |  | 
**tags** | **Vec<String>** |  | 
**thumbnail_image_url** | **String** |  | 
**unity_package_url_object** | [**serde_json::Value**](.md) |  | 
**unity_packages** | [**Vec<crate::models::UnityPackage>**](UnityPackage.md) |  | 
**updated_at** | **String** |  | 
**version** | **f32** |  | [default to 0]
**visits** | **f32** |  | [readonly][default to 0]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


