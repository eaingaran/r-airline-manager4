use chrono::NaiveTime;
use reqwest;
use scraper;
use serde::{Deserialize, Serialize};

pub(crate) async fn get_elements_by_selector(response: &str, selector: &str) -> Vec<String> {
    let document = scraper::Html::parse_document(response);

    let selector_p = scraper::Selector::parse(selector).unwrap();

    let elements = document.select(&selector_p).map(|x| x.inner_html());

    return elements.collect();
}

pub(crate) async fn get_texts_by_selector(response: &str, selector: &str) -> Vec<String> {
    let document = scraper::Html::parse_document(response);

    let selector_p = scraper::Selector::parse(selector).unwrap();

    let elements = document.select(&selector_p).map(|x| x.text().collect());

    let mut texts: Vec<String> = Vec::new();

    let elements: Vec<String> = elements.collect();

    for element in elements {
        texts.push(element.trim().to_string());
    }

    return texts;
}

pub(crate) async fn get_element_by_selector(response: &str, selector: &str) -> String {
    let document = scraper::Html::parse_document(response);

    let selector_p = scraper::Selector::parse(selector).unwrap();

    let elements: Vec<String> = document
        .select(&selector_p)
        .map(|x| x.inner_html())
        .collect();

    assert_eq!(elements.len(), 1, "There should only be one one element for this method {:#?}. To get multiple elemets, use get_elements_by_selector()", elements);

    let element = elements.get(0).map(String::as_str).unwrap().to_string();

    return element;
}

// This function returns only one element's text.
// If the selector returns multiple elements, it will return the first element's text.
// If you want texts of all the elements matching the selector, use get_texts_by_selector() instead.
pub(crate) async fn get_text_by_selector(response: &str, selector: &str) -> Option<String> {
    let document = scraper::Html::parse_document(response);

    let selector = scraper::Selector::parse(selector).unwrap();

    let texts: Vec<String> = document
        .select(&selector)
        .map(|x| x.text().collect())
        .collect();

    if texts.len() > 0 {
        return Some(texts.get(0).map(String::as_str).unwrap().trim().to_string());
    } else {
        println!("selector returned 0 results");
        return None;
    }
}

pub(crate) async fn get_attr_by_selector(
    response: &str,
    selector: &str,
    attribute: &str,
) -> String {
    let document = scraper::Html::parse_document(response);

    let selector = scraper::Selector::parse(selector).unwrap();

    let elements: Vec<String> = document
        .select(&selector)
        .map(|x| x.value().attr(attribute).unwrap().to_string())
        .collect();

    assert_eq!(
        elements.len(),
        1,
        "There should only be one one element for this method to work. found {:#?}.",
        elements
    );

    let element = elements
        .get(0)
        .map(String::as_str)
        .unwrap()
        .trim()
        .to_string();

    return element;
}

pub(crate) async fn get_response_text(url: &str, cookies: &str) -> String {
    let client = reqwest::Client::new();
    return client
        .get(url)
        .header("Cookie", cookies)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap_or_default();
}

pub(crate) async fn get_fight_duration(departure: &str, arrival: &str) -> i64 {
    let dep_hr: Vec<String> = vec![departure.split(":").collect()];
    let arr_hr: Vec<String> = vec![arrival.split(":").collect()];

    let dep_hr: i64 = dep_hr[0].parse().unwrap_or_default();
    let arr_hr: i64 = arr_hr[0].parse().unwrap_or_default();

    if arr_hr > dep_hr {
        return (NaiveTime::parse_from_str(arrival, "%H:%M:%S").unwrap()
            - NaiveTime::parse_from_str(departure, "%H:%M:%S").unwrap())
        .num_minutes()
        .abs();
    } else {
        return 1440
            - (NaiveTime::parse_from_str(arrival, "%H:%M:%S").unwrap()
                - NaiveTime::parse_from_str(departure, "%H:%M:%S").unwrap())
            .num_minutes()
            .abs();
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct airport {
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
pub(crate) struct route {
    id: String,
    pub economic_demand: i32,
    pub business_demand: i32,
    pub first_class_demand: i32,
    pub large_demand: i32,
    pub heavy_demand: i32,
    distance: i32,
    departure: airport,
    arrival: airport,
}

#[derive(Serialize, Deserialize)]

pub(crate) struct request {
    status: i32,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct response {
    request: request,
    routes: Vec<route>,
}

pub(crate) async fn find_routes(
    count: &u16,
    capacity: &u16,
    runway_length: &u16,
    range: &u32,
    speed: &u32,
) -> Vec<String> {
    println!("yet to be implemented...");
    let hub_iata = "DEL";
    let airport_runway = 12000;
    for page_num in 1..500 {
        let client = reqwest::Client::new();
        let response = client.get(&format!("https://am4tools.com/route/search?departure={hub_iata}&sort=large&order=desc&page={page_num}&mode=hub")).send().await.ok().unwrap();
        let status_code = response.status().as_u16();
        if status_code == 404 {
            println!("status code is {status_code}");
            break;
        }
        let response_payload = response.text().await.unwrap_or_default();
        let response_json: response = serde_json::from_str(&response_payload).unwrap();
        for route in response_json.routes {
            if route.distance as f32 > (*range as f32 * 1.1) {
                continue;
            }
            let trip = (route.distance as f32 / (*speed as f32 * 1.1)).ceil();
            if airport_runway < *runway_length {
                continue;
            }
            println!("{}-{}", route.departure.iata, route.arrival.iata);
        }
        break;
    }
    return Vec::new();
}

pub(crate) async fn get_route_details(departure: &str, arrival: &str) -> Vec<route> {
    let client = reqwest::Client::new();
    // https://am4tools.com/route/search?departure=del&arrival=fra&mode=direct
    let response = client
        .get(&format!(
            "https://am4tools.com/route/search?departure={departure}&arrival={arrival}&mode=direct"
        ))
        .send()
        .await
        .ok()
        .unwrap();
    let status_code = response.status().as_u16();
    if status_code == 404 {
        println!("status code is {status_code}");
        println!("route {departure}-{arrival} not found");
    }
    let response_payload = response.text().await.unwrap_or_default();
    let response_json: response = serde_json::from_str(&response_payload).unwrap();
    return response_json.routes;
}

pub(crate) async fn get_seat_config() -> (i16, i16, i16) {
    return (0, 0, 0);
}
