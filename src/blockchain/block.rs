use std::collections::hash_map::DefaultHasher;
use chrono::DateTime;
use uuid::Uuid;
use chrono::Utc;
use crate::blockchain::transaction::{Transaction, format_transactions, pretty_format_transactions};
use std::hash::{Hash, Hasher};


#[derive(Debug, Clone)]
pub struct Block{
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub transactions: Vec<Transaction>,
    pub previous_hash: u64
}

impl Block {
   pub fn pretty_fmt(self) -> String {
        format!("block id: {}, time: {}, previous: {}\n{}\n\n", self.id, self.timestamp.to_rfc3339().to_string(), self.previous_hash, pretty_format_transactions(self.transactions))
   }

   pub fn format(self) -> String{
    format!("{}{}{}{}", self.timestamp.to_rfc3339().to_string(), format_transactions(self.transactions), self.previous_hash, self.id,)
   }

   pub fn calculate_hash(self) -> u64 {
    let mut s = DefaultHasher::new();
    self.hash(&mut s);
    s.finish()
}
}

impl Hash for Block {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.clone().format().hash(state);
    }
}


#[cfg(test)]
mod tests {
    use uuid::Uuid;
use crate::blockchain::transaction::Transaction;
use chrono::Duration;
use chrono::Utc;
    use crate::blockchain::block::Block;

    #[test]
    fn hash_equal_with_same_block() {

        let b = Block{
            id: Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8").unwrap(),
            timestamp: Utc::now(),
            transactions: vec!(Transaction{
                from: Uuid::parse_str("de72546f-78ed-4b0e-ab4d-679a52eb9a5b").unwrap(),
                to: Uuid::parse_str("d136bd36-37a8-4bed-90d1-27c9cfafb821").unwrap(),
                amount: 3.9
            }),
            previous_hash: 14956075707643071887
        };

        let b2 = b.clone();

        assert_eq!(b.calculate_hash(), b2.calculate_hash());
    }

    #[test]
    fn hash_unequal_with_deliberately_different_value() {
        let b = Block{
            id: Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8").unwrap(),
            timestamp: Utc::now(),
            transactions: vec!(Transaction{
                from: Uuid::parse_str("de72546f-78ed-4b0e-ab4d-679a52eb9a5b").unwrap(),
                to: Uuid::parse_str("d136bd36-37a8-4bed-90d1-27c9cfafb821").unwrap(),
                amount: 3.9
            }),
            previous_hash: 14956075707643071887
        };

        let h1 = &b.clone().calculate_hash() + 1;
        let h2 = b.calculate_hash();

        assert_ne!(h1, h2);
    }

    #[test]
    fn hash_equal_with_same_inputs() {
        let time = Utc::now();
        let b1 = Block{
            id: Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8").unwrap(),
            timestamp: time,
            transactions: vec!(Transaction{
                from: Uuid::parse_str("de72546f-78ed-4b0e-ab4d-679a52eb9a5b").unwrap(),
                to: Uuid::parse_str("d136bd36-37a8-4bed-90d1-27c9cfafb821").unwrap(),
                amount: 3.9
            }),
            previous_hash: 14956075707643071887
        };
        let b2 = Block{
            id: Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8").unwrap(),
            timestamp: time,
            transactions: vec!(Transaction{
                from: Uuid::parse_str("de72546f-78ed-4b0e-ab4d-679a52eb9a5b").unwrap(),
                to: Uuid::parse_str("d136bd36-37a8-4bed-90d1-27c9cfafb821").unwrap(),
                amount: 3.9
            }),
            previous_hash: 14956075707643071887
        };

        assert_eq!(b1.calculate_hash(), b2.calculate_hash());
    }

