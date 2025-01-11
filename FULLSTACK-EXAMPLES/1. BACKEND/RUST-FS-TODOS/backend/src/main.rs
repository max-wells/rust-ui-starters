use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};

mod constants;
mod models;
mod services;

use crate::constants::others::{LOCALHOST_3000, PORT_8000};
use crate::services::service_todos;

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt_secret: String,
    pub jwt_maxage: i64,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let jwt_maxage: i64 = std::env::var("JWT_MAXAGE")
        .expect("JWT_MAXAGE must be set")
        .parse()
        .expect("JWT_MAXAGE must be a number");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let app_state = web::Data::new(AppState {
        pool: pool.clone(),
        jwt_secret,
        jwt_maxage,
    });

    println!("🚀 Server started successfully on port {}", PORT_8000);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(LOCALHOST_3000)
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
            ])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .service(root)
            .service(service_todos::service_todos())
    })
    .bind(("0.0.0.0", PORT_8000))?
    .run()
    .await
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[actix_web::get("/api/healthchecker")]
async fn root() -> impl Responder {
    HttpResponse::Ok()
        .json(serde_json::json!({"status": "success", "message": "Hello from rust and aws"}))
}
