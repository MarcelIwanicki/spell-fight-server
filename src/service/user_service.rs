use crate::model::user::User;
use crate::repository::repository::Repository;

pub struct UserService<T: Repository<User>> {
    user_repository: T,
}

impl<T: Repository<User>> UserService<T> {
    pub fn new(user_repository: T) -> Self {
        Self { user_repository }
    }

    pub async fn get_user(&self, id: &str) -> Option<User> {
        self.user_repository.find_by_id(id).await
    }

    pub async fn create_user(&self, user: User) {
        self.user_repository.save(user).await;
    }
}