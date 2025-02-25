use crate::smooth_operator::ClusterEntity::*;
use crate::smooth_operator::DefaultClusterOperations::*;
use clap::ArgMatches;
use std::fmt::Debug;
use std::rc::Weak;
use std::slice::Iter;
use Operation::*;

#[derive(Clone)]
pub struct ParsedParameters {
    pub namespace: Option<String>,
    pub operation_call: Option<OperationCall>,
    pub entity_call: Option<ClusterEntity>,
    pub argument_call: Option<ArgumentCall>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DefaultClusterOperations {
    Delete,
}

#[derive(Clone, Debug)]
pub enum EnvironmentSpecifiers {
    Namespace,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ClusterEntity {
    _PersistentVolumeClaim,
    _Deployment,
    _Service,
    _Pod,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Operation {
    FindNameLike,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OperationCall {
    pub operation: Operation,
    pub value: Option<String>,
    pub trailing: Option<Box<OperationCall>>,
}

#[derive(Clone)]
pub struct ArgumentCall {
    pub(crate) initial: DefaultClusterOperations,
    pub(crate) trailing: Option<Box<ArgumentCall>>,
    pub(crate) head_link: Option<Weak<ArgumentCall>>,
}

pub trait NamedArgument {
    fn name(&self) -> &str;
}

pub trait DescriptionArgument {
    fn description(&self) -> &str;
}

pub trait MatchParser<T> {
    fn find_operation_match(matches: &ArgMatches) -> Option<T>;
}

impl DescriptionArgument for Operation {
    fn description(&self) -> &str {
        match self {
            FindNameLike => {
                "Find matches for name like provided regexp from all-across the cluster"
            }
        }
    }
}

impl NamedArgument for Operation {
    fn name(&self) -> &str {
        match self {
            FindNameLike => "fanl",
        }
    }
}

//ordered in descending by operation priority
impl NamedArgument for DefaultClusterOperations {
    fn name(&self) -> &str {
        match self {
            Delete => "delete",
        }
    }
}

impl NamedArgument for EnvironmentSpecifiers {
    fn name(&self) -> &str {
        match self {
            EnvironmentSpecifiers::Namespace => "namespace",
        }
    }
}
impl NamedArgument for ClusterEntity {
    fn name(&self) -> &str {
        match self {
            _PersistentVolumeClaim => "pvc",
            _Service => "svc",
            _Pod => "pod",
            _Deployment => "dpl",
        }
    }
}

pub trait IterableEnum: Sized {
    fn iterator() -> Iter<'static, Self>;
}

impl IterableEnum for DefaultClusterOperations {
    fn iterator() -> Iter<'static, DefaultClusterOperations> {
        static OPERATIONS: [DefaultClusterOperations; 1] = [Delete];
        OPERATIONS.iter()
    }
}

impl IterableEnum for ClusterEntity {
    fn iterator() -> Iter<'static, ClusterEntity> {
        static OPERATIONS: [ClusterEntity; 4] =
            [_PersistentVolumeClaim, _Service, _Pod, _Deployment];
        OPERATIONS.iter()
    }
}

impl IterableEnum for Operation {
    fn iterator() -> Iter<'static, Operation> {
        static OPERATIONS: [Operation; 1] = [FindNameLike];
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
            let specified: u8 = matches.get_count(op.name());

            if specified > 0 {
                let mut argument = ArgumentCall {
                    initial: op.clone(),
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

impl MatchParser<OperationCall> for Operation {
    fn find_operation_match(matches: &ArgMatches) -> Option<OperationCall> {
        let mut operation: Option<OperationCall> = None;
        let mut trailing: Option<Box<OperationCall>> = None;

        for op in Operation::iterator() {
            if let Some(specified) = matches.get_one::<String>(op.name()) {
                let argument = OperationCall {
                    operation: op.clone(),
                    value: Some(specified.clone()),
                    trailing: None,
                };

                if operation.is_none() {
                    operation = Some(argument.clone());
                }

                if let Some(ref mut ex_trail) = trailing {
                    ex_trail.trailing = Some(Box::new(argument.clone()));
                    trailing = ex_trail.trailing.clone();
                } else {
                    trailing = Some(Box::new(argument.clone()));
                }
            }
        }

        operation
    }
}
