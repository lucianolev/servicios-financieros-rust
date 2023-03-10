#[cfg(test)]
mod tests {
    use crate::account_transaction::AccountTransaction;
    use crate::deposit::Deposit;
    use crate::receptive_account::ReceptiveAccount;
    use crate::withdraw::Withdraw;

    #[test]
    fn test01_receptive_account_have_zero_as_balance_when_created() {
        let account = ReceptiveAccount::new();
        assert_eq!(account.balance(), 0);
    }

    #[test]
    fn test02_deposit_increases_balance_on_transaction_value() {
        let mut account = ReceptiveAccount::new();
        Deposit::register_on(100, &mut account);
        assert_eq!(account.balance(), 100);
    }

    #[test]
    fn test03_withdraw_decreases_balance_on_transaction_value() {
        let mut account = ReceptiveAccount::new();
        Deposit::register_on(100, &mut account);
        Withdraw::register_on(50, &mut account);
        assert_eq!(account.balance(), 50);
    }

    #[test]
    fn test04_withdraw_value_must_be_positive() {
        let mut account = ReceptiveAccount::new();
        let withdraw_value = 50;
        let withdraw = Withdraw::register_on(withdraw_value, &mut account);
        assert_eq!(withdraw.value(), withdraw_value);
    }

    #[test]
    fn test05_receptive_account_know_its_registered_transactions() {
        let mut account = ReceptiveAccount::new();
        let deposit = Deposit::register_on(100, &mut account);
        let withdraw = Withdraw::register_on(50, &mut account);
        assert!(account.has_registered(deposit));
        assert!(account.has_registered(withdraw));
    }

    #[test]
    fn test06_receptive_account_does_not_know_not_registered_transactions() {
        let account = ReceptiveAccount::new();
        let deposit = Deposit::of(100);
        let withdraw = Withdraw::of(50);
        assert!(!account.has_registered(deposit));
        assert!(!account.has_registered(withdraw));
    }

    #[test]
    fn test07_account_knows_its_transactions() {
        let mut account = ReceptiveAccount::new();
        let deposit = Deposit::register_on(100, &mut account);
        assert_eq!(account.transactions().len(), 1);
        let transaction_box = Box::new(deposit) as Box<dyn AccountTransaction>;
        assert!(account.transactions().contains(&transaction_box));
    }
}
