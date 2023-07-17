use crate::entities::account::{DestAccount, SourceAccount};
use crate::entities::account_response::AccountResponse;
use crate::entities::entity::Entity;
use crate::entities::entity_response::EntityResponse;
use crate::entities::payment::Payment;
use crate::entities::payment_response::PaymentResponse;
use crate::utility::method_client::Error::{
    HTTPError, IOError, RequestBuilderError, SerializeError,
};
use axum::http;
use hyper::{Body, Client, Method, Request, StatusCode};
use hyper_tls::HttpsConnector;
use lazy_static::lazy_static;
use log::{debug, error};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

static METHOD_BASE_URL: &'static str = "https://dev.methodfi.com";
lazy_static! {
    static ref METHOD_API_KEY: String = env::var("METHOD_API_KEY").expect("METHOD_API_KEY was set");
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Http Error: {0}. {1}")]
    HTTPError(StatusCode, String),
    #[error("RequestBuilder Error: {0}")]
    RequestBuilderError(#[source] http::Error),
    #[error("IO Error: {0}")]
    IOError(#[source] Box<dyn std::error::Error>),
    #[error("Serialization Error: {0}")]
    SerializeError(#[source] Box<dyn std::error::Error>),
}

#[derive(Debug, Serialize, Deserialize)]
struct MethodResponse<ObjectType: Serialize> {
    pub success: bool,
    data: ObjectType,
    message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MethodError {
    pub error: ErrorDetails,
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorDetails {
    #[serde(rename = "type")]
    pub failure_type: String,
    pub sub_type: String,
    pub code: u32,
    pub message: String,
}

pub async fn post_source_account(account: SourceAccount) -> Result<AccountResponse, Error> {
    let response: MethodResponse<AccountResponse> =
        generic_request(Method::POST, "accounts", Some(account), HashMap::new()).await?;
    Ok(response.data)
}

pub async fn post_dest_account(account: DestAccount) -> Result<AccountResponse, Error> {
    let response: MethodResponse<AccountResponse> =
        generic_request(Method::POST, "accounts", Some(account), HashMap::new()).await?;
    Ok(response.data)
}

pub async fn post_payment(payment: Payment) -> Result<PaymentResponse, Error> {
    let response: MethodResponse<PaymentResponse> =
        generic_request(Method::POST, "payments", Some(payment), HashMap::new()).await?;
    Ok(response.data)
}

pub async fn get_payments(
    query_params: HashMap<&str, &str>,
) -> Result<Vec<PaymentResponse>, Error> {
    let response =
        generic_request::<Vec<PaymentResponse>, u32>(Method::GET, "payments", None, query_params)
            .await?;
    Ok(response.data)
}

pub async fn post_entity(entity: Entity) -> Result<EntityResponse, Error> {
    let response: MethodResponse<EntityResponse> =
        generic_request(Method::POST, "entities", Some(entity), HashMap::new()).await?;
    Ok(response.data)
}

pub async fn get_entities(query_params: HashMap<&str, &str>) -> Result<Vec<EntityResponse>, Error> {
    let response =
        generic_request::<Vec<EntityResponse>, u32>(Method::GET, "entities", None, query_params)
            .await?;
    Ok(response.data)
}

async fn generic_request<ResponseType, RequestType: Serialize>(
    method: Method,
    endpoint: &str,
    body: Option<RequestType>,
    query_params: HashMap<&str, &str>,
) -> Result<MethodResponse<ResponseType>, Error>
where
    ResponseType: for<'a> Deserialize<'a> + Serialize,
{
    let mut uri = format!("{}/{}", METHOD_BASE_URL, endpoint);

    if !query_params.is_empty() {
        uri.push_str("?");
    }
    for param in query_params {
        uri.push_str(param.0);
        uri.push_str("=");
        uri.push_str(param.1);
    }

    let entity_type = std::any::type_name::<RequestType>();
    let result_entity_type = std::any::type_name::<ResponseType>();

    debug!("Sending {} request to {}", method.as_str(), uri);
    let https_connector = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https_connector);

    let json: String = match serde_json::to_string(&body) {
        Ok(string) => string,
        Err(e) => {
            error!(
                "Failed to deserialize to string, from {} due to '{}'",
                entity_type, e
            );
            return Err(SerializeError(Box::new(e)));
        }
    };

    let mut builder = Request::builder()
        .method(method)
        .uri(uri.clone())
        .header("Authorization", format!("Bearer {}", *METHOD_API_KEY));

    if body.is_some() {
        builder = builder
            .header("Content-Type", "application/json")
            .header("Content-Length", json.len());
    }

    let request = match builder.body(Body::from(json)) {
        Ok(req) => req,
        Err(e) => {
            error!("Failed to build request to '{}' due to '{}'", uri, e);
            return Err(RequestBuilderError(e));
        }
    };

    let result = match client.request(request).await {
        Ok(res) => res,
        Err(e) => {
            error!("Failed to send request to '{}' due to '{}'", uri, e);
            return Err(IOError(Box::new(e)));
        }
    };

    let status = result.status();
    debug!("status: {}", status);

    let buf = match hyper::body::to_bytes(result).await {
        Ok(bytes) => bytes,
        Err(e) => {
            error!(
                "Failed to create bytes from {} due to '{}'",
                result_entity_type, e
            );
            return Err(SerializeError(Box::new(e)));
        }
    };

    let ret = if status.is_success() {
        match serde_json::from_slice::<MethodResponse<ResponseType>>(&buf) {
            Ok(account_response) => Ok(account_response),
            Err(e) => {
                error!(
                    "Failed to serialize from bytes, to {} due to '{}'",
                    result_entity_type, e
                );
                Err(SerializeError(Box::new(e)))
            }
        }
    } else {
        let mut cause = String::from("");
        match serde_json::from_slice::<MethodResponse<MethodError>>(&buf) {
            Ok(failure_response) => {
                cause = failure_response.data.error.message;
            }
            Err(e) => {
                error!(
                    "Failed to deserialize error response after Http Failure due to {}",
                    e
                );
            }
        };
        Err(HTTPError(status, cause))
    };
    ret
}
