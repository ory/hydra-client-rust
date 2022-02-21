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
pub struct RefreshTokenHookResponse {
    #[serde(rename = "session", skip_serializing_if = "Option::is_none")]
    pub session: Option<Box<crate::models::ConsentRequestSession>>,
}

impl RefreshTokenHookResponse {
    pub fn new() -> RefreshTokenHookResponse {
        RefreshTokenHookResponse {
            session: None,
        }
    }
}


