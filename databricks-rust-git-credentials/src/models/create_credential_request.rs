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
pub struct CreateCredentialRequest {
    /// The personal access token used to authenticate to the corresponding Git provider.
    #[serde(rename = "personal_access_token")]
    pub personal_access_token: String,
    /// Git username.
    #[serde(rename = "git_username")]
    pub git_username: String,
    /// Git provider. This field is case-insensitive. The available Git providers are awsCodeCommit, azureDevOpsServices, bitbucketCloud, bitbucketServer, gitHub, gitHubEnterprise, gitLab, and gitLabEnterpriseEdition.
    #[serde(rename = "git_provider")]
    pub git_provider: String,
}

impl CreateCredentialRequest {
    pub fn new(personal_access_token: String, git_username: String, git_provider: String) -> CreateCredentialRequest {
        CreateCredentialRequest {
            personal_access_token,
            git_username,
            git_provider,
        }
    }
}


