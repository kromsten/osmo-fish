use cosmwasm_std::{DepsMut, StdResult, StdError, Env, Deps, Storage};
use fish_common::MintData;
use rand::distributions::Distribution;
use rand_chacha::ChaChaRng;
use rand_chacha::rand_core::SeedableRng;
use sha2::{Digest, Sha256};

use crate::{state::{SEED, MINT_DATA}, ContractError};


pub fn get_hash(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}


pub fn update_seed(deps: DepsMut, env: Env, index: u64, sender: &str) -> StdResult<()> {
    let data = env.block.time.nanos().to_string()
        + index.to_string().as_str()
        + sender;

    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());

    SEED.update(deps.storage, |seed| {
        hasher.update(seed);
        let new_seed = hasher.finalize().into();
        Ok::<[u8; 32], StdError>(new_seed)
    })?;

    Ok(())
}


pub fn get_random_fish(
    deps: Deps,
) -> StdResult<MintData> {
    let fishes = MINT_DATA.load(deps.storage)?;
    
    let windex = rand::distributions::WeightedIndex::new(
        fishes.iter().map(|fish| fish.weight.unwrap())
    ).unwrap();

    let seed = SEED.load(deps.storage)?;
    let mut rng = ChaChaRng::from_seed(seed);
    
    Ok(fishes[windex.sample(&mut rng)].clone())
}


pub fn save_checked_mint_data(
    storage: &mut dyn Storage,
    mint_data: Vec<MintData>
) -> Result<(), ContractError> {

    let mut mint_data = mint_data; 
    
    if mint_data.is_empty() {
        return Err(ContractError::InvalidMintData {});
    }

    let weight_max = mint_data
        .iter()
        .max_by_key(|fish| fish.weight.unwrap())
        .unwrap().weight
        .unwrap();


    mint_data.push(MintData {
        name: "No Fish".to_string(),
        weight: Some(weight_max * 2),
        image: None,
    });

    MINT_DATA.save(storage, &mint_data)?;

    Ok(())
}