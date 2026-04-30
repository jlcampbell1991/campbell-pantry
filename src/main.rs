use ::axum::{Router, routing::get};

async fn ping() -> &'static str {
    "pong"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(ping));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8084").await.unwrap();
    println!("Listening on http://0.0.0.0:8084");
    axum::serve(listener, app).await.unwrap();
}
