use fake::Faker;
use fake::Fake;
use bradcoin::blockchain::transaction::Transaction;

pub fn generate_test_txns() -> Vec<Transaction> {
    let txns: Vec<Transaction> = Faker.fake();

    txns
}