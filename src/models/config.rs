/*
 * VRChat API Documentation
 *
 * ![VRChat API Banner](https://vrchatapi.github.io/assets/img/api_banner_1500x400.png)  # Welcome to the VRChat API  Before we begin, we would like to state this is a **COMMUNITY DRIVEN PROJECT**. This means that everything you read on here was written by the community itself and is **not** officially supported by VRChat. The documentation is provided \"AS IS\", and any action you take towards VRChat is completely your own responsibility.  The documentation and additional libraries SHALL ONLY be used for applications interacting with VRChat's API in accordance with their [Terms of Service](https://github.com/VRChatAPI), and MUST NOT be used for modifying the client, \"avatar ripping\", or other illegal activities. Malicious usage or spamming the API may result in account termination. Certain parts of the API are also more sensitive than others, for example moderation, so please read tread extra carefully and read the warnings when present.  ![Tupper Policy on API](https://i.imgur.com/yLlW7Ok.png)  Finally, use of the API using applications other than the approved methods (website, VRChat application, Unity SDK) is not officially supported. VRChat provides no guarantee or support for external applications using the API. Access to API endpoints may break **at any time, without notice**. Therefore, please **do not ping** VRChat Staff in the VRChat Discord if you are having API problems, as they do not provide API support. We will make a best effort in keeping this documentation and associated language libraries up to date, but things might be outdated or missing. If you find that something is no longer valid, please contact us on Discord or [create an issue](https://github.com/vrchatapi/specification/issues) and tell us so we can fix it.  # Getting Started  The VRChat API can be used to programmatically retrieve or update information regarding your profile, friends, avatars, worlds and more. The API consists of two parts, \"Photon\" which is only used in-game, and the \"Web API\" which is used by both the game and the website. This documentation focuses only on the Web API.  The API is designed around the REST ideology, providing semi-simple and usually predictable URIs to access and modify objects. Requests support standard HTTP methods like GET, PUT, POST, and DELETE and standard status codes. Response bodies are always UTF-8 encoded JSON objects, unless explicitly documented otherwise.  <div class=\"callout callout-error\">   <strong>🛑 Warning! Do not touch Photon!</strong><br>   Photon is only used by the in-game client and should <b>not</b> be touched. Doing so may result in permanent account termination. </div>  <div class=\"callout callout-info\">   <strong>ℹ️ API Key and Authentication</strong><br>   The API Key has always been the same and is currently <code>JlE5Jldo5Jibnk5O5hTx6XVqsJu4WJ26</code>.   Read <a href=\"#tag--authentication\">Authentication</a> for how to log in. </div>  # Using the API  For simply exploring what the API can do it is strongly recommended to download [Insomnia](https://insomnia.rest/download), a free and open-source API client that's great for sending requests to the API in an orderly fashion. Insomnia allows you to send data in the format that's required for VRChat's API. It is also possible to try out the API in your browser, by first logging in at [vrchat.com/home](https://vrchat.com/home/) and then going to [vrchat.com/api/1/auth/user](https://vrchat.com/api/1/auth/user), but the information will be much harder to work with.  For more permanent operation such as software development it is instead recommended to use one of the existing language SDKs. This community project maintains API libraries in several languages, which allows you to interact with the API with simple function calls rather than having to implement the HTTP protocol yourself. Most of these libraries are automatically generated from the API specification, sometimes with additional helpful wrapper code to make usage easier. This allows them to be almost automatically updated and expanded upon as soon as a new feature is introduced in the specification itself. The libraries can be found on [GitHub](https://github.com/vrchatapi) or following:  * [NodeJS (JavaScript)](https://www.npmjs.com/package/vrchat) * [Dart](https://pub.dev/packages/vrchat_dart) * [Rust](https://crates.io/crates/vrchatapi) * [C#](github.com/vrchatapi/vrchatapi-csharp) * [Python](https://github.com/vrchatapi/VRChatPython)  # Pagination  Most endpoints enforce pagination, meaning they will only return 10 entries by default, and never more than 100.<br> Using both the limit and offset parameters allows you to easily paginate through a large number of objects.  | Query Parameter | Type | Description | | ----------|--|------- | | `limit` | integer  | The number of objects to return. This value often defaults to 10. Highest limit is always 100.| | `offset` | integer  | A zero-based offset from the default object sorting.|  If a request returns fewer objects than the `limit` parameter, there are no more items available to return.  # Contribution  Do you want to get involved in the documentation effort? Do you want to help improve one of the language API libraries? This project is an [OPEN Open Source Project](https://openopensource.org)! This means that individuals making significant and valuable contributions are given commit-access to the project. It also means we are very open and welcoming of new people making contributions, unlike some more guarded open-source projects.  [![Discord](https://img.shields.io/static/v1?label=vrchatapi&message=discord&color=blueviolet&style=for-the-badge)](https://discord.gg/qjZE9C9fkB)
 *
 * The version of the OpenAPI document: 1.0.2
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    /// VRChat's office address
    #[serde(rename = "address")]
    pub address: String,
    /// Public Announcements
    #[serde(rename = "announcements")]
    pub announcements: Vec<crate::models::ConfigAnnouncements>,
    /// apiKey to be used for all other requests
    #[serde(rename = "apiKey")]
    pub api_key: String,
    /// Game name
    #[serde(rename = "appName")]
    pub app_name: String,
    /// Build tag of the API server
    #[serde(rename = "buildVersionTag")]
    pub build_version_tag: String,
    /// apiKey to be used for all other requests
    #[serde(rename = "clientApiKey")]
    pub client_api_key: String,
    /// Unknown
    #[serde(rename = "clientBPSCeiling", skip_serializing_if = "Option::is_none")]
    pub client_bps_ceiling: Option<f32>,
    /// Unknown
    #[serde(rename = "clientDisconnectTimeout", skip_serializing_if = "Option::is_none")]
    pub client_disconnect_timeout: Option<f32>,
    /// Unknown
    #[serde(rename = "clientReservedPlayerBPS", skip_serializing_if = "Option::is_none")]
    pub client_reserved_player_bps: Option<f32>,
    /// Unknown
    #[serde(rename = "clientSentCountAllowance", skip_serializing_if = "Option::is_none")]
    pub client_sent_count_allowance: Option<f32>,
    /// VRChat's contact email
    #[serde(rename = "contactEmail")]
    pub contact_email: String,
    /// VRChat's copyright-issues-related email
    #[serde(rename = "copyrightEmail")]
    pub copyright_email: String,
    /// Current version number of the Terms of Service
    #[serde(rename = "currentTOSVersion")]
    pub current_tos_version: f32,
    #[serde(rename = "defaultAvatar")]
    pub default_avatar: String,
    #[serde(rename = "deploymentGroup")]
    pub deployment_group: crate::models::DeploymentGroup,
    /// Version number for game development build
    #[serde(rename = "devAppVersionStandalone")]
    pub dev_app_version_standalone: String,
    /// Developer Download link
    #[serde(rename = "devDownloadLinkWindows")]
    pub dev_download_link_windows: String,
    /// Link to download the development SDK, use downloadUrls instead
    #[serde(rename = "devSdkUrl")]
    pub dev_sdk_url: String,
    /// Version of the development SDK
    #[serde(rename = "devSdkVersion")]
    pub dev_sdk_version: String,
    /// Version number for server development build
    #[serde(rename = "devServerVersionStandalone")]
    pub dev_server_version_standalone: String,
    /// Toggles if copying avatars should be disabled
    #[serde(rename = "disableAvatarCopying")]
    pub disable_avatar_copying: bool,
    /// Toggles if avatar gating should be disabled. Avatar gating restricts uploading of avatars to people with the `system_avatar_access` Tag or `admin_avatar_access` Tag
    #[serde(rename = "disableAvatarGating")]
    pub disable_avatar_gating: bool,
    /// Toggles if the Community Labs should be disabled
    #[serde(rename = "disableCommunityLabs")]
    pub disable_community_labs: bool,
    /// Toggles if promotion out of Community Labs should be disabled
    #[serde(rename = "disableCommunityLabsPromotion")]
    pub disable_community_labs_promotion: bool,
    /// Unknown
    #[serde(rename = "disableEmail", skip_serializing_if = "Option::is_none")]
    pub disable_email: Option<bool>,
    /// Toggles if Analytics should be disabled (this sreportedly not used in the Client)
    #[serde(rename = "disableEventStream")]
    pub disable_event_stream: bool,
    /// Toggles if feedback gating should be disabled. Feedback gating restricts submission of feedback (reporting a World or User) to people with the `system_feedback_access` Tag.
    #[serde(rename = "disableFeedbackGating")]
    pub disable_feedback_gating: bool,
    /// Unknown
    #[serde(rename = "disableHello", skip_serializing_if = "Option::is_none")]
    pub disable_hello: Option<bool>,
    /// Toggles if new user account registration should be disabled
    #[serde(rename = "disableRegistration")]
    pub disable_registration: bool,
    /// Toggles if Steam Networking should be disabled. VRChat these days uses Photon Unity Networking (PUN) instead.
    #[serde(rename = "disableSteamNetworking")]
    pub disable_steam_networking: bool,
    /// Toggles if 2FA should be disabled.
    #[serde(rename = "disableTwoFactorAuth")]
    pub disable_two_factor_auth: bool,
    /// Toggles if Udon should be universally disabled in-game.
    #[serde(rename = "disableUdon")]
    pub disable_udon: bool,
    /// Toggles if account upgrading \"linking with Steam/Oculus\" should be disabled.
    #[serde(rename = "disableUpgradeAccount")]
    pub disable_upgrade_account: bool,
    /// Download link for game on the Oculus Rift website.
    #[serde(rename = "downloadLinkWindows")]
    pub download_link_windows: String,
    #[serde(rename = "downloadUrls")]
    pub download_urls: Box<crate::models::ConfigDownloadUrls>,
    /// Array of DynamicWorldRow objects, used by the game to display the list of world rows
    #[serde(rename = "dynamicWorldRows")]
    pub dynamic_world_rows: Vec<crate::models::ConfigDynamicWorldRows>,
    #[serde(rename = "events")]
    pub events: Box<crate::models::ConfigEvents>,
    /// Unknown
    #[serde(rename = "gearDemoRoomId")]
    pub gear_demo_room_id: String,
    /// Redirect target if you try to open the base API domain in your browser
    #[serde(rename = "homepageRedirectTarget")]
    pub homepage_redirect_target: String,
    #[serde(rename = "homeWorldId")]
    pub home_world_id: String,
    #[serde(rename = "hubWorldId")]
    pub hub_world_id: String,
    /// VRChat's job application email
    #[serde(rename = "jobsEmail")]
    pub jobs_email: String,
    /// MOTD
    #[serde(rename = "messageOfTheDay")]
    pub message_of_the_day: String,
    /// VRChat's moderation related email
    #[serde(rename = "moderationEmail")]
    pub moderation_email: String,
    /// Unknown
    #[serde(rename = "moderationQueryPeriod")]
    pub moderation_query_period: f32,
    /// Used in-game to notify a user they aren't allowed to select avatars in private worlds
    #[serde(rename = "notAllowedToSelectAvatarInPrivateWorldMessage")]
    pub not_allowed_to_select_avatar_in_private_world_message: String,
    /// Extra [plugin](https://doc.photonengine.com/en-us/server/current/plugins/manual) to run in each instance
    #[serde(rename = "plugin")]
    pub plugin: String,
    /// Version number for game release build
    #[serde(rename = "releaseAppVersionStandalone")]
    pub release_app_version_standalone: String,
    /// Link to download the release SDK
    #[serde(rename = "releaseSdkUrl")]
    pub release_sdk_url: String,
    /// Version of the release SDK
    #[serde(rename = "releaseSdkVersion")]
    pub release_sdk_version: String,
    /// Version number for server release build
    #[serde(rename = "releaseServerVersionStandalone")]
    pub release_server_version_standalone: String,
    /// Link to the developer FAQ
    #[serde(rename = "sdkDeveloperFaqUrl")]
    pub sdk_developer_faq_url: String,
    /// Link to the official VRChat Discord
    #[serde(rename = "sdkDiscordUrl")]
    pub sdk_discord_url: String,
    /// Used in the SDK to notify a user they aren't allowed to upload avatars/worlds yet
    #[serde(rename = "sdkNotAllowedToPublishMessage")]
    pub sdk_not_allowed_to_publish_message: String,
    /// Unity version supported by the SDK
    #[serde(rename = "sdkUnityVersion")]
    pub sdk_unity_version: String,
    /// Server name of the API server currently responding
    #[serde(rename = "serverName")]
    pub server_name: String,
    /// VRChat's support email
    #[serde(rename = "supportEmail")]
    pub support_email: String,
    #[serde(rename = "timeOutWorldId")]
    pub time_out_world_id: String,
    #[serde(rename = "tutorialWorldId")]
    pub tutorial_world_id: String,
    /// Unknown
    #[serde(rename = "updateRateMsMaximum")]
    pub update_rate_ms_maximum: f32,
    /// Unknown
    #[serde(rename = "updateRateMsMinimum")]
    pub update_rate_ms_minimum: f32,
    /// Unknown
    #[serde(rename = "updateRateMsNormal")]
    pub update_rate_ms_normal: f32,
    /// Unknown
    #[serde(rename = "updateRateMsUdonManual")]
    pub update_rate_ms_udon_manual: f32,
    /// Unknown
    #[serde(rename = "uploadAnalysisPercent")]
    pub upload_analysis_percent: f32,
    /// List of allowed URLs that bypass the \"Allow untrusted URL's\" setting in-game
    #[serde(rename = "urlList")]
    pub url_list: Vec<String>,
    /// Unknown
    #[serde(rename = "useReliableUdpForVoice")]
    pub use_reliable_udp_for_voice: bool,
    /// Unknown
    #[serde(rename = "userUpdatePeriod")]
    pub user_update_period: f32,
    /// Unknown
    #[serde(rename = "userVerificationDelay")]
    pub user_verification_delay: f32,
    /// Unknown
    #[serde(rename = "userVerificationRetry")]
    pub user_verification_retry: f32,
    /// Unknown
    #[serde(rename = "userVerificationTimeout")]
    pub user_verification_timeout: f32,
    /// Download link for game on the Steam website.
    #[serde(rename = "viveWindowsUrl")]
    pub vive_windows_url: String,
    /// List of allowed URLs that are allowed to host avatar assets
    #[serde(rename = "whiteListedAssetUrls")]
    pub white_listed_asset_urls: Vec<String>,
    /// Unknown
    #[serde(rename = "worldUpdatePeriod")]
    pub world_update_period: f32,
    /// Currently used youtube-dl.exe hash in SHA-256-delimited format
    #[serde(rename = "youtubedl-hash")]
    pub youtubedl_hash: String,
    /// Currently used youtube-dl.exe version
    #[serde(rename = "youtubedl-version")]
    pub youtubedl_version: String,
}

impl Config {
    pub fn new(address: String, announcements: Vec<crate::models::ConfigAnnouncements>, api_key: String, app_name: String, build_version_tag: String, client_api_key: String, contact_email: String, copyright_email: String, current_tos_version: f32, default_avatar: String, deployment_group: crate::models::DeploymentGroup, dev_app_version_standalone: String, dev_download_link_windows: String, dev_sdk_url: String, dev_sdk_version: String, dev_server_version_standalone: String, disable_avatar_copying: bool, disable_avatar_gating: bool, disable_community_labs: bool, disable_community_labs_promotion: bool, disable_event_stream: bool, disable_feedback_gating: bool, disable_registration: bool, disable_steam_networking: bool, disable_two_factor_auth: bool, disable_udon: bool, disable_upgrade_account: bool, download_link_windows: String, download_urls: crate::models::ConfigDownloadUrls, dynamic_world_rows: Vec<crate::models::ConfigDynamicWorldRows>, events: crate::models::ConfigEvents, gear_demo_room_id: String, homepage_redirect_target: String, home_world_id: String, hub_world_id: String, jobs_email: String, message_of_the_day: String, moderation_email: String, moderation_query_period: f32, not_allowed_to_select_avatar_in_private_world_message: String, plugin: String, release_app_version_standalone: String, release_sdk_url: String, release_sdk_version: String, release_server_version_standalone: String, sdk_developer_faq_url: String, sdk_discord_url: String, sdk_not_allowed_to_publish_message: String, sdk_unity_version: String, server_name: String, support_email: String, time_out_world_id: String, tutorial_world_id: String, update_rate_ms_maximum: f32, update_rate_ms_minimum: f32, update_rate_ms_normal: f32, update_rate_ms_udon_manual: f32, upload_analysis_percent: f32, url_list: Vec<String>, use_reliable_udp_for_voice: bool, user_update_period: f32, user_verification_delay: f32, user_verification_retry: f32, user_verification_timeout: f32, vive_windows_url: String, white_listed_asset_urls: Vec<String>, world_update_period: f32, youtubedl_hash: String, youtubedl_version: String) -> Config {
        Config {
            address,
            announcements,
            api_key,
            app_name,
            build_version_tag,
            client_api_key,
            client_bps_ceiling: None,
            client_disconnect_timeout: None,
            client_reserved_player_bps: None,
            client_sent_count_allowance: None,
            contact_email,
            copyright_email,
            current_tos_version,
            default_avatar,
            deployment_group,
            dev_app_version_standalone,
            dev_download_link_windows,
            dev_sdk_url,
            dev_sdk_version,
            dev_server_version_standalone,
            disable_avatar_copying,
            disable_avatar_gating,
            disable_community_labs,
            disable_community_labs_promotion,
            disable_email: None,
            disable_event_stream,
            disable_feedback_gating,
            disable_hello: None,
            disable_registration,
            disable_steam_networking,
            disable_two_factor_auth,
            disable_udon,
            disable_upgrade_account,
            download_link_windows,
            download_urls: Box::new(download_urls),
            dynamic_world_rows,
            events: Box::new(events),
            gear_demo_room_id,
            homepage_redirect_target,
            home_world_id,
            hub_world_id,
            jobs_email,
            message_of_the_day,
            moderation_email,
            moderation_query_period,
            not_allowed_to_select_avatar_in_private_world_message,
            plugin,
            release_app_version_standalone,
            release_sdk_url,
            release_sdk_version,
            release_server_version_standalone,
            sdk_developer_faq_url,
            sdk_discord_url,
            sdk_not_allowed_to_publish_message,
            sdk_unity_version,
            server_name,
            support_email,
            time_out_world_id,
            tutorial_world_id,
            update_rate_ms_maximum,
            update_rate_ms_minimum,
            update_rate_ms_normal,
            update_rate_ms_udon_manual,
            upload_analysis_percent,
            url_list,
            use_reliable_udp_for_voice,
            user_update_period,
            user_verification_delay,
            user_verification_retry,
            user_verification_timeout,
            vive_windows_url,
            white_listed_asset_urls,
            world_update_period,
            youtubedl_hash,
            youtubedl_version,
        }
    }
}


