mod instructions;

use borsh::{ BorshDeserialize, BorshSerialize };
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};
use crate::instructions::CounterInstructions;

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct CounterAccount {
    pub counter: u32,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instructions_data: &[u8]
) -> ProgramResult {

    msg!("Counter program entry point");

    let instructions: CounterInstructions = CounterInstructions::unpack_instruction(instructions_data)?;

    let accounts_iter = &mut accounts.iter();
    let account: &AccountInfo = next_account_info(accounts_iter)?;

    let mut counter_account = CounterAccount::try_from_slice(&account.data.borrow())?;

    match instructions {
        CounterInstructions::Increment(args) => increment_counter(&mut counter_account, args.value),
        CounterInstructions::Decrement(args) => decrement_counter(&mut counter_account, args.value),
        CounterInstructions::Reset => reset_counter(&mut counter_account),
        CounterInstructions::Update(args) => update_counter(&mut counter_account, args.value),
    }

    counter_account.serialize(&mut &mut account.data.borrow_mut() [..])?;
    Ok(())
}

fn increment_counter(counter_account: &mut CounterAccount, value: u32) {
    counter_account.counter = counter_account.counter.saturating_add(value);
}

fn decrement_counter(counter_account: &mut CounterAccount, value: u32) {
    counter_account.counter = counter_account.counter.saturating_sub(value);
}

fn reset_counter(counter_account: &mut CounterAccount) {
    counter_account.counter = 0;
}

fn update_counter(counter_account: &mut CounterAccount, value: u32) {
    counter_account.counter = value;
}

#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;

    #[test]
    fn test_counter() {
        let program_id:Pubkey   = Pubkey::default();
        let key:Pubkey          = Pubkey::default();
        let mut lamports:u64    = 0;
        let mut data:Vec<u8>    = vec![0; size_of::<u32>()];
        let owner: Pubkey       = Pubkey::default();

        let account: AccountInfo = AccountInfo::new(
          &key,
          false,
          true,
          &mut lamports,
          &mut data,
          &owner,
          false,
          Epoch::default(),
        );

        let accounts: Vec<AccountInfo> = vec![account];

        let mut increment_instruction_data: Vec<u8> = vec![0];
        let mut decrement_instruction_data: Vec<u8> = vec![1];
        let mut update_instruction_data: Vec<u8> = vec![2];
        let reset_instruction_data: Vec<u8> = vec![3];

        // TEST INCREMENT
        let mut update_value = 25u32;
        increment_instruction_data.extend_from_slice(&update_value.to_le_bytes());
        process_instruction(&program_id, &accounts, &increment_instruction_data).unwrap();
        assert_eq!(CounterAccount::try_from_slice(&accounts[0].data.borrow()).unwrap().counter, 25);

        // TEST DECREMENT
        update_value = 22u32;
        decrement_instruction_data.extend_from_slice(&update_value.to_le_bytes());
        process_instruction(&program_id, &accounts, &decrement_instruction_data).unwrap();
        assert_eq!(CounterAccount::try_from_slice(&accounts[0].data.borrow()).unwrap().counter, 3);

        // TEST UPDATE
        update_value = 33u32;
        update_instruction_data.extend_from_slice(&update_value.to_le_bytes());
        process_instruction(&program_id, &accounts, &update_instruction_data).unwrap();
        assert_eq!(CounterAccount::try_from_slice(&accounts[0].data.borrow()).unwrap().counter, 33);

        // TEST RESET
        process_instruction(&program_id, &accounts, &reset_instruction_data).unwrap();
        assert_eq!(CounterAccount::try_from_slice(&accounts[0].data.borrow()).unwrap().counter, 0);
    }
}


