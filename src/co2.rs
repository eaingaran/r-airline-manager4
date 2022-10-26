use crate::utilities::get_attr_by_selector;
use crate::utilities::get_response_text;
use crate::utilities::get_text_by_selector;

pub(crate) async fn get_status(cookies: &str) -> (i16, i32, i32, i32, String) {
    let response = get_response_text("https://www.airlinemanager.com/co2.php", cookies).await;

    let price: i16 = get_text_by_selector(
        &response,
        "#co2Main > div > div:nth-child(2) > span.text-danger > b",
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

pub(crate) async fn purchase(cookies: &str) {
    let (co2_price, _co2_capacity, co2_holding, co2_to_buy, _airline_status) =
        get_status(&cookies).await;

    if co2_to_buy == 0 {
        return;
    }

    if co2_price <= 115 {
        get_response_text(
            &format!("https://www.airlinemanager.com/fuel.php?mode=do&amount={co2_to_buy}"),
            cookies,
        )
        .await;
        println!("purchased {co2_to_buy} Lbs of fuel")
    } else if co2_price <= 138 {
        let co2_to_buy = 60000000 - co2_holding;
        if co2_to_buy <= 0 {
            return;
        }
        get_response_text(
            &format!("https://www.airlinemanager.com/fuel.php?mode=do&amount={co2_to_buy}"),
            cookies,
        )
        .await;
        println!("purchased {co2_to_buy} Quotas of fuel")
    } else {
        println!("co2 quota price (${co2_price}) is too high");
    }
}
