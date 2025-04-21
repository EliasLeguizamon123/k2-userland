#[macro_use] extern crate rocket;

use rocket::serde::{Deserialize, Serialize, json::Json};
use sqlx::{PgPool, postgres::PgPoolOptions};
use dotenvy::dotenv;
use std::env;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Preorder {
    name: String,
    email: String,
}

#[get("/ping")]
fn ping() -> &'static str {
    "pong"
}

#[post("/preorder", format = "json", data = "<preorder>")]
async fn preorder(pool: &rocket::State<PgPool>, preorder: Json<Preorder>) -> Result<Json<Preorder>, String> {
    let preorder = preorder.into_inner();

    // Inserción en la base de datos
    let result = sqlx::query(
        "INSERT INTO users (name, email) VALUES ($1, $2)"
    )
    .bind(&preorder.name)
    .bind(&preorder.email)
    .execute(pool.inner())
    .await;

    match result {
        Ok(_) => Ok(Json(preorder)),
        Err(err) => Err(format!("DB error: {}", err)),
    }
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");

    let config = rocket::Config {
        address: "0.0.0.0".parse().unwrap(),
        port,
        ..rocket::Config::default()
    };

    rocket::custom(config)
        .manage(pool)
        .mount("/", routes![ping, preorder])
}
