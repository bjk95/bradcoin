mod common;

use crate::common::generate_test_txns;

pub fn setup() {
    println!("Running integration tests")
}

#[test]
fn test_add() {
    // using common code.
    let t = generate_test_txns();
    
    println!("{:?}", t);
    assert_eq!(5, 5);
}