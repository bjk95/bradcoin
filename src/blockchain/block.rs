use uuid::Uuid;
use std::time::Instant;
use crate::blockchain::transaction::Transaction;

#[derive(Debug, Copy, Clone)]
pub struct Block{
    pub id: Uuid,
    pub index: u128,
    pub timestamp: Instant,
    pub transaction: Transaction,
}

impl Block {
   
}


