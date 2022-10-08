use crate::utilities::get_attr_by_selector;
use crate::utilities::get_response_text;
use crate::utilities::get_text_by_selector;

pub(crate) async fn maintain_planes(cookies: &str) {
    let response =
        get_response_text("https://www.airlinemanager.com/maint_plan.php", cookies).await;

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
            let _result = get_response_text(&format!("https://www.airlinemanager.com/maint_plan_do.php?mode=do&type=check&id={plane_id}"), cookies).await;
            // result is in the below format
            // in future, try to get the status from the result
            // "\t\t\t\t\t<script>\r\n\t\t\t\t\tminus_content('headerAccount',6495861);maintUpdated = 1;statusListRemoveMaint(32552123);flightStatusChange(32552123,'pending',66567.18,1);markers[32552123].setIcon(selectedMarker[9]);$('#flightStatusWear32552123').html('0.00');$('#listTimer32552123Color').removeClass('glyphicons-wrench').addClass('glyphicons-asterisk');popup('maintenance_main.php','Maintenance');toast('Success','Maintenance planned','success');\t\t\t\t\t</script>\r\n\t\t\t\t\t<script>\r\n$('#maintView').hide();\r\n</script>"
            // println!("{:?}", result);
            println!("plane {plane_name} scheduled for maintenance")
        }
    }
}
