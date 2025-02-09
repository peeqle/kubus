use crate::smooth_operator::ClusterEntity::*;
use crate::smooth_operator::{ClusterEntity, IterableEnum, NamedArgument};
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::{Namespace, PersistentVolumeClaim, Pod, Service};
use k8s_openapi::serde::de::DeserializeOwned;
use k8s_openapi::{Metadata, NamespaceResourceScope};
use kube::api::{DeleteParams, ListParams, ObjectList};
use kube::core::Status;
use kube::runtime::reflector::Lookup;
use kube::{Api, Client, ResourceExt};
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
                .expect("Failed to create kubernetes client")
        })
        .await
}

pub async fn delete_entities(
    namespace: Option<String>,
    _type: ClusterEntity,
    names: Option<&Vec<String>>,
) {
    if let Some(name_) = namespace.clone() {
        println!("Operating at namespace: {:?}", name_.to_uppercase());
    }
    if let Some(names) = names {
        for name in names {
            delete_entity(name.clone(), namespace.clone(), _type.clone()).await;
        }
    } else {
        println!("No operational entities found to delete.");
    }
}

pub async fn delete_entity(
    name: String,
    namespace: Option<String>,
    _type: ClusterEntity,
) -> Option<Status> {
    if let Some(client) = CLIENT.get() {
        return match _type {
            _PersistentVolumeClaim => {
                let api;
                if let Some(names_) = namespace {
                    api = Api::<PersistentVolumeClaim>::namespaced(client.clone(), &names_);
                } else {
                    api = Api::<PersistentVolumeClaim>::all(client.clone());
                }
                let result = api.delete(&name, &DeleteParams::default()).await;
                println!("Deleting {} for name: {}", _type.name(), &name);
                result
                    .expect(format!("Cannot delete PersistentVolumeClaim {} ", &name).as_str())
                    .right()
            }
            _Deployment => {
                let api;
                if let Some(names_) = namespace {
                    api = Api::<Deployment>::namespaced(client.clone(), &names_);
                } else {
                    api = Api::<Deployment>::all(client.clone());
                }
                let result = api.delete(&name, &DeleteParams::default()).await;
                result
                    .expect(format!("Cannot delete Deployment {} ", &name).as_str())
                    .right()
            }
            _Service => {
                let api;
                if let Some(names_) = namespace {
                    api = Api::<Service>::namespaced(client.clone(), &names_);
                } else {
                    api = Api::<Service>::all(client.clone());
                }
                let result = api.delete(&name, &DeleteParams::default()).await;
                result
                    .expect(format!("Cannot delete Service {}", &name).as_str())
                    .right()
            }
            _Pod => {
                let api;
                if let Some(names_) = namespace {
                    api = Api::<Pod>::namespaced(client.clone(), &names_);
                } else {
                    api = Api::<Pod>::all(client.clone());
                }
                let result = api.delete(&name, &DeleteParams::default()).await;
                result
                    .expect(format!("Cannot delete Pod {} ", &name).as_str())
                    .right()
            }
        };
    }

    None
}

pub async fn find_entity_by_name_like(
    reg: &str,
    namespace: Option<String>,
) -> HashMap<ClusterEntity, Vec<String>> {
    let mut map: HashMap<ClusterEntity, Vec<String>> = HashMap::from([
        (_Pod, vec![]),
        (_Deployment, vec![]),
        (_Service, vec![]),
        (_PersistentVolumeClaim, vec![]),
    ]);

    if let Some(name_s) = namespace {
        find_entity_by_name_like_namespaced(reg, name_s, &mut map).await;
    } else {
        for namespace in find_all_namespaces().await.iter() {
            find_entity_by_name_like_namespaced(
                reg,
                namespace.name().unwrap().to_string(),
                &mut map,
            )
            .await;
        }
    }

    map
}

pub async fn find_entity_by_name_like_namespaced(
    reg: &str,
    namespace: String,
    map: &mut HashMap<ClusterEntity, Vec<String>>,
) {
    let regex = Regex::new(reg).unwrap();

    //need refactoring but hard(or impossible) with web of traits and bounds
    for typed in ClusterEntity::iterator() {
        if *typed == _Pod {
            let arc = find::<Pod>(&namespace).await;
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
            let arc = find::<Service>(&namespace).await;
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
            let arc = find::<Deployment>(&namespace).await;
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
            let arc = find::<PersistentVolumeClaim>(&namespace).await;
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
