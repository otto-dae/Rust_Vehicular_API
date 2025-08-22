use axum::{
    response::{Html, IntoResponse}, routing::{get}, Router
};

#[tokio::main]
async fn main(){
    let app = Router::new().route("/", handler_hello());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();

    axum::serve(listener, app).await.unwrap()
}

async fn handler_hello() -> Html<&'static str>{
    println!("->> {:<12} - handler_hello", "HANDLER");

    Html("Hello im Otto")
}