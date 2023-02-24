pub trait AccountTransaction {
    fn value(&self) -> i32;
    fn affect_balance(&self, balance: i32) -> i32;
    fn clone_boxed(&self) -> Box<dyn AccountTransaction>;
}

impl PartialEq for dyn AccountTransaction {
    fn eq(&self, other_transaction: &Self) -> bool {
        return self.value() == other_transaction.value();
    }
}

impl Clone for Box<dyn AccountTransaction> {
    fn clone(&self) -> Self {
        self.clone_boxed()
    }
}
