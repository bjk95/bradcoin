mod blockchain;

use uuid::Uuid;
use crate::blockchain::transaction::Transaction;
use crate::blockchain::blockchain::create_chain;

fn main() {
    let chain = create_chain();
    let t1 = Transaction{
        from: Uuid::new_v4(),
        to: Uuid::new_v4(),
        amount: 10.1
    };
    let updated_chain = chain.append_new_block_to_chain(t1);

    print!("{:?}", updated_chain)
}
