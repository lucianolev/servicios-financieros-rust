use crate::account_transaction::AccountTransaction;

pub struct ReceptiveAccount {
    transactions: Vec<Box<dyn AccountTransaction>>,
}

impl ReceptiveAccount {}

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

    pub fn has_registered(&self, a_transaction: Box<dyn AccountTransaction>) -> bool {
        return self.transactions.contains(&a_transaction);
    }

    pub fn transactions(&self) -> &Vec<Box<dyn AccountTransaction>> {
        return &self.transactions;
    }
}
