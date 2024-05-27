# LimitedWorld

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**author_id** | **String** | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | 
**author_name** | **String** |  | 
**capacity** | **i32** |  | 
**recommended_capacity** | Option<**i32**> |  | [optional]
**created_at** | **String** |  | 
**favorites** | **i32** |  | [default to 0]
**visits** | Option<**i32**> |  | [optional][default to 0]
**heat** | **i32** |  | [default to 0]
**id** | **String** | WorldID be \"offline\" on User profiles if you are not friends with that user. | 
**image_url** | **String** |  | 
**labs_publication_date** | **String** |  | 
**name** | **String** |  | 
**occupants** | **i32** |  | [default to 0]
**organization** | **String** |  | [default to vrchat]
**popularity** | **i32** |  | [default to 0]
**preview_youtube_id** | Option<**String**> |  | [optional]
**publication_date** | **String** |  | 
**release_status** | [**crate::models::ReleaseStatus**](ReleaseStatus.md) |  | 
**tags** | **Vec<String>** |   | 
**thumbnail_image_url** | **String** |  | 
**unity_packages** | [**Vec<crate::models::LimitedUnityPackage>**](LimitedUnityPackage.md) |   | 
**updated_at** | **String** |  | 
**udon_products** | Option<**Vec<String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


