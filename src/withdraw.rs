use crate::account_transaction::AccountTransaction;
use crate::receptive_account::ReceptiveAccount;

pub struct Withdraw {
    value: i32,
}

impl Withdraw {
    pub fn register_on(a_value: i32, an_account: &mut ReceptiveAccount) {
        let withdraw = Withdraw::of(a_value);
        an_account.register(Box::new(withdraw));
    }

    fn of(a_value: i32) -> Withdraw {
        return Withdraw {
            value: a_value
        };
    }
}

impl AccountTransaction for Withdraw {
    fn value(&self) -> i32 {
        return self.value;
    }

    fn affect_balance(&self, balance: i32) -> i32 {
        return balance - self.value;
    }
}
