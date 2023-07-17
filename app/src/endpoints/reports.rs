use crate::endpoints::transactions::TransactionQueryParams;
use crate::entities::payment_response::PaymentResponse;
use crate::schema::employee::Employee;
use crate::schema::xml_parse::XmlParse;
use crate::schema::{SqlString, CRUD};
use crate::utility::method_client::get_payments;
use crate::State;
use axum::extract::Query;
use axum::{Extension, Json};
use hyper::StatusCode;
use log::{debug, error};
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct ParseResponse {
    xml_id: u64,
    processing: bool,
    payment_map_acc: HashMap<String, u64>,
    payment_map_branch: HashMap<String, u64>,
    payment_statuses: Vec<PaymentStatus>,
}

#[derive(Serialize, Deserialize)]
pub struct PaymentStatus {
    id: String,
    destination: String,
    source: String,
    estimated_completion_date: String,
    status: String,
    amount: u64,
    metadata: Option<HashMap<String, String>>,
}

impl From<PaymentResponse> for PaymentStatus {
    fn from(payment_response: PaymentResponse) -> Self {
        Self {
            id: payment_response.id,
            destination: payment_response.destination,
            source: payment_response.source,
            estimated_completion_date: payment_response.estimated_completion_date,
            status: payment_response.status,
            amount: payment_response.amount,
            metadata: None,
        }
    }
}

pub async fn get_handler(
    Extension(state): Extension<State>,
    query: Query<TransactionQueryParams>,
) -> Result<Json<ParseResponse>, StatusCode> {
    let mut response = ParseResponse {
        xml_id: query.xml_id,
        processing: false,
        payment_map_acc: Default::default(),
        payment_map_branch: Default::default(),
        payment_statuses: vec![],
    };
    debug!("Generating report");

    let payment_responses = get_payments(HashMap::default()).await.map_err(|e| {
        error!("Failed to get payments due to {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    response.payment_statuses = payment_responses
        .into_iter()
        .filter(|pr| pr.description == query.xml_id.to_string())
        .map(|pr| PaymentStatus::from(pr))
        .collect();

    let xml = match XmlParse::get_by(
        &state.pool,
        HashMap::from([("Id", SqlString::from(query.xml_id))]),
    )
    .await
    .map_err(|e| {
        error!("Failed to get xml due to {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .first()
    {
        None => {
            error!("XML with id {} not found", query.xml_id);
            return Err(StatusCode::NOT_FOUND);
        }
        Some(xml) => xml.clone(),
    };

    response.processing = match xml.status.as_str() {
        "Finished" => false,
        _ => true,
    };

    let transactions = XmlParse::get_all_transactions_by_xml_id(&state.pool, query.xml_id)
        .await
        .map_err(|e| {
            error!("Failed to get transactions due to {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let employee_ids: Vec<String> = transactions
        .clone()
        .into_iter()
        .filter_map(|t| t.employee_id)
        .collect();
    let employee_map: HashMap<String, Employee> =
        Employee::get_in(&state.pool, HashMap::from([("MethodId", employee_ids)]))
            .await
            .map_err(|e| {
                error!("Failed to get employees due to {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?
            .into_iter()
            .map(|e| (e.method_id.clone().unwrap(), e))
            .collect();

    for transaction in transactions {
        debug!("transaction {:?}", transaction);
        *response
            .payment_map_acc
            .entry(transaction.payor_id.expect("Payor Id was set in parsing"))
            .or_default() += transaction.amount.expect("Amount was set in parsing");

        let emp = employee_map
            .get(
                transaction
                    .employee_id
                    .clone()
                    .expect("Valid employee id")
                    .as_str(),
            )
            .expect(
                format!(
                    "Employee id {} exists",
                    SqlString::from(transaction.employee_id)
                )
                .as_str(),
            )
            .clone();

        let branch = emp.dunkin_branch.expect("Dunkin branch was set in parsing");

        *response.payment_map_branch.entry(branch).or_default() +=
            transaction.amount.expect("Amount was set in parsing");
    }

    Ok(Json(response))
}
