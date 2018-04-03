use openssl::rsa::Rsa;
use openssl::pkey::Private;
use openssl::rsa::Padding;
use reqwest;
use rand::{Rng, thread_rng};
use std::fmt::Write;
use std::str;
use base64;
use serde_json;
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct PartialApi {
    pem: String,
    client_id: String,
    base_url: String,
    pub api_authorize_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Api {
    client_id: String,
    api_key: String,
    base_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Topic {
    pub id: i32,
    pub title: String,
    pub fancy_title: String,
    pub slug: String,
    pub posts_count: i32,
    pub reply_count: i32,
    pub highest_post_number: i32,
    pub image_url: Option<String>,
    pub created_at: String,
    pub last_posted_at: String,
    pub bumped: bool,
    pub bumped_at: String,
    pub unseen: bool,
    pub pinned: bool,
    pub unpinned: Option<Value>,
    pub excerpt: Option<String>,
    pub visible: bool,
    pub closed: bool,
    pub archived: bool,
    pub bookmarked: Option<Value>,
    pub liked: Option<Value>,
    pub views: i32,
    pub like_count: i32,
    pub has_summary: bool,
    pub archetype: String,
    pub last_poster_username: String,
    pub category_id: i32,
    pub pinned_globally: bool,
    pub posters: Option<Vec<Value>>,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub avatar_template: String,
    pub name: (),
    pub last_posted_at: String,
    pub last_seen_at: String,
    pub created_at: String,
    pub website_name: (),
    pub can_edit: bool,
    pub can_edit_username: bool,
    pub can_edit_email: bool,
    pub can_send_private_messages: bool,
    pub can_send_private_messages_to_user: bool,
    pub trust_level: i32,
    pub moderator: bool,
    pub admin: bool,
    pub title: (),
    pub uploaded_avatar_id: (),
    pub badge_count: i32,
    pub custom_fields: (),
    pub pending_count: i32,
    pub profile_view_count: i32,
    pub primary_group_name: (),
    pub primary_group_flair_url: (),
    pub primary_group_flair_bg_color: (),
    pub primary_group_flair_color: (),
    pub invited_by: Box<User>,
    pub groups: (),
    pub featured_user_badge_ids: (),
    pub card_badge: (),
}

#[derive(Serialize, Deserialize)]
pub struct Group {
    id: i32,
    automatic: bool,
    name: String,
    user_count: i32,
    alias_level: i32,
    visible: bool,
    automatic_membership_email_domains: (),
    automatic_membership_retroactive: bool,
    primary_group: bool,
    title: (),
    grant_trust_level: (),
    incoming_email: (),
    notification_level: i32,
    has_messages: bool,
    is_member: bool,
    mentionable: bool,
    flair_url: (),
    flair_bg_color: (),
    flair_color: (),
}

impl Api {
    pub fn get_latest_topics(&self) -> Result<Vec<Topic>, reqwest::Error> {
        match reqwest::get(&(self.base_url.clone() + "/latest.json")) {
            Ok(mut response) => {
                let mut json = response.text().unwrap();
                let v: Value = serde_json::from_str(&json).unwrap();
                json = v["topic_list"]["topics"].to_string();
                let topics: Vec<Topic> = serde_json::from_str(&json).unwrap();
                Ok(topics)
            },
            Err(err) => return Err(err),
        }
    }
}

impl PartialApi {
    pub fn gen_key_url(base_url: String) -> Result<PartialApi, String> {
        let mut rng = thread_rng();
        let scopes = "read,write";
        let mut client_id_bytes = [0u8;16];
        rng.fill_bytes(&mut client_id_bytes);
        let mut client_id = String::new();
        for &byte in client_id_bytes.iter() {
            write!(&mut client_id, "{:02x}", byte).expect("Unable to write");
        };
        let mut nonce_bytes = [0u8;8];
        rng.fill_bytes(&mut nonce_bytes);
        let mut nonce = String::new();
        for &byte in nonce_bytes.iter() {
            write!(nonce, "{:02x}", byte).expect("Unable to write");
        };

        let auth_redirect = "discourse://auth_redirect";
        let app_name = "Discourse TUI";
        let public_key: String;
        let keypair: Rsa<Private>;
        match Rsa::generate(2048) {
            Ok(kp) => {
                keypair = kp;
                match keypair.public_key_to_pem() {
                    Ok(pem) => public_key = String::from_utf8(pem).unwrap(),
                    Err(estack) => return Err(format!("{}", estack)),
                }
            },
            Err(estack) => return Err(format!("{}", estack)),
        }
        let client = reqwest::Client::new();
        let mut temp_url = base_url.to_owned();
        temp_url.push_str("/user-api-key/new");
        let req: reqwest::Request;
            match client.post(&temp_url)
            .query(&[("scopes", scopes), 
                    ("client_id", &client_id),
                    ("nonce", &nonce),
                    ("auth_redirect", auth_redirect),
                    ("application_name", app_name),
                    ("public_key", &public_key)])
            .build() {
                Ok(r) => req = r,
                Err(_) => return Err("Error building query url".to_string()),
            }
            let req_url = req.url().clone().into_string();
            let partial_api = PartialApi {
                pem: String::from_utf8(keypair.private_key_to_pem().unwrap()).unwrap(), 
                client_id: client_id, 
                base_url: base_url,
                api_authorize_url: req_url
            };
        Ok(partial_api)
    }

    pub fn decrypt_key(self, payload: String) -> Result<Api, String> {
        let mut buf = [0u8;256];
        let rsa = Rsa::private_key_from_pem(self.pem.as_bytes()).unwrap();
        let payload_bytes = base64::decode(&payload).unwrap(); 
        let api_key: String;
        match rsa.private_decrypt(&payload_bytes, &mut buf, Padding::PKCS1) {
            Ok(_) => {
                let s: String = String::from_utf8(buf.to_vec()).unwrap();
                let v: Value = serde_json::from_str(&s.trim_right_matches(char::from(0))).unwrap();
                api_key = v["key"].as_str().unwrap().to_string();
            },
            Err(estack) => return Err(format!("{}", estack)),
        }
        let api = Api {
            client_id: self.client_id.clone(),
            api_key: api_key, 
            base_url: self.base_url,
        };
        Ok(api)
    }
}
