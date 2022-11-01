use crate::{context::Context, error::DatabricksKubeError};

use databricks_rust_jobs::apis::configuration::Configuration;
use futures::Stream;
use futures::TryStreamExt;
use k8s_openapi::NamespaceResourceScope;

use kube::runtime::controller::Action;

use kube::Error;
use kube::{api::PostParams, Api, CustomResourceExt, Resource};
use serde::{de::DeserializeOwned, Serialize};
use std::hash::Hash;
use std::{fmt::Debug, pin::Pin, time::Duration};
use tokio::time::interval;

use futures::FutureExt;
use kube::ResourceExt;
use std::sync::Arc;

/// Generic sync task for pushing remote API resources into K8S
/// TAPIType is OpenAPI generated
/// TCRDType is the operator's wrapper
/// TDynamic is variable CRD metadata type for kube::Resource (varies)
async fn sync_task<TAPIType, TCRDType, TDynamic>(
    interval_period: Duration,
    context: Context,
) -> Result<(), DatabricksKubeError>
where
    TCRDType: From<TAPIType>,
    TCRDType: RemoteResource<TAPIType>,
    TCRDType: Resource<DynamicType = TDynamic, Scope = NamespaceResourceScope>,
    TCRDType: Default,
    TCRDType: Clone,
    TCRDType: CustomResourceExt,
    TCRDType: ResourceExt,
    TCRDType: Debug,
    TCRDType: DeserializeOwned,
    TCRDType: Send,
    TCRDType: Serialize,
    TCRDType: Sync,
    TDynamic: Default,
    TDynamic: 'static,
    TAPIType: 'static,
{
    let mut duration = interval(interval_period);
    let kube_api = Api::<TCRDType>::default_namespaced(context.client.clone());

    loop {
        duration.tick().await;

        let maybe_rest_config = context.make_jobs_rest_config().await;

        if maybe_rest_config.is_none() {
            log::info!("Waiting for REST configuration...");
            continue;
        }

        let rest_config = maybe_rest_config.unwrap();

        let mut resource_stream = TCRDType::remote_list_all(rest_config);

        while let Ok(Some(api_resource)) = resource_stream.try_next().await {
            let resource_as_kube: TCRDType = api_resource.into();
            let name = resource_as_kube.name_unchecked();
            let kube_resource = kube_api.get(&name).await;

            if kube_resource.is_err() {
                log::info!(
                    "{} missing, creating {}",
                    TCRDType::api_resource().kind,
                    name
                );
            }

            if let Ok(ref new_kube_resource) = kube_api
                .create(&PostParams::default(), &resource_as_kube)
                .await
            {
                log::info!(
                    "Created {} {}",
                    TCRDType::api_resource().kind,
                    new_kube_resource.name_unchecked(),
                );
            }
        }
    }
}

/// Implement this on the macroexpanded CRD type, against the SDK type
pub trait RemoteResource<TAPIType: 'static> {
    fn spawn_remote_sync_task<TDynamic>(
        context: Context,
    ) -> Pin<Box<dyn futures::Future<Output = Result<(), DatabricksKubeError>> + 'static>>
    where
        Self: From<TAPIType>,
        Self: Resource<DynamicType = TDynamic, Scope = NamespaceResourceScope>,
        Self: Default,
        Self: Clone,
        Self: CustomResourceExt,
        Self: Debug,
        Self: DeserializeOwned,
        Self: Send,
        Self: Serialize,
        Self: Sync,
        Self: 'static,
        TDynamic: Default,
        TDynamic: 'static,
        TAPIType: Send,
    {
        sync_task::<TAPIType, Self, TDynamic>(Duration::from_secs(60), context).boxed()
    }

    fn default_error_policy<TDynamic>(obj: Arc<Self>, err: &Error, _ctx: Arc<Context>) -> Action
    where
        Self: From<TAPIType>,
        Self: Resource<DynamicType = TDynamic, Scope = NamespaceResourceScope>,
        Self: Default,
        Self: Clone,
        Self: CustomResourceExt,
        Self: ResourceExt,
        Self: Debug,
        Self: DeserializeOwned,
        Self: Send,
        Self: Serialize,
        Self: Sync,
        Self: 'static,
        TDynamic: Default,
        TDynamic: Eq,
        TDynamic: Hash,
        TDynamic: 'static,
        TAPIType: Send,
    {
        log::error!(
            "Reconciliation failed for {} {} -- with error {} -- retrying in 30s",
            Self::api_resource().kind,
            err,
            obj.name_unchecked()
        );
        Action::requeue(Duration::from_secs(30))
    }

    fn remote_list_all(
        config: Configuration,
    ) -> Pin<Box<dyn Stream<Item = Result<TAPIType, DatabricksKubeError>> + Send>>;

    fn remote_get_self(
        &self,
        config: Configuration,
    ) -> Pin<Box<dyn Stream<Item = Result<TAPIType, DatabricksKubeError>> + Send>>;
}
