use crate::instruction::GameInstruction;
use crate::state::{UserAccount,Player,UserName,SellOffer,RentOffer,UpgradePlayer
};
use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint::ProgramResult,
  pubkey::Pubkey,
  sysvar::{clock::Clock, Sysvar,},
  system_instruction,
  program::invoke_signed,
  program_pack::Pack,
  keccak,
};

use spl_token::instruction::transfer;
use spl_token::instruction::close_account;
use spl_token::state::Account;


pub struct Processor;
impl Processor {
  pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
  ) -> ProgramResult {
    let instruction: GameInstruction = GameInstruction::unpack(instruction_data)?;

    match instruction {

      GameInstruction::RegisterUser {username} => {
        Self::register_user(accounts,username, program_id)
      }
      GameInstruction::GeneratePlayer {} => {
        Self::generate_player(accounts)
      }
      GameInstruction::BuyPlayer {} => {
        Self::buy_player(accounts, program_id)
      }      
      GameInstruction::RentAnotherPlayer {} => {
        Self::rent_another_player(accounts, program_id)
      }
      GameInstruction::SellPlayer {selloffer} => {
        Self::sell_player(accounts,selloffer)
      }
      GameInstruction::RentYourPlayer {rentoffer} => {
        Self::rent_your_player(accounts,rentoffer)
      }
      GameInstruction::SetTeam {} => {
        Self::set_team(accounts)
      }
      GameInstruction::Challenge {} => {
        Self::challenge(accounts, program_id)
      }
      GameInstruction::AbortRent {} => {
        Self::abort_rent(accounts)
      }
      GameInstruction::AbortSell {} => {
        Self::abort_sell(accounts)
      }
      GameInstruction::ClaimRentOverPlayer {} => {
        Self::claim_player_back(accounts)
      }
      GameInstruction::ClaimNFTPlayer {} => {
        Self::claim_nft_player(accounts)
      }
      GameInstruction::CreatePDA {} => {
        Self::create_pda(accounts,program_id)
      }
      GameInstruction::Upgrade {upgrade} => {
        Self::upgrade_player(accounts,upgrade)
      }
    }
  }

  fn register_user(
    accounts: &[AccountInfo],
    username: UserName,
    program_id:&Pubkey) -> ProgramResult {

    
    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let user: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let user_derived_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    let seed: String = String::from("acc");
    let derived_pubkey: Pubkey = Pubkey::create_with_seed(&user.key, &seed, &program_id)?;


    if user_derived_account.key != &derived_pubkey{panic!()}
    if user_derived_account.owner != program_id{panic!()}

    if username.user_name.len() > 10 {panic!();}

    let username_len: u8 = username.user_name.len() as u8;

    //user name can be 10 character long
    let mut user_name_holder: String = String::from("XXXXXXXXXX");
    let offset: &usize = &username.user_name.len();
    user_name_holder.replace_range(..offset, &username.user_name);



    let user_account: UserAccount = UserAccount{
      user_address:user.key.to_bytes(),
      user_name:user_name_holder,
      user_name_length:username_len,
      team_is_ready:0,
      team_power:0,
      team_class:"aa".to_string(),
      defence_1:[0;32],
      defence_2:[0;32],
      middle:[0;32],
      offence_1:[0;32],
      offence_2:[0;32],
      experienced:0
    };

    
    user_account.serialize(&mut &mut user_derived_account.data.borrow_mut()[..])?;
   
    Ok(())
  }
  fn generate_player(
    accounts: &[AccountInfo]) -> ProgramResult {

      let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

      let user: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let user_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let player: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let nft: &AccountInfo<'_> = next_account_info(accounts_iter)?;


      if user_ata.owner!=&spl_token::id(){panic!()}
      let user_ata_unpacked: Account = Account::unpack_from_slice(&user_ata.data.borrow())?;//
      if user.key != &user_ata_unpacked.owner{panic!()}//is the owner of ata user?
      if user_ata_unpacked.amount != 1{panic!()} //has user the nft?
      if  nft.key != &user_ata_unpacked.mint {panic!()}//is ata and nft(toekn mint) related?


      let nft_seed:&[u8] = &nft.key.to_bytes();
      let user_seed:&[u8] = &user.key.to_bytes();

      let rand: keccak::Hash = keccak::hashv(&[nft_seed,user_seed]);
      let rand_str: String = rand.to_string();
      let rand_bytes: &[u8] = rand_str.as_bytes();

      let offence:u64;
      let defence:u64;

      if rand_bytes[0] > 20 {
        offence = (rand_bytes[0] - 10) as u64;
      }else{
        offence = rand_bytes[0] as u64;
      }

      if rand_bytes[1] > 20 {
        defence = (rand_bytes[1] - 10) as u64;
      }else{
        defence = rand_bytes[1] as u64;
      }


      let player_data: Player = Player{
    
        nft_adress:nft.key.to_bytes(),
        owner:user.key.to_bytes(),
        for_sale:"aa".to_string(),
        sale_required_price:0,
        for_rent:"aa".to_string(),
        rent_required_price:0,
        who_rents:[0;32],//address
        rented:0,
        rented_for_time:0,
        rent_end_on:0,
        offence:offence,
        defence:defence,
    };


    player_data.serialize(&mut &mut player.data.borrow_mut()[..])?;
    
   
    Ok(())
  }//////////
  fn buy_player(
    accounts: &[AccountInfo],
    program_id:&Pubkey) -> ProgramResult {
  
      let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

      let user: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let player: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let player_nft: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let seller: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let transfer_lamports: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let token_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let pda: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let pda_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let user_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;


      let player_acc_data: Player = Player::try_from_slice(&player.data.borrow())?; 

      let owner_key: Pubkey = Pubkey::new_from_array(player_acc_data.owner);

      if seller.key != &owner_key{panic!()}


      let nft_key: Pubkey = Pubkey::new_from_array(player_acc_data.nft_adress);

      if player_nft.key != &nft_key{panic!()}//if nft and player not related panic

      if player.owner != program_id{panic!()} //if not a correct account panic

      let account_value: u64 = **transfer_lamports.lamports.borrow();

      if account_value < player_acc_data.sale_required_price {panic!()} //if not enough lamports to buy panic

      if player_acc_data.for_sale == "aa".to_string(){panic!()} //if not for sale panic


      let new_owner: Player = Player{
        nft_adress:player_acc_data.nft_adress,
        owner:user.key.to_bytes(),
        for_sale:"aa".to_string(),
        sale_required_price:0,
        for_rent:"aa".to_string(),
        rent_required_price:0,
        who_rents:player_acc_data.who_rents,
        rented:0,
        rented_for_time:0,
        rent_end_on:0,
        offence:player_acc_data.offence,
        defence:player_acc_data.defence,
      };

      //transfer token from pda_ATA to user_ata
      //close pda_ata & return value to seller

      let amount:u64 = 1;

      let trans_ix: solana_program::instruction::Instruction = transfer( &token_program.key,
          &pda_ata.key, 
          &user_ata.key, 
          &pda.key, 
          &[&pda.key], 
          amount)?;

      invoke_signed(
        &trans_ix,
        &[
          token_program.clone(),
          pda_ata.clone(),
          user_ata.clone(),
          pda.clone(),
        ],
        &[&[b"battleballs", &[254]]],
      )?;

      let close_ix: solana_program::instruction::Instruction = close_account(&token_program.key, 
        &pda_ata.key,
         &seller.key, 
         &pda.key,
         &[&pda.key])?;

      invoke_signed(
          &close_ix,
          &[
            token_program.clone(),
            pda_ata.clone(),
            seller.clone(),
            pda.clone(),
          ],
          &[&[b"battleballs", &[254]]],
        )?;

      **transfer_lamports.lamports.borrow_mut() -= player_acc_data.sale_required_price;
      **seller.lamports.borrow_mut() += player_acc_data.sale_required_price;

      new_owner.serialize(&mut &mut player.data.borrow_mut()[..])?;


    Ok(())
  }
  fn rent_another_player(
    accounts: &[AccountInfo],
    program_id:&Pubkey) -> ProgramResult {

      let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

      let user: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let player: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let seller: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let transfer_lamports: &AccountInfo<'_> = next_account_info(accounts_iter)?;


      let player_acc_data: Player = Player::try_from_slice(&player.data.borrow())?; 


      let owner_key: Pubkey = Pubkey::new_from_array(player_acc_data.owner);

      if seller.key != &owner_key{panic!()}

      if player.owner != program_id{panic!()} //if not a correct account panic

      let account_value: u64 = **transfer_lamports.lamports.borrow();

      if account_value < player_acc_data.sale_required_price {panic!()} //if not enough lamports to buy panic

      if player_acc_data.for_sale == "aa".to_string(){panic!()} //if not for rent panic


      let clock: Clock= Clock::get()?;
      let current_time: u64 = clock.unix_timestamp as u64;

      let rent_ends_on: u64 = &current_time + &player_acc_data.rented_for_time;

      let new_owner: Player = Player{
        nft_adress:player_acc_data.nft_adress,
        owner:player_acc_data.owner,
        for_sale:"aa".to_string(),
        sale_required_price:0,
        for_rent:"aa".to_string(),
        rent_required_price:0,
        who_rents:user.key.to_bytes(),
        rented:1,
        rented_for_time:player_acc_data.rented_for_time,
        rent_end_on:rent_ends_on,
        offence:player_acc_data.offence,
        defence:player_acc_data.defence,
      };

      **transfer_lamports.lamports.borrow_mut() -= player_acc_data.rent_required_price;
      **seller.lamports.borrow_mut() += player_acc_data.rent_required_price;

      new_owner.serialize(&mut &mut player.data.borrow_mut()[..])?;


    Ok(())
  }
  fn sell_player(
    accounts: &[AccountInfo],
    selloffer : SellOffer) -> ProgramResult {

      let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

      let user: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let player: &AccountInfo<'_> = next_account_info(accounts_iter)?;


      if !user.is_signer{panic!()}

      let player_acc_data: Player = Player::try_from_slice(&player.data.borrow())?;


      let owner_key: Pubkey = Pubkey::new_from_array(player_acc_data.owner);

      if user.key != &owner_key{panic!()} // if you are not the owner you cant sell

      if player_acc_data.rented != 0{panic!()} //if rented you cant sell

      //add price filter by changing the for sale_string according to price
      //for now it is just XX

      let forsale: Player = Player{
        nft_adress:player_acc_data.nft_adress,
        owner:player_acc_data.owner,
        for_sale:"XX".to_string(),
        sale_required_price:selloffer.sell_required_price,
        for_rent:"aa".to_string(),
        rent_required_price:0,
        who_rents:player_acc_data.who_rents,
        rented:0,
        rented_for_time:0,
        rent_end_on:0,
        offence:player_acc_data.offence,
        defence:player_acc_data.defence,
      };

      forsale.serialize(&mut &mut player.data.borrow_mut()[..])?;

   
    Ok(())
  }
  fn rent_your_player(
    accounts: &[AccountInfo],
    rentoffer: RentOffer) -> ProgramResult {

      let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

      let user: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let player: &AccountInfo<'_> = next_account_info(accounts_iter)?;

      if !user.is_signer{panic!()}

      let player_acc_data: Player = Player::try_from_slice(&player.data.borrow())?;

      let owner_key: Pubkey = Pubkey::new_from_array(player_acc_data.owner);

      if user.key != &owner_key{panic!()} // if you are not the owner you cant rent

      if player_acc_data.rented != 0{panic!()} //if already rented you cant rent

      //add price filter by changing the for_rent string according to price
      //for now it is just XX

      let forsale: Player = Player{
        nft_adress:player_acc_data.nft_adress,
        owner:player_acc_data.owner,
        for_sale:"aa".to_string(),
        sale_required_price:0,
        for_rent:"XX".to_string(),
        rent_required_price:rentoffer.rent_required_price,
        who_rents:player_acc_data.who_rents,
        rented:0,
        rented_for_time:rentoffer.rented_for_time,
        rent_end_on:0,
        offence:player_acc_data.offence,
        defence:player_acc_data.defence,
      };

      forsale.serialize(&mut &mut player.data.borrow_mut()[..])?;
    
   
    Ok(())
  }
  fn set_team(
    accounts: &[AccountInfo]) -> ProgramResult {

      let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

      let user: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let user_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let offence_1: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let offence_2: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let middle: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let defence_1: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let defence_2: &AccountInfo<'_> = next_account_info(accounts_iter)?;

      let offence_1_data: Player = Player::try_from_slice(&offence_1.data.borrow())?;
      let offence_2_data: Player = Player::try_from_slice(&offence_2.data.borrow())?;
      let middle_data: Player = Player::try_from_slice(&middle.data.borrow())?;
      let defence_1_data: Player = Player::try_from_slice(&defence_1.data.borrow())?;
      let defence_2_data: Player = Player::try_from_slice(&defence_2.data.borrow())?;
      let user_account_data: UserAccount = UserAccount::try_from_slice(&user_account.data.borrow())?;


      let user_key: Pubkey = Pubkey::new_from_array(user_account_data.user_address);

      if user.key != &user_key{panic!()}


      if offence_1_data.rented == 1 {
        let owner_address: Pubkey = Pubkey::new_from_array(offence_1_data.who_rents);
        if &owner_address != user.key{panic!()}
      }else{
        let owner_address: Pubkey = Pubkey::new_from_array(offence_1_data.owner);
        if &owner_address != user.key{panic!()}
      }
      if offence_2_data.rented == 1 {
        let owner_address: Pubkey = Pubkey::new_from_array(offence_2_data.who_rents);
        if &owner_address != user.key{panic!()}
      }else{
        let owner_address: Pubkey = Pubkey::new_from_array(offence_2_data.owner);
        if &owner_address != user.key{panic!()}
      }
      if middle_data.rented == 1 {
        let owner_address: Pubkey = Pubkey::new_from_array(middle_data.who_rents);
        if &owner_address != user.key{panic!()}
      }else{
        let owner_address: Pubkey = Pubkey::new_from_array(middle_data.owner);
        if &owner_address != user.key{panic!()}
      }
      if defence_1_data.rented == 1 {
        let owner_address: Pubkey = Pubkey::new_from_array(defence_1_data.who_rents);
        if &owner_address != user.key{panic!()}
      }else{
        let owner_address: Pubkey = Pubkey::new_from_array(defence_1_data.owner);
        if &owner_address != user.key{panic!()}
      }
      if defence_2_data.rented == 1 {
        let owner_address: Pubkey = Pubkey::new_from_array(defence_2_data.who_rents);
        if &owner_address != user.key{panic!()}
      }else{
        let owner_address: Pubkey = Pubkey::new_from_array(defence_2_data.owner);
        if &owner_address != user.key{panic!()}
      }

      if offence_1_data.for_sale == "aa"{panic!()}
      if offence_2_data.for_sale == "aa"{panic!()}
      if middle_data.for_sale == "aa"{panic!()}
      if defence_1_data.for_sale == "aa"{panic!()}
      if defence_1_data.for_sale == "aa"{panic!()}
      if offence_1_data.for_rent == "aa"{panic!()}
      if offence_2_data.for_rent == "aa"{panic!()}
      if middle_data.for_rent == "aa"{panic!()}
      if defence_1_data.for_rent == "aa"{panic!()}
      if defence_1_data.for_rent == "aa"{panic!()}

      let d1: u64 = defence_1_data.defence as u64;
      let d2: u64 = defence_2_data.defence as u64;
      let d1o: u64 = defence_1_data.defence as u64;
      let d2o: u64 = defence_2_data.defence as u64;
      let o1: u64 = offence_1_data.offence as u64;
      let o2: u64 = offence_2_data.offence as u64;
      let o1d: u64 = offence_1_data.offence as u64;
      let o2d: u64 = offence_2_data.offence as u64;
      let m1: u64 = middle_data.offence as u64;
      let m2: u64 = middle_data.defence as u64;

      let defence_power: u64 = ((d1+d2)*2)+((m1)*3);
      let offence_power: u64 = ((o1+o2)*2)+((m2)*3);
      let team_power: u64  = &defence_power + &offence_power+(d1o+d2o+o1d+o2d);

      let formation: UserAccount = UserAccount{
        user_address:user_account_data.user_address,
        user_name:user_account_data.user_name,
        user_name_length:user_account_data.user_name_length,
        team_is_ready:1,
        team_power:team_power,
        team_class:"XX".to_string(),
        defence_1:defence_1_data.nft_adress,
        defence_2:defence_2_data.nft_adress,
        middle:middle_data.nft_adress,
        offence_1:offence_1_data.nft_adress,
        offence_2:offence_2_data.nft_adress,
        experienced:0
      };

      formation.serialize(&mut &mut user_account.data.borrow_mut()[..])?;


    Ok(())
  }
  fn challenge(
    accounts: &[AccountInfo],
    program_id:&Pubkey) -> ProgramResult {

      let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

      let user: &AccountInfo<'_> = next_account_info(accounts_iter)?; 
      let user_account: &AccountInfo<'_> = next_account_info(accounts_iter)?; 
      let opponent_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;

      let user_account_data: UserAccount = UserAccount::try_from_slice(&user_account.data.borrow())?;
      let opponent_account_data: UserAccount = UserAccount::try_from_slice(&opponent_account.data.borrow())?;

      if user_account.owner != program_id{panic!()}
      if opponent_account.owner != program_id{panic!()}


      let user_key: Pubkey = Pubkey::new_from_array(user_account_data.user_address);

      if user.key != &user_key {panic!()}

      let  mut exp: u64 = user_account_data.experienced;

      if opponent_account_data.team_power > user_account_data.team_power{

        let difference: u64 = opponent_account_data.team_power - user_account_data.team_power;

        if difference < 5{
          exp += 7;

        }else if difference < 20{
          exp += 8;

        }else if difference < 35{
          exp += 9;

        }else if difference < 50{
          exp += 10;
          
        }else if difference < 65{
          exp += 111;
          
        }else if difference < 80{
          exp += 12;
          
        }else{
          exp += 13;

        }

      }else{

        let difference: u64 = opponent_account_data.team_power - user_account_data.team_power;

        if difference < 5{
          exp += 7;

        }else if difference < 20{
          exp += 6;

        }else if difference < 35{
          exp += 5;

        }else if difference < 50{
          exp += 4;
          
        }else if difference < 65{
          exp += 3;
          
        }else if difference < 80{
          exp += 2;
          
        }else{
          exp += 1;
        }

      }

      let seed:&[u8] = &user.key.to_bytes();


      let rand: keccak::Hash = keccak::hashv(&[seed,
      &user_account_data.user_name.to_string().as_bytes(),&opponent_account_data.user_name.as_bytes()]);

      let ran_str: String = rand.to_string();

      let ran_bytes: &[u8] = ran_str.as_bytes();

      let mut index: usize = 0;
      let mut opponent_score: i32 = 0;
      let mut user_score: i32 = 0;

      loop {
          
        let  opponent_team_power: u64 = opponent_account_data.team_power + ran_bytes[index] as u64;
        if opponent_team_power > user_account_data.team_power{
          opponent_score+=1;
        }else{
          user_score+=1;
        }
        index+=1;
        if index == 3{break}
      };

      loop {
  
        index+=1;

        let user_team_power: u64 = user_account_data.team_power + ran_bytes[index] as u64;
        if user_team_power > opponent_account_data.team_power{
          user_score+=1;
        }else{
          opponent_score+=1;
        }
        index+=1;
        if index == 4{break}
      };

      let experienced: UserAccount = UserAccount{
        user_address:user_account_data.user_address,
        user_name:user_account_data.user_name,
        user_name_length:user_account_data.user_name_length,
        team_is_ready:user_account_data.team_is_ready,
        team_power:user_account_data.team_power,
        team_class:user_account_data.team_class,
        defence_1:user_account_data.defence_1,
        defence_2:user_account_data.defence_2,
        middle:user_account_data.middle,
        offence_1:user_account_data.offence_1,
        offence_2:user_account_data.offence_2,
        experienced:exp,
      };


      if user_score > opponent_score {

        experienced.serialize(&mut &mut user_account.data.borrow_mut()[..])?;

      }


    
        Ok(())
  }  
  fn abort_rent(
    accounts: &[AccountInfo]) -> ProgramResult {

      let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

      let user: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let player: &AccountInfo<'_> = next_account_info(accounts_iter)?;

      if !user.is_signer{panic!()}

      let player_acc_data: Player = Player::try_from_slice(&player.data.borrow())?;

      let owner_key: Pubkey = Pubkey::new_from_array(player_acc_data.owner);

      if user.key != &owner_key{panic!()} // if you are not the owner you cant cancel rent

      if player_acc_data.rented != 0{panic!()} //if already rented you cant cancel rent

      //add price filter by changing the for_rent string according to price
      //for now it is just XX

      let abortrent: Player = Player{
        nft_adress:player_acc_data.nft_adress,
        owner:player_acc_data.owner,
        for_sale:"aa".to_string(),
        sale_required_price:0,
        for_rent:"aa".to_string(),
        rent_required_price:0,
        who_rents:player_acc_data.who_rents,
        rented:0,
        rented_for_time:0,
        rent_end_on:0,
        offence:player_acc_data.offence,
        defence:player_acc_data.defence,
      };

      abortrent.serialize(&mut &mut player.data.borrow_mut()[..])?;
   
    Ok(())
  }  
  fn abort_sell(
    accounts: &[AccountInfo]) -> ProgramResult {

      let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

      let user: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let player: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let token_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let pda: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let pda_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let user_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;


      if !user.is_signer{panic!()}

      let player_acc_data: Player = Player::try_from_slice(&player.data.borrow())?;

      let owner_key: Pubkey = Pubkey::new_from_array(player_acc_data.owner);

      if user.key != &owner_key{panic!()} // if you are not the owner you cant cancel rent

      if player_acc_data.rented != 0{panic!()} //if already rented you cant cancel rent


      let abortsale: Player = Player{
        nft_adress:player_acc_data.nft_adress,
        owner:player_acc_data.owner,
        for_sale:"aa".to_string(),
        sale_required_price:0,
        for_rent:"aa".to_string(),
        rent_required_price:0,
        who_rents:player_acc_data.who_rents,
        rented:0,
        rented_for_time:0,
        rent_end_on:0,
        offence:player_acc_data.offence,
        defence:player_acc_data.defence,
      };

      abortsale.serialize(&mut &mut player.data.borrow_mut()[..])?;

      let amount:u64 = 1;

    //transfer from pda_ata to user
      let trans_ix = transfer( &token_program.key,
        &pda_ata.key, 
        &user_ata.key, 
        &pda.key, 
        &[&pda.key], 
        amount)?;

      invoke_signed(
        &trans_ix,
        &[
          token_program.clone(),
          pda_ata.clone(),
          user_ata.clone(),
          pda.clone(),
        ],
        &[&[b"battleballs", &[254]]],
      )?;

      let close_ix = close_account(&token_program.key, 
        &pda_ata.key,
         &user.key, 
         &pda.key,
         &[&pda.key])?;

      invoke_signed(
          &close_ix,
          &[
            token_program.clone(),
            pda_ata.clone(),
            user.clone(),
            pda.clone(),
          ],
          &[&[b"battleballs", &[254]]],
        )?;


   
    Ok(())
  }////////////
  fn claim_player_back(
    accounts: &[AccountInfo]) -> ProgramResult {

      let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

      let user: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let player: &AccountInfo<'_> = next_account_info(accounts_iter)?;

      let player_acc_data: Player = Player::try_from_slice(&player.data.borrow())?;


      let owner_key: Pubkey = Pubkey::new_from_array(player_acc_data.owner);

      if user.key != &owner_key{panic!()} // if you are not the owner you cant claim back

      let clock: Clock= Clock::get()?;
      let current_time: u64 = clock.unix_timestamp as u64;

      if current_time < player_acc_data.rent_end_on{panic!()} // rent is no over yet


      let rentover: Player = Player{
        nft_adress:player_acc_data.nft_adress,
        owner:player_acc_data.owner,
        for_sale:"aa".to_string(),
        sale_required_price:0,
        for_rent:"aa".to_string(),
        rent_required_price:0,
        who_rents:[0;32],
        rented:0,
        rented_for_time:0,
        rent_end_on:0,
        offence:player_acc_data.offence,
        defence:player_acc_data.defence,
      };

      rentover.serialize(&mut &mut player.data.borrow_mut()[..])?;
   
    Ok(())
  }
  fn claim_nft_player(
    accounts: &[AccountInfo]) -> ProgramResult {

      let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

      let user: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let user_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let player: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let nft: &AccountInfo<'_> = next_account_info(accounts_iter)?;

      let player_acc_data: Player = Player::try_from_slice(&player.data.borrow())?;

      let player_nft_key: Pubkey = Pubkey::new_from_array(player_acc_data.nft_adress);

      if nft.key != &player_nft_key{panic!()} // nft adress of player account and nft(token mint) matches
      if user_ata.owner!=&spl_token::id(){panic!()}
      let user_ata_unpacked: Account = Account::unpack_from_slice(&user_ata.data.borrow())?;//
      if user.key != &user_ata_unpacked.owner{panic!()}//is the owner of ata user?
      if user_ata_unpacked.amount != 1{panic!()} //has user the nft?
      if  nft.key != &user_ata_unpacked.mint {panic!()}//is ata and nft(toekn mint) related?

      let mut user_address_holder: String = String::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
      let user_address: &String = &user.key.to_string();
      let offset: usize = user_address.len();
      user_address_holder.replace_range(..offset, &user_address);

      let rentover: Player = Player{
        nft_adress:player_acc_data.nft_adress,
        owner:user.key.to_bytes(),
        for_sale:"aa".to_string(),
        sale_required_price:0,
        for_rent:"aa".to_string(),
        rent_required_price:0,
        who_rents:[0;32],
        rented:0,
        rented_for_time:0,
        rent_end_on:0,
        offence:player_acc_data.offence,
        defence:player_acc_data.defence,
      };

      rentover.serialize(&mut &mut player.data.borrow_mut()[..])?;

    Ok(())
  }
  fn create_pda(
    accounts: &[AccountInfo],
    program_id:&Pubkey) -> ProgramResult {

      let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();
      let pda: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let user: &AccountInfo<'_> = next_account_info(accounts_iter)?;

      invoke_signed(
        &system_instruction::create_account( 
            &user.key, 
            &pda.key,
            1000000,
            0,
            &program_id
        ),
        &[
            user.clone(), 
            pda.clone(),
        ],
        &[&[b"battleballs", &[254]]],
      )?;
   
    Ok(())
  }
  fn upgrade_player(
    accounts: &[AccountInfo],
    upgrade: UpgradePlayer) -> ProgramResult {

      let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

      let user_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let player: &AccountInfo<'_> = next_account_info(accounts_iter)?;


      let mut player_data: Player = Player::try_from_slice(&player.data.borrow())?;
      let user_account_data: UserAccount = UserAccount::try_from_slice(&user_account.data.borrow())?;

      if user_account_data.experienced < upgrade.exp{panic!()}

      let mut new_team_power: u64 = 0;
        let old_team_power: u64 = user_account_data.team_power;
        let offence: u64 = player_data.offence as u64;
      let defence: u64 = player_data.defence as u64;
      let player_power: u64 = (offence+defence)*2;


      if upgrade.player_no == 1 {
        let player_key: Pubkey = Pubkey::new_from_array(user_account_data.offence_1);
        if player.key != &player_key{panic!()}
        new_team_power = (old_team_power - player_power) + (((offence+upgrade.exp)*2)+(defence+upgrade.exp));
      }else if upgrade.player_no == 2{
        let player_key: Pubkey = Pubkey::new_from_array(user_account_data.offence_2);
        if player.key != &player_key{panic!()}
        new_team_power = (old_team_power - player_power) + (((offence+upgrade.exp)*2)+(defence+upgrade.exp));
      }else if upgrade.player_no == 3{
        let player_key: Pubkey = Pubkey::new_from_array(user_account_data.middle);
        if player.key != &player_key{panic!()}
        new_team_power = (old_team_power - player_power) + (((offence+upgrade.exp)+(defence+upgrade.exp))*3);
      }else if upgrade.player_no == 4{
        let player_key: Pubkey = Pubkey::new_from_array(user_account_data.defence_1);
        if player.key != &player_key{panic!()}
        new_team_power = (old_team_power - player_power) + (((defence+upgrade.exp)*2)+(offence+upgrade.exp));
      }else if upgrade.player_no == 5{
        let player_key: Pubkey = Pubkey::new_from_array(user_account_data.defence_2);
        if player.key != &player_key{panic!()}
        new_team_power = (old_team_power - player_power) + (((defence+upgrade.exp)*2)+(offence+upgrade.exp));
      }


      let experience_left: u64 = user_account_data.experienced - upgrade.exp;


      let formation: UserAccount = UserAccount{
        user_address:user_account_data.user_address,
        user_name:user_account_data.user_name,
        user_name_length:user_account_data.user_name_length,
        team_is_ready:1,
        team_power:new_team_power,
        team_class:user_account_data.team_class,
        defence_1:user_account_data.defence_1,
        defence_2:user_account_data.defence_2,
        middle:user_account_data.middle,
        offence_1:user_account_data.offence_1,
        offence_2:user_account_data.offence_2,
        experienced:experience_left
      };

      player_data.offence += upgrade.exp;
      player_data.defence += upgrade.exp;
  
      player_data.serialize(&mut &mut player.data.borrow_mut()[..])?;
      formation.serialize(&mut &mut user_account.data.borrow_mut()[..])?;

   
    Ok(())
  }
  
}




