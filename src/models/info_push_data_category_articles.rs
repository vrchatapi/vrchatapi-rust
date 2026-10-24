use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushDataCategoryArticles {
    #[serde(flatten)]
    pub info_push_data_category_base: models::InfoPushDataCategoryBase,
    #[serde(rename = "ipsQuery")]
    pub ips_query: models::InfoPushIpsQuery,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl InfoPushDataCategoryArticles {
    pub fn new(
        info_push_data_category_base: models::InfoPushDataCategoryBase,
        ips_query: models::InfoPushIpsQuery,
        r#type: Type,
    ) -> InfoPushDataCategoryArticles {
        InfoPushDataCategoryArticles {
            info_push_data_category_base,
            ips_query,
            r#type,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "articles")]
    Articles,
}

impl Default for Type {
    fn default() -> Type {
        Self::Articles
    }
}
