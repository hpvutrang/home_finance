use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum AccountFamily {
    Asset,
    Liability,
    Equity,
    Revenue,
    Expense,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Account {
    pub name: String,
    pub family: AccountFamily,
}

impl Clone for Account {
    fn clone(&self) -> Self {
        Account {
            name: self.name.clone(),
            family: self.family.clone(),
        }
    }
}

impl Clone for AccountFamily {
    fn clone(&self) -> Self {
        match self {
            AccountFamily::Asset => AccountFamily::Asset,
            AccountFamily::Liability => AccountFamily::Liability,
            AccountFamily::Equity => AccountFamily::Equity,
            AccountFamily::Revenue => AccountFamily::Revenue,
            AccountFamily::Expense => AccountFamily::Expense,
        }
    }
}
