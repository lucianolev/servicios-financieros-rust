pub trait AccountTransaction {
    fn value(&self) -> i32;
    fn affect_balance(&self, balance: i32) -> i32;
}
