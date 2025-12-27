# UpdatePropRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asset_url** | Option<**String**> |  | [optional]
**asset_version** | Option<**i32**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**image_url** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**platform** | Option<**String**> | This is normally `android`, `ios`, `standalonewindows`, `web`, or the empty value ``, but also supposedly can be any random Unity version such as `2019.2.4-801-Release` or `2019.2.2-772-Release` or even `unknownplatform`. | [optional]
**prop_signature** | Option<**String**> |  | [optional]
**spawn_type** | Option<**i32**> | How a prop is summoned and interacted with. 0: the prop fixed to some surface in the world 1: the prop is a pickup and may be held by users 2: ??? | [optional][default to 1]
**tags** | Option<**Vec<String>**> |  | [optional]
**unity_version** | Option<**String**> |  | [optional]
**world_placement_mask** | Option<**i32**> | Bitmask for restrictions on what world surfaces a prop may be summoned. 0: no restrictions 1: floors 2: walls 4: ceilings | [optional][default to 1]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


