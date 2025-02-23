# ApiConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**voice_enable_degradation** | **bool** | Unknown, probably voice optimization testing | [default to false]
**voice_enable_receiver_limiting** | **bool** | Unknown, probably voice optimization testing | [default to true]
**access_logs_urls** | [**models::ApiConfigAccessLogsUrls**](APIConfig_accessLogsUrls.md) |  | 
**address** | **String** | VRChat's office address | 
**age_verification_invite_visible** | **bool** |  | 
**age_verification_p** | **bool** |  | 
**age_verification_status_visible** | **bool** |  | 
**analysis_max_retries** | **i32** | Max retries for avatar analysis requests | 
**analysis_retry_interval** | **i32** | Interval between retries for avatar analysis requests | 
**announcements** | [**Vec<models::ApiConfigAnnouncement>**](APIConfigAnnouncement.md) | Public Announcements | 
**analytics_segment_new_ui_pct_of_users** | **i32** | Unknown | 
**analytics_segment_new_ui_salt** | **String** | Unknown | 
**app_name** | **String** | Game name | [default to VrChat]
**available_language_codes** | **Vec<String>** | List of supported Languages | 
**available_languages** | **Vec<String>** | List of supported Languages | 
**avatar_perf_limiter** | [**models::ApiConfigAvatarPerfLimiter**](APIConfig_avatarPerfLimiter.md) |  | 
**build_version_tag** | **String** | Build tag of the API server | 
**chatbox_log_buffer_seconds** | **i32** | Unknown | [default to 40]
**client_api_key** | **String** | apiKey to be used for all other requests | 
**client_bps_ceiling** | **i32** | Unknown | [default to 18432]
**client_disconnect_timeout** | **i32** | Unknown | [default to 30000]
**client_net_dispatch_thread** | Option<**bool**> | Unknown | [optional][default to false]
**client_net_dispatch_thread_mobile** | **bool** | Unknown | [default to true]
**client_net_in_thread** | Option<**bool**> | Unknown | [optional][default to false]
**client_net_in_thread2** | Option<**bool**> | Unknown | [optional][default to false]
**client_net_in_thread_mobile** | Option<**bool**> | Unknown | [optional][default to false]
**client_net_in_thread_mobile2** | Option<**bool**> | Unknown | [optional][default to false]
**client_net_out_thread** | Option<**bool**> | Unknown | [optional][default to false]
**client_net_out_thread2** | Option<**bool**> | Unknown | [optional][default to false]
**client_net_out_thread_mobile** | Option<**bool**> | Unknown | [optional][default to false]
**client_net_out_thread_mobile2** | Option<**bool**> | Unknown | [optional][default to false]
**client_qr** | Option<**i32**> | Unknown | [optional][default to 1]
**client_reserved_player_bps** | **i32** | Unknown | [default to 7168]
**client_sent_count_allowance** | **i32** | Unknown | [default to 15]
**constants** | [**models::ApiConfigConstants**](APIConfigConstants.md) |  | 
**contact_email** | **String** | VRChat's contact email | 
**copyright_email** | **String** | VRChat's copyright-issues-related email | 
**current_privacy_version** | Option<**i32**> | Current version number of the Privacy Agreement | [optional][default to 1]
**current_tos_version** | **i32** | Current version number of the Terms of Service | 
**default_avatar** | **String** |  | 
**default_sticker_set** | **String** |  | 
**deployment_group** | [**models::DeploymentGroup**](DeploymentGroup.md) |  | 
**dev_language_codes** | Option<**Vec<String>**> | Unknown | [optional]
**dev_sdk_url** | **String** | Link to download the development SDK, use downloadUrls instead | 
**dev_sdk_version** | **String** | Version of the development SDK | 
**dis_countdown** | **String** | Unknown, \"dis\" maybe for disconnect? | 
**disable_av_pro_in_proton** | Option<**bool**> | Unknown | [optional][default to false]
**disable_avatar_copying** | **bool** | Toggles if copying avatars should be disabled | [default to false]
**disable_avatar_gating** | **bool** | Toggles if avatar gating should be disabled. Avatar gating restricts uploading of avatars to people with the `system_avatar_access` Tag or `admin_avatar_access` Tag | [default to false]
**disable_community_labs** | **bool** | Toggles if the Community Labs should be disabled | [default to false]
**disable_community_labs_promotion** | **bool** | Toggles if promotion out of Community Labs should be disabled | [default to false]
**disable_email** | **bool** | Unknown | [default to false]
**disable_captcha** | Option<**bool**> | Unknown | [optional][default to true]
**disable_event_stream** | **bool** | Toggles if Analytics should be disabled. | [default to false]
**disable_feedback_gating** | **bool** | Toggles if feedback gating should be disabled. Feedback gating restricts submission of feedback (reporting a World or User) to people with the `system_feedback_access` Tag. | [default to false]
**disable_frontend_builds** | **bool** | Unknown, probably toggles compilation of frontend web builds? So internal flag? | [default to false]
**disable_gift_drops** | **bool** | Toggles if gift drops should be disabled | [default to false]
**disable_hello** | **bool** | Unknown | [default to false]
**disable_oculus_subs** | **bool** | Toggles if signing up for Subscriptions in Oculus is disabled or not. | [default to false]
**disable_registration** | **bool** | Toggles if new user account registration should be disabled. | [default to false]
**disable_steam_networking** | **bool** | Toggles if Steam Networking should be disabled. VRChat these days uses Photon Unity Networking (PUN) instead. | [default to true]
**disable_two_factor_auth** | **bool** | Toggles if 2FA should be disabled. | [default to false]
**disable_udon** | **bool** | Toggles if Udon should be universally disabled in-game. | [default to false]
**disable_upgrade_account** | **bool** | Toggles if account upgrading \"linking with Steam/Oculus\" should be disabled. | [default to false]
**download_link_windows** | **String** | Download link for game on the Oculus Rift website. | 
**download_urls** | [**models::ApiConfigDownloadUrlList**](APIConfigDownloadURLList.md) |  | 
**dynamic_world_rows** | [**Vec<models::DynamicContentRow>**](DynamicContentRow.md) | Array of DynamicWorldRow objects, used by the game to display the list of world rows | 
**economy_pause_end** | Option<**String**> | Unknown | [optional]
**economy_pause_start** | Option<**String**> | Unknown | [optional]
**economy_state** | Option<**i32**> | Unknown | [optional][default to 1]
**events** | [**models::ApiConfigEvents**](APIConfigEvents.md) |  | 
**force_use_latest_world** | **bool** | Unknown | [default to true]
**gift_display_type** | **String** | Display type of gifts | 
**google_api_client_id** | **String** | Unknown | [default to 827942544393-r2ouvckvouldn9dg9uruseje575e878f.apps.googleusercontent.com]
**home_world_id** | **String** | WorldID be \"offline\" on User profiles if you are not friends with that user. | 
**homepage_redirect_target** | **String** | Redirect target if you try to open the base API domain in your browser | [default to https://hello.vrchat.com]
**hub_world_id** | **String** | WorldID be \"offline\" on User profiles if you are not friends with that user. | 
**image_host_url_list** | **Vec<String>** | A list of explicitly allowed origins that worlds can request images from via the Udon's [VRCImageDownloader#DownloadImage](https://creators.vrchat.com/worlds/udon/image-loading/#downloadimage). | 
**jobs_email** | **String** | VRChat's job application email | 
**min_supported_client_build_number** | [**models::ApiConfigMinSupportedClientBuildNumber**](APIConfig_minSupportedClientBuildNumber.md) |  | 
**minimum_unity_version_for_uploads** | **String** | Minimum Unity version required for uploading assets | [default to 2019.0.0f1]
**moderation_email** | **String** | VRChat's moderation related email | 
**not_allowed_to_select_avatar_in_private_world_message** | **String** | Used in-game to notify a user they aren't allowed to select avatars in private worlds | 
**offline_analysis** | [**models::ApiConfigOfflineAnalysis**](APIConfig_offlineAnalysis.md) |  | 
**photon_nameserver_overrides** | **Vec<String>** | Unknown | 
**photon_public_keys** | **Vec<String>** | Unknown | 
**report_categories** | [**models::ApiConfigReportCategories**](APIConfig_reportCategories.md) |  | 
**report_form_url** | **String** | URL to the report form | [default to https://help.vrchat.com/hc/en-us/requests/new?ticket_form_id=1500000182242&tf_360056455174=user_report&tf_360057451993={userId}&tf_1500001445142={reportedId}&tf_subject={reason} {category} By {contentType} {reportedName}&tf_description={description}]
**report_options** | [**models::ApiConfigReportOptions**](APIConfig_reportOptions.md) |  | 
**report_reasons** | [**models::ApiConfigReportReasons**](APIConfig_reportReasons.md) |  | 
**require_age_verification_beta_tag** | **bool** |  | 
**sdk_developer_faq_url** | **String** | Link to the developer FAQ | 
**sdk_discord_url** | **String** | Link to the official VRChat Discord | 
**sdk_not_allowed_to_publish_message** | **String** | Used in the SDK to notify a user they aren't allowed to upload avatars/worlds yet | 
**sdk_unity_version** | **String** | Unity version supported by the SDK | 
**server_name** | **String** | Server name of the API server currently responding | 
**string_host_url_list** | **Vec<String>** | A list of explicitly allowed origins that worlds can request strings from via the Udon's [VRCStringDownloader.LoadUrl](https://creators.vrchat.com/worlds/udon/string-loading/#ivrcstringdownload). | 
**support_email** | **String** | VRChat's support email | 
**support_form_url** | **String** | VRChat's support form | 
**timekeeping** | **bool** | Unknown | [default to true]
**time_out_world_id** | **String** | WorldID be \"offline\" on User profiles if you are not friends with that user. | 
**tutorial_world_id** | **String** | WorldID be \"offline\" on User profiles if you are not friends with that user. | 
**update_rate_ms_maximum** | **i32** | Unknown | 
**update_rate_ms_minimum** | **i32** | Unknown | 
**update_rate_ms_normal** | **i32** | Unknown | 
**update_rate_ms_udon_manual** | **i32** | Unknown | 
**upload_analysis_percent** | **i32** | Unknown | 
**url_list** | **Vec<String>** | List of allowed URLs that bypass the \"Allow untrusted URL's\" setting in-game | 
**use_reliable_udp_for_voice** | **bool** | Unknown | [default to false]
**vive_windows_url** | **String** | Download link for game on the Steam website. | 
**white_listed_asset_urls** | **Vec<String>** | List of allowed URLs that are allowed to host avatar assets | 
**player_url_resolver_version** | **String** | Currently used youtube-dl.exe version | 
**player_url_resolver_sha1** | **String** | Currently used youtube-dl.exe hash in SHA1-delimited format | 
**websocket_max_friends_refresh_delay** | **i32** | Unknown | [default to 900]
**websocket_quick_reconnect_time** | **i32** | Unknown | [default to 2]
**websocket_reconnect_max_delay** | **i32** | Unknown | [default to 2]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


