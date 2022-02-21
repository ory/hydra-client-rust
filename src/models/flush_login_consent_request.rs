/*
 * Ory Oathkeeper API
 *
 * Documentation for all of Ory Oathkeeper's APIs. 
 *
 * The version of the OpenAPI document: v1.11.5
 * Contact: hi@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlushLoginConsentRequest {
    /// NotAfter sets after which point tokens should not be flushed. This is useful when you want to keep a history of recent login and consent database entries for auditing.
    #[serde(rename = "notAfter", skip_serializing_if = "Option::is_none")]
    pub not_after: Option<String>,
}

impl FlushLoginConsentRequest {
    pub fn new() -> FlushLoginConsentRequest {
        FlushLoginConsentRequest {
            not_after: None,
        }
    }
}


