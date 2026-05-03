use crate::{
    actions::action::Action,
    predicates::predicate::Predicate,
    resolvers::{
        action_resolver::ActionResolver, if_resolver::IfResolver, list_resolver::ListResolver,
        no_op_resolver::NoOpResolver, resolver::Resolver,
    },
};

pub struct ResolversFactory;

impl ResolversFactory {
    pub fn action(&self, action: Action) -> Resolver {
        ActionResolver::new(action)
    }
    pub fn if_else(&self, condition: Predicate, if_true: Resolver, if_false: Resolver) -> Resolver {
        IfResolver::new(condition, if_true, if_false)
    }
    pub fn list(&self, resolvers: Vec<Resolver>) -> Resolver {
        ListResolver::new(resolvers)
    }
    pub fn no_op(&self) -> Resolver {
        NoOpResolver::new()
    }
}
