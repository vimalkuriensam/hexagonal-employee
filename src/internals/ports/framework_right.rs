pub trait DBPort {
    async fn connect(&self);
}