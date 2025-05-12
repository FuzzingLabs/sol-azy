use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};

entrypoint!(process_instruction);

fn win() -> u64 {
    msg!("You win!");
    987654321
}

fn loose() -> u64 {
    msg!("You lose!");
    123456789
}

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.len() < 8 {
        msg!("Not enough data. Need two u32 values.");
        return Err(solana_program::program_error::ProgramError::InvalidInstructionData);
    }

    let a = u32::from_le_bytes(instruction_data[0..4].try_into().unwrap());
    let b = u32::from_le_bytes(instruction_data[4..8].try_into().unwrap());

    msg!("Inputs: {} + {}", a, b);

    let result = if a + b == 1337 {
        win()
    } else {
        loose()
    };

    msg!("Result: {}", result);

    Ok(())
}
