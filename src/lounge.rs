use crate::utilities::get_response_text;
use crate::utilities::get_attrs_by_selector;


pub(crate) async fn maintain_lounges(cookies: &str) {
    let response = get_response_text("https://www.airlinemanager.com/hubs_lounge_manage.php", cookies).await;

    let lounge_elements = get_attrs_by_selector(&response, "body > div > div > table > tbody > tr > td > button", "onclick").await;
    

    for element in lounge_elements {
        let element = element.replace("Ajax('", "https://www.airlinemanager.com/").replace("&ref=manage','runme',this);", "");
        get_response_text(&format!("{element}"), cookies).await;
    }
}