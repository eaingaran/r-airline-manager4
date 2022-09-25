use crate::utilities::get_attr_by_selector;
use crate::utilities::get_elements_by_selector;
use crate::utilities::get_response_text;
use crate::utilities::get_text_by_selector;
use chrono::NaiveTime;
use std::collections::HashMap;


#[tokio::main]
pub(crate) async fn get_aircraft_wise(cookies: &str, aircraft_type_id: &i16) {
    let response = get_response_text(
        &format!("https://www.airlinemanager.com/fleet.php?type={aircraft_type_id}"),
        cookies,
    )
    .await;

    let planes: Vec<String> =
        get_elements_by_selector(&response, "body > div:nth-child(2) > div > div").await;
    
    let mut plane_performance_index: HashMap<String, i64> = HashMap::new();

    for plane in planes.iter() {
        let url: String = get_attr_by_selector(&plane, "a", "onclick")
            .await
            .replace(r#"Ajax('"#, "")
            .replace(r#"','detailsAction');"#, "");

        let response =
            get_response_text(&format!("https://www.airlinemanager.com/{url}"), cookies).await;

        let plane_name = get_text_by_selector(&response, "#ff-name").await;
        let plane_age = get_text_by_selector(&response, "#detailsGroundedBg > div.col-sm-6.bg-light.border > div > div:nth-child(1) > span:nth-child(7)").await;
        let departure = get_text_by_selector(
            &response,
            "#detailsGroundedBg > div.col-sm-6.text-center > div > div:nth-child(5)",
        )
        .await
        .replace(" UTC", "");
        let arrival = get_text_by_selector(
            &response,
            "#detailsGroundedBg > div.col-sm-6.text-center > div > div:nth-child(6)",
        )
        .await
        .replace(" UTC", "");

        let mut total_income: i64 = 0;

        for i in 1..=5 {
            let income: i64 = get_text_by_selector(
                &response,
                &format!(
                    "#flight-history > div:nth-child({i}) > div.col-3.text-right.text-success > b"
                ),
            )
            .await
            .replace("$", "")
            .replace(",", "")
            .parse()
            .unwrap_or_default();
            total_income = total_income + income;
        }

        let average_income = total_income / 5;
        let (departure, arrival) = (
            NaiveTime::parse_from_str(&departure, "%H:%M:%S").unwrap(),
            NaiveTime::parse_from_str(&arrival, "%H:%M:%S").unwrap(),
        );
        let flight_duration = (arrival - departure).num_minutes().abs();
        let performance_index = average_income / flight_duration; // performance index in $/min

        println!("{plane_name} was bought {plane_age} has a performance index of {performance_index} $/min");

        plane_performance_index.insert(plane_name, performance_index);
    }

    // sort the performance index map based on value.
}
