mod vehicles;

use axum::{
    http::StatusCode, response::Html, routing::get, Router
};

#[tokio::main]
async fn main(){
    
    let vehicle_routes = Router::new()
    .route("/", get(vehicles::handler_vehicles));

    let owner_routes = Router::new()
    .route("/", get(handler_owners));
    
    let api_routes = Router::new()
    .nest("/vehicles", vehicle_routes)
    .nest("/owners", owner_routes)
    .fallback(fallback);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();


    axum::serve(listener, api_routes).await.unwrap()
}



async fn handler_owners() -> Html<&'static str>{
        println!("->> {:<12} - handler_vehicles", "HANDLER");
        Html("<h1>Hello, This is owners</h1>")
}

async fn fallback() -> (StatusCode, &'static str){
    (StatusCode::NOT_FOUND, "Not found")
}