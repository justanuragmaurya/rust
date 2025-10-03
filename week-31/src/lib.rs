use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo,next_account_info},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    entrypoint
};

entrypoint!(my_contract);

#[derive(BorshSerialize,BorshDeserialize)]
struct Data{
    count:u32
}

#[derive(BorshSerialize,BorshDeserialize)]
enum InstructionType{
    Incerement(u32),
    Decerement(u32)
}

pub fn my_contract(
    _program_id:&Pubkey,
    accounts:&[AccountInfo],
    instruction_data: &[u8]
)->ProgramResult{
    let acc = next_account_info(&mut accounts.iter())?;
    let mut data = Data::try_from_slice(&acc.data.borrow())?;

    match InstructionType::try_from_slice(instruction_data)?{
        InstructionType::Incerement(amount)=>{
            msg!("Incrementing");
            data.count += amount;
        }
        InstructionType::Decerement(amount)=>{
            msg!("Decrementing");
            data.count -= amount;
        }
    }

    data.serialize(&mut *acc.data.borrow_mut())?;
    msg!("Contract Complete");
    Ok(())
}