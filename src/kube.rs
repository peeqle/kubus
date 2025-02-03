use k8s_openapi::api::core::v1::Namespace;
use k8s_openapi::serde::de::DeserializeOwned;
use k8s_openapi::{Metadata, NamespaceResourceScope};
use kube::api::{ListParams, ObjectList};
use kube::{Api, Client};
use log::error;
use regex::Regex;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::{Mutex, OnceCell};

static CLIENT: OnceCell<Client> = OnceCell::const_new();

pub async fn get_client() -> &'static Client {
    CLIENT
        .get_or_init(|| async {
            Client::try_default()
                .await
                .expect("Failed to create client")
        })
        .await
}

pub async fn find_entity_by_name_like(reg: &str) {
    let regex = Regex::new(reg).unwrap();
}

pub async fn find<
    T: Clone
        + DeserializeOwned
        + Debug
        + kube::Resource<DynamicType = (), Scope = NamespaceResourceScope>
        + Metadata,
>(
    namespace: &str,
) -> Arc<Mutex<Vec<T>>> {
    let list = Arc::new(Mutex::new(Vec::<T>::new()));

    if let Some(client) = CLIENT.get() {
        let api: Api<T> = Api::namespaced(client.clone(), namespace);

        let list_clone = Arc::clone(&list);
        match api.list(&ListParams::default()).await {
            Ok(ent_list) => {
                for ent in ent_list.items {
                    list_clone.lock().await.push(ent.clone());
                }
            }
            Err(e) => {
                error!("Error fetching resources: {}", e);
            }
        }
    } else {
        error!("Failed to get the client.");
    }

    list
}

pub async fn find_all_namespaces() -> ObjectList<Namespace> {
    Api::<Namespace>::all(Client::try_default().await.unwrap())
        .list(&Default::default())
        .await
        .expect("Fetch error")
}
