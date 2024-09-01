<div align="center">
  <h1>Firebase Auth</h1>
    <p>A simple and small Rust library for handling Firebase Authorization.</p>
    <p>Supports the two most popular frameworks: Tokio's Axum and Actix-web.</p>
</div>

[![Build badge]][Build] [![crates.io badge]][crates.io]

[Build]: https://github.com/trchopan/firebase-auth/actions/workflows/rust.yml
[Build badge]: https://github.com/trchopan/firebase-auth/actions/workflows/rust.yml/badge.svg
[crates.io]: https://crates.io/crates/firebase-auth
[crates.io badge]: https://img.shields.io/crates/v/firebase-auth.svg?color=%23B48723

## Notice

Version 0.4.x supports Axum 0.7.

Version 0.3.x will continue to provide support and fix bugs for Axum 0.6.

## Setup

_Actix_

```toml
[dependencies]
actix-firebase-auth = { version = "<version>", features = ["apistos"] }
actix-web = "4"
```

# Examples

## Actix

[https://github.com/trchopan/firebase-auth/tree/main/examples/actix_basic.rs](https://github.com/trchopan/firebase-auth/tree/main/examples/actix_basic.rs)

```rust
use actix_web::{get, middleware::Logger, web::Data, App, HttpServer, Responder};
use firebase_auth::{FirebaseAuth, FirebaseUser};

// Use `FirebaseUser` extractor to verify the user token and decode the claims
#[get("/hello")]
async fn greet(user: FirebaseUser) -> impl Responder {
    let email = user.email.unwrap_or("empty email".to_string());
    format!("Hello {}!", email)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // TODO: Change to your firebase project id
    let firebase_auth = FirebaseAuth::new("my-project-id").await;

    let app_data = Data::new(firebase_auth);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_data.clone())
            .service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

## Using Custom Claims

[examples/actix-web-custom-claims](https://github.com/trchopan/firebase-auth/blob/main/examples/actix-custom-claims/src/main.rs)

Custom claims are provided as defined `FirebaseUser` struct and use actix or axum trait to implement the extraction from the request.

# How to call the endpoint with Bearer Token

## Obtain the Bearer token

Use firebase sdk to get the User Token.

For example: [getIdToken()](https://firebase.google.com/docs/reference/js/v8/firebase.User#getidtoken)

## Request the endpoint with Authorization Bearer

Make the request using the User's token. Note that it will expire so you will need to get it again if expired.

```
TOKEN="<paste your token here>"

curl --header "Authorization: Bearer $TOKEN" http://127.0.0.1:8080/hello
```

## Firebase Document

[Verify ID tokens using a third-party JWT library](https://firebase.google.com/docs/auth/admin/verify-id-tokens#verify_id_tokens_using_a_third-party_jwt_library)

## License

[MIT](https://opensource.org/licenses/MIT)

Copyright (c) 2022-, Quang Tran.
