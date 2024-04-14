# World

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**author_id** | **String** | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | 
**author_name** | **String** |  | 
**capacity** | **i32** |  | 
**recommended_capacity** | **i32** |  | 
**created_at** | **String** |  | 
**description** | **String** |  | 
**favorites** | Option<**i32**> |  | [optional][default to 0]
**featured** | **bool** |  | [default to false]
**heat** | **i32** |  | [default to 0]
**id** | **String** | WorldID be \"offline\" on User profiles if you are not friends with that user. | 
**image_url** | **String** |  | 
**instances** | Option<[**Vec<Vec<serde_json::Value>>**](array.md)> | Will always be an empty list when unauthenticated. | [optional]
**labs_publication_date** | **String** |  | 
**name** | **String** |  | 
**namespace** | Option<**String**> |  | [optional]
**occupants** | Option<**i32**> | Will always be `0` when unauthenticated. | [optional][default to 0]
**organization** | **String** |  | [default to vrchat]
**popularity** | **i32** |  | [default to 0]
**preview_youtube_id** | Option<**String**> |  | [optional]
**private_occupants** | Option<**i32**> | Will always be `0` when unauthenticated. | [optional][default to 0]
**public_occupants** | Option<**i32**> | Will always be `0` when unauthenticated. | [optional][default to 0]
**publication_date** | **String** |  | 
**release_status** | [**crate::models::ReleaseStatus**](ReleaseStatus.md) |  | 
**tags** | **Vec<String>** |   | 
**thumbnail_image_url** | **String** |  | 
**unity_packages** | Option<[**Vec<crate::models::UnityPackage>**](UnityPackage.md)> | Empty if unauthenticated. | [optional]
**updated_at** | **String** |  | 
**version** | **i32** |  | [default to 0]
**visits** | **i32** |  | [default to 0]
**udon_products** | Option<**Vec<String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


