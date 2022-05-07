use uuid::Uuid;

#[derive(Debug, Copy, Clone)]
pub struct Transaction {
    pub from: Uuid,
    pub to: Uuid,
    pub amount: f64,
}