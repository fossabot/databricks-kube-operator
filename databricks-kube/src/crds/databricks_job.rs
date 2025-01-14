use std::pin::Pin;
use std::sync::Arc;
use std::time::SystemTime;

use crate::{
    context::Context, error::DatabricksKubeError, traits::rest_config::RestConfig,
    traits::synced_api_resource::SyncedAPIResource,
};

use databricks_rust_jobs::{
    apis::{configuration::Configuration, default_api},
    models::{
        job::Job, job_settings, jobs_create_request, JobsCreate200Response, JobsCreateRequest,
        JobsDeleteRequest, JobsGet200Response, JobsList200Response, JobsUpdateRequest,
    },
};

use async_stream::try_stream;
use futures::{Stream, StreamExt, TryFutureExt};
use k8s_openapi::serde::{Deserialize, Serialize};
use kube::{core::object::HasSpec, CustomResource, ResourceExt};
use schemars::JsonSchema;

#[derive(Clone, CustomResource, Debug, Default, Deserialize, PartialEq, Serialize, JsonSchema)]
#[kube(
    group = "com.dstancu.databricks",
    version = "v1",
    kind = "DatabricksJob",
    derive = "Default",
    namespaced
)]
pub struct DatabricksJobSpec {
    pub job: Job,
}

/// API -> CRD
impl From<Job> for DatabricksJob {
    fn from(job: Job) -> Self {
        let k8s_name = job
            .settings
            .iter()
            .flat_map(|s| s.name.clone())
            .next()
            .unwrap_or(format!(
                "noname-{}",
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            ));

        Self::new(&k8s_name, DatabricksJobSpec { job })
    }
}

/// CRD -> API
impl From<DatabricksJob> for Job {
    fn from(value: DatabricksJob) -> Self {
        value.spec().job.clone()
    }
}

impl SyncedAPIResource<Job, Configuration> for DatabricksJob {
    fn remote_list_all(
        context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<Job, DatabricksKubeError>> + Send>> {
        try_stream! {
            let mut offset: i32 = 0;
            let config = Job::get_rest_config(context.clone()).await.unwrap();

            while let JobsList200Response {
                jobs,
                has_more,
                ..
            } = default_api::jobs_list(&config, None, Some(offset), Some(true)).await? {
                if let Some(jobs) = jobs {
                    offset = jobs.len() as i32;

                    for job in jobs {
                        yield job;
                    }
                }

                let more = has_more.unwrap_or(false);
                if !more { break; }
            }
        }
        .boxed()
    }

    fn remote_get(
        &self,
        context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<Job, DatabricksKubeError>> + Send>> {
        let job_id = self.spec().job.job_id;

        try_stream! {
            let config = Job::get_rest_config(context.clone()).await.unwrap();

            let JobsGet200Response {
                job_id,
                creator_user_name,
                settings,
                created_time,
                ..
            } = default_api::jobs_get(&config, job_id).map_err(
                |e| DatabricksKubeError::APIError(e.to_string())
            ).await?;

            yield Job {
                job_id,
                creator_user_name,
                settings,
                created_time
            }
        }
        .boxed()
    }

    fn remote_create(
        &self,
        context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<Self, DatabricksKubeError>> + Send + '_>>
    where
        Self: Sized,
    {
        let job = self.spec().job.clone();
        let job_settings = job.settings.as_ref().unwrap().clone();

        try_stream! {
            let config = Job::get_rest_config(context.clone()).await.unwrap();

            let JobsCreate200Response { job_id } = default_api::jobs_create(
                &config,

                /// TODO: unsupported atm
                // access_control_list: job.access_control_list
                Some(JobsCreateRequest {
                    name: Some(self.name_unchecked()),
                    tags: job_settings.tags,
                    tasks: job_settings.tasks,
                    job_clusters: job_settings.job_clusters,
                    email_notifications: job_settings.email_notifications,
                    timeout_seconds: job_settings.timeout_seconds,
                    schedule: job_settings.schedule,
                    max_concurrent_runs: job_settings.max_concurrent_runs,
                    git_source: job_settings.git_source,
                    format: job_settings.format.map(job_settings_to_create_format),
                    ..JobsCreateRequest::default()
                })
            ).map_err(
                |e| DatabricksKubeError::APIError(e.to_string())
            ).await?;

            // The response only contains an ID, but there are other fields
            // that are API populated which we need, so read again.
            let created: Self = Job { job_id, ..job }.into();
            let created_job = created.remote_get(context.clone()).next().await.unwrap()?;

            let mut with_response = self.clone();
            with_response.spec.job = Job { job_id, ..created_job };
            yield with_response;
        }
        .boxed()
    }

    fn remote_update(
        &self,
        context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<Self, DatabricksKubeError>> + Send + '_>>
    where
        Self: Sized,
    {
        let job = self.spec().job.clone();
        let job_settings = job.settings.as_ref().cloned();

        let job_id = self.spec().job.job_id;

        try_stream! {
            let config = Job::get_rest_config(context.clone()).await.unwrap();

            default_api::jobs_update(
                &config,

                /// TODO: unsupported atm
                // access_control_list: job.access_control_list
                Some(JobsUpdateRequest {
                    job_id: job_id.unwrap(),
                    new_settings: job_settings,
                    ..JobsUpdateRequest::default()
                })
            ).map_err(
                |e| DatabricksKubeError::APIError(e.to_string())
            ).await?;

            let mut with_response = self.clone();
            with_response.spec.job = self.remote_get(context.clone()).next().await.unwrap()?;
            yield with_response;
        }
        .boxed()
    }

    fn remote_delete(
        &self,
        context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<(), DatabricksKubeError>> + Send + '_>> {
        let job_id = self.spec().job.job_id;

        try_stream! {
            let config = Job::get_rest_config(context.clone()).await.unwrap();
            default_api::jobs_delete(
                &config,
                Some(JobsDeleteRequest { job_id: job_id.unwrap(), })
            ).map_err(
                |e| DatabricksKubeError::APIError(e.to_string())
            ).await?;

            yield ()
        }
        .boxed()
    }
}

fn job_settings_to_create_format(value: job_settings::Format) -> jobs_create_request::Format {
    match value {
        job_settings::Format::MultiTask => jobs_create_request::Format::MultiTask,
        job_settings::Format::SingleTask => jobs_create_request::Format::SingleTask,
    }
}
