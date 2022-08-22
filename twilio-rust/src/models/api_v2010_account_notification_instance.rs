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
pub struct ApiV2010AccountNotificationInstance {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The API version used to generate the notification
    #[serde(rename = "api_version", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// The SID of the Call the resource is associated with
    #[serde(rename = "call_sid", skip_serializing_if = "Option::is_none")]
    pub call_sid: Option<String>,
    /// The RFC 2822 date and time in GMT that the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The RFC 2822 date and time in GMT that the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// A unique error code corresponding to the notification
    #[serde(rename = "error_code", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// An integer log level
    #[serde(rename = "log", skip_serializing_if = "Option::is_none")]
    pub log: Option<String>,
    /// The date the notification was generated
    #[serde(rename = "message_date", skip_serializing_if = "Option::is_none")]
    pub message_date: Option<String>,
    /// The text of the notification
    #[serde(rename = "message_text", skip_serializing_if = "Option::is_none")]
    pub message_text: Option<String>,
    /// A URL for more information about the error code
    #[serde(rename = "more_info", skip_serializing_if = "Option::is_none")]
    pub more_info: Option<String>,
    /// HTTP method used with the request url
    #[serde(rename = "request_method", skip_serializing_if = "Option::is_none")]
    pub request_method: Option<RequestMethod>,
    /// URL of the resource that generated the notification
    #[serde(rename = "request_url", skip_serializing_if = "Option::is_none")]
    pub request_url: Option<String>,
    /// Twilio-generated HTTP variables sent to the server
    #[serde(rename = "request_variables", skip_serializing_if = "Option::is_none")]
    pub request_variables: Option<String>,
    /// The HTTP body returned by your server
    #[serde(rename = "response_body", skip_serializing_if = "Option::is_none")]
    pub response_body: Option<String>,
    /// The HTTP headers returned by your server
    #[serde(rename = "response_headers", skip_serializing_if = "Option::is_none")]
    pub response_headers: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The URI of the resource, relative to `https://api.twilio.com`
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

impl ApiV2010AccountNotificationInstance {
    pub fn new() -> ApiV2010AccountNotificationInstance {
        ApiV2010AccountNotificationInstance {
            account_sid: None,
            api_version: None,
            call_sid: None,
            date_created: None,
            date_updated: None,
            error_code: None,
            log: None,
            message_date: None,
            message_text: None,
            more_info: None,
            request_method: None,
            request_url: None,
            request_variables: None,
            response_body: None,
            response_headers: None,
            sid: None,
            uri: None,
        }
    }
}

/// HTTP method used with the request url
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RequestMethod {
    #[serde(rename = "HEAD")]
    HEAD,
    #[serde(rename = "GET")]
    GET,
    #[serde(rename = "POST")]
    POST,
    #[serde(rename = "PATCH")]
    PATCH,
    #[serde(rename = "PUT")]
    PUT,
    #[serde(rename = "DELETE")]
    DELETE,
}

impl Default for RequestMethod {
    fn default() -> RequestMethod {
        Self::HEAD
    }
}

