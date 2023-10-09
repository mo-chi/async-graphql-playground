use async_graphql::{MergedObject, Object, Result};

use crate::models::User;

#[derive(Default)]
struct UserQuery;

#[Object]
impl UserQuery {
    async fn user(&self) -> Result<User> {
        Ok(User {
            name: "alice".to_string(),
            age: 101,
        })
    }

    async fn users(&self) -> Result<Vec<User>> {
        Ok(vec![
            User {
                name: "alice".to_string(),
                age: 101,
            },
            User {
                name: "bob".to_string(),
                age: 103,
            },
        ])
    }
}

#[derive(MergedObject, Default)]
pub struct Query(UserQuery);
