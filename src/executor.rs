use crate::kube::{delete_entitties, find_entity_by_name_like};
use crate::smooth_operator::ClusterEntity;
use crate::smooth_operator::DefaultClusterOperations::Delete;
use crate::smooth_operator::Operation::FindNameLike;
use crate::smooth_operator::ParsedParameters;
use std::collections::HashMap;

pub async fn execute(params: Option<ParsedParameters>) {
    let output = match params {
        Some(op) => {
            let mut existing_entities: HashMap<ClusterEntity, Vec<String>> = HashMap::new();
            let namespace = op.namespace;

            if op.operation_call.is_some() {
                let call = op.operation_call.unwrap();
                if call.operation == FindNameLike {
                    existing_entities =
                        find_entity_by_name_like(&*call.value.unwrap(), namespace.clone()).await;
                    println!("{:?}", existing_entities.clone());
                }
            }

            if let Some(argument) = op.argument_call {
                if argument.initial == Delete {
                    let entity = op.entity_call.unwrap();

                    delete_entitties(
                        namespace.clone().unwrap_or("default".parse().unwrap()),
                        entity.clone(),
                        existing_entities.get(&entity),
                    ).await;
                }
            }
            //todo complete operations
        }
        None => panic!("Cannot find operation."),
    };
}
