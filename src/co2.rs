use crate::utilities::get_element_by_selector;
use crate::utilities::get_element_classes_by_id;
use crate::utilities::get_element_text_by_id;
use reqwest;

#[tokio::main]
pub(crate) async fn get_status(cookies: &str) -> (i16, i32, i32, i32, String) {
    let client = reqwest::Client::new();

    let response = client
        .post("https://www.airlinemanager.com/co2.php")
        .header("Cookie", cookies)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let price: i16 = get_element_by_selector(&response, "span.text-danger>b")
        .await
        .replace("$", "")
        .replace(" ", "")
        .replace(",", "")
        .parse()
        .unwrap_or_default();
    let rem_capacity: i32 = get_element_text_by_id(&response, "remCapacity")
        .await
        .replace(",", "")
        .parse()
        .unwrap_or_default();
    let holding: i32 = get_element_text_by_id(&response, "holding")
        .await
        .replace(",", "")
        .parse()
        .unwrap_or_default();
    let capacity = rem_capacity + holding;

    if get_element_classes_by_id(&response, "eco-state-2", "b")
        .await
        .iter()
        .any(|i| i == "hidden")
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
