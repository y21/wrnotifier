use reqwest::Client;

pub fn driver_id_to_string(id: u8) -> Option<&'static str> {
    match id {
        0 => Some("Mario"),
        1 => Some("Baby Peach"),
        2 => Some("Waluigi"),
        3 => Some("Bowser"),
        4 => Some("Baby Daisy"),
        5 => Some("Dry Bones"),
        6 => Some("Baby Mario"),
        7 => Some("Luigi"),
        8 => Some("Toad"),
        9 => Some("Donkey Kong"),
        10 => Some("Yoshi"),
        11 => Some("Wario"),
        12 => Some("Baby Luigi"),
        13 => Some("Toadette"),
        14 => Some("Koopa Troopa"),
        15 => Some("Daisy"),
        16 => Some("Peach"),
        17 => Some("Birdo"),
        18 => Some("Diddy Kong"),
        19 => Some("King Boo"),
        20 => Some("Bowser Jr."),
        21 => Some("Dry Bowser"),
        22 => Some("Funky Kong"),
        23 => Some("Rosalina"),
        24 => Some("Small Mii Outfit A (Male)"),
        25 => Some("Small Mii Outfit A (Female)"),
        26 => Some("Small Mii Outfit B (Male)"),
        27 => Some("Small Mii Outfit B (Female)"),
        28 => Some("Small Mii Outfit C (Male)"),
        29 => Some("Small Mii Outfit C (Female)"),
        30 => Some("Medium Mii Outfit A (Male)"),
        31 => Some("Medium Mii Outfit A (Female)"),
        32 => Some("Medium Mii Outfit B (Male)"),
        33 => Some("Medium Mii Outfit B (Female)"),
        34 => Some("Medium Mii Outfit C (Male)"),
        35 => Some("Medium Mii Outfit C (Female)"),
        36 => Some("Large Mii Outfit A (Male)"),
        37 => Some("Large Mii Outfit A (Female)"),
        38 => Some("Large Mii Outfit B (Male)"),
        39 => Some("Large Mii Outfit B (Female)"),
        40 => Some("Large Mii Outfit C (Male)"),
        41 => Some("Large Mii Outfit C (Female)"),
        42 => Some("Medium Mii"),
        43 => Some("Small Mii"),
        44 => Some("Large Mii"),
        45 => Some("Peach Biker Outfit"),
        46 => Some("Daisy Biker Outfit"),
        47 => Some("Rosalina Biker Outfit"),
        _ => None,
    }
}

pub fn vehicle_id_to_string(id: u8) -> Option<&'static str> {
    match id {
        0x00 => Some("Standard Kart S"),
        0x01 => Some("Standard Kart M"),
        0x02 => Some("Standard Kart L"),
        0x03 => Some("Booster Seat"),
        0x04 => Some("Classic Dragster"),
        0x05 => Some("Offroader"),
        0x06 => Some("Mini Beast"),
        0x07 => Some("Wild Wing"),
        0x08 => Some("Flame Flyer"),
        0x09 => Some("Cheep Charger"),
        0x0A => Some("Super Blooper"),
        0x0B => Some("Piranha Prowler"),
        0x0C => Some("Tiny Titan"),
        0x0D => Some("Daytripper"),
        0x0E => Some("Jetsetter"),
        0x0F => Some("Blue Falcon"),
        0x10 => Some("Sprinter"),
        0x11 => Some("Honeycoupe"),
        0x12 => Some("Standard Bike S"),
        0x13 => Some("Standard Bike M"),
        0x14 => Some("Standard Bike L"),
        0x15 => Some("Bullet Bike"),
        0x16 => Some("Mach Bike"),
        0x17 => Some("Flame Runner"),
        0x18 => Some("Bit Bike"),
        0x19 => Some("Sugarscoot"),
        0x1A => Some("Wario Bike"),
        0x1B => Some("Quacker"),
        0x1C => Some("Zip Zip"),
        0x1D => Some("Shooting Star"),
        0x1E => Some("Magikruiser"),
        0x1F => Some("Sneakster"),
        0x20 => Some("Spear"),
        0x21 => Some("Jet Bubble"),
        0x22 => Some("Dolphin Dasher"),
        0x23 => Some("Phantom"),
        _ => None,
    }
}

pub fn to_webhook_url(webhook_id: &str, webhook_token: &str) -> String {
    format!(
        "https://discordapp.com/api/webhooks/{}/{}",
        webhook_id, webhook_token
    )
}

pub async fn validate_webhook(
    client: &Client,
    webhook_id: &str,
    webhook_token: &str,
) -> Result<(), reqwest::Error> {
    let url = to_webhook_url(webhook_id, webhook_token);
    client
        .get(&url)
        .send()
        .await?
        .error_for_status()
        .map(|_| ())
}
