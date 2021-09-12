# Avatar

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asset_url** | Option<**String**> | Not present from general serach `/avatars`, only on specific requests `/avatars/{avatarId}`. | [optional]
**asset_url_object** | Option<[**serde_json::Value**](.md)> | Not present from general serach `/avatars`, only on specific requests `/avatars/{avatarId}`. **Deprecation:** `Object` has unknown usage/fields, and is always empty. Use normal `Url` field instead. | [optional]
**author_id** | **String** |  | 
**author_name** | **String** |  | [readonly]
**created_at** | **String** |  | [readonly]
**description** | **String** |  | 
**featured** | **bool** |  | [default to false]
**id** | **String** |  | 
**image_url** | **String** |  | 
**name** | **String** |  | 
**release_status** | [**crate::models::ReleaseStatus**](ReleaseStatus.md) |  | 
**tags** | **Vec<String>** |  | 
**thumbnail_image_url** | **String** |  | 
**unity_packages** | [**Vec<crate::models::UnityPackage>**](UnityPackage.md) |  | 
**unity_package_url** | **String** |  | 
**unity_package_url_object** | [**crate::models::AvatarUnityPackageUrlObject**](Avatar_unityPackageUrlObject.md) |  | 
**updated_at** | **String** |  | 
**version** | **f32** |  | [readonly][default to 0]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


