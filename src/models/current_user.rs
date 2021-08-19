/*
 * VRChat API Documentation
 *
 * ![VRChat API Banner](https://vrchatapi.github.io/assets/img/api_banner_1500x400.png)  # Welcome to the VRChat API  Before we begin, we would like to state this is a **COMMUNITY DRIVEN PROJECT**. This means that everything you read on here was written by the community itself, and is **not** officially supported by VRChat. The documentation and additional libraries SHALL ONLY be used for applications interacting with VRChat's API, in accordance with their [Terms of Service](https://github.com/VRChatAPI), and MUST NOT be used for modifying the client, \"avatar ripping\", or other illegal activities.  This documentation is provided \"AS IS\", and any action you take towards VRChat is completely your own responsibility. Malicious usage or spamming the API may result in account termination. Certain parts of the API are also more sensitive than others, for example moderation, so please read tread extra carefully and read the warnings when present.  ![Tupper Policy on API](https://i.imgur.com/yLlW7Ok.png)  Finally, use of the API using applications other than the approved methods (website, VRChat application, Unity SDK) is not officially supported. VRChat provides no guarantee for external applications using the API. Access to API endpoints may break **at any time, without notice**. We will make a best effort in keeping this documentation and associated language libraries up to date, but things might be outdated or missing. If you find that something is no longer valid, please contact us on Discord or [create an issue](https://github.com/vrchatapi/specification/issues) and tell us so we can fix it.  # Getting Started  The VRChat API can be used to programatically retrive or update information regarding your profile, friends, avatars, worlds and more. The API consists of two parts, \"Photon\" which is only used in-game, and the \"Web API\" which is used by both the game and the website. This documentation focuses only on the Web API.  The API is designed around the REST ideology, providing semi-simple and usually predictable URIs to access and modify objects. Requests support standard HTTP methods like GET, PUT, POST, and DELETE and standard status codes. Response bodies are always UTF-8 encoded JSON objects, unless explicitly documented otherwise.  <div class=\"callout callout-error\">   <strong>🛑 Warning! Do not touch Photon!</strong><br>   Photon is only used by the in-game client and should <b>not</b> be touched. Doing so may result in permanent account termination. </div>  <div class=\"callout callout-info\">   <strong>ℹ️ API Key and Authentication</strong><br>   The API Key has always been the same, and is currently <code>JlE5Jldo5Jibnk5O5hTx6XVqsJu4WJ26</code>.   Read <a href=\"#tag--authentication\">Authentication</a> for how to log in. </div>  # Using the API  For simply exploring what the API can do it is strongly recommended to download [Insomnia](https://insomnia.rest/download), a free and open-source API client that's great for sending requests to the API in an orderly fashion. Insomnia allows you to send data in the format that's required for VRChat's API. It is also possible to try out the API in your browser, by first logging in at [vrchat.com/home](https://vrchat.com/home/) and then going to [vrchat.com/api/1/auth/user](https://vrchat.com/api/1/auth/user), but the information will be much harder to work with.  For more permanent operation such as software development it is instead recommended to use one of the existing language SDKs. This community project maintain API libraries in several languages, which allows you to interact with the API with simple function calls rather than having to implement the HTTP protocol yourself. Most of these libraries are automatically generated from the API specification, sometimes with additional helpful wrapper code to make usage easier. This allows them to be almost automatically updated and expanded upon as soon as a new feature is introduced in the specification itself. The libraries can be found on [GitHub](https://github.com/vrchatapi) or following:  * [NodeJS (JavaScript)](https://www.npmjs.com/package/vrchat) * [Dart](https://pub.dev/packages/vrchat_dart) * [Rust](https://crates.io/crates/vrchatapi) * [C#](github.com/vrchatapi/vrchatapi-csharp) * [Python](https://github.com/vrchatapi/VRChatPython)  # Pagination  Most endpoints enforce pagination, meaning they will only return 10 entries by default, and never more than 100.<br> Using both the limit and offset parameters allows you to easily paginate through a large number of objects.  | Query Parameter | Type | Description | | ----------|--|------- | | `limit` | integer  | The number of objects to return. This value often defaults to 10. Highest limit is always 100.| | `offset` | integer  | A zero-based offset from the default object sorting.|  If a request returns fewer objects than the `limit` parameter, there are no more items available to return.  # Contribution  Do you want to get involved in the documentation effort? Do you want to help improve one of the language API libraries? This project is an [OPEN Open Source Project](https://openopensource.org)! This means that individuals making significant and valuable contributions are given commit-access to the project. It also means we are very open and welcoming of new people making contributions, unlike some more guarded open source projects.  [![Discord](https://img.shields.io/static/v1?label=vrchatapi&message=discord&color=blueviolet&style=for-the-badge)](https://discord.gg/qjZE9C9fkB)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurrentUser {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "userIcon")]
    pub user_icon: String,
    #[serde(rename = "bio")]
    pub bio: String,
    #[serde(rename = "bioLinks")]
    pub bio_links: Vec<String>,
    #[serde(rename = "profilePicOverride")]
    pub profile_pic_override: String,
    #[serde(rename = "statusDescription")]
    pub status_description: String,
    #[serde(rename = "pastDisplayNames")]
    pub past_display_names: Vec<String>,
    #[serde(rename = "hasEmail")]
    pub has_email: bool,
    #[serde(rename = "hasPendingEmail")]
    pub has_pending_email: bool,
    #[serde(rename = "obfuscatedEmail")]
    pub obfuscated_email: String,
    #[serde(rename = "obfuscatedPendingEmail")]
    pub obfuscated_pending_email: String,
    #[serde(rename = "emailVerified")]
    pub email_verified: bool,
    #[serde(rename = "hasBirthday")]
    pub has_birthday: bool,
    #[serde(rename = "unsubscribe")]
    pub unsubscribe: bool,
    #[serde(rename = "statusHistory")]
    pub status_history: Vec<String>,
    #[serde(rename = "statusFirstTime")]
    pub status_first_time: bool,
    #[serde(rename = "friends")]
    pub friends: Vec<String>,
    /// Always empty array.
    #[serde(rename = "friendGroupNames")]
    pub friend_group_names: Vec<String>,
    #[serde(rename = "currentAvatarImageUrl")]
    pub current_avatar_image_url: String,
    #[serde(rename = "currentAvatarThumbnailImageUrl")]
    pub current_avatar_thumbnail_image_url: String,
    #[serde(rename = "fallbackAvatar")]
    pub fallback_avatar: String,
    #[serde(rename = "currentAvatar")]
    pub current_avatar: String,
    #[serde(rename = "currentAvatarAssetUrl")]
    pub current_avatar_asset_url: String,
    #[serde(rename = "accountDeletionDate", skip_serializing_if = "Option::is_none")]
    pub account_deletion_date: Option<String>,
    #[serde(rename = "acceptedTOSVersion")]
    pub accepted_tos_version: f32,
    #[serde(rename = "steamId")]
    pub steam_id: String,
    #[serde(rename = "steamDetails")]
    pub steam_details: serde_json::Value,
    #[serde(rename = "oculusId")]
    pub oculus_id: String,
    #[serde(rename = "hasLoggedInFromClient")]
    pub has_logged_in_from_client: bool,
    #[serde(rename = "homeLocation")]
    pub home_location: String,
    #[serde(rename = "twoFactorAuthEnabled")]
    pub two_factor_auth_enabled: bool,
    #[serde(rename = "state")]
    pub state: crate::models::UserState,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "developerType")]
    pub developer_type: crate::models::DeveloperType,
    #[serde(rename = "last_login")]
    pub last_login: String,
    #[serde(rename = "last_platform")]
    pub last_platform: crate::models::Platform,
    #[serde(rename = "allowAvatarCopying")]
    pub allow_avatar_copying: bool,
    #[serde(rename = "status")]
    pub status: crate::models::UserStatus,
    #[serde(rename = "date_joined")]
    pub date_joined: String,
    #[serde(rename = "isFriend")]
    pub is_friend: bool,
    #[serde(rename = "friendKey")]
    pub friend_key: String,
    #[serde(rename = "onlineFriends", skip_serializing_if = "Option::is_none")]
    pub online_friends: Option<Vec<String>>,
    #[serde(rename = "activeFriends", skip_serializing_if = "Option::is_none")]
    pub active_friends: Option<Vec<String>>,
    #[serde(rename = "offlineFriends", skip_serializing_if = "Option::is_none")]
    pub offline_friends: Option<Vec<String>>,
}

impl CurrentUser {
    pub fn new(id: String, username: String, display_name: String, user_icon: String, bio: String, bio_links: Vec<String>, profile_pic_override: String, status_description: String, past_display_names: Vec<String>, has_email: bool, has_pending_email: bool, obfuscated_email: String, obfuscated_pending_email: String, email_verified: bool, has_birthday: bool, unsubscribe: bool, status_history: Vec<String>, status_first_time: bool, friends: Vec<String>, friend_group_names: Vec<String>, current_avatar_image_url: String, current_avatar_thumbnail_image_url: String, fallback_avatar: String, current_avatar: String, current_avatar_asset_url: String, accepted_tos_version: f32, steam_id: String, steam_details: serde_json::Value, oculus_id: String, has_logged_in_from_client: bool, home_location: String, two_factor_auth_enabled: bool, state: crate::models::UserState, tags: Vec<String>, developer_type: crate::models::DeveloperType, last_login: String, last_platform: crate::models::Platform, allow_avatar_copying: bool, status: crate::models::UserStatus, date_joined: String, is_friend: bool, friend_key: String) -> CurrentUser {
        CurrentUser {
            id,
            username,
            display_name,
            user_icon,
            bio,
            bio_links,
            profile_pic_override,
            status_description,
            past_display_names,
            has_email,
            has_pending_email,
            obfuscated_email,
            obfuscated_pending_email,
            email_verified,
            has_birthday,
            unsubscribe,
            status_history,
            status_first_time,
            friends,
            friend_group_names,
            current_avatar_image_url,
            current_avatar_thumbnail_image_url,
            fallback_avatar,
            current_avatar,
            current_avatar_asset_url,
            account_deletion_date: None,
            accepted_tos_version,
            steam_id,
            steam_details,
            oculus_id,
            has_logged_in_from_client,
            home_location,
            two_factor_auth_enabled,
            state,
            tags,
            developer_type,
            last_login,
            last_platform,
            allow_avatar_copying,
            status,
            date_joined,
            is_friend,
            friend_key,
            online_friends: None,
            active_friends: None,
            offline_friends: None,
        }
    }
}


