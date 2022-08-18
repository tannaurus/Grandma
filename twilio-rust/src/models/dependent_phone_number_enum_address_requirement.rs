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
pub enum DependentPhoneNumberEnumAddressRequirement {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "foreign")]
    Foreign,

}

impl ToString for DependentPhoneNumberEnumAddressRequirement {
    fn to_string(&self) -> String {
        match self {
            Self::None => String::from("none"),
            Self::Any => String::from("any"),
            Self::Local => String::from("local"),
            Self::Foreign => String::from("foreign"),
        }
    }
}

impl Default for DependentPhoneNumberEnumAddressRequirement {
    fn default() -> DependentPhoneNumberEnumAddressRequirement {
        Self::None
    }
}




