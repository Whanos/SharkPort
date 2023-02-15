use std::collections::HashMap;
use crate::parse_json::{parse_listing, Sale};
use lazy_static::lazy_static;
use crate::config::Data;

lazy_static! {
    static ref CONFIG: Data = crate::config::read_config();
    static ref WEB_CLIENT: reqwest::blocking::Client = reqwest::blocking::Client::new();
}

const MINIMUM_PRICE: f64 = 15.0;
const PERCENTAGE_LIMIT: f64 = 75.0;
const IMPORTANT_CATEGORIES: &[&str] = &["Knife", "Gloves"];

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
        log_to_webhook(message);
    }

    return;
}

fn check_listing_value(sale: &Sale) -> String {
    let item = sale.clone();
    let item_name = item.market_name;
    let list_price_in_pounds = item.sale_price as f64 / 100.0;

    if list_price_in_pounds < MINIMUM_PRICE {
        // println!("{} is below the minimum price of £{}.", item_name, MINIMUM_PRICE);
        return "".to_string()
    }

    let suggested_price_in_pounds = item.suggested_price as f64 / 100.0;
    let skinport_link = format!("https://skinport.com/item/{}/{}", item.url, item.sale_id);
    let should_buy = should_purchase(sale.clone());
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

    if IMPORTANT_CATEGORIES.contains(&item_category.as_str()) && (listed_price as f64 / 100.0) <= 100.0 && (suggested_price as f64 / 100.0) > 200.0 {
        return "BIG%WTFBUYITLOSER".to_string();
    }

    let percent_off = listed_price as f64 / suggested_price as f64 * 100.0;
    if percent_off <= PERCENTAGE_LIMIT {
        return percent_off.to_string();
    }

    "".to_string()
}