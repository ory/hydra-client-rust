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
pub struct InlineResponse2001 {
    /// The version of Ory Hydra.
    #[serde(rename = "version")]
    pub version: String,
}

impl InlineResponse2001 {
    pub fn new(version: String) -> InlineResponse2001 {
        InlineResponse2001 {
            version,
        }
    }
}


