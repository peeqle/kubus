use crate::smooth_operator::ClusterEntity::*;
use crate::smooth_operator::DefaultClusterOperations::*;
use crate::smooth_operator::*;
use clap::{value_parser, Arg, ArgAction, Command};
use std::fmt::Debug;
use DefaultClusterOperations::Namespace;
use SpecifiedClusterOperations::{List, PrettyList};
use crate::smooth_operator::Operations::FindNameLike;

pub fn build_args() -> Option<ParsedParameters> {
    let mut command = Command::new("kubus")
        .args_override_self(true)
        .version("0.0.1")
        .author("peeqle")
        .arg(create_default_arg_item(&Namespace))
        .arg(create_default_arg_item(&Delete))
        .arg(create_default_arg_item(&Create))
        .arg(create_default_arg_item(&List))
        .arg(create_default_arg_item(&PrettyList));

    command = append_entity_context(command);
    let matches = append_operations(command).get_matches();

    Some(ParsedParameters {
        entity_call: ClusterEntity::find_operation_match(&matches),
        argument_call: DefaultClusterOperations::find_operation_match(&matches),
    })
}

fn append_entity_context(command: Command) -> Command {
    command
        .arg(create_default_arg_item_entity(&_Pod))
        .arg(create_default_arg_item_entity(&_Service))
        .arg(create_default_arg_item_entity(&_Deployment))
        .arg(create_default_arg_item_entity(&_PersistentVolumeClaim))
}

fn append_operations(command: Command) -> Command {
    command
        .arg(create_default_arg_operation(&FindNameLike))
}

pub fn create_default_arg_item<T: NamedArgument>(entity: &'static T) -> Arg {
    let name = entity.name();
    Arg::new(name)
        .value_parser(value_parser!(String))
        .long(name)
        .action(ArgAction::Set)
        .required(false)
        .help("Cluster default operation")
}

pub fn create_default_arg_item_entity<T: NamedArgument>(entity: &'static T) -> Arg {
    let name = entity.name();
    Arg::new(name)
        .long(name)
        .action(ArgAction::Count)
        .required(false)
        .help(format!("Cluster default {} operation", entity.name()))
}

pub fn create_default_arg_operation<T: NamedArgument + DescriptionArgument>(entity: &'static T) -> Arg {
    let name = entity.name();
    Arg::new(name)
        .long(name)
        .action(ArgAction::Set)
        .required(false)
        .help(entity.description())
}
