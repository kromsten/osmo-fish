use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct MintData {
    pub name: String,
    pub image: String,
    pub rarity: Option<u8>,
}

impl From<cw721_metadata_onchain::Extension> for MintData {
    fn from(extension: cw721_metadata_onchain::Extension) -> Self {
        let ext = extension.unwrap();
        Self {
            name: ext.name.unwrap(),
            image: ext.image.unwrap(),
            rarity: None,
        }
    }
}
