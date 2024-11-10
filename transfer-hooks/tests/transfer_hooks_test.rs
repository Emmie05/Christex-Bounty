// tests/transfer_hooks_tests.rs

#[cfg(test)]
mod tests {
    use super::*;
    use spl_token::state::{Account, Mint};

    #[test]
    fn test_transfer_hook() {
        let from_account = Account::default(); // Initialize with default values
        let to_account = Account::default(); // Initialize with default values
        let mint = Mint::default(); // Initialize with default values
        let hook = TransferHook::new(10); // 10% royalties

        // Simulate a transfer
        let result = hook.on_transfer(1000, &from_account, &to_account, &mint);
        assert!(result.is_ok());

        // Test for insufficient funds
        from_account.amount = 50; // Less than royalties
        let result = hook.on_transfer(1000, &from_account, &to_account, &mint);
        assert!(result.is_err());
    }

    // Add more tests as needed
}