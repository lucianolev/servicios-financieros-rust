use crate::account_transaction::AccountTransaction;

pub struct ReceptiveAccount {
    transactions: Vec<Box<dyn AccountTransaction>>,
}

impl ReceptiveAccount {
    pub fn new() -> ReceptiveAccount {
        return ReceptiveAccount {
            transactions: Vec::new()
        };
    }

    pub fn balance(self) -> i32 {
        let mut balance = 0;
        for transaction in self.transactions {
            balance = transaction.affect_balance(balance)
        };
        return balance;
    }

    pub fn register(&mut self, a_transaction: impl AccountTransaction + 'static) {
        self.transactions.push(Box::new(a_transaction));
    }
}
