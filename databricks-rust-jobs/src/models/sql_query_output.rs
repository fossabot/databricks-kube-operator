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
pub struct SqlQueryOutput {
    /// The text of the SQL query. Can Run permission of the SQL query is required to view this field.
    #[serde(rename = "query_text", skip_serializing_if = "Option::is_none")]
    pub query_text: Option<String>,
    /// The canonical identifier of the SQL warehouse.
    #[serde(rename = "warehouse_id", skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
    #[serde(rename = "sql_statements", skip_serializing_if = "Option::is_none")]
    pub sql_statements: Option<Box<crate::models::SqlStatementOutput>>,
    /// The link to find the output results.
    #[serde(rename = "output_link", skip_serializing_if = "Option::is_none")]
    pub output_link: Option<String>,
}

impl SqlQueryOutput {
    pub fn new() -> SqlQueryOutput {
        SqlQueryOutput {
            query_text: None,
            warehouse_id: None,
            sql_statements: None,
            output_link: None,
        }
    }
}


