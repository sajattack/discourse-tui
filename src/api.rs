use openssl::rsa::Rsa;
use reqwest;

pub fn gen_key(url: String) -> Result<String, String> {
    let scopes = "notifications";
    let client_id = "bf721cf904a612260868b1a370057d7b";
    let nonce = "b44605f1d72a04d1";
    let auth_redirect = &url;
    let app_name = "Discourse TUI";
    let public_key: String;
    match Rsa::generate(2048) {
        Ok(keypair) => {
            match keypair.public_key_to_pem() {
                Ok(pem) => public_key = String::from_utf8(pem).unwrap(),
                Err(estack) => return Err(format!("{}", estack)),
            }
        },
        Err(estack) => return Err(format!("{}", estack)),
    }
    let client = reqwest::Client::new();
    let mut temp_url = url.to_owned();
    temp_url.push_str("/user-api-key/new");
    let req: reqwest::Request;
        match client.post(&temp_url)
        .query(&[("scopes", scopes), 
                 ("client_id", client_id),
                 ("nonce", nonce),
                 ("auth_redirect", auth_redirect),
                 ("application_name", app_name),
                 ("public_key", &public_key)])
        .build() {
            Ok(r) => req = r,
            Err(_) => return Err("Error building query url".to_string()),
        }
        let req_url = req.url().clone().into_string();
    Ok(req_url)
}
