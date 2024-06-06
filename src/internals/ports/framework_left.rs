use axum::Router;

pub trait ServerPort {
    fn serve();
}

pub trait RoutesPort {
    fn routes(&self) -> Router;
}