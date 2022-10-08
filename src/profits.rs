use crate::utilities::get_attr_by_selector;
use crate::utilities::get_elements_by_selector;
use crate::utilities::get_fight_duration;
use crate::utilities::get_response_text;
use crate::utilities::get_text_by_selector;
use std::collections::HashMap;

struct PerformanceIndex {
    age: i64,
    income_index: i64,
    fuel_index: i64,
    co2_index: i64,
}

pub(crate) async fn get_aircraft_wise(cookies: &str, aircraft_type_id: &i16) {
    let response = get_response_text(
        &format!("https://www.airlinemanager.com/fleet.php?type={aircraft_type_id}"),
        cookies,
    )
    .await;

    let planes: Vec<String> =
        get_elements_by_selector(&response, "body > div:nth-child(2) > div > div").await;

    let mut plane_performance_index: HashMap<String, PerformanceIndex> = HashMap::new();
    let mut aircraft_name = String::new();

    let mut total_avg_income_index = 0;
    let mut total_avg_fuel_index = 0;
    let mut total_avg_co2_index = 0;
    let mut total_age = 0;
    let mut plane_count = planes.len() as i64;

    for plane in planes.iter() {
        let url: String = get_attr_by_selector(&plane, "a", "onclick")
            .await
            .replace(r#"Ajax('"#, "")
            .replace(r#"','detailsAction');"#, "");

        let response =
            get_response_text(&format!("https://www.airlinemanager.com/{url}"), cookies).await;

        let plane_name = get_text_by_selector(&response, "#ff-name")
            .await
            .unwrap_or_else(|| "".to_string());
        let plane_age = get_text_by_selector(&response, "#detailsGroundedBg > div.col-sm-6.bg-light.border > div > div:nth-child(1) > span:nth-child(7)").await.unwrap_or_else(|| "".to_string()).replace(" months ago", "");

        if plane_age.contains("hours") {
            // println!("{plane_name} is less than a day old");
            plane_count = plane_count - 1;
            continue;
        }

        let plane_age = plane_age.parse().unwrap_or_default();

        let departure = get_text_by_selector(
            &response,
            "#detailsGroundedBg > div.col-sm-6.text-center > div > div:nth-child(5)",
        )
        .await
        .unwrap_or_else(|| "".to_string())
        .replace(" UTC", "");
        let arrival = get_text_by_selector(
            &response,
            "#detailsGroundedBg > div.col-sm-6.text-center > div > div:nth-child(6)",
        )
        .await
        .unwrap_or_else(|| "".to_string())
        .replace(" UTC", "");

        aircraft_name = get_text_by_selector(
            &response,
            &format!("#detailsGroundedBg > div.col-sm-6.bg-light.border > div > div:nth-child(1) > span:nth-child(3)"),
        ).await.unwrap_or_else(|| "".to_string());

        let mut total_income: i64 = 0;
        let mut fuel: i64 = 0;
        let mut total_co2: i64 = 0;
        let mut count: i64 = 0;

        for i in 1..=5 {
            // if the flight history has only 1 flight record, the following code will not work.
            // for this reason, the history is checked only when the plane age is more than a day.

            let income: i64 = get_text_by_selector(
                &response,
                &format!(
                    "#flight-history > div:nth-child({i}) > div.col-3.text-right.text-success > b"
                ),
            )
            .await
            .unwrap_or_else(|| "".to_string())
            .replace("$", "")
            .replace(",", "")
            .parse()
            .unwrap_or_default();

            if income == 0 {
                break;
            }

            count = count + 1;

            let fuel_use = get_text_by_selector(
                &response,
                &format!("#flight-history > div:nth-child({i}) > div:nth-child(3) > span"),
            )
            .await
            .unwrap_or_else(|| "".to_string())
            .replace(" Lbs", "")
            .replace(",", "")
            .parse()
            .unwrap_or_default();

            let co2_use: i64 = get_text_by_selector(
                &response,
                &format!("#flight-history > div:nth-child({i}) > div:nth-child(2) > span"),
            )
            .await
            .unwrap_or_else(|| "".to_string())
            .replace(" Quotas", "")
            .replace(",", "")
            .parse()
            .unwrap_or_default();

            total_income = total_income + income;
            total_co2 = total_co2 + co2_use;
            fuel = fuel_use;
        }

        if count == 0 {
            // println!("{plane_name} is yet to make its maiden voyage");
            plane_count = plane_count - 1;
            continue;
        }

        let average_income: i64 = total_income / count;
        let average_co2: i64 = total_co2 / count;
        // TODO: time difference doesn't seem to work properly
        // (when the departure time is late night and arrival is early morning) due to lack of date info.
        // check and fix this, if possible.
        // possibly, write a custom time difference function to compute the different properly.
        // let flight_duration = (arrival - departure).num_minutes().abs();

        let flight_duration = get_fight_duration(&departure, &arrival).await;

        let performance_index = PerformanceIndex {
            age: plane_age,                                 // Plane age in months
            income_index: average_income / flight_duration, // income index in $/min
            fuel_index: fuel / flight_duration,             // co2 index in Quotas/min
            co2_index: average_co2 / flight_duration,       // fuel index in Lbs/min
        };

        total_avg_income_index = total_avg_income_index + performance_index.income_index;
        total_avg_fuel_index = total_avg_fuel_index + performance_index.fuel_index;
        total_avg_co2_index = total_avg_co2_index + performance_index.co2_index;

        total_age = total_age + performance_index.age;

        if !!!(performance_index.fuel_index == 374 || performance_index.fuel_index == 373) {
            println!("-------------------------------------------------------------");
            println!("Debug info");
            println!("----------");
            println!("Plane Name        :: {plane_name}");
            println!("Plane Type        :: {aircraft_name}");
            println!("Plane Age         :: {plane_age}");
            println!("Trip Count        :: {count}");
            println!("Plane Avg. Income :: {average_income}");
            println!("Plane Avg. CO2    :: {average_co2}");
            println!("Plane Fuel Use    :: {fuel}");
            println!("Plane Departure   :: {departure}");
            println!("Plane Arrival     :: {arrival}");
            println!("Plane Duration    :: {flight_duration}");
            println!("Plane URL         :: https://www.airlinemanager.com/{url}");
            println!("-------------------------------------------------------------");
            println!();
            println!("{} was bought {} months ago, has a income index of {} $/min, fuel index of {} Lbs/min and co2 index of {} Quotas/min", plane_name, plane_age, performance_index.income_index, performance_index.fuel_index, performance_index.co2_index);
            println!();
        }
        // println!("{} was bought {} months ago,  has a income index of {} $/min, fuel index of {} Lbs/min and co2 index of {} Quotas/min", plane_name, plane_age, performance_index.income_index, performance_index.fuel_index, performance_index.co2_index);
        // println!();

        plane_performance_index.insert(plane_name, performance_index);
    }
    // println!();
    println!("Average performance index for the {aircraft_name} fleet with {plane_count} planes");
    println!("   Average Age     :: {} months", total_age / plane_count);
    println!(
        "   Income Index    :: {} $/min",
        total_avg_income_index / plane_count
    );
    println!(
        "   Fuel Index      :: {} Lbs/min",
        total_avg_fuel_index / plane_count
    );
    println!(
        "   CO2 Index       :: {} Quotas/min",
        total_avg_co2_index / plane_count
    );
    println!();
}
