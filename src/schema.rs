use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::GraphQL;

use crate::mutation::Mutation;
use crate::query::Query;

type GraphQLSchema = Schema<Query, Mutation, EmptySubscription>;

fn schema() -> GraphQLSchema {
    Schema::new(Query::default(), Mutation::default(), EmptySubscription)
}

pub fn service() -> GraphQL<GraphQLSchema> {
    GraphQL::new(schema())
}
