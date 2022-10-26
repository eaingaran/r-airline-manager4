use crate::utilities::get_response_text;
use crate::utilities::get_route_details;
use crate::utilities::get_text_by_selector;

pub(crate) async fn validate_pax_plane(cookies: &str, plane_id: i32) {
    let response = get_response_text(
        &format!("https://www.airlinemanager.com/flight_info.php?id={plane_id}"),
        cookies,
    )
    .await;

    let route = get_text_by_selector(
        &response,
        ":root > body > div:nth-child(1) > div > div:nth-child(4)",
    )
    .await
    .unwrap_or_else(|| "".to_string());

    match route.split_once('-') {
        Some((departure, arrival)) => {
            println!("departure: {}", departure);
            println!("arrival: {}", arrival);
            // get the preferred seat configuration for this route
            let route_details = get_route_details(departure, arrival).await;
            let route_details = route_details.get(0).unwrap();

            let y_current: f32 = get_text_by_selector(
                &response,
                "#singleDeparter > div:nth-child(3) > div:nth-child(3)",
            )
            .await
            .unwrap_or_else(|| "".to_string())
            .parse::<f32>()
            .unwrap_or_default();
            let j_current: f32 = get_text_by_selector(
                &response,
                "#singleDeparter > div:nth-child(3) > div:nth-child(6)",
            )
            .await
            .unwrap_or_else(|| "".to_string())
            .parse::<f32>()
            .unwrap_or_default();
            let f_current: f32 = get_text_by_selector(
                &response,
                "#singleDeparter > div:nth-child(3) > div:nth-child(9)",
            )
            .await
            .unwrap_or_else(|| "".to_string())
            .parse::<f32>()
            .unwrap_or_default();
            // /html/body/div[3]/div[2]/div[2]
            let speed = get_text_by_selector(
                &response,
                ":root > body > div:nth-child(3) > div:nth-child(2) > div:nth-child(4)",
            )
            .await
            .unwrap();
            println!("speed is {speed}");

            let distance = get_text_by_selector(
                &response,
                ":root > body > div:nth-child(3) > div:nth-child(2) > div:nth-child(2)",
            )
            .await
            .unwrap();
            println!("distance is {distance}");

            // /html/body/div[3]/div[2]/div[2]/span
            // #departFuelDistance
            let distance = get_text_by_selector(
                &response,
                ":root > body > div:nth-child(3) > div:nth-child(2) > div:nth-child(2) > span",
            )
            .await
            .unwrap_or_else(|| "".to_string())
            .replace(",", "")
            .parse::<i32>()
            .unwrap_or_default();

            /// html/body/div[3]/div[2]/div[4]/span
            // #departCruiseSpeed
            let speed = get_text_by_selector(
                &response,
                ":root > body > div:nth-child(3) > div:nth-child(2) > div:nth-child(4) > span",
            )
            .await
            .unwrap_or_else(|| "".to_string())
            .replace(",", "")
            .parse::<i32>()
            .unwrap_or_default();

            let trips = 23 / (distance * speed);

            println!("Total trips :: {trips}");
            println!("{}", route_details.first_class_demand);
        }
        None => {
            println!("expected a departure-arrival. found {}", route);
        }
    }
}
