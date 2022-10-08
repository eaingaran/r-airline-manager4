use crate::utilities::get_attr_by_selector;
use crate::utilities::get_response_text;
use crate::utilities::get_text_by_selector;

#[tokio::main]
pub(crate) async fn maintain_planes(cookies: &str) -> Vec<String> {
    let response =
        get_response_text("https://www.airlinemanager.com/maint_plan.php", cookies).await;

    let mut maintained_planes: Vec<String> = Vec::new();

    for i in 1..=300 {
        let plane_name = get_text_by_selector(
            &response,
            &format!(
                "#acListView > div:nth-child({i}) > div.col-sm-4.text-center.text-secondary.s-text"
            ),
        )
        .await
        .unwrap_or_else(|| "".to_string());

        if plane_name == "".to_string() {
            break;
        }

        let location = get_text_by_selector(&response, &format!("#acListView > div:nth-child({i}) > div.col-sm-8.m-text > div > div:nth-child(1) > span.badge.badge-success.s-text"))
        .await.unwrap_or_else(|| "".to_string());

        if location == "Not at base" {
            continue;
        }

        let hours_to_check: i16 = get_text_by_selector(&response, &format!("#acListView > div:nth-child({i}) > div.col-sm-8.m-text > div > div:nth-child(2) > b.text-success"))
        .await.unwrap_or_else(|| "".to_string()).parse().unwrap_or_default();

        if hours_to_check < 15 {
            // #acListView > div:nth-child(1) > div.controls
            // #controls29418557
            let plane_id = get_attr_by_selector(
                &response,
                &format!("#acListView > div:nth-child({i}) > div.controls"),
                "id",
            )
            .await
            .replace("controls", "");
            // as of now, we are just checking. no need to take any actions.
            // let result = get_response_text(format!("https://www.airlinemanager.com/maint_plan_do.php?mode=do&type=check&id={plane_id}"), cookies).await;
            // println!("{:?}", result);
            // temporarily added cloning because i need to use the value in a print statement.
            // remove clone() after removing the print statement.
            maintained_planes.push(plane_name.clone());
            println!("plane {plane_name} needs maintenance")
        }
    }
    return maintained_planes;
}
