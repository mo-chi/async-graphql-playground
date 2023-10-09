use async_graphql::{MergedObject, Object, Result};

use crate::models::User;

#[derive(Default)]
struct UserMutation;

#[Object]
impl UserMutation {
    async fn add_user(
        &self,
        name: String,
        #[graphql(validator(minimum = 0))] age: i32,
    ) -> Result<User> {
        Ok(User { name, age })
    }
}

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutation);
