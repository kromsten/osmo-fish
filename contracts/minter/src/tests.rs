mod tests {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info}, Response, DepsMut, Env, Storage, from_json, 
    };
    use fish_common::MintData;

    use crate::{contract::instantiate, msg::InstantiateMsg, ContractError, state::MINT_DATA};


    fn instantiate_contract(
        deps: DepsMut, 
        env: Env,
        mint_data: Vec<MintData>
    ) -> Result<Response, ContractError> {
        let info = mock_info("creator", &[]);

        let msg = InstantiateMsg {
            collection: "test".to_string(),
            mint_denom: "BAIT".to_string(),
            mint_data,
            owner: None,
        };

        let res = instantiate(deps, env, info, msg);
      
        res
    }

    #[test]
    fn can_instantiate() {


        let mut deps = mock_dependencies();
        let env = mock_env();

        // empty mint data
        let err = instantiate_contract(
            deps.as_mut(),
            env.clone(),
            vec![]
        ).unwrap_err();
        assert_eq!(err, ContractError::InvalidMintData {});
   
        let res = instantiate_contract(
            deps.as_mut(),
            env.clone(),
            vec![
                MintData {
                    name: "test".to_string(),
                    weight: Some(1),
                    image: None,
                }
            ]
        ).unwrap();

        assert_eq!(res.messages.len(), 0);
        assert_eq!(res.attributes.len(), 3);

    
        let mint_data: Option<Vec<u8>> = deps.storage.get("d".as_bytes());
        assert_eq!(mint_data.is_some(), true);

        let mint_data : Vec<MintData> = from_json(&mint_data.unwrap()).unwrap();
        assert_eq!(mint_data.len(), 2);

        let mint_data = MINT_DATA.load(&deps.storage).unwrap();
        assert_eq!(mint_data.len(), 2);

        let test_fish = mint_data.iter().find(|fish| fish.name == "test");
        assert_eq!(test_fish.is_some(), true);
        assert_eq!(test_fish.unwrap().weight, Some(1));

        let no_fish = mint_data.iter().find(|fish| fish.name == "No Fish");
        assert_eq!(no_fish.is_some(), true);
        assert_eq!(no_fish.unwrap().weight, Some(2));
    }


    #[test]
    fn multi_mint_data_ok() {

        let mut deps = mock_dependencies();
        let env = mock_env();

        instantiate_contract(
            deps.as_mut(),
            env.clone(),
            vec![
              
                MintData {
                    name: "Normal Fish".to_string(),
                    weight: Some(20),
                    image: None,
                },

                MintData {
                    name: "Tropical Fish".to_string(),
                    weight: Some(15),
                    image: None,
                },

                MintData {
                    name: "Blowfish".to_string(),
                    weight: Some(30),
                    image: None,
                },

                MintData {
                    name: "Crabs".to_string(),
                    weight: Some(8),
                    image: None,
                }
            ]
        ).unwrap();

  
        let mint_data = MINT_DATA.load(&deps.storage).unwrap();
        assert_eq!(mint_data.len(), 5);

        let tropical = mint_data.iter().find(|fish| fish.name == "Tropical Fish");
        assert_eq!(tropical.unwrap().weight, Some(15));

        let no_fish = mint_data.iter().find(|fish| fish.name == "No Fish");
        assert_eq!(no_fish.unwrap().weight, Some(60));
    }

}