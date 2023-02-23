use crate::deposit::Deposit;

pub struct ReceptiveAccount {
    transactions: Vec<Deposit>,
}

impl ReceptiveAccount {
    pub fn new() -> ReceptiveAccount {
        return ReceptiveAccount {
            transactions: Vec::new()
        };
    }

    pub fn balance(self) -> i32 {
        let transactions = self.transactions.iter();
        return transactions.fold(0,
                                 |sum, a_transaction: &Deposit| sum + a_transaction.value());
    }

    pub fn register(&mut self, a_transaction: Deposit) {
        self.transactions.push(a_transaction);
    }
}
