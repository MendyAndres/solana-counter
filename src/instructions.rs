use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::program_error::ProgramError;

#[derive(Debug,BorshSerialize, BorshDeserialize)]
pub struct UpdateArgs {
    pub value: u32,
}
pub enum CounterInstructions {
    Increment(UpdateArgs),
    Decrement(UpdateArgs),
    Update(UpdateArgs),
    Reset,
}

const INCREMENT: u8 = 0;
const DECREMENT: u8 = 1;
const UPDATE: u8    = 2;
const RESET: u8     = 3;

impl CounterInstructions {
    pub fn unpack_instruction(input: &[u8]) -> Result<Self,ProgramError> {
        let (&variant, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match variant {
            INCREMENT => Self::Increment(UpdateArgs::try_from_slice(rest)?),
            DECREMENT => Self::Decrement(UpdateArgs::try_from_slice(rest)?),
            UPDATE => Self::Update(UpdateArgs::try_from_slice(rest)?),
            RESET => Self::Reset,
            _ => return Err(ProgramError::InvalidInstructionData)
        })
    }
}