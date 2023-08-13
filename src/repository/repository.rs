#[async_trait::async_trait]
pub trait Repository<T> {
    async fn find_by_id(&self, id: &str) -> Option<T>;
    async fn save(&self, entity: T);
}