    #[test]
    fn hash_equal_with_different_ids() {
        let time = Utc::now();
        let b1 = Block{
            id: Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c7").unwrap(),
            timestamp: time,
            transactions: vec!(Transaction{
                from: Uuid::parse_str("de72546f-78ed-4b0e-ab4d-679a52eb9a5b").unwrap(),
                to: Uuid::parse_str("d136bd36-37a8-4bed-90d1-27c9cfafb821").unwrap(),
                amount: 3.9
            }),
            previous_hash: 14956075707643071887
        };
        let b2 = Block{
            id: Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8").unwrap(),
            timestamp: time,
            transactions: vec!(Transaction{
                from: Uuid::parse_str("de72546f-78ed-4b0e-ab4d-679a52eb9a5b").unwrap(),
                to: Uuid::parse_str("d136bd36-37a8-4bed-90d1-27c9cfafb821").unwrap(),
                amount: 3.9
            }),
            previous_hash: 14956075707643071887
        };

        assert_ne!(b1.calculate_hash(), b2.calculate_hash());
    }


    #[test]
    fn hash_equal_with_different_timestamp() {
        let t1 = Utc::now();
        let t2 = Utc::now() + Duration::seconds(5);
        let b1 = Block{
            id: Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8").unwrap(),
            timestamp: t1,
            transactions: vec!(Transaction{
                from: Uuid::parse_str("de72546f-78ed-4b0e-ab4d-679a52eb9a5b").unwrap(),
                to: Uuid::parse_str("d136bd36-37a8-4bed-90d1-27c9cfafb821").unwrap(),
                amount: 3.9
            }),
            previous_hash: 14956075707643071887
        };
        let b2 = Block{
            id: Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8").unwrap(),
            timestamp: t2,
            transactions: vec!(Transaction{
                from: Uuid::parse_str("de72546f-78ed-4b0e-ab4d-679a52eb9a5b").unwrap(),
                to: Uuid::parse_str("d136bd36-37a8-4bed-90d1-27c9cfafb821").unwrap(),
                amount: 3.9
            }),
            previous_hash: 14956075707643071887
        };

        assert_ne!(b1.calculate_hash(), b2.calculate_hash());
    }

    #[test]
    fn hash_equal_with_different_from() {
        let time = Utc::now();
        let b1 = Block{
            id: Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8").unwrap(),
            timestamp: time,
            transactions: vec!(Transaction{
                from: Uuid::parse_str("de72546f-78ed-4b0e-ab4d-679a52eb9a5a").unwrap(),
                to: Uuid::parse_str("d136bd36-37a8-4bed-90d1-27c9cfafb821").unwrap(),
                amount: 3.9
            }),
            previous_hash: 14956075707643071887
        };
        let b2 = Block{
            id: Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8").unwrap(),
            timestamp: time,
            transactions: vec!(Transaction{
                from: Uuid::parse_str("de72546f-78ed-4b0e-ab4d-679a52eb9a5b").unwrap(),
                to: Uuid::parse_str("d136bd36-37a8-4bed-90d1-27c9cfafb821").unwrap(),
                amount: 3.9
            }),
            previous_hash: 14956075707643071887
        };

        assert_ne!(b1.calculate_hash(), b2.calculate_hash());
    }

    #[test]
    fn hash_equal_with_different_amount() {
        let time = Utc::now();
        let b1 = Block{
            id: Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8").unwrap(),
            timestamp: time,
            transactions: vec!(Transaction{
                from: Uuid::parse_str("de72546f-78ed-4b0e-ab4d-679a52eb9a5b").unwrap(),
                to: Uuid::parse_str("d136bd36-37a8-4bed-90d1-27c9cfafb821").unwrap(),
                amount: 3.8
            }),
            previous_hash: 14956075707643071887
        };
        let b2 = Block{
            id: Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8").unwrap(),
            timestamp: time,
            transactions: vec!(Transaction{
                from: Uuid::parse_str("de72546f-78ed-4b0e-ab4d-679a52eb9a5b").unwrap(),
                to: Uuid::parse_str("d136bd36-37a8-4bed-90d1-27c9cfafb821").unwrap(),
                amount: 3.9
            }),
            previous_hash: 14956075707643071887
        };

        assert_ne!(b1.calculate_hash(), b2.calculate_hash());
    }
}

