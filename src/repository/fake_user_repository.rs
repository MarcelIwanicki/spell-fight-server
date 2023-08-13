use crate::model::user::User;
use crate::repository::repository::Repository;

pub struct FakeUserRepository {
    users: Vec<User>,
}

impl FakeUserRepository {
    pub fn _new() -> Self {
        Self { users: Vec::new() }
    }
}

#[async_trait::async_trait]
impl Repository<User> for FakeUserRepository {
    async fn find_by_id(&self, id: &str) -> Option<User> {
        self.users.iter().find(|user| user.id == id).cloned()
    }

    async fn save(&self, user: User) {
        println!("Fake save: {:?}", user);
    }
}
