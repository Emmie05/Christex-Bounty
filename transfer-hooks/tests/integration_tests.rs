// tests/integration_tests.rs

#[cfg(test)]
mod tests {
    use super::*;
    use my_sol_token::MyToken;

    #[test]
    fn test_token_creation() {
        let mint = Pubkey::new_unique();
        let token = MyToken::new(mint, 9);
        assert_eq!(token.decimals, 9);
    }

    // Add more integration tests as needed
}