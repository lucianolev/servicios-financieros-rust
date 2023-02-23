use crate::receptive_account::ReceptiveAccount;

pub struct Deposit {
    value: i32,
}

impl Deposit {
    pub fn register_on(a_value: i32, an_account: &mut ReceptiveAccount) {
        let deposit = Deposit::of(a_value);
        an_account.register(deposit);
    }

    fn of(a_value: i32) -> Deposit {
        return Deposit {
            value: a_value
        };
    }

    pub fn value(&self) -> i32 {
        return self.value;
    }
}
