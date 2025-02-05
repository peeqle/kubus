use crate::kube::find_entity_by_name_like;
use crate::smooth_operator::ClusterEntity;
use crate::smooth_operator::Operation::FindNameLike;
use crate::smooth_operator::ParsedParameters;
use std::collections::HashMap;

pub async fn execute(params: Option<ParsedParameters>) {
    let output = match params {
        Some(op) => {
            let mut existing_entities: HashMap<ClusterEntity, Vec<String>> = HashMap::new();
            if op.operation_call.is_some() {
                let call = op.operation_call.unwrap();
                if call.operation == FindNameLike {
                    existing_entities = find_entity_by_name_like(&*call.value.unwrap()).await;
                    println!("{:?}", existing_entities.clone());
                }
            }
            //todo complete operations 
        }
        None => panic!("Cannot find operation."),
    };
}
