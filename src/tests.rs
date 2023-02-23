#[cfg(test)]
mod tests {
    use crate::deposit::Deposit;
    use crate::receptive_account::ReceptiveAccount;

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
}
