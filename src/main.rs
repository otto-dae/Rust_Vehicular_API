use axum::{
    response::{Html}, routing::{get}, Router
};

#[tokio::main]
async fn main(){
    let app = Router::new().route("/", get(handler_hello));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();

    axum::serve(listener, app).await.unwrap()
}

async fn handler_hello() -> Html<&'static str>{
    println!("->> {:<12} - handler_hello", "HANDLER");

    Html("<h1>Hello, Axum!</h1><p>This is some HTML content.</p>")
}