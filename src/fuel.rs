use crate::utilities::get_response_text;
use crate::utilities::get_text_by_selector;

#[tokio::main]
pub(crate) async fn get_status(cookies: &str) -> (i16, i32, i32, i32) {
    let response = get_response_text("https://www.airlinemanager.com/fuel.php", cookies).await;

    let price: i16 = get_text_by_selector(
        &response,
        "#fuelMain > div > div:nth-child(1) > span.text-danger > b",
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

    return (price, capacity, holding, rem_capacity);
}
