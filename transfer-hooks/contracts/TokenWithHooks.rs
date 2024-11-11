use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    program::invoke,
    system_instruction,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Initialize accounts iterator
    let accounts_iter = &mut accounts.iter();
    let from_account = next_account_info(accounts_iter)?;
    let to_account = next_account_info(accounts_iter)?;
    let royalty_receiver_account = next_account_info(accounts_iter)?;

    // Parse the transfer amount from instruction_data
    let amount = instruction_data
        .get(..8)
        .and_then(|slice| slice.try_into().ok())
        .map(u64::from_le_bytes)
        .ok_or(ProgramError::InvalidInstructionData)?;

    // Calculate royalty amount (5%)
    let royalty_percentage = 5;
    let royalty_amount = amount * royalty_percentage / 100;

    // Transfer royalty amount to the royalty receiver
    invoke(
        &system_instruction::transfer(from_account.key, royalty_receiver_account.key, royalty_amount),
        &[from_account.clone(), royalty_receiver_account.clone()],
    )?;

    // Transfer the remaining amount to the recipient
    let remaining_amount = amount - royalty_amount;
    invoke(
        &system_instruction::transfer(from_account.key, to_account.key, remaining_amount),
        &[from_account.clone(), to_account.clone()],
    )?;

    Ok(())
}