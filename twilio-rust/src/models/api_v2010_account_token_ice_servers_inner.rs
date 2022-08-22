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
pub struct ApiV2010AccountTokenIceServersInner {
    #[serde(rename = "credential", skip_serializing_if = "Option::is_none")]
    pub credential: Option<String>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "urls", skip_serializing_if = "Option::is_none")]
    pub urls: Option<String>,
}

impl ApiV2010AccountTokenIceServersInner {
    pub fn new() -> ApiV2010AccountTokenIceServersInner {
        ApiV2010AccountTokenIceServersInner {
            credential: None,
            username: None,
            url: None,
            urls: None,
        }
    }
}


