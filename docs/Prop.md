# Prop

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_created_at** | **String** |  | 
**_updated_at** | **String** |  | 
**author_id** | **String** | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | 
**author_name** | **String** |  | 
**description** | **String** |  | 
**id** | **String** |  | 
**image_url** | **String** |  | 
**max_count_per_user** | **i32** |  | [default to 1]
**name** | **String** |  | 
**release_status** | [**models::ReleaseStatus**](ReleaseStatus.md) |  | 
**spawn_type** | **i32** |  | [default to 0]
**tags** | **Vec<String>** |  | 
**thumbnail_image_url** | **String** |  | 
**unity_package_url** | Option<**String**> |  | 
**unity_packages** | [**Vec<models::PropUnityPackage>**](PropUnityPackage.md) |  | 
**world_placement_mask** | **i32** |  | [default to 1]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


