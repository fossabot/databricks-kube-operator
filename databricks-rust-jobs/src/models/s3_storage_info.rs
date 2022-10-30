use schemars::JsonSchema;
/*
 * Jobs API 2.1
 *
 * The Jobs API allows you to create, edit, and delete jobs. You should never hard code secrets or store them in plain text. Use the [Secrets API](https://docs.databricks.com/dev-tools/api/latest/secrets.html) to manage secrets in the [Databricks CLI](https://docs.databricks.com/dev-tools/cli/index.html). Use the [Secrets utility](https://docs.databricks.com/dev-tools/databricks-utils.html#dbutils-secrets) to reference secrets in notebooks and jobs.
 *
 * The version of the OpenAPI document: 2.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(JsonSchema, Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct S3StorageInfo {
    /// S3 destination. For example: `s3://my-bucket/some-prefix` You must configure the cluster with an instance profile and the instance profile must have write access to the destination. You _cannot_ use AWS keys.
    #[serde(rename = "destination", skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// S3 region. For example: `us-west-2`. Either region or endpoint must be set. If both are set, endpoint is used.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// S3 endpoint. For example: `https://s3-us-west-2.amazonaws.com`. Either region or endpoint must be set. If both are set, endpoint is used.
    #[serde(rename = "endpoint", skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// (Optional)Enable server side encryption, `false` by default.
    #[serde(rename = "enable_encryption", skip_serializing_if = "Option::is_none")]
    pub enable_encryption: Option<bool>,
    /// (Optional) The encryption type, it could be `sse-s3` or `sse-kms`. It is used only when encryption is enabled and the default type is `sse-s3`.
    #[serde(rename = "encryption_type", skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    /// (Optional) KMS key used if encryption is enabled and encryption type is set to `sse-kms`.
    #[serde(rename = "kms_key", skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
    /// (Optional) Set canned access control list. For example: `bucket-owner-full-control`. If canned_acl is set, the cluster instance profile must have `s3:PutObjectAcl` permission on the destination bucket and prefix. The full list of possible canned ACLs can be found at <https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html#canned-acl>. By default only the object owner gets full control. If you are using cross account role for writing data, you may want to set `bucket-owner-full-control` to make bucket owner able to read the logs.
    #[serde(rename = "canned_acl", skip_serializing_if = "Option::is_none")]
    pub canned_acl: Option<String>,
}

impl S3StorageInfo {
    pub fn new() -> S3StorageInfo {
        S3StorageInfo {
            destination: None,
            region: None,
            endpoint: None,
            enable_encryption: None,
            encryption_type: None,
            kms_key: None,
            canned_acl: None,
        }
    }
}


