use crate::smooth_operator::ClusterEntity::*;
use crate::smooth_operator::DefaultClusterOperations::*;
use crate::smooth_operator::EnvironmentSpecifiers::Namespace;
use crate::smooth_operator::Operation::FindNameLike;
use crate::smooth_operator::*;
use clap::{value_parser, Arg, ArgAction, Command, Id};
use SpecifiedClusterOperations::{List, PrettyList};

pub fn parse_args() -> Option<ParsedParameters> {
    let mut command = Command::new("kubus")
        .args_override_self(true)
        .version("0.0.1")
        .author("peeqle")
        .arg(create_default_arg_item_valued(&Namespace))
        .arg(create_default_arg_item(&Delete))
        .arg(create_default_arg_item(&Create).conflicts_with(Delete.name()))
        .arg(create_default_arg_item(&List).conflicts_with(PrettyList.name()))
        .arg(create_default_arg_item(&PrettyList));

    command = append_entity_context(command);
    let matches = append_operations(command).get_matches();

    let namespace = matches.get_one::<String>(Namespace.name());
    Some(ParsedParameters {
        namespace: if namespace.is_some() {Some(namespace.unwrap().clone())} else { None },
        operation_call: Operation::find_operation_match(&matches),
        entity_call: ClusterEntity::find_operation_match(&matches),
        argument_call: DefaultClusterOperations::find_operation_match(&matches),
    })
}

fn append_entity_context(command: Command) -> Command {
    command
        .arg(create_conflicting_cluster_item(&_Pod))
        .arg(create_conflicting_cluster_item(&_Service))
        .arg(create_conflicting_cluster_item(&_Deployment))
        .arg(create_conflicting_cluster_item(&_PersistentVolumeClaim))
}

fn append_operations(command: Command) -> Command {
    command.arg(create_default_arg_operation(&FindNameLike))
}

pub fn create_default_arg_item<T: NamedArgument>(entity: &'static T) -> Arg {
    let name = entity.name();
    Arg::new(name)
        .long(name)
        .action(ArgAction::Count)
        .required(false)
        .help("Cluster default operation")
}

pub fn create_default_arg_item_valued<T: NamedArgument>(entity: &'static T) -> Arg {
    let name = entity.name();
    Arg::new(name)
        .value_parser(value_parser!(String))
        .long(name)
        .action(ArgAction::Set)
        .required(false)
        .help("Cluster default operation")
}

pub fn create_default_arg_item_entity(entity: &'static impl NamedArgument) -> Arg {
    let name = entity.name();
    Arg::new(name)
        .long(name)
        .action(ArgAction::Count)
        .required(false)
        .help(format!("Cluster default {} operation", entity.name()))
}

pub fn create_default_arg_operation<T: NamedArgument + DescriptionArgument>(
    entity: &'static T,
) -> Arg {
    let name = entity.name();
    Arg::new(name)
        .long(name)
        .action(ArgAction::Set)
        .required(false)
        .help(entity.description())
}

fn create_conflicting_cluster_item(entity: &'static ClusterEntity) -> Arg {
    create_default_arg_item_entity(entity).conflicts_with_all(
        ClusterEntity::iterator()
            .filter(|x| *x != entity)
            .map(|x| Id::from(x.name())),
    )
}
