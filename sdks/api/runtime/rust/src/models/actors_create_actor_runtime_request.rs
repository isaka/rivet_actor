/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ActorsCreateActorRuntimeRequest {
    #[serde(rename = "environment", skip_serializing_if = "Option::is_none")]
    pub environment: Option<::std::collections::HashMap<String, String>>,
}

impl ActorsCreateActorRuntimeRequest {
    pub fn new() -> ActorsCreateActorRuntimeRequest {
        ActorsCreateActorRuntimeRequest {
            environment: None,
        }
    }
}


