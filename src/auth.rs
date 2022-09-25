use http;
use reqwest;

#[tokio::main]
pub(crate) async fn login(username: &str, password: &str) -> String {
    let client = reqwest::Client::new();

    let params = [("lEmail", username), ("lPass", password), ("remember", "0")];

    let response = client
        .post("https://www.airlinemanager.com/weblogin/login.php")
        .form(&params)
        .send()
        .await
        .unwrap();

    let cookies = response
        .headers()
        .get(http::header::SET_COOKIE)
        .unwrap()
        .to_str()
        .unwrap();
    
    if !!!cookies.contains("PHPSESSID=") {
        panic!("Login failed!");
    }

    return cookies.to_string();
}

#[tokio::main]
pub(crate) async fn logout(cookies: String) {
    let client = reqwest::Client::new();

    let _response = client
        .post("https://www.airlinemanager.com/weblogin/logout.php")
        .header("Cookie", cookies)
        .send()
        .await
        .unwrap();
}
