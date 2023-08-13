use mongodb::Client;
use mongodb::options::ClientOptions;

use crate::model::user::User;
use crate::repository::repository::Repository;

pub struct MongoDBUserRepository {
    collection: mongodb::Collection<User>,
}

impl MongoDBUserRepository {
    pub async fn new() -> Result<Self, mongodb::error::Error> {
        let client_options = ClientOptions::parse("mongodb://127.0.0.1:27017").await?;
        let client = Client::with_options(client_options)?;
        let db = client.database("spell-fight-database");
        let collection = db.collection::<User>("users");
        Ok(Self { collection })
    }
}

#[async_trait::async_trait]
impl Repository<User> for MongoDBUserRepository {
    async fn find_by_id(&self, id: &str) -> Option<User> {
        let filter = mongodb::bson::doc! { "id": id };
        self.collection.find_one(filter, None).await.unwrap()
    }

    async fn save(&self, user: User) {
        match self.find_by_id(user.id.as_str()).await {
            Some(_) => {}
            None => {
                println!("MongoDB save: {:?}", user);
                self.collection.insert_one(user, None).await.unwrap();
            }
        }
    }
}