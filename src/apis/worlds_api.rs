/*
 * VRChat API Documentation
 *
 * ![VRChat API Banner](https://vrchatapi.github.io/assets/img/api_banner_1500x400.png)  # Welcome to the VRChat API  Before we begin, we would like to state this is a **COMMUNITY DRIVEN PROJECT**. This means that everything you read on here was written by the community itself and is **not** officially supported by VRChat. The documentation is provided \"AS IS\", and any action you take towards VRChat is completely your own responsibility.  The documentation and additional libraries SHALL ONLY be used for applications interacting with VRChat's API in accordance with their [Terms of Service](https://github.com/VRChatAPI), and MUST NOT be used for modifying the client, \"avatar ripping\", or other illegal activities. Malicious usage or spamming the API may result in account termination. Certain parts of the API are also more sensitive than others, for example moderation, so please read tread extra carefully and read the warnings when present.  ![Tupper Policy on API](https://i.imgur.com/yLlW7Ok.png)  Finally, use of the API using applications other than the approved methods (website, VRChat application, Unity SDK) is not officially supported. VRChat provides no guarantee or support for external applications using the API. Access to API endpoints may break **at any time, without notice**. Therefore, please **do not ping** VRChat Staff in the VRChat Discord if you are having API problems, as they do not provide API support. We will make a best effort in keeping this documentation and associated language libraries up to date, but things might be outdated or missing. If you find that something is no longer valid, please contact us on Discord or [create an issue](https://github.com/vrchatapi/specification/issues) and tell us so we can fix it.  # Getting Started  The VRChat API can be used to programmatically retrieve or update information regarding your profile, friends, avatars, worlds and more. The API consists of two parts, \"Photon\" which is only used in-game, and the \"Web API\" which is used by both the game and the website. This documentation focuses only on the Web API.  The API is designed around the REST ideology, providing semi-simple and usually predictable URIs to access and modify objects. Requests support standard HTTP methods like GET, PUT, POST, and DELETE and standard status codes. Response bodies are always UTF-8 encoded JSON objects, unless explicitly documented otherwise.  <div class=\"callout callout-error\">   <strong>🛑 Warning! Do not touch Photon!</strong><br>   Photon is only used by the in-game client and should <b>not</b> be touched. Doing so may result in permanent account termination. </div>  <div class=\"callout callout-info\">   <strong>ℹ️ API Key and Authentication</strong><br>   The API Key has always been the same and is currently <code>JlE5Jldo5Jibnk5O5hTx6XVqsJu4WJ26</code>.   Read <a href=\"#tag--authentication\">Authentication</a> for how to log in. </div>  # Using the API  For simply exploring what the API can do it is strongly recommended to download [Insomnia](https://insomnia.rest/download), a free and open-source API client that's great for sending requests to the API in an orderly fashion. Insomnia allows you to send data in the format that's required for VRChat's API. It is also possible to try out the API in your browser, by first logging in at [vrchat.com/home](https://vrchat.com/home/) and then going to [vrchat.com/api/1/auth/user](https://vrchat.com/api/1/auth/user), but the information will be much harder to work with.  For more permanent operation such as software development it is instead recommended to use one of the existing language SDKs. This community project maintains API libraries in several languages, which allows you to interact with the API with simple function calls rather than having to implement the HTTP protocol yourself. Most of these libraries are automatically generated from the API specification, sometimes with additional helpful wrapper code to make usage easier. This allows them to be almost automatically updated and expanded upon as soon as a new feature is introduced in the specification itself. The libraries can be found on [GitHub](https://github.com/vrchatapi) or following:  * [NodeJS (JavaScript)](https://www.npmjs.com/package/vrchat) * [Dart](https://pub.dev/packages/vrchat_dart) * [Rust](https://crates.io/crates/vrchatapi) * [C#](github.com/vrchatapi/vrchatapi-csharp) * [Python](https://github.com/vrchatapi/VRChatPython)  # Pagination  Most endpoints enforce pagination, meaning they will only return 10 entries by default, and never more than 100.<br> Using both the limit and offset parameters allows you to easily paginate through a large number of objects.  | Query Parameter | Type | Description | | ----------|--|------- | | `limit` | integer  | The number of objects to return. This value often defaults to 10. Highest limit is always 100.| | `offset` | integer  | A zero-based offset from the default object sorting.|  If a request returns fewer objects than the `limit` parameter, there are no more items available to return.  # Contribution  Do you want to get involved in the documentation effort? Do you want to help improve one of the language API libraries? This project is an [OPEN Open Source Project](https://openopensource.org)! This means that individuals making significant and valuable contributions are given commit-access to the project. It also means we are very open and welcoming of new people making contributions, unlike some more guarded open-source projects.  [![Discord](https://img.shields.io/static/v1?label=vrchatapi&message=discord&color=blueviolet&style=for-the-badge)](https://discord.gg/qjZE9C9fkB)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `create_world`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateWorldError {
    Status400(crate::models::Error),
    Status401(crate::models::InlineResponse401),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_world`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteWorldError {
    Status401(crate::models::InlineResponse401),
    Status404(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_active_worlds`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetActiveWorldsError {
    Status401(crate::models::InlineResponse401),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_favorited_worlds`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFavoritedWorldsError {
    Status401(crate::models::InlineResponse401),
    Status403(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_recent_worlds`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRecentWorldsError {
    Status401(crate::models::InlineResponse401),
    Status403(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_world`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWorldError {
    Status404(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_world_instance`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWorldInstanceError {
    Status401(crate::models::InlineResponse401),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_world_metadata`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWorldMetadataError {
    Status404(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_world_publish_status`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWorldPublishStatusError {
    Status401(crate::models::InlineResponse401),
    Status404(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `publish_world`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PublishWorldError {
    Status401(crate::models::InlineResponse401),
    Status404(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `search_worlds`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchWorldsError {
    Status401(crate::models::InlineResponse401),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `unpublish_world`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnpublishWorldError {
    Status401(crate::models::InlineResponse401),
    Status404(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `update_world`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateWorldError {
    Status401(crate::models::InlineResponse401),
    Status404(crate::models::Error),
    UnknownValue(serde_json::Value),
}


/// Create a new world. This endpoint requires `assetUrl` to be a valid File object with `.vrcw` file extension, and `imageUrl` to be a valid File object with an image file extension.
pub fn create_world(configuration: &configuration::Configuration, inline_object6: Option<crate::models::InlineObject6>) -> Result<crate::models::World, Error<CreateWorldError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/worlds", configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&inline_object6);

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateWorldError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete a world. Notice a world is never fully \"deleted\", only its ReleaseStatus is set to \"hidden\" and the linked Files are deleted. The WorldID is permanently reserved.
pub fn delete_world(configuration: &configuration::Configuration, world_id: &str) -> Result<(), Error<DeleteWorldError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/worlds/{worldId}", configuration.base_path, worldId=crate::apis::urlencode(world_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteWorldError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Search and list currently Active worlds by query filters.
pub fn get_active_worlds(configuration: &configuration::Configuration, featured: Option<&str>, sort: Option<&str>, n: Option<i32>, order: Option<&str>, offset: Option<i32>, search: Option<&str>, tag: Option<&str>, notag: Option<&str>, release_status: Option<&str>, max_unity_version: Option<&str>, min_unity_version: Option<&str>, platform: Option<&str>) -> Result<Vec<crate::models::LimitedWorld>, Error<GetActiveWorldsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/worlds/active", configuration.base_path);
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
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetActiveWorldsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Search and list favorited worlds by query filters.
pub fn get_favorited_worlds(configuration: &configuration::Configuration, featured: Option<&str>, sort: Option<&str>, n: Option<i32>, order: Option<&str>, offset: Option<i32>, search: Option<&str>, tag: Option<&str>, notag: Option<&str>, release_status: Option<&str>, max_unity_version: Option<&str>, min_unity_version: Option<&str>, platform: Option<&str>, user_id: Option<&str>) -> Result<Vec<crate::models::LimitedWorld>, Error<GetFavoritedWorldsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/worlds/favorites", configuration.base_path);
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
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetFavoritedWorldsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Search and list recently visited worlds by query filters.
pub fn get_recent_worlds(configuration: &configuration::Configuration, featured: Option<&str>, sort: Option<&str>, n: Option<i32>, order: Option<&str>, offset: Option<i32>, search: Option<&str>, tag: Option<&str>, notag: Option<&str>, release_status: Option<&str>, max_unity_version: Option<&str>, min_unity_version: Option<&str>, platform: Option<&str>, user_id: Option<&str>) -> Result<Vec<crate::models::LimitedWorld>, Error<GetRecentWorldsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/worlds/recent", configuration.base_path);
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
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetRecentWorldsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get information about a specific World.
pub fn get_world(configuration: &configuration::Configuration, world_id: &str) -> Result<crate::models::World, Error<GetWorldError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/worlds/{worldId}", configuration.base_path, worldId=crate::apis::urlencode(world_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetWorldError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a worlds instance.
pub fn get_world_instance(configuration: &configuration::Configuration, world_id: &str, instance_id: &str) -> Result<crate::models::Instance, Error<GetWorldInstanceError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/worlds/{worldId}/{instanceId}", configuration.base_path, worldId=crate::apis::urlencode(world_id), instanceId=crate::apis::urlencode(instance_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetWorldInstanceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Return a worlds custom metadata. This is currently believed to be unused. Metadata can be set with `updateWorld` and can be any arbitrary object.
pub fn get_world_metadata(configuration: &configuration::Configuration, world_id: &str) -> Result<crate::models::InlineResponse2006, Error<GetWorldMetadataError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/worlds/{worldId}/metadata", configuration.base_path, worldId=crate::apis::urlencode(world_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetWorldMetadataError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a worlds publish status.
pub fn get_world_publish_status(configuration: &configuration::Configuration, world_id: &str) -> Result<crate::models::InlineResponse2007, Error<GetWorldPublishStatusError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/worlds/{worldId}/publish", configuration.base_path, worldId=crate::apis::urlencode(world_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetWorldPublishStatusError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Publish a world. You can only publish one world per week.
pub fn publish_world(configuration: &configuration::Configuration, world_id: &str) -> Result<(), Error<PublishWorldError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/worlds/{worldId}/publish", configuration.base_path, worldId=crate::apis::urlencode(world_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<PublishWorldError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Search and list any worlds by query filters.
pub fn search_worlds(configuration: &configuration::Configuration, featured: Option<&str>, sort: Option<&str>, user: Option<&str>, user_id: Option<&str>, n: Option<i32>, order: Option<&str>, offset: Option<i32>, search: Option<&str>, tag: Option<&str>, notag: Option<&str>, release_status: Option<&str>, max_unity_version: Option<&str>, min_unity_version: Option<&str>, platform: Option<&str>) -> Result<Vec<crate::models::LimitedWorld>, Error<SearchWorldsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/worlds", configuration.base_path);
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
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SearchWorldsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Unpublish a world.
pub fn unpublish_world(configuration: &configuration::Configuration, world_id: &str) -> Result<(), Error<UnpublishWorldError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/worlds/{worldId}/publish", configuration.base_path, worldId=crate::apis::urlencode(world_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<UnpublishWorldError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update information about a specific World.
pub fn update_world(configuration: &configuration::Configuration, world_id: &str, inline_object7: Option<crate::models::InlineObject7>) -> Result<crate::models::World, Error<UpdateWorldError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/worlds/{worldId}", configuration.base_path, worldId=crate::apis::urlencode(world_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&inline_object7);

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateWorldError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

