use crate::utilities::get_element_by_selector;
use reqwest;

#[tokio::main]
pub(crate) async fn get_pax_status(cookies: &str) -> (i16, i16, i16) {
    let client = reqwest::Client::new();

    let response = client
        .post("https://www.airlinemanager.com/hangars.php?type=pax")
        .header("Cookie", cookies)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let max_capacity: i16 = get_element_by_selector(
        &response,
        "body > div:nth-child(3) > div.col-12.p-4.text-center.exo.text-white > div.xl-text > span",
    )
    .await
    .trim()
    .parse()
    .unwrap_or_default();
    let in_use: i16 = get_element_by_selector(&response, "body > div:nth-child(3) > div.col-12.bg-white > table > tbody > tr:nth-child(1) > td:nth-child(3)").await.trim().parse().unwrap_or_default();
    let free: i16 = get_element_by_selector(&response, "body > div:nth-child(3) > div.col-12.bg-white > table > tbody > tr:nth-child(2) > td:nth-child(3) > span").await.trim().parse().unwrap_or_default();

    return (max_capacity, in_use, free);
}

#[tokio::main]
pub(crate) async fn get_cargo_status(cookies: &str) -> (i16, i16, i16) {
    let client = reqwest::Client::new();

    let response = client
        .post("https://www.airlinemanager.com/hangars.php?type=cargo")
        .header("Cookie", cookies)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let max_capacity: i16 = get_element_by_selector(
        &response,
        "body > div:nth-child(3) > div.col-12.p-4.text-center.exo.text-white > div.xl-text > span",
    )
    .await
    .trim()
    .parse()
    .unwrap_or_default();
    let in_use: i16 = get_element_by_selector(&response, "body > div:nth-child(3) > div.col-12.bg-white > table > tbody > tr:nth-child(1) > td:nth-child(3)").await.trim().parse().unwrap_or_default();
    let free: i16 = get_element_by_selector(&response, "body > div:nth-child(3) > div.col-12.bg-white > table > tbody > tr:nth-child(2) > td:nth-child(3) > span").await.trim().parse().unwrap_or_default();

    return (max_capacity, in_use, free);
}
