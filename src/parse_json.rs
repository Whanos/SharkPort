use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Listing {
    pub event_type: String,
    pub sales: Vec<Sale>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sale {
    pub appid: i64,
    pub asset_id: i64,
    pub assetid: String,
    pub bg_color: Value,
    pub can_have_screenshots: bool,
    pub category: String,
    #[serde(rename = "category_localized")]
    pub category_localized: String,
    pub classid: String,
    pub collection: String,
    #[serde(rename = "collection_localized")]
    pub collection_localized: String,
    pub color: String,
    pub currency: String,
    pub custom_name: Value,
    pub exterior: Value,
    pub family: String,
    #[serde(rename = "family_localized")]
    pub family_localized: String,
    pub finish: Value,
    pub id: i64,
    pub image: String,
    pub item_id: i64,
    pub link: Value,
    pub lock: String,
    pub market_hash_name: String,
    pub market_name: String,
    pub name: String,
    pub own_item: bool,
    pub pattern: Value,
    pub product_id: i64,
    pub quality: String,
    pub rarity: String,
    #[serde(rename = "rarity_localized")]
    pub rarity_localized: String,
    pub rarity_color: String,
    pub sale_id: i64,
    pub sale_price: i64,
    pub sale_status: String,
    pub sale_type: String,
    pub screenshots: Vec<Value>,
    pub souvenir: bool,
    pub stack_able: bool,
    pub stattrak: bool,
    pub steamid: String,
    pub stickers: Vec<Sticker>,
    pub sub_category: Value,
    #[serde(rename = "subCategory_localized")]
    pub sub_category_localized: Value,
    pub suggested_price: i64,
    pub tags: Vec<Tag>,
    pub text: String,
    pub title: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
    pub version: String,
    pub version_type: String,
    pub wear: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sticker {
    pub color: Value,
    pub img: String,
    pub name: String,
    #[serde(rename = "name_localized")]
    pub name_localized: String,
    pub slot: i64,
    #[serde(rename = "sticker_id")]
    pub sticker_id: Value,
    #[serde(rename = "type")]
    pub type_field: Value,
    #[serde(rename = "type_localized")]
    pub type_localized: Value,
    pub value: Value,
    pub wear: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub name: String,
    #[serde(rename = "name_localized")]
    pub name_localized: String,
}


pub(crate) fn parse_listing(json: &str) -> Listing {
    let listing: Listing = serde_json::from_str(json).unwrap_or_default();
    listing
}