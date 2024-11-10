// src/transfer_hooks.rs

use spl_token::state::{Account, Mint};

pub struct TransferHook {
    pub royalties_percentage: u8,
}

impl TransferHook {
    pub fn new(royalties_percentage: u8) -> Self {
        TransferHook { royalties_percentage }
    }

    pub fn on_transfer(
        &self,
        amount: u64,
        from: &Account,
        to: &Account,
        mint: &Mint,
    ) -> Result<(), String> {
        // Calculate royalties
        let royalties = (amount * self.royalties_percentage as u64) / 100;

        // Implement logic to handle royalties (e.g., transfer to a specific account)
        // This is a placeholder for actual royalty logic
        if from.amount < royalties {
            return Err("Insufficient funds for royalties".to_string());
        }

        // Deduct royalties from the sender's account
        // Placeholder: Actual token transfer logic should be implemented here

        Ok(())
    }
}// src/transfer_hooks.rs

use spl_token::state::{Account, Mint};

pub struct TransferHook {
    pub royalties_percentage: u8,
}

impl TransferHook {
    pub fn new(royalties_percentage: u8) -> Self {
        TransferHook { royalties_percentage }
    }

    pub fn on_transfer(
        &self,
        amount: u64,
        from: &Account,
        to: &Account,
        mint: &Mint,
    ) -> Result<(), String> {
        // Calculate royalties
        let royalties = (amount * self.royalties_percentage as u64) / 100;

        // Implement logic to handle royalties (e.g., transfer to a specific account)
        // This is a placeholder for actual royalty logic
        if from.amount < royalties {
            return Err("Insufficient funds for royalties".to_string());
        }

        // Deduct royalties from the sender's account
        // Placeholder: Actual token transfer logic should be implemented here

        Ok(())
    }
}