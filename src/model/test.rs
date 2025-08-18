#[cfg(test)]
mod tests {
    use chrono::DateTime;
    use chrono::Utc;

    use crate::model::account::*;
    use crate::model::entry::*;

    #[test]
    fn test_serialization() {

        let account = Account {
            name: "Test Account".to_string(),
            family: AccountFamily::Asset,
        };

        let expected_account_json = serde_json::json!({
            "name": "Test Account",
            "family": "Asset"
        });

        let account_json = serde_json::json!(&account);

        assert_eq!(expected_account_json, account_json);


        let credit = Account {
            name: "Credit Account".to_string(),
            family: AccountFamily::Liability,
        };

        let debit = Account {
            name: "Debit Account".to_string(),
            family: AccountFamily::Expense,
        };

        let entry = Entry {
            description: "Test Entry".to_string(),
            amount: 100.0,
            event_date: DateTime::parse_from_rfc3339("2023-10-01T12:00:00.000Z")
                .unwrap()
                .with_timezone(&Utc), 
            credit: credit,
            debit: debit,
        };

        let expected_entry_json = serde_json::json!({
            "description": "Test Entry",
            "amount": 100.0,
            "event_date": "2023-10-01T12:00:00Z",
            "credit": {
                "name": "Credit Account",
                "family": "Liability"
            },
            "debit": {
                "name": "Debit Account",
                "family": "Expense"
            }
        });

        let entry_json = serde_json::json!(&entry);

        assert_eq!(expected_entry_json, entry_json);
    }
}