pub mod endpoints;
pub mod entities;
pub mod schema;
pub mod utility;

use crate::schema::db::create_from_env;
use axum::extract::DefaultBodyLimit;
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, ORIGIN};
use axum::http::Method;
use axum::{routing::get, routing::post, Extension, Router};
use log::{info, warn};
use sqlx::{MySql, Pool};
use std::env;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct State {
    pool: Pool<MySql>,
}

pub async fn serve() {
    let max_upload = get_max_upload_size();

    let state = setup_state().await;

    // Configure the CORS layer
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(vec![ORIGIN, AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        // allow requests from any origin
        .allow_origin(Any);

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/transactions", post(endpoints::transactions::post_handler))
        .route("/transactions", get(endpoints::transactions::get_handler))
        .route("/reports", get(endpoints::reports::get_handler))
        .route("/xmls", get(endpoints::xmls::get_handler))
        .layer(Extension(state))
        .layer(DefaultBodyLimit::max(max_upload))
        .layer(cors);

    let socket_addr: SocketAddr = match "0.0.0.0:3001".parse::<SocketAddr>() {
        Ok(socket_addr) => {
            info!("Socket address setup sucessfully");
            socket_addr
        }
        Err(_) => {
            panic!("Failed to Setup socket address")
        }
    };

    // run it with hyper on localhost:3000
    match axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await
    {
        Ok(_) => {
            info!("Running server at {}", socket_addr);
        }
        Err(e) => {
            panic!("Failed to start server due to '{}'", e)
        }
    }
}

async fn setup_state() -> State {
    let db_client = match create_from_env().await {
        Ok(client) => client,
        Err(e) => {
            panic!("Failed to initialize client due to '{}'", e)
        }
    };

    State {
        pool: db_client.pool.unwrap(),
    }
}

fn get_max_upload_size() -> usize {
    let default = 128 * 1024 * 1024 as usize;
    match env::var("MAX_UPLOAD") {
        Ok(size) => match size.parse::<usize>() {
            Ok(size_as_usize) => size_as_usize,
            Err(_) => {
                warn!("Environment Variable MAX_UPLOAD with value {} could not be parsed as usize defaulting to 128MB", size);
                default
            }
        },
        Err(_) => {
            warn!("Environment Variable MAX_UPLOAD not set, defaulting to 128MB");
            default
        }
    }
}
