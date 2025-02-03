use crate::smooth_operator::*;
use clap::{value_parser, Arg, ArgAction, Command};
use std::fmt::Debug;
use DefaultClusterOperations::Namespace;
use SpecifiedClusterOperations::{List, PrettyList};
use crate::smooth_operator::ClusterEntity::*;
use crate::smooth_operator::DefaultClusterOperations::*;

pub fn build_args() -> Option<ParsedParameters> {
    let command = Command::new("kubus")
        .version("0.0.1")
        .author("peeqle")
        .arg(
            Arg::new(Namespace.name())
                .value_parser(value_parser!(String))
                .short(Namespace.micro_name())
                .long(Namespace.name())
                .action(ArgAction::Set)
                .required(false)
                .help("Namespace for cluster operation, default is 'default'"),
        )
        .arg(
            Arg::new(Delete.name())
                .value_parser(value_parser!(String))
                .short(Delete.micro_name())
                .long(Delete.name())
                .action(ArgAction::Set)
                .required(false)
                .help("Delete operation"),
        )
        .arg(
            Arg::new(Create.name())
                .value_parser(value_parser!(String))
                .short(Create.micro_name())
                .long(Create.name())
                .action(ArgAction::Set)
                .required(false)
                .help("Cluster default operation"),
        )
        .arg(
            Arg::new(List.name())
                .value_parser(value_parser!(String))
                .long(List.name())
                .action(ArgAction::Set)
                .required(false)
                .help("Cluster default operation"),
        )
        .arg(
            Arg::new(PrettyList.name())
                .value_parser(value_parser!(String))
                .long(PrettyList.name())
                .action(ArgAction::Set)
                .required(false)
                .help("Cluster default operation"),
        );

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
            Arg::new(PersistentVolumeClaim.name())
                .long(PersistentVolumeClaim.name())
                .action(ArgAction::Count)
                .required(false)
                .help("Cluster default operation"),
        )
        .arg(
            Arg::new(PersistentVolume.name())
                .long(PersistentVolume.name())
                .action(ArgAction::Count)
                .required(false)
                .help("Cluster default operation"),
        )
        .arg(
            Arg::new(Service.name())
                .long(Service.name())
                .action(ArgAction::Count)
                .required(false)
                .help("Cluster default operation"),
        )
        .arg(
            Arg::new(Pod.name())
                .long(Pod.name())
                .action(ArgAction::Count)
                .required(false)
                .help("Cluster default operation"),
        )
        .arg(
            Arg::new(Deployment.name())
                .long(Deployment.name())
                .action(ArgAction::Count)
                .required(false)
                .help("Cluster default operation"),
        )
}
