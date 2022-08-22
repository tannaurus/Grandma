/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.33.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CallEnumStatus {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "ringing")]
    Ringing,
    #[serde(rename = "in-progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "busy")]
    Busy,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "no-answer")]
    NoAnswer,
    #[serde(rename = "canceled")]
    Canceled,

}

impl ToString for CallEnumStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Queued => String::from("queued"),
            Self::Ringing => String::from("ringing"),
            Self::InProgress => String::from("in-progress"),
            Self::Completed => String::from("completed"),
            Self::Busy => String::from("busy"),
            Self::Failed => String::from("failed"),
            Self::NoAnswer => String::from("no-answer"),
            Self::Canceled => String::from("canceled"),
        }
    }
}

impl Default for CallEnumStatus {
    fn default() -> CallEnumStatus {
        Self::Queued
    }
}




