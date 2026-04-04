use super::{configuration, ContentType, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{de::Error as _, Deserialize, Serialize};

/// struct for typed errors of method [`delete_jam_submission`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteJamSubmissionError {
    Status401(models::Error),
    Status403(models::Error),
    Status404(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_jam`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetJamError {
    Status404(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_jam_submissions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetJamSubmissionsError {
    Status404(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_jams`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetJamsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`submit_jam_content`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubmitJamContentError {
    Status401(models::Error),
    Status404(models::Error),
    UnknownValue(serde_json::Value),
}

/// Withdraws a content submission from a jam.
pub async fn delete_jam_submission(
    configuration: &configuration::Configuration,
    jam_id: &str,
    jam_submission_id: &str,
) -> Result<models::Success, Error<DeleteJamSubmissionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_jam_id = jam_id;
    let p_path_jam_submission_id = jam_submission_id;

    let uri_str = format!(
        "{}/jams/{jamId}/submissions/{jamSubmissionId}",
        configuration.base_path,
        jamId = crate::apis::urlencode(p_path_jam_id),
        jamSubmissionId = crate::apis::urlencode(p_path_jam_submission_id)
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Success`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Success`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteJamSubmissionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Returns a jam.
pub async fn get_jam(
    configuration: &configuration::Configuration,
    jam_id: &str,
) -> Result<models::Jam, Error<GetJamError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_jam_id = jam_id;

    let uri_str = format!(
        "{}/jams/{jamId}",
        configuration.base_path,
        jamId = crate::apis::urlencode(p_path_jam_id)
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Jam`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Jam`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetJamError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Returns all submissions of a jam. Can filter by contentId (for world or avatar jams) or submitterId (for a participant).
pub async fn get_jam_submissions(
    configuration: &configuration::Configuration,
    jam_id: &str,
    content_id: Option<&str>,
    submitter_id: Option<&str>,
) -> Result<Vec<models::JamSubmission>, Error<GetJamSubmissionsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_jam_id = jam_id;
    let p_query_content_id = content_id;
    let p_query_submitter_id = submitter_id;

    let uri_str = format!(
        "{}/jams/{jamId}/submissions",
        configuration.base_path,
        jamId = crate::apis::urlencode(p_path_jam_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_content_id {
        req_builder = req_builder.query(&[("contentId", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_submitter_id {
        req_builder = req_builder.query(&[("submitterId", &param_value.to_string())]);
    }
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::JamSubmission&gt;`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;models::JamSubmission&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetJamSubmissionsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Lists World Jams or Avatar Jams, both currently running and ones that have ended.  `isActive` is used to select only active or already ended jams.  `type` is used to select only world or avatar jams, and can only take `world` or `avatar`. ``
pub async fn get_jams(
    configuration: &configuration::Configuration,
    r#type: Option<&str>,
) -> Result<Vec<models::Jam>, Error<GetJamsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_type = r#type;

    let uri_str = format!("{}/jams", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_type {
        req_builder = req_builder.query(&[("type", &param_value.to_string())]);
    }
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::Jam&gt;`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;models::Jam&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetJamsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Submits content to a jam. The content must have been uploaded by the submitter, and both the content upload and jam submission must be made within the jam's designated times.
pub async fn submit_jam_content(
    configuration: &configuration::Configuration,
    jam_id: &str,
    create_jam_submission_request: Option<models::CreateJamSubmissionRequest>,
) -> Result<models::JamSubmission, Error<SubmitJamContentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_jam_id = jam_id;
    let p_body_create_jam_submission_request = create_jam_submission_request;

    let uri_str = format!(
        "{}/jams/{jamId}/submissions",
        configuration.base_path,
        jamId = crate::apis::urlencode(p_path_jam_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_body_create_jam_submission_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::JamSubmission`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::JamSubmission`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SubmitJamContentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
