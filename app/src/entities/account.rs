use crate::schema::payee::Payee;
use crate::schema::payor::Payor;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DestAccount {
    pub holder_id: String,
    pub liability: Liability,
}

impl From<Payee> for DestAccount {
    fn from(payee: Payee) -> Self {
        Self {
            holder_id: payee.plaid_id.expect("plaid_id was set"),
            // Todo: what is mch?
            liability: Liability {
                mch_id: "mch_2".to_string(),
                account_number: payee
                    .loan_account_number
                    .expect("account number was set")
                    .to_string(),
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Liability {
    pub mch_id: String,
    pub account_number: String,
}

#[derive(Serialize, Deserialize)]
pub struct SourceAccount {
    pub holder_id: String,
    pub ach: ACH,
}

impl From<Payor> for SourceAccount {
    fn from(payer: Payor) -> Self {
        Self {
            holder_id: "".to_string(),
            ach: ACH {
                routing: payer.aba_routing.expect("routing num was set").to_string(),
                number: payer
                    .account_number
                    .expect("account number was set")
                    .to_string(),
                // Todo: should other xmls have different vals?
                ach_type: "checking".to_string(),
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ACH {
    pub routing: String,
    pub number: String,
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub ach_type: String,
}
