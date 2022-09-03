# ApiConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**voice_enable_degradation** | **bool** | Unknown, probably voice optimization testing | [default to false]
**voice_enable_receiver_limiting** | **bool** | Unknown, probably voice optimization testing | [default to true]
**address** | **String** | VRChat's office address | 
**announcements** | [**Vec<crate::models::PublicAnnouncement>**](Public_Announcement.md) | Public Announcements | 
**api_key** | **String** | apiKey to be used for all other requests | 
**app_name** | **String** | Game name | [default to VrChat]
**build_version_tag** | **String** | Build tag of the API server | 
**client_api_key** | **String** | apiKey to be used for all other requests | 
**client_bps_ceiling** | **i32** | Unknown | [default to 18432]
**client_disconnect_timeout** | **i32** | Unknown | [default to 30000]
**client_reserved_player_bps** | **i32** | Unknown | [default to 7168]
**client_sent_count_allowance** | **i32** | Unknown | [default to 15]
**contact_email** | **String** | VRChat's contact email | 
**copyright_email** | **String** | VRChat's copyright-issues-related email | 
**current_tos_version** | **i32** | Current version number of the Terms of Service | 
**default_avatar** | **String** |  | 
**deployment_group** | [**crate::models::DeploymentGroup**](DeploymentGroup.md) |  | 
**dev_app_version_standalone** | **String** | Version number for game development build | 
**dev_download_link_windows** | **String** | Developer Download link | 
**dev_sdk_url** | **String** | Link to download the development SDK, use downloadUrls instead | 
**dev_sdk_version** | **String** | Version of the development SDK | 
**dev_server_version_standalone** | **String** | Version number for server development build | 
**dis_countdown** | **String** | Unknown, \"dis\" maybe for disconnect? | 
**disable_avatar_copying** | **bool** | Toggles if copying avatars should be disabled | [default to false]
**disable_avatar_gating** | **bool** | Toggles if avatar gating should be disabled. Avatar gating restricts uploading of avatars to people with the `system_avatar_access` Tag or `admin_avatar_access` Tag | [default to false]
**disable_community_labs** | **bool** | Toggles if the Community Labs should be disabled | [default to false]
**disable_community_labs_promotion** | **bool** | Toggles if promotion out of Community Labs should be disabled | [default to false]
**disable_email** | **bool** | Unknown | [default to false]
**disable_event_stream** | **bool** | Toggles if Analytics should be disabled. | [default to false]
**disable_feedback_gating** | **bool** | Toggles if feedback gating should be disabled. Feedback gating restricts submission of feedback (reporting a World or User) to people with the `system_feedback_access` Tag. | [default to false]
**disable_frontend_builds** | **bool** | Unknown, probably toggles compilation of frontend web builds? So internal flag? | [default to false]
**disable_hello** | **bool** | Unknown | [default to false]
**disable_oculus_subs** | **bool** | Toggles if signing up for Subscriptions in Oculus is disabled or not. | [default to false]
**disable_registration** | **bool** | Toggles if new user account registration should be disabled. | [default to false]
**disable_steam_networking** | **bool** | Toggles if Steam Networking should be disabled. VRChat these days uses Photon Unity Networking (PUN) instead. | [default to true]
**disable_two_factor_auth** | **bool** | Toggles if 2FA should be disabled. | [default to false]
**disable_udon** | **bool** | Toggles if Udon should be universally disabled in-game. | [default to false]
**disable_upgrade_account** | **bool** | Toggles if account upgrading \"linking with Steam/Oculus\" should be disabled. | [default to false]
**download_link_windows** | **String** | Download link for game on the Oculus Rift website. | 
**download_urls** | [**crate::models::DownloadUrlList**](DownloadURLList.md) |  | 
**dynamic_world_rows** | [**Vec<crate::models::DynamicContentRow>**](DynamicContentRow.md) | Array of DynamicWorldRow objects, used by the game to display the list of world rows | 
**events** | [**crate::models::ApiEventConfig**](APIEventConfig.md) |  | 
**gear_demo_room_id** | **String** | Unknown | 
**home_world_id** | **String** | WorldID be \"offline\" on User profiles if you are not friends with that user. | 
**homepage_redirect_target** | **String** | Redirect target if you try to open the base API domain in your browser | [default to https://hello.vrchat.com]
**hub_world_id** | **String** | WorldID be \"offline\" on User profiles if you are not friends with that user. | 
**jobs_email** | **String** | VRChat's job application email | 
**message_of_the_day** | **String** | MOTD | 
**moderation_email** | **String** | VRChat's moderation related email | 
**moderation_query_period** | **i32** | Unknown | 
**not_allowed_to_select_avatar_in_private_world_message** | **String** | Used in-game to notify a user they aren't allowed to select avatars in private worlds | 
**plugin** | **String** | Extra [plugin](https://doc.photonengine.com/en-us/server/current/plugins/manual) to run in each instance | 
**release_app_version_standalone** | **String** | Version number for game release build | 
**release_sdk_url** | **String** | Link to download the release SDK | 
**release_sdk_version** | **String** | Version of the release SDK | 
**release_server_version_standalone** | **String** | Version number for server release build | 
**sdk_developer_faq_url** | **String** | Link to the developer FAQ | 
**sdk_discord_url** | **String** | Link to the official VRChat Discord | 
**sdk_not_allowed_to_publish_message** | **String** | Used in the SDK to notify a user they aren't allowed to upload avatars/worlds yet | 
**sdk_unity_version** | **String** | Unity version supported by the SDK | 
**server_name** | **String** | Server name of the API server currently responding | 
**support_email** | **String** | VRChat's support email | 
**time_out_world_id** | **String** | WorldID be \"offline\" on User profiles if you are not friends with that user. | 
**tutorial_world_id** | **String** | WorldID be \"offline\" on User profiles if you are not friends with that user. | 
**update_rate_ms_maximum** | **i32** | Unknown | 
**update_rate_ms_minimum** | **i32** | Unknown | 
**update_rate_ms_normal** | **i32** | Unknown | 
**update_rate_ms_udon_manual** | **i32** | Unknown | 
**upload_analysis_percent** | **i32** | Unknown | 
**url_list** | **Vec<String>** | List of allowed URLs that bypass the \"Allow untrusted URL's\" setting in-game | 
**use_reliable_udp_for_voice** | **bool** | Unknown | [default to false]
**user_update_period** | **i32** | Unknown | 
**user_verification_delay** | **i32** | Unknown | 
**user_verification_retry** | **i32** | Unknown | 
**user_verification_timeout** | **i32** | Unknown | 
**vive_windows_url** | **String** | Download link for game on the Steam website. | 
**white_listed_asset_urls** | **Vec<String>** | List of allowed URLs that are allowed to host avatar assets | 
**world_update_period** | **i32** | Unknown | 
**player_url_resolver_hash** | **String** | Currently used youtube-dl.exe hash in SHA-256-delimited format | 
**player_url_resolver_version** | **String** | Currently used youtube-dl.exe version | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


