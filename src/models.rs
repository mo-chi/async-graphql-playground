use async_graphql::SimpleObject;

#[derive(SimpleObject, Debug)]
pub struct User {
    pub name: String,
    pub age: i32,
}
