use crate::{config::Config, error::DatabricksKubeError};

use databricks_rust_jobs::{
    apis::{configuration::Configuration},
};
use futures::{Stream};
use futures::{TryStreamExt};
use k8s_openapi::{NamespaceResourceScope};
use kube::{
    api::PostParams,
    Api, CustomResourceExt, Resource,
};
use serde::{de::DeserializeOwned, Serialize};
use std::{fmt::Debug, pin::Pin, time::Duration};
use tokio::{task::JoinHandle, time::interval};

async fn sync_task<TAPIType, TCRDType, TDynamic>(
    interval_period: Duration,
    config: Config,
) -> Result<(), DatabricksKubeError>
where
    TCRDType: From<TAPIType>,
    TCRDType: RemoteResource<TAPIType>,
    TCRDType: Resource<DynamicType = TDynamic, Scope = NamespaceResourceScope>,
    TCRDType: Default,
    TCRDType: Clone,
    TCRDType: CustomResourceExt,
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
    let kube_api = Api::<TCRDType>::default_namespaced(config.client.clone());

    loop {
        duration.tick().await;

        let maybe_creds = config.get_databricks_url_token().await;
        if maybe_creds.is_none() {
            continue;
        }

        let (url, token) = maybe_creds.unwrap();

        let databricks_config = Configuration {
            base_path: url,
            bearer_access_token: Some(token),
            ..Configuration::default()
        };

        let mut resource_stream = TCRDType::remote_list_all(databricks_config);

        while let Ok(Some(api_resource)) = resource_stream.try_next().await {
            let resource_as_kube: TCRDType = api_resource.into();
            let name = resource_as_kube.meta().name.clone().unwrap();
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
                    new_kube_resource.meta().name.clone().unwrap(),
                );
            }
        }
    }
}

/// Implement this on the macroexpanded CRD type, against the SDK type
pub trait RemoteResource<TAPIType: 'static> {
    fn task_sync_remote_to_kube<TDynamic>(
        config: Config,
    ) -> JoinHandle<Result<(), DatabricksKubeError>>
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
        tokio::spawn(sync_task::<TAPIType, Self, TDynamic>(
            Duration::from_secs(60),
            config,
        ))
    }

    fn remote_list_all(
        config: Configuration,
    ) -> Pin<Box<dyn Stream<Item = Result<TAPIType, DatabricksKubeError>> + Send>>;
}