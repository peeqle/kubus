mod args;
mod kube;
mod smooth_operator;

use crate::args::*;
use std::process::Command;
use crate::smooth_operator::{ArgumentCall, NamedArgument};

static KUBERNETES_RUNNER: &str = "kubectl";

#[tokio::main]
async fn main() {
    kube::get_client().await;
    //ask kubernetes about cluster state

    let output = match build_args() {
        Some(mut op) => {
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
