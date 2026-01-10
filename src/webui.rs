use crate::{CODES_FOLDER, DB_FILE, database};
use askama::Template;
use axum::{
    Router,
    extract::{Json, Path, State},
    http::StatusCode,
    response::Html,
    routing::{get, post},
};
use serde::Deserialize;
use tower_http::services::ServeDir;

pub async fn run() {
    let db_data = database::read_database(DB_FILE).expect("Something went wrong");

    let serve_dir = ServeDir::new(CODES_FOLDER);

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/greet/{name}", get(greet))
        .route("/list", get(main_page))
        .route("/camera", get(camera_page))
        .route("/submit-qr", post(submit_qr_code))
        .nest_service("/codes/", serve_dir)
        .with_state(db_data);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Running Server on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn greet(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}

async fn main_page(State(db_data): State<Vec<database::BoxData>>) -> Html<String> {
    let template = MainPage { data: db_data };
    Html(template.render().unwrap())
}

async fn camera_page() -> Html<String> {
    let template = CameraTemplate;
    Html(template.render().unwrap())
}

async fn submit_qr_code(Json(payload): Json<QRCodeData>) -> Result<String, (StatusCode, String)> {
    println!("Received QR code content: {}", payload.qr_content);
    // Process the QR code content (e.g., save to database, validate, etc.)
    Ok(format!(
        "Successfully received QR code: {}",
        payload.qr_content
    ))
}

#[derive(Template)]
#[template(path = "list_boxes.html")]
pub struct MainPage {
    pub data: Vec<database::BoxData>,
}

#[derive(Deserialize)]
struct QRCodeData {
    qr_content: String,
}

#[derive(Template)]
#[template(path = "qr_camera.html")]
struct CameraTemplate;
