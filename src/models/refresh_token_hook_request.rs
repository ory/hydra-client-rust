/*
 * Ory Oathkeeper API
 *
 * Documentation for all of Ory Oathkeeper's APIs. 
 *
 * The version of the OpenAPI document: v1.11.4
 * Contact: hi@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RefreshTokenHookRequest {
    /// ClientID is the identifier of the OAuth 2.0 client.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// GrantedAudience is the list of audiences granted to the OAuth 2.0 client.
    #[serde(rename = "granted_audience", skip_serializing_if = "Option::is_none")]
    pub granted_audience: Option<Vec<String>>,
    /// GrantedScopes is the list of scopes granted to the OAuth 2.0 client.
    #[serde(rename = "granted_scopes", skip_serializing_if = "Option::is_none")]
    pub granted_scopes: Option<Vec<String>>,
    /// Subject is the identifier of the authenticated end-user.
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
}

impl RefreshTokenHookRequest {
    pub fn new() -> RefreshTokenHookRequest {
        RefreshTokenHookRequest {
            client_id: None,
            granted_audience: None,
            granted_scopes: None,
            subject: None,
        }
    }
}


