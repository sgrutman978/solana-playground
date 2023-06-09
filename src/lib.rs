use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
// use std::str;

// use byteorder::{BigEndian, ReadBytesExt};

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// number of greetings
    pub highscore: u32,
    pub games: u32,
    pub score_sent: u32
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("Hello World Rust program entrypoint");

    // Iterating accounts is safer than indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }



    let payload = GreetingAccount::try_from_slice(&_instruction_data).unwrap();
    msg!("Greetf {}", payload.name);
    // Increment and store the number of times the account has been greeted
    let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;
    msg!("Greetf2 {}", payload.name);
    // let mut buf: &[u8] = _instruction_data;
    // let num = buf.read_u32::<BigEndian>().unwrap();
    
    // let num1 = u32::from_ne_bytes((*_instruction_data)[0..4].try_into().unwrap());
    // let num2 = u32::from_ne_bytes((*_instruction_data)[4..8].try_into().unwrap());
    // let num3 = u32::from_ne_bytes((*_instruction_data)[8..12].try_into().unwrap());
    // let num4 = u32::from_ne_bytes((*_instruction_data)[12..16].try_into().unwrap());

    // let txt = str::from_utf8(&_instruction_data[16..20]).unwrap().to_string();
    // greeting_account.greeting = "hellos".to_string();
    
    // let num_sum = num1 + num2 + num3 + num4;
    // greeting_account.score_sent = num_sum;
    // greeting_account.games += 1;
    // if num_sum > greeting_account.highscore {
    //     greeting_account.highscore = num_sum;
    // }

    greeting_account.highscore = payload.highscore;
    greeting_account.score_sent = payload.score_sent;
    msg!("Greetf3 {}", greeting_account.name);
    greeting_account.games = payload.games;
    greeting_account.name = (*payload.name).to_string();
    msg!("Greetf4 {}", payload.name);
    msg!("Greetf6 {}", greeting_account.games);

    // greeting_account.highscore = num;
    greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
    msg!("Greetf5 {}", payload.name);
    // msg!("Greeted {} time(s)!", greeting_account.highscore);

    Ok(())
}

// Sanity tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let instruction_data: Vec<u8> = Vec::new();

        let accounts = vec![account];

        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .highscore,
            0
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .highscore,
            1
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .highscore,
            2
        );
    }
}



// use solana_program::{
//     account_info::AccountInfo,
//     entrypoint,
//     entrypoint::ProgramResult,
//     pubkey::Pubkey,
//     msg,
// };

// // declare and export the program's entrypoint
// entrypoint!(process_instruction);

// // program entrypoint's implementation
// pub fn process_instruction(
//     program_id: &Pubkey,
//     accounts: &[AccountInfo],
//     instruction_data: &[u8]
// ) -> ProgramResult {
//     // log a message to the blockchain
//     msg!("Hello, world!");

//     // gracefully exit the program
//     Ok(())
// }



// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
