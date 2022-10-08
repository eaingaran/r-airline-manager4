use crate::utilities::get_response_text;
use crate::utilities::get_text_by_selector;

pub(crate) async fn get_balance(cookies: &str) -> i64 {
    let response = get_response_text("https://www.airlinemanager.com/banking.php", cookies).await;

    return get_text_by_selector(
        &response,
        "#bankingAction > table > tbody > tr > td.text-success",
    )
    .await
    .unwrap_or_else(|| "".to_string())
    .replace("$", "")
    .replace(" ", "")
    .replace(",", "")
    .parse()
    .unwrap_or_default();
}
