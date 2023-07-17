use crate::schema::transaction::Transaction;
use crate::schema::xml_parse::XmlParse;
use crate::schema::CRUD;
use crate::utility::parser::parse;
use crate::State;
use axum::extract::Query;
use axum::{extract::Multipart, http::StatusCode, Extension, Json};
use log::{debug, error, info};
use serde::Deserialize;
use serde::Serialize;
use tokio::task;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Payload {
    base64_xml: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct TransactionQueryParams {
    pub xml_id: u64,
}

pub async fn post_handler(
    Extension(state): Extension<State>,
    mut multipart: Multipart,
) -> Result<Json<Vec<XmlParse>>, StatusCode> {
    let mut xmls: Vec<XmlParse> = vec![];

    // Todo: support multiple xml at the same time
    // loop {
    match multipart.next_field().await {
        Ok(next_part) => {
            match next_part {
                None => {
                    info!("Executed all fields");
                    // break;
                }
                Some(field) => {
                    let field_name = field.name().unwrap_or("No field").to_string();
                    debug!("Parsing field: {}", field_name);

                    debug!(
                        "Creating {} entry for: {}",
                        XmlParse::COLUMN_NAME,
                        field_name
                    );

                    let mut xml = XmlParse::new(field_name.clone(), String::from("Init"));
                    let id = xml.insert(&state.pool).await.map_err(|e| {
                        error!("Failed to insert {} due to {}", XmlParse::COLUMN_NAME, e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })?;
                    xml.id = Some(id);
                    xmls.push(xml.clone());

                    match field.bytes().await {
                        Ok(bytes) => {
                            task::spawn(async move {
                                if let Err(_e) = parse(&state.pool, bytes, id).await {
                                    // TODO: Fix
                                    // error!("Failed to parse due to {}" , e);
                                    if let Err(e) =
                                        xml.set_finished(&state.pool, String::from("Failed")).await
                                    {
                                        error!(
                                            "Failed to set xml with id {} as failed due to {}",
                                            id, e
                                        );
                                    }
                                }
                                if let Err(e) = xml
                                    .set_finished(&state.pool, String::from("Finished"))
                                    .await
                                {
                                    error!(
                                        "Failed to set xml with id {} as finished due to {}",
                                        id, e
                                    );
                                }
                            });
                        }
                        Err(e) => {
                            error!("Failed to get bytes of field {}, due to {}", field_name, e);
                            if let Err(e) =
                                xml.set_finished(&state.pool, String::from("Failed")).await
                            {
                                error!("Failed to set xml with id {} as failed due to {}", id, e);
                            }
                        }
                    }
                }
            };
        }
        Err(e) => {
            error!("Failed to get next part due to {}", e);
            // break;
        }
    }
    // }

    Ok(Json(xmls))
}

pub async fn get_handler(
    Extension(state): Extension<State>,
    query: Query<TransactionQueryParams>,
) -> Result<Json<Vec<Transaction>>, StatusCode> {
    let transactions = XmlParse::get_all_transactions_by_xml_id(&state.pool, query.xml_id)
        .await
        .map_err(|e| {
            error!("Failed to get transactions due to {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(transactions))
}
