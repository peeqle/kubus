use crate::smooth_operator::ClusterEntity::*;
use crate::smooth_operator::DefaultClusterOperations::*;
use crate::smooth_operator::*;
use clap::{value_parser, Arg, ArgAction, Command};
use std::fmt::Debug;
use DefaultClusterOperations::Namespace;
use SpecifiedClusterOperations::{List, PrettyList};

pub fn build_args() -> Option<ParsedParameters> {
    let command = Command::new("kubus")
        .version("0.0.1")
        .author("peeqle")
        .arg(create_default_arg_item(&Namespace))
        .arg(create_default_arg_item(&Delete))
        .arg(create_default_arg_item(&Create))
        .arg(create_default_arg_item(&List))
        .arg(create_default_arg_item(&PrettyList));

    let matches = append_entity_context(&command).get_matches();

    Some(ParsedParameters {
        entity_call: ClusterEntity::find_operation_match(&matches),
        argument_call: DefaultClusterOperations::find_operation_match(&matches),
    })
}

fn append_entity_context(command: &Command) -> Command {
    command
        .clone()
        .arg(
            Arg::new(_PersistentVolumeClaim.name())
                .long(_PersistentVolumeClaim.name())
                .action(ArgAction::Count)
                .required(false)
                .help("Cluster default operation"),
        )
        .arg(
            Arg::new(_Service.name())
                .long(_Service.name())
                .action(ArgAction::Count)
                .required(false)
                .help("Cluster default operation"),
        )
        .arg(
            Arg::new(_Pod.name())
                .long(_Pod.name())
                .action(ArgAction::Count)
                .required(false)
                .help("Cluster default operation"),
        )
        .arg(
            Arg::new(_Deployment.name())
                .long(_Deployment.name())
                .action(ArgAction::Count)
                .required(false)
                .help("Cluster default operation"),
        )
}

pub fn create_default_arg_item<T: NamedArgument>(entity: &'static T) -> Arg {
    let name = entity.name().clone();
    Arg::new(name)
        .value_parser(value_parser!(String))
        .long(name)
        .action(ArgAction::Set)
        .required(false)
        .help(format!("Cluster default {} operation", entity.name()))
}
