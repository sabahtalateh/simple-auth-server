#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate diesel;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpServer};
use diesel::{r2d2, PgConnection};

mod email_service;
mod errors;
mod handlers;
mod models;
mod schema;
mod utils;

fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = r2d2::ConnectionManager::<PgConnection>::new(db_url);
    let pool: models::Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let domain = std::env::var("DOMAIN").unwrap_or("localhost:80".to_string());
    let cookie_domain = domain.clone();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(utils::SECRET_KEY.as_bytes())
                    .name("auth")
                    .path("/")
                    .domain(cookie_domain.as_str())
                    .max_age_time(chrono::Duration::days(1))
                    .secure(false),
            ))
            .data(web::JsonConfig::default().limit(4096))
            .service(
                web::scope("/api")
                    .service(
                        web::resource("/invitation")
                            .route(web::post().to_async(handlers::invitation::post_invitation)),
                    )
                    .service(
                        web::resource("/register/{invitation_id}")
                            .route(web::post().to_async(handlers::register::register_user)),
                    )
                    .service(
                        web::resource("/auth")
                            .route(web::post().to_async(handlers::auth_handler::login))
                            .route(web::get().to(handlers::auth_handler::get_me))
                            .route(web::delete().to(handlers::auth_handler::logout)),
                    ),
            )
    })
    .bind(domain)?
    .run()
}
