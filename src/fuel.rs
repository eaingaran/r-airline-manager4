use crate::utilities::get_response_text;
use crate::utilities::get_text_by_selector;

pub(crate) async fn get_status(cookies: &str) -> (i16, i32, i32, i32) {
    let response = get_response_text("https://www.airlinemanager.com/fuel.php", cookies).await;

    let price: i16 = get_text_by_selector(
        &response,
        "#fuelMain > div > div:nth-child(1) > span.text-danger > b",
    )
    .await
    .unwrap_or_else(|| "".to_string())
    .replace("$", "")
    .replace(" ", "")
    .replace(",", "")
    .parse()
    .unwrap_or_default();

    let rem_capacity: i32 = get_text_by_selector(&response, "#remCapacity")
        .await
        .unwrap_or_else(|| "".to_string())
        .replace(",", "")
        .parse()
        .unwrap_or_default();

    let holding: i32 = get_text_by_selector(&response, "#holding")
        .await
        .unwrap_or_else(|| "".to_string())
        .replace(",", "")
        .parse()
        .unwrap_or_default();

    let capacity = rem_capacity + holding;

    return (price, capacity, holding, rem_capacity);
}

pub(crate) async fn purchase(cookies: &str) {
    let (fuel_price, _fuel_capacity, fuel_holding, fuel_to_buy) = get_status(&cookies).await;

    if fuel_to_buy == 0 {
        return;
    }

    if fuel_price <= 400 {
        get_response_text(
            &format!("https://www.airlinemanager.com/fuel.php?mode=do&amount={fuel_to_buy}"),
            cookies,
        )
        .await;
        println!("purchased {fuel_to_buy} Lbs of fuel")
    } else if fuel_price <= 700 {
        let fuel_to_buy = 50000000 - fuel_holding;
        if fuel_to_buy <= 0 {
            return;
        }
        get_response_text(
            &format!("https://www.airlinemanager.com/fuel.php?mode=do&amount={fuel_to_buy}"),
            cookies,
        )
        .await;
        println!("purchased {fuel_to_buy}Lbs of fuel")
    } else {
        println!("fuel price (${fuel_price}) is too high");
    }
}
