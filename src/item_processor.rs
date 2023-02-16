use std::collections::HashMap;
use crate::parse_json::{parse_listing, Sale};
use lazy_static::lazy_static;
use crate::config::Data;
use serde_json::{Value, Map, Number};

lazy_static! {
    static ref CONFIG: Data = crate::config::read_config();
    static ref WEB_CLIENT: reqwest::blocking::Client = reqwest::blocking::Client::new();
}

const MINIMUM_PRICE: f64 = 15.0;
const PERCENTAGE_LIMIT: f64 = 80.0;
const IMPORTANT_CATEGORIES: &[&str] = &["Knife", "Gloves"];
const IMPORTANT_STICKERS: &[&str] = &[
    // "Eternal Fire | Antwerp 2022", // Example
    "Titan | Katowice 2014",
    "Titan (Holo) | Katowice 2014",
    "iBUYPOWER | Katowice 2014",
    "iBUYPOWER (Holo) | Katowice 2014",
    "Fnatic | Katowice 2014",
    "Fnatic (Holo) | Katowice 2014",
    "Reason Gaming | Katowice 2014",
    "Reason Gaming (Holo) | Katowice 2014",
    "Natus Vincere | Katowice 2014",
    "Natus Vincere (Holo) | Katowice 2014",
    "compLexity Gaming | Katowice 2014",
    "compLexity Gaming (Holo) | Katowice 2014",
    "Ninjas in Pyjamas | Katowice 2014",
    "Ninjas in Pyjamas (Holo) | Katowice 2014",
    "Virtus.Pro | Katowice 2014",
    "Virtus.Pro (Holo) | Katowice 2014",
    "Clan-Mystik | Katowice 2014",
    "Clan-Mystik (Holo) | Katowice 2014",
    "3DMAX | Katowice 2014",
    "3DMAX (Holo) | Katowice 2014",
    "mousesports | Katowice 2014",
    "mousesports (Holo) | Katowice 2014",
    "Vox Eminor | Katowice 2014",
    "Vox Eminor (Holo) | Katowice 2014",
    "Team Dignitas | Katowice 2014",
    "Team Dignitas (Holo) | Katowice 2014",
    "HellRaisers | Katowice 2014",
    "HellRaisers (Holo) | Katowice 2014",
    "Team LDLC.com | Katowice 2014",
    "Team LDLC.com (Holo) | Katowice 2014",
    "LGB eSports | Katowice 2014",
    "LGB eSports (Holo) | Katowice 2014"
];

fn log_to_webhook(message: String) {
    let binding = CONFIG.discord.role_id.clone();
    let role_id = binding.as_str();
    let webhook = CONFIG.discord.skinport_url.clone();
    let skinport_url = webhook.as_str();

    let message_to_send = format!("<@&{}>\n{}", role_id, message);

    let mut map = HashMap::new();
    map.insert("content", message_to_send);

    WEB_CLIENT.post(skinport_url)
        .json(&map)
        .send()
        .expect("Unable to send message to webhook");
}

pub(crate) fn disc_embed_webhook(mesag: &str) {
    println!("FUCKERS IN THE ARYAN TRAPHOUSE!!!");
    let binding = CONFIG.discord.role_id.clone();
    let role_id = binding.as_str();
    let webhook = CONFIG.discord.skinport_url.clone();
    let skinport_url = webhook.as_str();

    let raw_text_mention = format!("<@&.{}>", role_id);

    let mut innermap = Map::new();
    innermap.insert("description".to_string(),Value::String(mesag.to_string()));
    innermap.insert("title".to_string(),Value::String("[$] Skin Notification".to_string()));

    let mut map = Map::new();
    map.insert("content".to_string(), Value::String(raw_text_mention));
    map.insert("username".to_string(), Value::String("Listing Tracker".to_string()));
    map.insert("avatar_url".to_string(),Value::String("https://i.kym-cdn.com/entries/icons/original/000/040/219/cover1.jpg".to_string()));
    map.insert("embeds".to_string(), Value::Array(vec));

    println!("FUCK 12: {}", serde_json::to_string(&map).unwrap());

    WEB_CLIENT.post(skinport_url)
        .json(&map)
        .send()
        .expect("Unable to send message to webhook");
}

pub(crate) fn process_new_listing(listing: String) {
    let listing = parse_listing(&listing);
    if listing.event_type != "listed" {
        return;
    }

    let mut message: String = String::from("");

    listing.sales.iter().for_each(|sale| {
        let concat = check_listing_value(sale);
        if concat != String::from("") {
            message.push_str(concat.as_str());
        }
    });

    if message != String::from("") {
        disc_embed_webhook(message.as_str());
    }

    return;
}

fn check_listing_value(sale: &Sale) -> String {
    let item = sale.clone();
    let item_name = item.market_name;
    let list_price_in_pounds = item.sale_price as f64 / 100.0;
    let item_stickers = item.stickers;

    if list_price_in_pounds < MINIMUM_PRICE {
        // println!("{} is below the minimum price of £{}.", item_name, MINIMUM_PRICE);
        return "".to_string()
    }

    let suggested_price_in_pounds = item.suggested_price as f64 / 100.0;
    let skinport_link = format!("https://skinport.com/item/{}/{}", item.url, item.sale_id);
    let should_buy = should_purchase(sale.clone());

    if !item_stickers.is_empty() {
        let kato_stickers: Vec<String>;
        kato_stickers = item_stickers.iter().filter(|sticker | IMPORTANT_STICKERS.contains(&&*sticker.name)).map(|stick | stick.clone().name).collect();
        if !kato_stickers.is_empty() {
            let sticker_string = kato_stickers.join("\n");
            let msg = format!("[!] KATO/VALUABLE STICKERS: {}\n[$] Item: `{}`\nListed Price: `£{}`\nSuggested Price: `£{}`\nPercentage: `%{}`\nLink: {}\n", sticker_string, item_name, list_price_in_pounds, suggested_price_in_pounds, should_buy, skinport_link);
            return msg;
        }
        return "".to_string();
    }

/*    if !item_stickers.is_empty() {
        item_stickers.iter().for_each(|sticker_data| {
            let sticker_name = sticker_data.clone().name;
            if IMPORTANT_STICKERS.contains(&sticker_name.as_str()) {
                kato_stickers.push(sticker_name);
            }
        });

    }*/

    if should_buy != String::from("") {
        let msg = format!("[$] Item: `{}`\nListed Price: `£{}`\nSuggested Price: `£{}`\nPercentage: `%{}`\nLink: {}\n", item_name, list_price_in_pounds, suggested_price_in_pounds, should_buy, skinport_link);
        return msg;
    }

    "".to_string()
}


fn should_purchase(item: Sale) -> String {
    let listed_price = item.sale_price;
    let suggested_price = item.suggested_price;
    let item_category = item.category;
    //let item_stickers = item.stickers;

    if IMPORTANT_CATEGORIES.contains(&item_category.as_str()) && (listed_price as f64 / 100.0) <= 100.0 && (suggested_price as f64 / 100.0) > 200.0 {
        return "BIG%WTFBUYITLOSER".to_string();
    }

    let percent_off = listed_price as f64 / suggested_price as f64 * 100.0;
    if percent_off <= PERCENTAGE_LIMIT {
        return percent_off.to_string();
    }

    "".to_string() // No cracked price, No %<%lim
}