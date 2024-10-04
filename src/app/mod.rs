use crate::routes::routes;
use axum::{serve, Router};
use tokio::net::TcpListener;

pub struct BasicServer;

impl BasicServer {
    pub async fn run() -> Result<(), std::io::Error> {
        let app: Router = routes();
        let listener = TcpListener::bind("127.0.0.1:5000").await?;

        serve(listener, app).await
    }
}
