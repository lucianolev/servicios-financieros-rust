pub trait AccountTransaction {
    fn value(&self) -> i32;
    fn affect_balance(&self, balance: i32) -> i32;
}

impl PartialEq for dyn AccountTransaction {
    fn eq(&self, other_transaction: &Self) -> bool {
        return self.value() == other_transaction.value();
    }
}
