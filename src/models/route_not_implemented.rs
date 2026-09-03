use crate::models;
use serde::{Deserialize, Serialize};

use serde_repr::{Deserialize_repr, Serialize_repr};

/// RouteNotImplemented : The body VRChat returns for a route it does not serve. The shape differs from every other error in this description: `error` is a string here, not an `Error` object with `message` and `status_code` inside it.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RouteNotImplemented {
    #[serde(rename = "error")]
    pub error: Error,
    #[serde(rename = "status_code")]
    pub status_code: StatusCode,
}

impl RouteNotImplemented {
    /// The body VRChat returns for a route it does not serve. The shape differs from every other error in this description: `error` is a string here, not an `Error` object with `message` and `status_code` inside it.
    pub fn new(error: Error, status_code: StatusCode) -> RouteNotImplemented {
        RouteNotImplemented { error, status_code }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Error {
    #[serde(rename = "The endpoint you're looking for is not implemented by our system.")]
    TheEndpointYouQuoteReLookingForIsNotImplementedByOurSystem,
}

impl Default for Error {
    fn default() -> Error {
        Self::TheEndpointYouQuoteReLookingForIsNotImplementedByOurSystem
    }
}
#[repr(i64)]
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize_repr, Deserialize_repr,
)]
pub enum StatusCode {
    Variant404 = 404,
}

impl std::fmt::Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Variant404 => "404",
            }
        )
    }
}

impl Default for StatusCode {
    fn default() -> StatusCode {
        Self::Variant404
    }
}
