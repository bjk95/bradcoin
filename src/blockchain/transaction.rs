
use fake::Dummy;
use fake::Fake;
use fake::uuid::UUIDv4;
use uuid::Uuid;


#[derive(Debug, Copy, Clone, Dummy)]
pub struct Transaction {
    #[dummy(faker = "UUIDv4")]
    pub from: Uuid,

    #[dummy(faker = "UUIDv4")]
    pub to: Uuid,
    pub amount: f64,
}

impl Transaction {
    pub fn format(self) -> String {
        format!("{}{}{}", self.from, self.to, self.amount)
    }
    pub fn pretty_format(self) -> String {
        format!("from: {}, to: {}, amount: {}", self.from, self.to, self.amount)
    }
}

pub fn format_transactions(txns: Vec<Transaction>) -> String {
    txns.iter().fold(String::new(), |x, y| format!("{}{}", x, y.format()))
}
pub fn pretty_format_transactions(txns: Vec<Transaction>) -> String {
    txns.iter().fold("Transactions".to_owned(), |x, y| format!("{}\n{}", x, y.pretty_format()))
}