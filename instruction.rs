use crate::error::MailError::InvalidInstruction;
use crate::state::{UserName,SellOffer,RentOffer,UpgradePlayer};
use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

#[derive(Debug, PartialEq)]
pub enum GameInstruction {

  RegisterUser{username: UserName},//
  GeneratePlayer,//user account
  BuyPlayer,//player account, user account, seller
  RentAnotherPlayer,//player account, user account, seller
  SellPlayer{selloffer : SellOffer},//player account, user account, 
  RentYourPlayer{rentoffer : RentOffer},//player account, user account, owner
  SetTeam,//user account, player address, nft adresses, //team formation
  Challenge,//player account opponent account
  AbortRent,//player account,user account
  AbortSell,//player account, user account
  ClaimRentOverPlayer,
  ClaimNFTPlayer,
  CreatePDA,
  Upgrade{upgrade:UpgradePlayer},

}

impl GameInstruction {
  pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
    let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
    
    Ok(match tag {
      0 => Self::RegisterUser{
        username: UserName::try_from_slice(&rest)?,
      },
      1 => Self::GeneratePlayer,
      3 => Self::BuyPlayer,
      4 => Self::RentAnotherPlayer,
      5 => Self::SellPlayer{
        selloffer: SellOffer::try_from_slice(&rest)?,
      },
      6 => Self::RentYourPlayer{
        rentoffer: RentOffer::try_from_slice(&rest)?,
      },
      7 => Self::SetTeam,
      8 => Self::Challenge,
      9 => Self::AbortRent,
      10 => Self::AbortSell,
      11 => Self::ClaimRentOverPlayer,
      12 => Self::ClaimNFTPlayer,
      13 => Self::CreatePDA,
      14 => Self::Upgrade{
        upgrade: UpgradePlayer::try_from_slice(&rest)?,
      },
      _ => return Err(InvalidInstruction.into()),
    })
  }
}
