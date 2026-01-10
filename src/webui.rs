use axum::{Router, extract::{Path, State}, routing::get, response::Html};
use tower_http::services::ServeDir;
use crate::{database, DB_FILE, CODES_FOLDER};
use askama::Template;

pub async fn run() {
    let db_data = database::read_database(DB_FILE).expect("Something went wrong");

    let serve_dir = ServeDir::new(CODES_FOLDER);

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/greet/{name}", get(greet))
        .route("/list", get(main_page))
        .nest_service("/codes/", serve_dir)
        .with_state(db_data);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Running Server on http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn greet(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}

async fn main_page(State(db_data): State<Vec<database::BoxData>>) -> Html<String> {
    let template = MainPage {
        data: db_data
    };
    Html(template.render().unwrap())
}

#[derive(Template)]
#[template(path = "list_boxes.html")]
pub struct MainPage {
    pub data: Vec<database::BoxData>
}
