use crate::utilities::get_response_text;

pub(crate) async fn buy_a388(cookies: &str) -> bool {
    println!("yet to be implemented...");
    return false;
}

pub(crate) async fn buy_a388f(cookies: &str) -> bool {
    println!("yet to be implemented...");
    return false;
}

pub(crate) async fn buy_hanger_space(cookies: &str) -> bool {
    let response = get_response_text(
        &format!("https://www.airlinemanager.com/staff_train_action.php?type=24"),
        cookies,
    )
    .await;

    if response.contains("Error") {
        println!("hanger space not purchased");
        false
    } else {
        println!("hanger space purchased");
        true
    }
}
