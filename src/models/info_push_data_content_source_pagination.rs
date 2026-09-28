use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushDataContentSourcePagination {
    #[serde(rename = "cursorParam")]
    pub cursor_param: String,
    #[serde(rename = "cursorResponseField")]
    pub cursor_response_field: String,
    #[serde(rename = "pageSize")]
    pub page_size: i32,
    #[serde(rename = "pageSizeParam")]
    pub page_size_param: String,
    #[serde(rename = "style")]
    pub style: String,
}

impl InfoPushDataContentSourcePagination {
    pub fn new(
        cursor_param: String,
        cursor_response_field: String,
        page_size: i32,
        page_size_param: String,
        style: String,
    ) -> InfoPushDataContentSourcePagination {
        InfoPushDataContentSourcePagination {
            cursor_param,
            cursor_response_field,
            page_size,
            page_size_param,
            style,
        }
    }
}
