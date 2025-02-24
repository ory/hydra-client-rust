/*
 * Ory Hydra API
 *
 * Documentation for all of Ory Hydra's APIs. 
 *
 * The version of the OpenAPI document: v2.4.0-alpha.1
 * Contact: hi@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VerifiableCredentialResponse {
    #[serde(rename = "credential_draft_00", skip_serializing_if = "Option::is_none")]
    pub credential_draft_00: Option<String>,
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

impl VerifiableCredentialResponse {
    pub fn new() -> VerifiableCredentialResponse {
        VerifiableCredentialResponse {
            credential_draft_00: None,
            format: None,
        }
    }
}

