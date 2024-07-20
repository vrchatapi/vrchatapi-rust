# Avatar

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asset_url** | Option<**String**> | Not present from general serach `/avatars`, only on specific requests `/avatars/{avatarId}`. | [optional]
**asset_url_object** | Option<[**serde_json::Value**](.md)> | Not present from general serach `/avatars`, only on specific requests `/avatars/{avatarId}`. **Deprecation:** `Object` has unknown usage/fields, and is always empty. Use normal `Url` field instead. | [optional]
**author_id** | **String** | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | 
**author_name** | **String** |  | 
**created_at** | **String** |  | 
**description** | **String** |  | 
**featured** | **bool** |  | [default to false]
**id** | **String** |  | 
**image_url** | **String** |  | 
**name** | **String** |  | 
**release_status** | [**models::ReleaseStatus**](ReleaseStatus.md) |  | 
**tags** | **Vec<String>** |   | 
**thumbnail_image_url** | **String** |  | 
**unity_package_url** | **String** |  | 
**unity_package_url_object** | [**models::AvatarUnityPackageUrlObject**](Avatar_unityPackageUrlObject.md) |  | 
**unity_packages** | [**Vec<models::UnityPackage>**](UnityPackage.md) |  | 
**updated_at** | **String** |  | 
**version** | **i32** |  | [default to 0]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


