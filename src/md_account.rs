use reqwest::{header::USER_AGENT, Client, ClientBuilder};
use std::{error::Error, future::Future};

const BASE_URL: &'static str = "https://api.mangadex.org";
pub async fn request_with_agent(
    url: String,username:&'static str, password:&'static str
) -> Result<impl Future<Output = Result<reqwest::Response, reqwest::Error>>, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let response = client
        .post(url)
        // .form(form)        
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .send();

    Ok(response)
}
async fn get_session_token() {
    let url: String = format!("{BASE_URL}/auth/login");
    // let resp = request_with_agent(url).await;

}

async fn get_refresh_token() {}
