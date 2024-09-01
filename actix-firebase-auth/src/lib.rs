//! [Firebase](https://firebase.google.com) authentication layer for popular frameworks.
//!
//! Support:
//!
//! - [Axum](https://github.com/tokio-rs/axum)
//! - [Actix](https://github.com/actix/actix-web)
//!
//! ## Example:
//!
//! ### Actix
//!
//! ```rust
//! use actix_web::{get, middleware::Logger, web::Data, App, HttpServer, Responder};
//! use firebase_auth::{FirebaseAuth, FirebaseUser};
//!
//! #[get("/hello")]
//! async fn greet(user: FirebaseUser) -> impl Responder {
//!     let email = user.email.unwrap_or("empty email".to_string());
//!     format!("Hello {}!", email)
//! }
//!
//! #[get("/public")]
//! async fn public() -> impl Responder {
//!     "ok"
//! }
//!
//! #[actix_web::main]
//! async fn main() -> std::io::Result<()> {
//!     let firebase_auth = FirebaseAuth::new("my-project-id").await;
//!
//!     let app_data = Data::new(firebase_auth);
//!
//!     HttpServer::new(move || {
//!         App::new()
//!             .wrap(Logger::default())
//!             .app_data(app_data.clone())
//!             .service(greet)
//!             .service(public)
//!     })
//!     .bind(("127.0.0.1", 8080))?
//!     .run()
//!     .await
//! }
//! ```
//!
//!Visit [README.md](https://github.com/Pravez/actix-firebase-auth) for more details.

mod firebase_auth;
pub use firebase_auth::FirebaseAuth;

mod structs;
pub use structs::{FirebaseUser, PublicKeysError, FirebaseProvider};

mod actix;


