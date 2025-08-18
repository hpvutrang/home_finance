use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::model::account::Account;

use crate::utils::{datefmt_serialize, datefmt_deserialize};

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct Entry {
    pub description: String,
    pub amount: f64,

    #[serde(serialize_with = "datefmt_serialize", deserialize_with = "datefmt_deserialize")]
    pub event_date: DateTime<Utc>, 

    pub credit: Account,
    pub debit: Account,
}

impl Clone for Entry {
    fn clone(&self) -> Self {
        Entry {
            description: self.description.clone(),
            amount: self.amount,
            event_date: self.event_date,
            credit: self.credit.clone(),
            debit: self.debit.clone(),
        }
    }
}
