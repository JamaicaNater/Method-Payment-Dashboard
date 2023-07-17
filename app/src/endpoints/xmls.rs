use crate::schema::xml_parse::XmlParse;
use crate::schema::CRUD;
use crate::State;
use axum::{Extension, Json};
use hyper::StatusCode;
use log::error;
use std::collections::HashMap;

pub async fn get_handler(
    Extension(state): Extension<State>,
) -> Result<Json<Vec<XmlParse>>, StatusCode> {
    match XmlParse::get_by(&state.pool, HashMap::default()).await {
        Ok(xmls) => Ok(Json(xmls)),
        Err(e) => {
            error!("Failed to get xmls due to {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
