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

/// TerminationCode : * USER_REQUEST: A user terminated the cluster directly. Parameters should include a `username` field that indicates the specific user who terminated the cluster. * JOB_FINISHED: The cluster was launched by a job, and terminated when the job completed. * INACTIVITY: The cluster was terminated since it was idle. * CLOUD_PROVIDER_SHUTDOWN: The instance that hosted the Spark driver was terminated by the cloud provider. In AWS, for example, AWS may retire instances and directly shut them down. Parameters should include an `aws_instance_state_reason` field indicating the AWS-provided reason why the instance was terminated. * COMMUNICATION_LOST: Databricks lost connection to services on the driver instance. For example, this can happen when problems arise in cloud networking infrastructure, or when the instance itself becomes unhealthy. * CLOUD_PROVIDER_LAUNCH_FAILURE: Databricks experienced a cloud provider failure when requesting instances to launch clusters. For example, AWS limits the number of running instances and EBS volumes. If you ask Databricks to launch a cluster that requires instances or EBS volumes that exceed your AWS limit, the cluster fails with this status code. Parameters should include one of `aws_api_error_code`, `aws_instance_state_reason`, or `aws_spot_request_status` to indicate the AWS-provided reason why Databricks could not request the required instances for the cluster. * SPARK_STARTUP_FAILURE: The cluster failed to initialize. Possible reasons may include failure to create the environment for Spark or issues launching the Spark master and worker processes. * INVALID_ARGUMENT: Cannot launch the cluster because the user specified an invalid argument. For example, the user might specify an invalid runtime version for the cluster. * UNEXPECTED_LAUNCH_FAILURE: While launching this cluster, Databricks failed to complete critical setup steps, terminating the cluster. * INTERNAL_ERROR: Databricks encountered an unexpected error that forced the running cluster to be terminated. Contact Databricks support for additional details. * SPARK_ERROR: The Spark driver failed to start. Possible reasons may include incompatible libraries and initialization scripts that corrupted the Spark container. * METASTORE_COMPONENT_UNHEALTHY: The cluster failed to start because the external metastore could not be reached. Refer to [Troubleshooting](https://docs.databricks.com/data/metastores/external-hive-metastore.html#troubleshooting). * DBFS_COMPONENT_UNHEALTHY: The cluster failed to start because Databricks File System (DBFS) could not be reached.  * DRIVER_UNREACHABLE: Databricks was not able to access the Spark driver, because it was not reachable. * DRIVER_UNRESPONSIVE: Databricks was not able to access the Spark driver, because it was unresponsive. * INSTANCE_UNREACHABLE: Databricks was not able to access instances in order to start the cluster. This can be a transient networking issue. If the problem persists, this usually indicates a networking environment misconfiguration. * CONTAINER_LAUNCH_FAILURE: Databricks was unable to launch containers on worker nodes for the cluster. Have your admin check your network configuration. * INSTANCE_POOL_CLUSTER_FAILURE: Pool backed cluster specific failure. Refer to [Pools](https://docs.databricks.com/clusters/instance-pools/index.html) for details. * REQUEST_REJECTED: Databricks cannot handle the request at this moment. Try again later and contact Databricks if the problem persists. * INIT_SCRIPT_FAILURE: Databricks cannot load and run a cluster-scoped init script on one of the cluster’s nodes, or the init script terminates with a non-zero exit code. Refer to [Init script logs](https://docs.databricks.com/clusters/init-scripts.html#init-script-log). * TRIAL_EXPIRED: The Databricks trial subscription expired.

