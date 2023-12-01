use cosmwasm_std::Coin;
use cw_storage_plus::Item;
use fish_common::MintData;

pub const COLLECTION : Item<String> = Item::new("c");
pub const MINT_COIN  : Item<Coin>   = Item::new("m");
pub const INDEX      : Item<u64>    = Item::new("i");
pub const MINT_DATA  : Item<Vec<MintData>> = Item::new("d");
pub const SEED       : Item<[u8; 32]> = Item::new("s");