use super::{configuration, ContentType, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{de::Error as _, Deserialize, Serialize};

/// struct for typed errors of method [`get_balance_earnings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBalanceEarningsError {
    Status401(models::Error),
    Status404(models::RouteNotImplemented),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_prop_publish_status`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPropPublishStatusError {
    Status401(models::Error),
    Status403(models::Error),
    Status404(models::RouteNotImplemented),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_seller_eligibility`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSellerEligibilityError {
    Status401(models::Error),
    Status404(models::RouteNotImplemented),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_tilia_status`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTiliaStatusError {
    Status401(models::Error),
    Status404(models::RouteNotImplemented),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_tilia_tos`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTiliaTosError {
    Status401(models::Error),
    Status404(models::RouteNotImplemented),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_user_credits_eligible`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserCreditsEligibleError {
    Status401(models::Error),
    Status404(models::RouteNotImplemented),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`publish_prop`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PublishPropError {
    Status401(models::Error),
    Status403(models::Error),
    Status404(models::RouteNotImplemented),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`unpublish_prop`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnpublishPropError {
    Status401(models::Error),
    Status403(models::Error),
    Status404(models::RouteNotImplemented),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_tilia_tos`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateTiliaTosError {
    Status401(models::Error),
    Status404(models::RouteNotImplemented),
    UnknownValue(serde_json::Value),
}

/// Return the user's balance from earnings.
#[deprecated]
pub async fn get_balance_earnings(
    configuration: &configuration::Configuration,
    user_id: &str,
) -> Result<models::Balance, Error<GetBalanceEarningsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_user_id = user_id;

    let uri_str = format!(
        "{}/user/{userId}/balance/earnings",
        configuration.base_path,
        userId = crate::apis::urlencode(p_path_user_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Balance`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Balance`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetBalanceEarningsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Return the PropPublishStatus object. `/props/{propId}` is still served.
#[deprecated]
pub async fn get_prop_publish_status(
    configuration: &configuration::Configuration,
    prop_id: &str,
) -> Result<models::PropPublishStatus, Error<GetPropPublishStatusError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_prop_id = prop_id;

    let uri_str = format!(
        "{}/props/{propId}/publish",
        configuration.base_path,
        propId = crate::apis::urlencode(p_path_prop_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PropPublishStatus`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PropPublishStatus`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetPropPublishStatusError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Return the current user's eligibility to become a seller.
#[deprecated]
pub async fn get_seller_eligibility(
    configuration: &configuration::Configuration,
) -> Result<models::SellerEligibility, Error<GetSellerEligibilityError>> {
    let uri_str = format!("{}/economy/seller/eligibility", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SellerEligibility`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SellerEligibility`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetSellerEligibilityError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Return the Tilia integration status.
#[deprecated]
pub async fn get_tilia_status(
    configuration: &configuration::Configuration,
) -> Result<models::TiliaStatus, Error<GetTiliaStatusError>> {
    let uri_str = format!("{}/tilia/status", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::TiliaStatus`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::TiliaStatus`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetTiliaStatusError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Return the user's Tilia TOS agreement status.
#[deprecated]
pub async fn get_tilia_tos(
    configuration: &configuration::Configuration,
    user_id: &str,
) -> Result<models::TiliaTos, Error<GetTiliaTosError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_user_id = user_id;

    let uri_str = format!(
        "{}/user/{userId}/tilia/tos",
        configuration.base_path,
        userId = crate::apis::urlencode(p_path_user_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::TiliaTos`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::TiliaTos`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetTiliaTosError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Return the user's subscription credit eligibility.
#[deprecated]
pub async fn get_user_credits_eligible(
    configuration: &configuration::Configuration,
    user_id: &str,
    subscription_id: &str,
) -> Result<models::UserCreditsEligible, Error<GetUserCreditsEligibleError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_user_id = user_id;
    let p_query_subscription_id = subscription_id;

    let uri_str = format!(
        "{}/users/{userId}/credits/eligible",
        configuration.base_path,
        userId = crate::apis::urlencode(p_path_user_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("subscriptionId", &p_query_subscription_id.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::UserCreditsEligible`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::UserCreditsEligible`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetUserCreditsEligibleError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Publish a prop and return the updated PropPublishStatus object. `/props/{propId}` is still served.
#[deprecated]
pub async fn publish_prop(
    configuration: &configuration::Configuration,
    prop_id: &str,
) -> Result<models::PropPublishStatus, Error<PublishPropError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_prop_id = prop_id;

    let uri_str = format!(
        "{}/props/{propId}/publish",
        configuration.base_path,
        propId = crate::apis::urlencode(p_path_prop_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PropPublishStatus`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PropPublishStatus`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PublishPropError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Unpublish a prop and return the updated PropPublishStatus object. `/props/{propId}` is still served.
#[deprecated]
pub async fn unpublish_prop(
    configuration: &configuration::Configuration,
    prop_id: &str,
) -> Result<models::PropPublishStatus, Error<UnpublishPropError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_prop_id = prop_id;

    let uri_str = format!(
        "{}/props/{propId}/publish",
        configuration.base_path,
        propId = crate::apis::urlencode(p_path_prop_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PropPublishStatus`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PropPublishStatus`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UnpublishPropError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Update the user's Tilia TOS agreement status.
#[deprecated]
pub async fn update_tilia_tos(
    configuration: &configuration::Configuration,
    user_id: &str,
    update_tilia_tos_request: Option<models::UpdateTiliaTosRequest>,
) -> Result<serde_json::Value, Error<UpdateTiliaTosError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_user_id = user_id;
    let p_body_update_tilia_tos_request = update_tilia_tos_request;

    let uri_str = format!(
        "{}/user/{userId}/tilia/tos",
        configuration.base_path,
        userId = crate::apis::urlencode(p_path_user_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_body_update_tilia_tos_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `serde_json::Value`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `serde_json::Value`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateTiliaTosError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
