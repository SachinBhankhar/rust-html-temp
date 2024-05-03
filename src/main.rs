use axum::{routing::get, Router};

#[path = "./routes/get_profile.rs"]
mod get_profile;

#[path = "./routes/home.rs"]
mod home;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home::home))
        .route("/dup", get(home::home))
        .route("/profile/:id", get(get_profile::get_profile));

    let tcp_listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let _ = axum::serve(tcp_listener, app.into_make_service())
        .await
        .unwrap();
}
