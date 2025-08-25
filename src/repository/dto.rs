use chrono::{DateTime, Utc};

use crate::model;

pub struct Account {
    pub id: i32,
    pub name: String,
    pub family: i32,
}

pub struct Entry {
    #[allow(dead_code)]
    pub id: i32,

    pub description: String,
    pub amount: f64,
    pub event_date: DateTime<Utc>,
    pub credit_id: i32,
    pub debit_id: i32,
}

pub trait DtoModelNoRef<T> {
    fn from_model(t: &T) -> Self;
    fn to_model(&self) -> T;
}

impl DtoModelNoRef<model::account::Account> for Account {
    fn from_model(t: &model::account::Account) -> Self {
        Self {
            id: -1,
            name: t.name.clone(),
            family: family_to_int(&t.family),
        }
    }

    fn to_model(&self) -> model::account::Account {
        model::account::Account {
            name: self.name.clone(),
            family: family_from_int(self.family),
        }
    }
}

impl DtoModelNoRef<model::entry::Entry> for Entry {
    fn from_model(t: &model::entry::Entry) -> Self {
        Self {
            id: -1,
            description: t.description.clone(),
            amount: t.amount,
            event_date: t.event_date.clone(),
            credit_id: -1,
            debit_id: -1,
        }
    }

    fn to_model(&self) -> model::entry::Entry {
        model::entry::Entry {
            description: self.description.clone(),
            amount: self.amount,
            event_date: self.event_date.clone(),
            credit: model::account::Account {
                name: String::new(), // Placeholder, should fetch account details
                family: model::account::AccountFamily::Asset, // Placeholder, should fetch account details
            },
            debit: model::account::Account {
                name: String::new(), // Placeholder, should fetch account details
                family: model::account::AccountFamily::Asset, // Placeholder, should fetch account details
            },
        }
    }
}

/// Converts an integer to a model::account::AccountFamily enum.
/// Postgres stores account families as integers, so this function maps them accordingly.
fn family_from_int(family: i32) -> model::account::AccountFamily {
    match family {
        1 => model::account::AccountFamily::Asset,
        2 => model::account::AccountFamily::Liability,
        3 => model::account::AccountFamily::Equity,
        4 => model::account::AccountFamily::Revenue,
        5 => model::account::AccountFamily::Expense,
        _ => panic!("Invalid account family integer"),
    }
}

/// Converts a model::account::AccountFamily enum to an integer.
/// Postgres stores account families as integers, so this function maps them accordingly.
fn family_to_int(family: &model::account::AccountFamily) -> i32 {
    match family {
        model::account::AccountFamily::Asset => 1,
        model::account::AccountFamily::Liability => 2,
        model::account::AccountFamily::Equity => 3,
        model::account::AccountFamily::Revenue => 4,
        model::account::AccountFamily::Expense => 5,
    }
}
