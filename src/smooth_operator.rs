use clap::ArgMatches;
use std::rc::Weak;
use std::slice::Iter;
use SpecifiedClusterOperations::*;
use crate::smooth_operator::ClusterEntity::*;
use crate::smooth_operator::DefaultClusterOperations::*;

#[derive(Clone)]
pub struct ParsedParameters {
    pub entity_call: Option<ClusterEntity>,
    pub argument_call: Option<ArgumentCall>,
}

#[derive(Clone, Debug)]
pub enum DefaultClusterOperations {
    Delete,
    Create,
    Namespace,
}

#[derive(Clone, Debug)]
pub enum ClusterEntity {
    PersistentVolumeClaim,
    PersistentVolume,
    Service,
    Pod,
    Deployment,
}

#[derive(Clone, Debug)]
pub enum SpecifiedClusterOperations {
    List,
    PrettyList,
}

#[derive(Clone)]
pub struct ArgumentCall {
    pub(crate) initial: DefaultClusterOperations,
    pub(crate) trailing: Option<Box<ArgumentCall>>,
    pub(crate) value: Option<String>,
    pub(crate) head_link: Option<Weak<ArgumentCall>>,
}

pub trait ClusterName {
    fn cluster_name(&self) -> &str;
}
pub trait NamedArgument {
    fn name(&self) -> &str;
}

pub trait MicroArgument {
    //pretty average actually
    fn micro_name(&self) -> char;
}

pub trait MatchParser<T> {
    fn find_operation_match(matches: &ArgMatches) -> Option<T>;
}

impl ClusterName for ClusterEntity {
    fn cluster_name(&self) -> &str {
        match self {
            PersistentVolumeClaim => "pvc",
            PersistentVolume => "pv",
            Service => "svc",
            Pod => "pod",
            Deployment => "dpl",
            _ => {
                panic!("Unknown cluster operation")
            }
        }
    }
}

//ordered in descending by operation priority
impl NamedArgument for DefaultClusterOperations {
    fn name(&self) -> &str {
        match self {
            Delete => "delete",
            Create => "create",
            Namespace => "namespace",
            _ => {
                panic!("Unknown cluster operation")
            }
        }
    }
}

impl NamedArgument for ClusterEntity {
    fn name(&self) -> &str {
        match self {
            PersistentVolumeClaim => "pvc",
            PersistentVolume => "pv",
            Service => "svc",
            Pod => "pod",
            Deployment => "dpl",
            _ => {
                panic!("Unknown cluster operation")
            }
        }
    }
}

impl NamedArgument for SpecifiedClusterOperations {
    fn name(&self) -> &str {
        match self {
            List => "list",
            PrettyList => "pretty-list",
            _ => {
                panic!("Unknown cluster operation")
            }
        }
    }
}

impl MicroArgument for DefaultClusterOperations {
    fn micro_name(&self) -> char {
        match self {
            Delete => 'd',
            Create => 'c',
            Namespace => 'n',
            _ => {
                panic!("Unknown cluster operation")
            }
        }
    }
}

impl DefaultClusterOperations {
    pub fn iterator() -> Iter<'static, DefaultClusterOperations> {
        static OPERATIONS: [DefaultClusterOperations; 3] = [Delete, Create, Namespace];
        OPERATIONS.iter()
    }
}

impl ClusterEntity {
    pub fn iterator() -> Iter<'static, ClusterEntity> {
        static OPERATIONS: [ClusterEntity; 5] = [
            PersistentVolumeClaim,
            PersistentVolume,
            Service,
            Pod,
            Deployment,
        ];
        OPERATIONS.iter()
    }
}

impl MatchParser<ClusterEntity> for ClusterEntity {
    //finds first match in a sequence
    fn find_operation_match(matches: &ArgMatches) -> Option<ClusterEntity> {
        for op in ClusterEntity::iterator() {
            if matches.get_count(op.name()) > 0 {
                return Some(op.clone());
            }
        }
        None
    }
}

impl MatchParser<ArgumentCall> for DefaultClusterOperations {
    fn find_operation_match(matches: &ArgMatches) -> Option<ArgumentCall> {
        let mut operations: Option<ArgumentCall> = None;
        let mut last: Option<ArgumentCall> = None;

        if !matches.args_present() {
            panic!("No cluster operation argument provided");
        }

        for op in DefaultClusterOperations::iterator() {
            let specified: Option<&String> = matches.get_one::<String>(op.name());

            if specified.is_some() {
                let mut argument = ArgumentCall {
                    initial: op.clone(),
                    value: Some(String::from(specified.unwrap())),
                    trailing: None,
                    head_link: None,
                };

                if let Some(last_item) = last.as_mut() {
                    argument.head_link = last_item.head_link.clone();
                    last_item.trailing = Some(Box::new(argument.clone()));
                } else {
                    operations = Some(argument.clone());
                }

                last = Some(argument.clone());
            }
        }
        operations
    }
}
