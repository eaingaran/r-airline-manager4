use crate::utilities::get_response_text;
use crate::utilities::get_text_by_selector;
use crate::utilities::get_texts_by_selector;

pub(crate) async fn get_reputation(cookies: &str) -> (i16, i16) {
    let response = get_response_text("https://www.airlinemanager.com/marketing.php", cookies).await;

    let airline_reputation: i16 = get_text_by_selector(
        &response,
        "body > div > div:nth-child(1) > div:nth-child(1) > div",
    )
    .await
    .unwrap_or_else(|| "".to_string())
    .parse()
    .unwrap_or_default();

    let cargo_reputation: i16 = get_text_by_selector(
        &response,
        "body > div > div:nth-child(1) > div:nth-child(2) > div",
    )
    .await
    .unwrap_or_else(|| "".to_string())
    .parse()
    .unwrap_or_default();

    return (airline_reputation, cargo_reputation);
}

// Marketing campaign cheatsheet
// 'type': {1: 'Airline', 2: 'Cargo', 5: 'Eco Friendly'},
// 'campaign': {1: '5-10%', 2: '10-18%', 3: '19-25%', 4: '25-35%'},
// 'duration': {1: '4', 2: '8', 3: '12', 4: '16', 5: '20', 6: '24'}
// 'url_params': {'type': 'type', 'c': 'campaign', 'd': 'duration'}

pub(crate) async fn start_airline_campaign(cookies: &str) {
    get_response_text(
        &format!("https://www.airlinemanager.com/marketing_new.php?type=1&c=4&mode=do&d=6"),
        cookies,
    )
    .await;
}

pub(crate) async fn start_eco_campaign(cookies: &str) {
    get_response_text(
        &format!("https://www.airlinemanager.com/marketing_new.php?type=5&c=4&mode=do&d=6"),
        cookies,
    )
    .await;
}

pub(crate) async fn start_cargo_campaign(cookies: &str) {
    get_response_text(
        &format!("https://www.airlinemanager.com/marketing_new.php?type=2&c=4&mode=do&d=6"),
        cookies,
    )
    .await;
}

pub(crate) async fn get_active_campaigns(cookies: &str) -> Vec<String> {
    let response = get_response_text("https://www.airlinemanager.com/marketing.php", cookies).await;

    return get_texts_by_selector(&response, "#active-campaigns > table > tbody > tr").await;
}

pub(crate) async fn validate_campaigns(
    cookies: &str,
    airline_marketing_required: bool,
    eco_marketing_required: bool,
    cargo_marketing_required: bool,
) {
    if !!!(airline_marketing_required || eco_marketing_required || cargo_marketing_required) {
        return;
    }

    let active_campaigns: Vec<String> = get_active_campaigns(cookies).await;

    if !!!active_campaigns.contains(&"Eco friendly".to_string()) && eco_marketing_required {
        start_eco_campaign(cookies).await;
    }

    if !!!active_campaigns.contains(&"Airline reputation".to_string()) && airline_marketing_required
    {
        start_airline_campaign(cookies).await;
    }

    if !!!active_campaigns.contains(&"Cargo reputation".to_string()) && cargo_marketing_required {
        start_cargo_campaign(cookies).await;
    }
}
