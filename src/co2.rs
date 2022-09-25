use crate::utilities::get_attr_by_selector;
use crate::utilities::get_response_text;
use crate::utilities::get_text_by_selector;

#[tokio::main]
pub(crate) async fn get_status(cookies: &str) -> (i16, i32, i32, i32, String) {
    let response = get_response_text("https://www.airlinemanager.com/co2.php", cookies).await;

    let price: i16 = get_text_by_selector(
        &response,
        "#co2Main > div > div:nth-child(2) > span.text-danger > b",
    )
    .await
    .replace("$", "")
    .replace(" ", "")
    .replace(",", "")
    .parse()
    .unwrap_or_default();

    let rem_capacity: i32 = get_text_by_selector(&response, "#remCapacity")
        .await
        .replace(",", "")
        .parse()
        .unwrap_or_default();

    let holding: i32 = get_text_by_selector(&response, "#holding")
        .await
        .replace(",", "")
        .parse()
        .unwrap_or_default();

    let capacity = rem_capacity + holding;

    if get_attr_by_selector(&response, "#eco-state-2", "class")
        .await
        .contains("hidden")
    {
        return (
            price,
            capacity,
            holding,
            rem_capacity,
            "Eco-friendly".to_string(),
        );
    } else {
        return (
            price,
            capacity,
            holding,
            rem_capacity,
            "Eco-unfriendly".to_string(),
        );
    }
}
