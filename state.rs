use borsh::{BorshDeserialize, BorshSerialize};


#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct UserAccount{

    pub user_address:[u8;32], //48
    pub user_name:String, //14
    pub user_name_length:u8, //1
    pub team_is_ready:u8, //1
    pub team_power:u64, //8
    pub team_class:String, //4
    pub defence_1:[u8;32],
    pub defence_2:[u8;32],
    pub middle:[u8;32],
    pub offence_1:[u8;32],
    pub offence_2:[u8;32],
    pub experienced:u64,

}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Player{
    
    pub nft_adress:[u8;32],          //44
    pub owner:[u8;32],               //44
    pub for_sale:String,            //2
    pub sale_required_price:u64,    //8
    pub for_rent:String,            //2
    pub rent_required_price:u64,    //8
    pub who_rents:[u8;32],           //44
    pub rented:u8,                  //1
    pub rented_for_time:u64,        //8
    pub rent_end_on:u64,            //8
    pub offence:u64,                //8
    pub defence:u64,                //8
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct UserName{

    pub user_name:String,

}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct RentOffer{

    pub rent_required_price:u64,
    pub rented_for_time:u64,

}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct SellOffer{

    pub sell_required_price:u64,

}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct UpgradePlayer{

    pub player_no:u8,
    pub exp:u64,

}
