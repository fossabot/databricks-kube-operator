use schemars::JsonSchema;
/*
 * Git Credentials API
 *
 * The Git credentials API allows users to manage their [Git credentials](https://docs.databricks.com/repos.html#configure-your-git-integration-with-databricks) to use [Databricks Repos](https://docs.databricks.com/repos.html).
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(JsonSchema, Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Error {
    /// Error code
    #[serde(rename = "error_code", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// Human-readable error message describing the cause of the error.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl Error {
    pub fn new() -> Error {
        Error {
            error_code: None,
            message: None,
        }
    }
}

