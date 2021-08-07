# Config

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | **String** | VRChat's office address | [readonly]
**announcements** | [**Vec<crate::models::ConfigAnnouncements>**](Config_announcements.md) | PSA, Public Announcements | [readonly]
**api_key** | **String** | apiKey to be used for all other requests | [readonly]
**app_name** | **String** | Game name | [readonly][default to VrChat]
**build_version_tag** | **String** | Build tag of the API server | [readonly]
**client_api_key** | **String** | apiKey to be used for all other requests | [readonly]
**client_bps_ceiling** | Option<**f32**> | Unknown | [optional]
**client_disconnect_timeout** | Option<**f32**> | Unknown | [optional]
**client_reserved_player_bps** | Option<**f32**> | Unknown | [optional]
**client_sent_count_allowance** | Option<**f32**> | Unknown | [optional]
**contact_email** | **String** | VRChat's contact email | [readonly]
**copyright_email** | **String** | VRChat's copyright-issues-related email | [readonly]
**current_tos_version** | **f32** | Current version number of the Terms of Service | [readonly]
**default_avatar** | **String** |  | 
**deployment_group** | [**crate::models::DeploymentGroup**](DeploymentGroup.md) |  | 
**dev_app_version_standalone** | **String** | Version number for game development build | [readonly]
**dev_download_link_windows** | **String** | Developer Download link | [readonly]
**dev_sdk_url** | **String** | Link to download the development SDK, use downloadUrls instead | [readonly]
**dev_sdk_version** | **String** | Version of the development SDK | [readonly]
**dev_server_version_standalone** | **String** | Version number for server development build | [readonly]
**disable_avatar_copying** | **bool** | Toggles if copying avatars should be disabled | [default to false]
**disable_avatar_gating** | **bool** | Toggles if avatar gating should be disabled. Avatar gating restricts uploading of avatars to people with the `system_avatar_access` Tag or `admin_avatar_access` Tag | [default to false]
**disable_community_labs** | **bool** | Toggles if the Community Labs should be disabled | [default to false]
**disable_community_labs_promotion** | **bool** | Toggles if promotion out of Community Labs should be disabled | [default to false]
**disable_email** | Option<**bool**> | Unknown | [optional][default to false]
**disable_event_stream** | **bool** | Toggles if Analytics should be disabled (this sreportedly not used in the Client) | [default to false]
**disable_feedback_gating** | **bool** | Toggles if feedback gating should be disabled. Feedback gating restricts submission of feedback (reporting a World or User) to people with the `system_feedback_access` Tag. | [default to false]
**disable_hello** | Option<**bool**> | Unknown | [optional][default to false]
**disable_registration** | **bool** | Toggles if new user account registration should be disabled | [default to false]
**disable_steam_networking** | **bool** | Toggles if Steam Networking should be disabled. VRChat these days uses Photon Unity Networking (PUN) instead. | [default to true]
**disable_two_factor_auth** | **bool** | Toggles if 2FA should be disabled. | [default to false]
**disable_udon** | **bool** | Toggles if Udon should be universally disabled in-game. | [default to false]
**disable_upgrade_account** | **bool** | Toggles if account upgrading \"linking with Steam/Oculus\" should be disabled. | [default to false]
**download_link_windows** | **String** | Download link for game on the Oculus Rift website. | [readonly]
**download_urls** | [**crate::models::ConfigDownloadUrls**](Config_downloadUrls.md) |  | 
**dynamic_world_rows** | [**Vec<crate::models::ConfigDynamicWorldRows>**](Config_dynamicWorldRows.md) | Array of DynamicWorldRow objects, used by the game to display the list of world rows | [readonly]
**events** | [**crate::models::ConfigEvents**](Config_events.md) |  | 
**gear_demo_room_id** | **String** | Unknown | [readonly]
**homepage_redirect_target** | **String** | Redirect target if you try to open the base API domain in your browser | [readonly][default to https://hello.vrchat.com]
**home_world_id** | **String** |  | 
**hub_world_id** | **String** |  | 
**jobs_email** | **String** | VRChat's job application email | [readonly]
**message_of_the_day** | **String** | MOTD | [readonly]
**moderation_email** | **String** | VRChat's moderation related email | [readonly]
**moderation_query_period** | **f32** | Unknown | 
**not_allowed_to_select_avatar_in_private_world_message** | **String** | Used in-game to notify a user they aren't allowed to select avatars in private worlds | [readonly]
**plugin** | **String** | Extra [plugin](https://doc.photonengine.com/en-us/server/current/plugins/manual) to run in each instance | [readonly]
**release_app_version_standalone** | **String** | Version number for game release build | [readonly]
**release_sdk_url** | **String** | Link to download the release SDK | [readonly]
**release_sdk_version** | **String** | Version of the release SDK | [readonly]
**release_server_version_standalone** | **String** | Version number for server release build | [readonly]
**sdk_developer_faq_url** | **String** | Link to the developer FAQ | [readonly]
**sdk_discord_url** | **String** | Link to the official VRChat Discord | [readonly]
**sdk_not_allowed_to_publish_message** | **String** | Used in the SDK to notify a user they aren't allowed to upload avatars/worlds yet | [readonly]
**sdk_unity_version** | **String** | Unity version supported by the SDK | [readonly]
**server_name** | **String** | Server name of the API server currently responding | [readonly]
**support_email** | **String** | VRChat's support email | [readonly]
**time_out_world_id** | **String** |  | 
**tutorial_world_id** | **String** |  | 
**update_rate_ms_maximum** | **f32** | Unknown | [readonly]
**update_rate_ms_minimum** | **f32** | Unknown | [readonly]
**update_rate_ms_normal** | **f32** | Unknown | [readonly]
**update_rate_ms_udon_manual** | **f32** | Unknown | [readonly]
**upload_analysis_percent** | **f32** | Unknown | [readonly]
**url_list** | **Vec<String>** | List of allowed URLs that bypass the \"Allow untrusted URL's\" setting in-game | [readonly]
**use_reliable_udp_for_voice** | **bool** | Unknown | [default to false]
**user_update_period** | **f32** | Unknown | [readonly]
**user_verification_delay** | **f32** | Unknown | [readonly]
**user_verification_retry** | **f32** | Unknown | [readonly]
**user_verification_timeout** | **f32** | Unknown | [readonly]
**vive_windows_url** | **String** | Download link for game on the Steam website. | [readonly]
**white_listed_asset_urls** | **Vec<String>** | List of allowed URLs that are allowed to host avatar assets | [readonly]
**world_update_period** | **f32** | Unknown | [readonly]
**youtubedl_hash** | **String** | Currently used youtube-dl.exe hash in SHA-256-delimited format | [readonly]
**youtubedl_version** | **String** | Currently used youtube-dl.exe version | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


