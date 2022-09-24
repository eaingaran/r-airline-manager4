use reqwest;
use scraper;

#[tokio::main]
pub(crate) async fn get_balance(cookies: &str) -> i64 {
    let client = reqwest::Client::new();

    let response = client
        .post("https://www.airlinemanager.com/banking.php")
        .header("Cookie", cookies)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let document = scraper::Html::parse_document(&response);

    let balance_selector = scraper::Selector::parse("td.text-success").unwrap();

    let balances = document.select(&balance_selector).map(|x| x.inner_html());

    let balances_v: Vec<String> = balances.collect();

    assert_eq!(balances_v.len(), 1);

    let balance: i64 = balances_v
        .get(0)
        .map(String::as_str)
        .unwrap()
        .replace("$", "")
        .replace(" ", "")
        .replace(",", "")
        .parse()
        .unwrap();

    return balance;
}
