/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.33.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApiV2010AccountUsageUsageRecordUsageRecordAllTime {
    /// The SID of the Account accrued the usage
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The API version used to create the resource
    #[serde(rename = "api_version", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// Usage records up to date as of this timestamp
    #[serde(rename = "as_of", skip_serializing_if = "Option::is_none")]
    pub as_of: Option<String>,
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<crate::models::UsageRecordAllTimeEnumCategory>,
    /// The number of usage events
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<String>,
    /// The units in which count is measured
    #[serde(rename = "count_unit", skip_serializing_if = "Option::is_none")]
    pub count_unit: Option<String>,
    /// A plain-language description of the usage category
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The last date for which usage is included in the UsageRecord
    #[serde(rename = "end_date", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// The total price of the usage
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<f32>,
    /// The currency in which `price` is measured
    #[serde(rename = "price_unit", skip_serializing_if = "Option::is_none")]
    pub price_unit: Option<String>,
    /// The first date for which usage is included in this UsageRecord
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// A list of related resources identified by their relative URIs
    #[serde(rename = "subresource_uris", skip_serializing_if = "Option::is_none")]
    pub subresource_uris: Option<serde_json::Value>,
    /// The URI of the resource, relative to `https://api.twilio.com`
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    /// The amount of usage
    #[serde(rename = "usage", skip_serializing_if = "Option::is_none")]
    pub usage: Option<String>,
    /// The units in which usage is measured
    #[serde(rename = "usage_unit", skip_serializing_if = "Option::is_none")]
    pub usage_unit: Option<String>,
}

impl ApiV2010AccountUsageUsageRecordUsageRecordAllTime {
    pub fn new() -> ApiV2010AccountUsageUsageRecordUsageRecordAllTime {
        ApiV2010AccountUsageUsageRecordUsageRecordAllTime {
            account_sid: None,
            api_version: None,
            as_of: None,
            category: None,
            count: None,
            count_unit: None,
            description: None,
            end_date: None,
            price: None,
            price_unit: None,
            start_date: None,
            subresource_uris: None,
            uri: None,
            usage: None,
            usage_unit: None,
        }
    }
}


