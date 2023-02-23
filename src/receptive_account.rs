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
        let transactions = self.transactions;
        let mut balance = 0;
        for transaction in transactions {
            balance = transaction.affect_balance(balance)
        };
        return balance;
    }

    pub fn register(&mut self, a_transaction: Box<dyn AccountTransaction>) {
        self.transactions.push(a_transaction);
    }
}
