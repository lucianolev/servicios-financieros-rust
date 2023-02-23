struct Account {
    transactions: Vec<Deposit>
}

impl Account {
    fn new() -> Account {
        return Account {
            transactions: Vec::new()
        };
    }

    fn balance(self) -> i32 {
        let mut balance = 0;
        for transaction in self.transactions {
            balance = balance + transaction.value();
        }
        return balance;
    }

    fn register(&mut self, a_transaction: Deposit) {
        self.transactions.push(a_transaction);
    }
}

struct Deposit {
    value: i32
}

impl Deposit {
    fn register_on(a_value: i32, an_account: &mut Account) {
        let deposit = Deposit::of(a_value);
        an_account.register(deposit);
    }

    fn of(a_value: i32) -> Deposit {
        return Deposit {
            value: a_value
        }
    }

    fn value(self) -> i32 {
        return self.value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_receptive_account_have_zero_as_balance_when_created() {
        let account = Account::new();
        assert_eq!(account.balance(), 0);
    }

    #[test]
    fn test02_deposit_increases_balance_on_transaction_value() {
        let mut account = Account::new();
        Deposit::register_on(100, &mut account);
        assert_eq!(account.balance(), 100);
    }
}