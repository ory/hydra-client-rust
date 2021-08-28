/*
 * ORY Hydra
 *
 * Welcome to the ORY Hydra HTTP API documentation. You will find documentation for all HTTP APIs here.
 *
 * The version of the OpenAPI document: v1.10.6
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PluginSettings {
    /// args
    #[serde(rename = "Args")]
    pub args: Vec<String>,
    /// devices
    #[serde(rename = "Devices")]
    pub devices: Vec<crate::models::PluginDevice>,
    /// env
    #[serde(rename = "Env")]
    pub env: Vec<String>,
    /// mounts
    #[serde(rename = "Mounts")]
    pub mounts: Vec<crate::models::PluginMount>,
}

impl PluginSettings {
    pub fn new(args: Vec<String>, devices: Vec<crate::models::PluginDevice>, env: Vec<String>, mounts: Vec<crate::models::PluginMount>) -> PluginSettings {
        PluginSettings {
            args,
            devices,
            env,
            mounts,
        }
    }
}