/// * USER_REQUEST: A user terminated the cluster directly. Parameters should include a `username` field that indicates the specific user who terminated the cluster. * JOB_FINISHED: The cluster was launched by a job, and terminated when the job completed. * INACTIVITY: The cluster was terminated since it was idle. * CLOUD_PROVIDER_SHUTDOWN: The instance that hosted the Spark driver was terminated by the cloud provider. In AWS, for example, AWS may retire instances and directly shut them down. Parameters should include an `aws_instance_state_reason` field indicating the AWS-provided reason why the instance was terminated. * COMMUNICATION_LOST: Databricks lost connection to services on the driver instance. For example, this can happen when problems arise in cloud networking infrastructure, or when the instance itself becomes unhealthy. * CLOUD_PROVIDER_LAUNCH_FAILURE: Databricks experienced a cloud provider failure when requesting instances to launch clusters. For example, AWS limits the number of running instances and EBS volumes. If you ask Databricks to launch a cluster that requires instances or EBS volumes that exceed your AWS limit, the cluster fails with this status code. Parameters should include one of `aws_api_error_code`, `aws_instance_state_reason`, or `aws_spot_request_status` to indicate the AWS-provided reason why Databricks could not request the required instances for the cluster. * SPARK_STARTUP_FAILURE: The cluster failed to initialize. Possible reasons may include failure to create the environment for Spark or issues launching the Spark master and worker processes. * INVALID_ARGUMENT: Cannot launch the cluster because the user specified an invalid argument. For example, the user might specify an invalid runtime version for the cluster. * UNEXPECTED_LAUNCH_FAILURE: While launching this cluster, Databricks failed to complete critical setup steps, terminating the cluster. * INTERNAL_ERROR: Databricks encountered an unexpected error that forced the running cluster to be terminated. Contact Databricks support for additional details. * SPARK_ERROR: The Spark driver failed to start. Possible reasons may include incompatible libraries and initialization scripts that corrupted the Spark container. * METASTORE_COMPONENT_UNHEALTHY: The cluster failed to start because the external metastore could not be reached. Refer to [Troubleshooting](https://docs.databricks.com/data/metastores/external-hive-metastore.html#troubleshooting). * DBFS_COMPONENT_UNHEALTHY: The cluster failed to start because Databricks File System (DBFS) could not be reached.  * DRIVER_UNREACHABLE: Databricks was not able to access the Spark driver, because it was not reachable. * DRIVER_UNRESPONSIVE: Databricks was not able to access the Spark driver, because it was unresponsive. * INSTANCE_UNREACHABLE: Databricks was not able to access instances in order to start the cluster. This can be a transient networking issue. If the problem persists, this usually indicates a networking environment misconfiguration. * CONTAINER_LAUNCH_FAILURE: Databricks was unable to launch containers on worker nodes for the cluster. Have your admin check your network configuration. * INSTANCE_POOL_CLUSTER_FAILURE: Pool backed cluster specific failure. Refer to [Pools](https://docs.databricks.com/clusters/instance-pools/index.html) for details. * REQUEST_REJECTED: Databricks cannot handle the request at this moment. Try again later and contact Databricks if the problem persists. * INIT_SCRIPT_FAILURE: Databricks cannot load and run a cluster-scoped init script on one of the cluster’s nodes, or the init script terminates with a non-zero exit code. Refer to [Init script logs](https://docs.databricks.com/clusters/init-scripts.html#init-script-log). * TRIAL_EXPIRED: The Databricks trial subscription expired.
#[derive(
    JsonSchema, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum TerminationCode {
    #[serde(rename = "USER_REQUEST")]
    UserRequest,
    #[serde(rename = "JOB_FINISHED")]
    JobFinished,
    #[serde(rename = "INACTIVITY")]
    Inactivity,
    #[serde(rename = "CLOUD_PROVIDER_SHUTDOWN")]
    CloudProviderShutdown,
    #[serde(rename = "COMMUNICATION_LOST")]
    CommunicationLost,
    #[serde(rename = "CLOUD_PROVIDER_LAUNCH_FAILURE")]
    CloudProviderLaunchFailure,
    #[serde(rename = "SPARK_STARTUP_FAILURE")]
    SparkStartupFailure,
    #[serde(rename = "INVALID_ARGUMENT")]
    InvalidArgument,
    #[serde(rename = "UNEXPECTED_LAUNCH_FAILURE")]
    UnexpectedLaunchFailure,
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError,
    #[serde(rename = "SPARK_ERROR")]
    SparkError,
    #[serde(rename = "METASTORE_COMPONENT_UNHEALTHY")]
    MetastoreComponentUnhealthy,
    #[serde(rename = "DBFS_COMPONENT_UNHEALTHY")]
    DbfsComponentUnhealthy,
    #[serde(rename = "DRIVER_UNREACHABLE")]
    DriverUnreachable,
    #[serde(rename = "DRIVER_UNRESPONSIVE")]
    DriverUnresponsive,
    #[serde(rename = "INSTANCE_UNREACHABLE")]
    InstanceUnreachable,
    #[serde(rename = "CONTAINER_LAUNCH_FAILURE")]
    ContainerLaunchFailure,
    #[serde(rename = "INSTANCE_POOL_CLUSTER_FAILURE")]
    InstancePoolClusterFailure,
    #[serde(rename = "REQUEST_REJECTED")]
    RequestRejected,
    #[serde(rename = "INIT_SCRIPT_FAILURE")]
    InitScriptFailure,
    #[serde(rename = "TRIAL_EXPIRED")]
    TrialExpired,
}

impl ToString for TerminationCode {
    fn to_string(&self) -> String {
        match self {
            Self::UserRequest => String::from("USER_REQUEST"),
            Self::JobFinished => String::from("JOB_FINISHED"),
            Self::Inactivity => String::from("INACTIVITY"),
            Self::CloudProviderShutdown => String::from("CLOUD_PROVIDER_SHUTDOWN"),
            Self::CommunicationLost => String::from("COMMUNICATION_LOST"),
            Self::CloudProviderLaunchFailure => String::from("CLOUD_PROVIDER_LAUNCH_FAILURE"),
            Self::SparkStartupFailure => String::from("SPARK_STARTUP_FAILURE"),
            Self::InvalidArgument => String::from("INVALID_ARGUMENT"),
            Self::UnexpectedLaunchFailure => String::from("UNEXPECTED_LAUNCH_FAILURE"),
            Self::InternalError => String::from("INTERNAL_ERROR"),
            Self::SparkError => String::from("SPARK_ERROR"),
            Self::MetastoreComponentUnhealthy => String::from("METASTORE_COMPONENT_UNHEALTHY"),
            Self::DbfsComponentUnhealthy => String::from("DBFS_COMPONENT_UNHEALTHY"),
            Self::DriverUnreachable => String::from("DRIVER_UNREACHABLE"),
            Self::DriverUnresponsive => String::from("DRIVER_UNRESPONSIVE"),
            Self::InstanceUnreachable => String::from("INSTANCE_UNREACHABLE"),
            Self::ContainerLaunchFailure => String::from("CONTAINER_LAUNCH_FAILURE"),
            Self::InstancePoolClusterFailure => String::from("INSTANCE_POOL_CLUSTER_FAILURE"),
            Self::RequestRejected => String::from("REQUEST_REJECTED"),
            Self::InitScriptFailure => String::from("INIT_SCRIPT_FAILURE"),
            Self::TrialExpired => String::from("TRIAL_EXPIRED"),
        }
    }
}

impl Default for TerminationCode {
    fn default() -> TerminationCode {
        Self::UserRequest
    }
}
