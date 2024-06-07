use axum::Router;

pub trait ServerPort {
    async fn serve(&self);
}

pub trait RoutesPort {
    fn routes(&self) -> Router;
}
