mod blockchain;

use uuid::Uuid;
use crate::blockchain::transaction::Transaction;
use crate::blockchain::blockchain::create_chain;

fn main() {
    let mut chain = create_chain();
    let w1 = Uuid::new_v4();
    let w2 = Uuid::new_v4();
    let w3 = Uuid::new_v4();
    let t1 = Transaction{
        from: w1,
        to: w2,
        amount: 10.1
    };
    let t2 = Transaction{
        from: w2,
        to: w3,
        amount: 5.2
    };
    let t3 = Transaction{
        from: w3,
        to: w1,
        amount: 3.9
    };
    chain = chain.append_new_block_to_chain(vec!(t1));
    chain = chain.append_new_block_to_chain(vec!(t2));
    chain = chain.append_new_block_to_chain(vec!(t3));

    chain.print_blocks()
}
