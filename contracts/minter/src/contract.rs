#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, WasmMsg, to_json_binary};
use cw2::set_contract_version;
use cw721_metadata_onchain::{MintMsg, Metadata};

use crate::achievements::achievements_middleware;
use crate::error::ContractError;
use crate::helpers::{get_hash, get_random_fish, update_seed, save_checked_mint_data};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, CollectionResponse};
use crate::state::{COLLECTION, INDEX, MINT_COIN, SEED};

const CONTRACT_NAME: &str = "crates.io:fish_minter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    COLLECTION.save(deps.storage, &msg.collection.to_string())?;
    INDEX.save(deps.storage, &0u64)?;
    SEED.save(deps.storage, &get_hash(env.block.time.to_string().as_bytes()))?;
    
    save_checked_mint_data(deps.storage, msg.mint_data)?;

    let owner = msg.owner.unwrap_or(info.sender);
    cw_ownable::initialize_owner(deps.storage, deps.api, Some(owner.as_str()))?;
    
    Ok(Response::new()
        .add_attributes(vec![
            ("action", "instantiate"),
            ("owner", owner.as_str()),
            ("collection", msg.collection.as_str())
        ])
    )
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Mint {} => fish(deps, env, info),
    }
}


pub fn fish(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let to_pay = MINT_COIN.load(deps.storage)?;
    let paid = cw_utils::must_pay(&info, &to_pay.denom)?;

    if to_pay.amount != paid {
        return Err(ContractError::InvalidAmount(to_pay.amount.u128(), paid.u128()));
    }

    let collection = COLLECTION.load(deps.storage)?;
    let new_index = INDEX.load(deps.storage)? + 1;

    let mint_data = get_random_fish(deps.as_ref())?;
    
    let mut res = Response::new();
    
    if mint_data.name != "No Fish" {
        res = res.add_message(WasmMsg::Execute { 
            contract_addr: collection.to_string(), 
            msg: to_json_binary(&cw721_metadata_onchain::ExecuteMsg::Mint(MintMsg {
                token_id: new_index.to_string(),
                owner: info.sender.to_string(),
                token_uri: None,
                extension: Some(Metadata {
                    image: mint_data.image,
                    name: Some(mint_data.name.clone()),
                    image_data: None,
                    external_url: None,
                    description: None,
                    attributes: None,
                    background_color: None,
                    animation_url: None,
                    youtube_url: None,
                }),
            }))?,
            funds: vec![] 
        })
        .add_attributes(vec![
            ("action", String::from("fish")),
            ("status", String::from("success")),
            ("owner", info.sender.to_string()),
            ("token_id", new_index.to_string()),
            ("fish", mint_data.name),
        ]);

        INDEX.save(deps.storage, &new_index)?;

    } else {
        res = res.add_attributes(vec![
            ("action", String::from("fish")),
            ("status", String::from("fail")),
        ]);
    }

    update_seed(deps, env, new_index, info.sender.as_str())?;
    achievements_middleware(&mut res)?;
    
    Ok(res)
}



#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Collection {} => to_json_binary(&CollectionResponse { 
            address: COLLECTION.load(deps.storage)?.to_string() 
        }),
    }
}

