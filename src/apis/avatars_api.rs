/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`create_avatar`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAvatarError {
    Status401(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_avatar`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAvatarError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_avatar`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAvatarError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_favorited_avatars`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFavoritedAvatarsError {
    Status401(crate::models::Error),
    Status403(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_own_avatar`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOwnAvatarError {
    Status401(crate::models::Error),
    Status403(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`search_avatars`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchAvatarsError {
    Status401(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`select_avatar`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SelectAvatarError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`select_fallback_avatar`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SelectFallbackAvatarError {
    Status401(crate::models::Error),
    Status403(crate::models::Error),
    Status404(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_avatar`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAvatarError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    UnknownValue(serde_json::Value),
}


/// Create an avatar. It's possible to optionally specify a ID if you want a custom one. Attempting to create an Avatar with an already claimed ID will result in a DB error.
pub fn create_avatar(configuration: &configuration::Configuration, create_avatar_request: Option<crate::models::CreateAvatarRequest>) -> Result<crate::models::Avatar, Error<CreateAvatarError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/avatars", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&create_avatar_request);

    let local_var_req = local_var_req_builder.build()?;
    #[allow(unused_mut)] let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateAvatarError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete an avatar. Notice an avatar is never fully \"deleted\", only its ReleaseStatus is set to \"hidden\" and the linked Files are deleted. The AvatarID is permanently reserved.
pub fn delete_avatar(configuration: &configuration::Configuration, avatar_id: &str) -> Result<crate::models::Avatar, Error<DeleteAvatarError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/avatars/{avatarId}", local_var_configuration.base_path, avatarId=crate::apis::urlencode(avatar_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    #[allow(unused_mut)] let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeleteAvatarError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get information about a specific Avatar.
pub fn get_avatar(configuration: &configuration::Configuration, avatar_id: &str) -> Result<crate::models::Avatar, Error<GetAvatarError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/avatars/{avatarId}", local_var_configuration.base_path, avatarId=crate::apis::urlencode(avatar_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    #[allow(unused_mut)] let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetAvatarError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Search and list favorited avatars by query filters.
pub fn get_favorited_avatars(configuration: &configuration::Configuration, featured: Option<bool>, sort: Option<crate::models::SortOption>, n: Option<i32>, order: Option<crate::models::OrderOption>, offset: Option<i32>, search: Option<&str>, tag: Option<&str>, notag: Option<&str>, release_status: Option<crate::models::ReleaseStatus>, max_unity_version: Option<&str>, min_unity_version: Option<&str>, platform: Option<&str>, user_id: Option<&str>) -> Result<Vec<crate::models::Avatar>, Error<GetFavoritedAvatarsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/avatars/favorites", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = featured {
        local_var_req_builder = local_var_req_builder.query(&[("featured", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = n {
        local_var_req_builder = local_var_req_builder.query(&[("n", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = order {
        local_var_req_builder = local_var_req_builder.query(&[("order", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = search {
        local_var_req_builder = local_var_req_builder.query(&[("search", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = tag {
        local_var_req_builder = local_var_req_builder.query(&[("tag", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = notag {
        local_var_req_builder = local_var_req_builder.query(&[("notag", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = release_status {
        local_var_req_builder = local_var_req_builder.query(&[("releaseStatus", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_unity_version {
        local_var_req_builder = local_var_req_builder.query(&[("maxUnityVersion", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = min_unity_version {
        local_var_req_builder = local_var_req_builder.query(&[("minUnityVersion", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = platform {
        local_var_req_builder = local_var_req_builder.query(&[("platform", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_id {
        local_var_req_builder = local_var_req_builder.query(&[("userId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    #[allow(unused_mut)] let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetFavoritedAvatarsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get the current avatar for the user. This will return an error for any other user than the one logged in.
pub fn get_own_avatar(configuration: &configuration::Configuration, user_id: &str) -> Result<crate::models::Avatar, Error<GetOwnAvatarError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/users/{userId}/avatar", local_var_configuration.base_path, userId=crate::apis::urlencode(user_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    #[allow(unused_mut)] let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetOwnAvatarError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Search and list avatars by query filters. You can only search your own or featured avatars. It is not possible as a normal user to search other peoples avatars.
pub fn search_avatars(configuration: &configuration::Configuration, featured: Option<bool>, sort: Option<crate::models::SortOption>, user: Option<&str>, user_id: Option<&str>, n: Option<i32>, order: Option<crate::models::OrderOption>, offset: Option<i32>, tag: Option<&str>, notag: Option<&str>, release_status: Option<crate::models::ReleaseStatus>, max_unity_version: Option<&str>, min_unity_version: Option<&str>, platform: Option<&str>) -> Result<Vec<crate::models::Avatar>, Error<SearchAvatarsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/avatars", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = featured {
        local_var_req_builder = local_var_req_builder.query(&[("featured", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user {
        local_var_req_builder = local_var_req_builder.query(&[("user", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_id {
        local_var_req_builder = local_var_req_builder.query(&[("userId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = n {
        local_var_req_builder = local_var_req_builder.query(&[("n", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = order {
        local_var_req_builder = local_var_req_builder.query(&[("order", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = tag {
        local_var_req_builder = local_var_req_builder.query(&[("tag", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = notag {
        local_var_req_builder = local_var_req_builder.query(&[("notag", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = release_status {
        local_var_req_builder = local_var_req_builder.query(&[("releaseStatus", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_unity_version {
        local_var_req_builder = local_var_req_builder.query(&[("maxUnityVersion", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = min_unity_version {
        local_var_req_builder = local_var_req_builder.query(&[("minUnityVersion", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = platform {
        local_var_req_builder = local_var_req_builder.query(&[("platform", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    #[allow(unused_mut)] let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SearchAvatarsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Switches into that avatar.
pub fn select_avatar(configuration: &configuration::Configuration, avatar_id: &str) -> Result<crate::models::CurrentUser, Error<SelectAvatarError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/avatars/{avatarId}/select", local_var_configuration.base_path, avatarId=crate::apis::urlencode(avatar_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    #[allow(unused_mut)] let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SelectAvatarError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Switches into that avatar as your fallback avatar.
pub fn select_fallback_avatar(configuration: &configuration::Configuration, avatar_id: &str) -> Result<crate::models::CurrentUser, Error<SelectFallbackAvatarError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/avatars/{avatarId}/selectFallback", local_var_configuration.base_path, avatarId=crate::apis::urlencode(avatar_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    #[allow(unused_mut)] let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SelectFallbackAvatarError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update information about a specific avatar.
pub fn update_avatar(configuration: &configuration::Configuration, avatar_id: &str, update_avatar_request: Option<crate::models::UpdateAvatarRequest>) -> Result<crate::models::Avatar, Error<UpdateAvatarError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/avatars/{avatarId}", local_var_configuration.base_path, avatarId=crate::apis::urlencode(avatar_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&update_avatar_request);

    let local_var_req = local_var_req_builder.build()?;
    #[allow(unused_mut)] let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateAvatarError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

