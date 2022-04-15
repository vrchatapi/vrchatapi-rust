/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ariesclark.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApiEventConfig {
    /// Unknown
    #[serde(rename = "distanceClose")]
    pub distance_close: i32,
    /// Unknown
    #[serde(rename = "distanceFactor")]
    pub distance_factor: i32,
    /// Unknown
    #[serde(rename = "distanceFar")]
    pub distance_far: i32,
    /// Unknown
    #[serde(rename = "groupDistance")]
    pub group_distance: i32,
    /// Unknown
    #[serde(rename = "maximumBunchSize")]
    pub maximum_bunch_size: i32,
    /// Unknown
    #[serde(rename = "notVisibleFactor")]
    pub not_visible_factor: i32,
    /// Unknown
    #[serde(rename = "playerOrderBucketSize")]
    pub player_order_bucket_size: i32,
    /// Unknown
    #[serde(rename = "playerOrderFactor")]
    pub player_order_factor: i32,
    /// Unknown
    #[serde(rename = "slowUpdateFactorThreshold")]
    pub slow_update_factor_threshold: i32,
    /// Unknown
    #[serde(rename = "viewSegmentLength")]
    pub view_segment_length: i32,
}

impl ApiEventConfig {
    pub fn new(distance_close: i32, distance_factor: i32, distance_far: i32, group_distance: i32, maximum_bunch_size: i32, not_visible_factor: i32, player_order_bucket_size: i32, player_order_factor: i32, slow_update_factor_threshold: i32, view_segment_length: i32) -> ApiEventConfig {
        ApiEventConfig {
            distance_close,
            distance_factor,
            distance_far,
            group_distance,
            maximum_bunch_size,
            not_visible_factor,
            player_order_bucket_size,
            player_order_factor,
            slow_update_factor_threshold,
            view_segment_length,
        }
    }
}


