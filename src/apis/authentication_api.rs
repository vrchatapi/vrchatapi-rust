/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`check_user_exists`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CheckUserExistsError {
    Status400(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteUserError {
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_current_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCurrentUserError {
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`logout`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LogoutError {
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`verify2_fa`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Verify2FaError {
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`verify2_fa_email_code`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Verify2FaEmailCodeError {
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`verify_auth_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VerifyAuthTokenError {
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`verify_recovery_code`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VerifyRecoveryCodeError {
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}


/// Checks if a user by a given `username`, `displayName` or `email` exist. This is used during registration to check if a username has already been taken, during change of displayName to check if a displayName is available, and during change of email to check if the email is already used. In the later two cases the `excludeUserId` is used to exclude oneself, otherwise the result would always be true.  It is **REQUIRED** to include **AT LEAST** `username`, `displayName` **or** `email` query parameter. Although they can be combined - in addition with `excludeUserId` (generally to exclude yourself) - to further fine-tune the search.
pub async fn check_user_exists(configuration: &configuration::Configuration, email: Option<&str>, display_name: Option<&str>, user_id: Option<&str>, exclude_user_id: Option<&str>) -> Result<models::UserExists, Error<CheckUserExistsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/auth/exists", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = email {
        local_var_req_builder = local_var_req_builder.query(&[("email", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = display_name {
        local_var_req_builder = local_var_req_builder.query(&[("displayName", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_id {
        local_var_req_builder = local_var_req_builder.query(&[("userId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = exclude_user_id {
        local_var_req_builder = local_var_req_builder.query(&[("excludeUserId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CheckUserExistsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes the account with given ID. Normal users only have permission to delete their own account. Account deletion is 14 days from this request, and will be cancelled if you do an authenticated request with the account afterwards.  **VRC+ NOTE:** Despite the 14-days cooldown, any VRC+ subscription will be cancelled **immediately**.  **METHOD NOTE:** Despite this being a Delete action, the method type required is PUT.
pub async fn delete_user(configuration: &configuration::Configuration, user_id: &str) -> Result<models::CurrentUser, Error<DeleteUserError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/users/{userId}/delete", local_var_configuration.base_path, userId=crate::apis::urlencode(user_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeleteUserError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This endpoint does the following two operations:   1) Checks if you are already logged in by looking for a valid `auth` cookie. If you are have a valid auth cookie then no additional auth-related actions are taken. If you are **not** logged in then it will log you in with the `Authorization` header and set the `auth` cookie. The `auth` cookie will only be sent once.   2) If logged in, this function will also return the CurrentUser object containing detailed information about the currently logged in user.  The auth string after `Authorization: Basic {string}` is a base64-encoded string of the username and password, both individually url-encoded, and then joined with a colon.    > base64(urlencode(username):urlencode(password))  **WARNING: Session Limit:** Each authentication with login credentials counts as a separate session, out of which you have a limited amount. Make sure to save and reuse the `auth` cookie if you are often restarting the program. The provided API libraries automatically save cookies during runtime, but does not persist during restart. While it can be fine to use username/password during development, expect in production to very fast run into the rate-limit and be temporarily blocked from making new sessions until older ones expire. The exact number of simultaneous sessions is unknown/undisclosed.
pub async fn get_current_user(configuration: &configuration::Configuration, ) -> Result<models::EitherUserOrTwoFactor, Error<GetCurrentUserError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/auth/user", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetCurrentUserError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Invalidates the login session.
pub async fn logout(configuration: &configuration::Configuration, ) -> Result<models::Success, Error<LogoutError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/logout", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<LogoutError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Finishes the login sequence with a normal 2FA-generated code for accounts with 2FA-protection enabled.
pub async fn verify2_fa(configuration: &configuration::Configuration, two_factor_auth_code: models::TwoFactorAuthCode) -> Result<models::Verify2FaResult, Error<Verify2FaError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/auth/twofactorauth/totp/verify", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&two_factor_auth_code);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<Verify2FaError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Finishes the login sequence with an 2FA email code.
pub async fn verify2_fa_email_code(configuration: &configuration::Configuration, two_factor_email_code: models::TwoFactorEmailCode) -> Result<models::Verify2FaEmailCodeResult, Error<Verify2FaEmailCodeError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/auth/twofactorauth/emailotp/verify", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&two_factor_email_code);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<Verify2FaEmailCodeError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Verify whether the currently provided Auth Token is valid.
pub async fn verify_auth_token(configuration: &configuration::Configuration, ) -> Result<models::VerifyAuthTokenResult, Error<VerifyAuthTokenError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/auth", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<VerifyAuthTokenError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Finishes the login sequence with an OTP (One Time Password) recovery code for accounts with 2FA-protection enabled.
pub async fn verify_recovery_code(configuration: &configuration::Configuration, two_factor_auth_code: models::TwoFactorAuthCode) -> Result<models::Verify2FaResult, Error<VerifyRecoveryCodeError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/auth/twofactorauth/otp/verify", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&two_factor_auth_code);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<VerifyRecoveryCodeError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

