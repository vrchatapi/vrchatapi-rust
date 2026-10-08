# WebsocketUserLocation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**instance** | **String** | InstanceID can be \"offline\" on User profiles if you are not friends with that user and \"private\" if you are friends and user is in private instance. | 
**location** | **String** | Represents a unique location, consisting of a world identifier and an instance identifier, or \"offline\" if the user is not on your friends list. | 
**traveling_to_location** | **String** | Represents a unique location, consisting of a world identifier and an instance identifier, or \"offline\" if the user is not on your friends list. | 
**user** | [**models::User**](User.md) |  | 
**user_id** | **String** | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


