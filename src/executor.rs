use crate::kube::find_entity_by_name_like;
use crate::smooth_operator::NamedArgument;
use crate::smooth_operator::Operation::FindNameLike;
use crate::smooth_operator::{ArgumentCall, ParsedParameters};
use crate::KUBERNETES_RUNNER;
use std::process::Command;

pub async fn execute(params: Option<ParsedParameters>) {
    let output = match params {
        Some(op) => {
            if op.operation_call.is_some() {
                let call = op.operation_call.unwrap();
                if call.operation == FindNameLike {
                    println!("{:?}", find_entity_by_name_like(&*call.value.unwrap()).await);
                }
                return;
            }
            let mut command = Command::new(KUBERNETES_RUNNER);
            let mut current_argument_call: Option<ArgumentCall> = op.argument_call;

            while let Some(ref current_cl) = current_argument_call {
                command.arg(current_cl.initial.name());

                if let Some(ref cs) = op.entity_call {
                    command.arg(cs.name());
                }

                if let Some(ref value) = current_cl.value {
                    command.arg(value);
                }

                current_argument_call = current_cl.trailing.as_deref().cloned();
            }
            println!("Executing script {:?}", command);
            command.output().expect("failed to execute process")
        }
        None => panic!("Cannot find operation."),
    };

    println!("{:?}", String::from_utf8(output.stderr));
    println!("{:?}", String::from_utf8(output.stdout));
    println!("{:?}", output.status);
}
