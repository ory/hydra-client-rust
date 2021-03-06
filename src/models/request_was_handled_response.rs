/*
 * Ory Hydra API
 *
 * Documentation for all of Ory Hydra's APIs. 
 *
 * The version of the OpenAPI document: v1.11.8
 * Contact: hi@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestWasHandledResponse {
    /// Original request URL to which you should redirect the user if request was already handled.
    #[serde(rename = "redirect_to")]
    pub redirect_to: String,
}

impl RequestWasHandledResponse {
    pub fn new(redirect_to: String) -> RequestWasHandledResponse {
        RequestWasHandledResponse {
            redirect_to,
        }
    }
}


