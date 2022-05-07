use crate::blockchain::transaction::Transaction;
use uuid::Uuid;
use std::time::Instant;
use crate::blockchain::block::Block;

#[derive(Debug)]
pub struct BlockChain{
    pub chain: Vec<Block>
}

impl BlockChain {
     pub fn new_block(self, previous_block: Block, transaction: Transaction) -> Block {
        Block{
            id: Uuid::new_v4(),
            index: previous_block.index + 1,
            timestamp: Instant::now(),
            transaction: transaction,
        }
    }
    pub fn append_new_block_to_chain(mut self, transaction: Transaction) -> BlockChain {

        match self.chain.last().copied() {
            Some(b) => {
                let new_block = Block{
                    id: Uuid::new_v4(),
                    index: b.index + 1,
                    timestamp: Instant::now(),
                    transaction: transaction,
                };

                self.chain.push(new_block);

                self
            }

            None => {
                let new_block = Block{
                    id: Uuid::new_v4(),
                    index: 0,
                    timestamp: Instant::now(),
                    transaction: transaction,
                };

                self.chain.push(new_block);

                self
            }
        }


    }
}

pub fn create_chain() -> BlockChain {
    BlockChain{
        chain: vec!{}
    }
}