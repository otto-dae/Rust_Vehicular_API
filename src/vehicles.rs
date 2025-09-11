use axum::{
};

use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
struct Vehicle{
    id: Uuid,
    model: String,
    year: i32,
}


pub async fn create_vehcile(){
    let id = Uuid::new_v4();

    let vehicle1 = Vehicle{
        id: id,
        model: String::from("Ford"),
        year: 2004
    };

}
