# FavoritedWorld

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**author_id** | Option<**String**> | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | [optional]
**author_name** | **String** |  | 
**capacity** | **i32** |  | 
**created_at** | Option<**String**> |  | [optional]
**default_content_settings** | Option<[**models::InstanceContentSettings**](InstanceContentSettings.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**favorite_group** | **String** |  | 
**favorite_id** | **String** |  | 
**favorites** | Option<**i32**> |  | [optional][default to 0]
**featured** | Option<**bool**> |  | [optional][default to false]
**heat** | Option<**i32**> |  | [optional][default to 0]
**id** | **String** | WorldID be \"offline\" on User profiles if you are not friends with that user. | 
**image_url** | **String** |  | 
**labs_publication_date** | Option<**String**> |  | [optional]
**name** | **String** |  | 
**occupants** | **i32** |  | [default to 0]
**organization** | Option<**String**> |  | [optional][default to vrchat]
**popularity** | Option<**i32**> |  | [optional][default to 0]
**preview_youtube_id** | Option<**String**> |  | [optional]
**publication_date** | Option<**String**> |  | [optional]
**recommended_capacity** | Option<**i32**> |  | [optional]
**release_status** | [**models::ReleaseStatus**](ReleaseStatus.md) |  | 
**tags** | Option<**Vec<String>**> |   | [optional]
**thumbnail_image_url** | **String** |  | 
**udon_products** | Option<**Vec<String>**> |  | [optional]
**unity_packages** | Option<[**Vec<models::UnityPackage>**](UnityPackage.md)> |   | [optional]
**updated_at** | Option<**String**> |  | [optional]
**url_list** | Option<**Vec<String>**> |  | [optional]
**version** | Option<**i32**> |  | [optional]
**visits** | Option<**i32**> |  | [optional][default to 0]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


