use chrono::Utc;
use crate::blockchain::transaction::Transaction;
use uuid::Uuid;
use crate::blockchain::block::Block;

#[derive(Debug)]
pub struct BlockChain{
    pub chain: Vec<Block>
}

impl BlockChain {

    pub fn append_new_block_to_chain(mut self, transactions: Vec<Transaction>) -> BlockChain {

        match self.chain.last() {
            Some(b) => {
                let new_block = Block{
                    id: Uuid::new_v4(),
                    timestamp: Utc::now(),
                    transactions: transactions,
                    previous_hash: b.clone().calculate_hash()
                };

                self.chain.push(new_block);

                self
            }

            None => {
                let new_block = Block{
                    id: Uuid::new_v4(),
                    timestamp: Utc::now(),
                    transactions: transactions,
                    previous_hash: u64::MIN
                };

                self.chain.push(new_block);

                self
            }
        }
    }

    pub fn print_blocks(self) {
        let chain_string: String = self.chain.iter().map(|b| b.clone().pretty_fmt()).fold(String::new(), |x,y| format!("{}{}", x,y));
        println!("{}", chain_string)
    }
}

pub fn create_chain() -> BlockChain {
    BlockChain{
        chain: vec!{}
    }
}
