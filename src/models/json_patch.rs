/*
 * Ory Hydra API
 *
 * Documentation for all of Ory Hydra's APIs. 
 *
 * The version of the OpenAPI document: v2.1.0
 * Contact: hi@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// JsonPatch : A JSONPatch document as defined by RFC 6902



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonPatch {
    /// This field is used together with operation \"move\" and uses JSON Pointer notation.  Learn more [about JSON Pointers](https://datatracker.ietf.org/doc/html/rfc6901#section-5).
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// The operation to be performed. One of \"add\", \"remove\", \"replace\", \"move\", \"copy\", or \"test\".
    #[serde(rename = "op")]
    pub op: String,
    /// The path to the target path. Uses JSON pointer notation.  Learn more [about JSON Pointers](https://datatracker.ietf.org/doc/html/rfc6901#section-5).
    #[serde(rename = "path")]
    pub path: String,
    /// The value to be used within the operations.  Learn more [about JSON Pointers](https://datatracker.ietf.org/doc/html/rfc6901#section-5).
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}


impl JsonPatch {
    /// A JSONPatch document as defined by RFC 6902
    pub fn new(op: String, path: String) -> JsonPatch {
        JsonPatch {
                from: None,
                op,
                path,
                value: None,
        }
    }
}


