use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::Value;

pub(crate) async fn find_a388_routes(count: &u16) -> Vec<String> {
    println!("yet to be implemented...");
    return Vec::new();
}

pub(crate) async fn find_a388f_routes(count: &u16) -> Vec<String> {
    println!("yet to be implemented...");
    return Vec::new();
}

#[derive(Serialize, Deserialize)]
struct airport {
    id: i32,
    iata: String,
    icao: String,
    runway: i32,
    market: i32,
    latitude: f32,
    longitude: f32,
    city: String,
    country: String,
    country_code: String,
}

#[derive(Serialize, Deserialize)]
struct route {
    id: String,
    economic_demand: i32,
    business_demand: i32,
    first_class_demand: i32,
    large_demand: i32,
    heavy_demand: i32,
    distance: i32,
    departure: airport,
    arrival: airport,
}

#[derive(Serialize, Deserialize)]
struct routes {
    routes: Vec<route>,
}

// https://am4tools.com/route/search?departure=DEL&sort=large&order=desc&page=1&mode=hub
pub(crate) async fn find_an225_routes(count: &u16) -> Vec<String> {
    println!("yet to be implemented...");
    let hub_iata = "DEL";
    for page_num in 1..500 {
        let client = reqwest::Client::new();
        let response = client.get(&format!("https://am4tools.com/route/search?departure={hub_iata}&sort=large&order=desc&page={page_num}&mode=hub")).send().await.ok().unwrap();
        let status_code = response.status().as_u16();
        if status_code == 404 {
            println!("status code is {status_code}");
            break;
        }
        let response_payload = response.text().await.unwrap_or_default();
        let response_json: routes = serde_json::from_str(&response_payload).unwrap();
        let response_string = serde_json::to_string(&response_json).unwrap_or_default();
        println!("{:#?}", response_string);
        break;
    }
    return Vec::new();
}

pub(crate) async fn route_a388(cookies: &str) {
    // https://www.airlinemanager.com/fleet.php?type={aircraft_type_id}
    println!("yet to be implemented...");
}

pub(crate) async fn route_a388f(cookies: &str) {
    // https://www.airlinemanager.com/fleet.php?type={aircraft_type_id}
    println!("yet to be implemented...");
}
