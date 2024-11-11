#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::clock::Epoch;
    use solana_program::pubkey::Pubkey;
    use solana_program_test::*;
    use solana_sdk::{
        account::Account,
        signature::{Keypair, Signer},
        transaction::Transaction,
    };

    #[tokio::test]
    async fn test_transfer_with_royalty() {
        let program_id = Pubkey::new_unique();
        let from_pubkey = Pubkey::new_unique();
        let to_pubkey = Pubkey::new_unique();
        let royalty_receiver_pubkey = Pubkey::new_unique();

        let mut program_test = ProgramTest::new(
            "token_with_hooks",
            program_id,
            processor!(process_instruction),
        );

        // Add initial account with lamports
        program_test.add_account(
            from_pubkey,
            Account {
                lamports: 1000,
                ..Account::default()
            },
        );

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Create and sign transaction
        let mut transaction = Transaction::new_with_payer(
            &[instruction::transfer(
                &program_id,
                &from_pubkey,
                &to_pubkey,
                &royalty_receiver_pubkey,
                100,
            )],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);

        // Process transaction
        banks_client.process_transaction(transaction).await.unwrap();

        // Verify account balances
        let from_account = banks_client.get_account(from_pubkey).await.unwrap().unwrap();
        let to_account = banks_client.get_account(to_pubkey).await.unwrap().unwrap();
        let royalty_receiver_account = banks_client.get_account(royalty_receiver_pubkey).await.unwrap().unwrap();

        assert_eq!(from_account.lamports, 900);
        assert_eq!(to_account.lamports, 95);
        assert_eq!(royalty_receiver_account.lamports, 5);
    }
}