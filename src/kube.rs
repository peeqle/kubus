use crate::smooth_operator::ClusterEntity;
use crate::smooth_operator::ClusterEntity::*;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::{Namespace, PersistentVolumeClaim, Pod, Service};
use k8s_openapi::serde::de::DeserializeOwned;
use k8s_openapi::{Metadata, NamespaceResourceScope};
use kube::api::{ListParams, ObjectList};
use kube::runtime::reflector::Lookup;
use kube::{Api, Client};
use log::error;
use regex::Regex;
use std::collections::HashMap;
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

pub async fn find_entity_by_name_like(reg: &str) -> HashMap<ClusterEntity, Vec<String>> {
    let mut map: HashMap<ClusterEntity, Vec<String>> = HashMap::from([
        (_Pod, vec![]),
        (_Deployment, vec![]),
        (_Service, vec![]),
        (_PersistentVolumeClaim, vec![]),
    ]);
    let regex = Regex::new(reg).unwrap();

    //need refactoring but hard(or impossible) with web of traits and bounds
    for namespace in find_all_namespaces().await.iter() {
        for typed in ClusterEntity::iterator() {
            if *typed == _Pod {
                let arc = find::<Pod>(&namespace.name().unwrap()).await;
                let guard = arc.lock().await;

                for ent in guard.iter() {
                    let pod_name = ent.clone().metadata.name.unwrap();
                    if regex.find(pod_name.as_str()).is_some() {
                        if let Some(vec) = map.get_mut(&_Pod) {
                            vec.push(pod_name);
                        } else {
                            map.insert(_Pod, vec![pod_name]);
                        }
                    }
                }
            }

            if *typed == _Service {
                let arc = find::<Service>(&namespace.name().unwrap()).await;
                let guard = arc.lock().await;

                for ent in guard.iter() {
                    let pod_name = ent.clone().metadata.name.unwrap();
                    if regex.find(pod_name.as_str()).is_some() {
                        if let Some(vec) = map.get_mut(&_Service) {
                            vec.push(pod_name);
                        } else {
                            map.insert(_Service, vec![pod_name]);
                        }
                    }
                }
            }

            if *typed == _Deployment {
                let arc = find::<Deployment>(&namespace.name().unwrap()).await;
                let guard = arc.lock().await;

                for ent in guard.iter() {
                    let pod_name = ent.clone().metadata.name.unwrap();
                    if regex.find(pod_name.as_str()).is_some() {
                        if let Some(vec) = map.get_mut(&_Deployment) {
                            vec.push(pod_name);
                        } else {
                            map.insert(_Deployment, vec![pod_name]);
                        }
                    }
                }
            }

            if *typed == _PersistentVolumeClaim {
                let arc = find::<PersistentVolumeClaim>(&namespace.name().unwrap()).await;
                let guard = arc.lock().await;

                for ent in guard.iter() {
                    let pod_name = ent.clone().metadata.name.unwrap();
                    if regex.find(pod_name.as_str()).is_some() {
                        if let Some(vec) = map.get_mut(&_PersistentVolumeClaim) {
                            vec.push(pod_name);
                        } else {
                            map.insert(_PersistentVolumeClaim, vec![pod_name]);
                        }
                    }
                }
            }
        }
    }

    map
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
