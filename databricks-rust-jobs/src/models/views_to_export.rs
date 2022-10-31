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

/// ViewsToExport : * `CODE`: Code view of the notebook. * `DASHBOARDS`: All dashboard views of the notebook. * `ALL`: All views of the notebook.

/// * `CODE`: Code view of the notebook. * `DASHBOARDS`: All dashboard views of the notebook. * `ALL`: All views of the notebook.
#[derive(
    JsonSchema, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum ViewsToExport {
    #[serde(rename = "CODE")]
    Code,
    #[serde(rename = "DASHBOARDS")]
    Dashboards,
    #[serde(rename = "ALL")]
    All,
}

impl ToString for ViewsToExport {
    fn to_string(&self) -> String {
        match self {
            Self::Code => String::from("CODE"),
            Self::Dashboards => String::from("DASHBOARDS"),
            Self::All => String::from("ALL"),
        }
    }
}

impl Default for ViewsToExport {
    fn default() -> ViewsToExport {
        Self::Code
    }
}
