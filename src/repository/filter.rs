use std::marker::PhantomData;

pub enum EntryFields {
    EventDate,
}

impl Fields for EntryFields {}
impl ToSql for EntryFields {
    fn to_sql(&self) -> String {
        match self {
            EntryFields::EventDate => "event_date".to_string(),
        }
    }
}

pub trait ToSql {
    fn to_sql(&self) -> String;
}


pub trait Fields{}

#[allow(dead_code)]
pub enum Operator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

impl ToSql for Operator {
    fn to_sql(&self) -> String {
        match self {
            Operator::Equal => "=".to_string(),
            Operator::NotEqual => "!=".to_string(),
            Operator::GreaterThan => ">".to_string(),
            Operator::GreaterThanOrEqual => ">=".to_string(),
            Operator::LessThan => "<".to_string(),
            Operator::LessThanOrEqual => "<=".to_string(),
        }
    }
}

#[allow(dead_code)]
enum BinaryOperator {
    And,
    Or,
}

pub struct Filters<T> 
where T: Fields + ToSql 
{
    filter_sequences : Vec<String>,
    fantom: PhantomData<T>,
}

impl<T> Filters<T> 
where T: Fields + ToSql {
    pub(crate) fn new() -> Self {
        Filters {
            filter_sequences: vec![],
            fantom: PhantomData,
        }
    }

    pub(super) fn build(&self) -> String {
        self.filter_sequences.join(" ")
    }
}

impl<T> Filters<T> where T: Fields + ToSql {
    pub fn and(&mut self, field: &T, operator: Operator, value: String) -> &mut Self {
        if !self.filter_sequences.is_empty() {
            self.filter_sequences.push("AND".to_string());
        }
        self.filter_sequences.push(field.to_sql());
        self.filter_sequences.push(operator.to_sql());
        self.filter_sequences.push(value.to_string());
        self
    }

    #[allow(dead_code)]
    pub fn or(&mut self, field: &T, operator: Operator, value: String) -> &mut Self {
        if !self.filter_sequences.is_empty() {
            self.filter_sequences.push("OR".to_string());
        }
        self.filter_sequences.push(field.to_sql());
        self.filter_sequences.push(operator.to_sql());
        self.filter_sequences.push(value.to_string());
        self
    }
}

// Test filter builder
mod test {
    use super::*;

    #[allow(dead_code)]
    enum StudentFields {
        Name,
        Age,
    }

    impl Fields for StudentFields {}
    impl ToSql for StudentFields {
        fn to_sql(&self) -> String {
            match self {
                StudentFields::Name => "name".to_string(),
                StudentFields::Age => "age".to_string(),
            }
        }
    }

    #[test]
    fn test_filter_builder() {
        let mut filter_builder = Filters::<StudentFields> {
            filter_sequences: vec![],
            fantom: PhantomData,
        };

        filter_builder
            .and(&StudentFields::Name, Operator::Equal, "`Alice`".to_string())
            .or(&StudentFields::Age, Operator::GreaterThan, "20".to_string());

        let filter_sql = filter_builder.build();
        assert_eq!(filter_sql, "name = `Alice` OR age > 20");
    }
}