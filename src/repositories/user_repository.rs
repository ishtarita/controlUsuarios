use diesel::prelude::*;
use crate::models::User;
use crate::schema::users::dsl::*;

pub struct UserRepository;

impl UserRepository {
    pub fn get_all_users(conn: &mut PgConnection) -> Vec<User> {
        users.load::<User>(conn).expect("Error loading users")
    }
}
