use crate::utilities::get_response_text;
use crate::utilities::get_text_by_selector;

// get IDs of the planes

// https://www.airlinemanager.com/flight_info.php?id=35463037

// get remaining demand and seats

// seats       ||      demand
// #singleDeparter > div:nth-child(3) > div:nth-child(3)  ||  #singleDeparter > div:nth-child(3) > div:nth-child(4)  -- y
// #singleDeparter > div:nth-child(3) > div:nth-child(6)  ||  #singleDeparter > div:nth-child(3) > div:nth-child(7)  -- j
// #singleDeparter > div:nth-child(3) > div:nth-child(9)  ||  #singleDeparter > div:nth-child(3) > div:nth-child(10) -- f

// if the demand is greater than or equal to 1.1 times the seats in all the categories, depart the plane

// depart https://www.airlinemanager.com/route_depart.php?id=50569189&costIndex=200

#[tokio::main]
pub(crate) async fn depart_planes(cookies: &str) {
    // TODO: find a way to get ids of the planes to be departed.
    let response = get_response_text(
        "https://www.airlinemanager.com/hangars.php?type=cargo",
        cookies,
    )
    .await;

    let plane_ids = [];

    for plane_id in plane_ids {
        let depart_result = depart_plane(cookies, plane_id);
        if !!!depart_result {
            println!("Plane {plane_id} is not departed due to lack of demand")
        }
    }
}

#[tokio::main]
pub(crate) async fn depart_plane(cookies: &str, plane_id: i16) -> bool {
    let response = get_response_text(
        &format!("https://www.airlinemanager.com/flight_info.php?id={plane_id}"),
        cookies,
    )
    .await;

    // add 10% extra to the seats. This is because seats are almost never fully filled. so it is okay if the demand is slightly less than the seat capacity.
    let y_seat: f32 = get_text_by_selector(
        &response,
        "#singleDeparter > div:nth-child(3) > div:nth-child(3)",
    )
    .await
    .unwrap_or_else(|| "".to_string())
    .parse::<f32>()
    .unwrap_or_default()
        * 1.1;
    let j_seat: f32 = get_text_by_selector(
        &response,
        "#singleDeparter > div:nth-child(3) > div:nth-child(6)",
    )
    .await
    .unwrap_or_else(|| "".to_string())
    .parse::<f32>()
    .unwrap_or_default()
        * 1.1;
    let f_seat: f32 = get_text_by_selector(
        &response,
        "#singleDeparter > div:nth-child(3) > div:nth-child(9)",
    )
    .await
    .unwrap_or_else(|| "".to_string())
    .parse::<f32>()
    .unwrap_or_default()
        * 1.1;

    let y_demand: f32 = get_text_by_selector(
        &response,
        "#singleDeparter > div:nth-child(3) > div:nth-child(4)",
    )
    .await
    .unwrap_or_else(|| "".to_string())
    .parse()
    .unwrap_or_default();
    let j_demand: f32 = get_text_by_selector(
        &response,
        "#singleDeparter > div:nth-child(3) > div:nth-child(7)",
    )
    .await
    .unwrap_or_else(|| "".to_string())
    .parse()
    .unwrap_or_default();
    let f_demand: f32 = get_text_by_selector(
        &response,
        "#singleDeparter > div:nth-child(3) > div:nth-child(10)",
    )
    .await
    .unwrap_or_else(|| "".to_string())
    .parse()
    .unwrap_or_default();

    if (y_demand < y_seat) || (j_demand < j_seat) || (f_demand < f_seat) {
        return false;
    } else {
        get_response_text(
            &format!("https://www.airlinemanager.com/route_depart.php?id={plane_id}&costIndex=200"),
            cookies,
        )
        .await;
        return true;
    }
}
