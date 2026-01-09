use axum::{extract::Path, routing::get, Router};
use tower_http::services::ServeDir;

pub async fn run() {
    let serve_dir = ServeDir::new("codes");

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/greet/{name}", get(greet))
        .nest_service("/codes/", serve_dir);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Running Server!");
    axum::serve(listener, app).await.unwrap();
}

async fn greet(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}
