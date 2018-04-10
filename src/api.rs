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
use hyper::header::Headers;

#[derive(Serialize, Deserialize)]
pub struct PartialApi {
    pem: String,
    client_id: String,
    base_url: String,
    pub api_authorize_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Api {
    client_id: Option<String>,
    api_key: Option<String>,
    base_url: String,
}

// Discourse is dumb and retuns different types of topics depending how you query.
// Topic is returned when you query by ID
// LatestTopic is returned when you query for the latest topics
// The latter doesn't actually contain a reference to the posts

#[derive(Serialize, Deserialize)]
pub struct Topic {
    pub post_stream: Value,
    pub timeline_lookup: Value,
    pub id: i32,
    pub title: String,
    pub fancy_title: String,
    pub posts_count: i32,
    pub created_at: String,
    pub views: i32,
    pub reply_count: i32,
    pub participant_count: i32,
    pub like_count: i32,
    pub last_posted_at: Value,
    pub visible: bool,
    pub closed: bool,
    pub archived: bool,
    pub has_summary: bool,
    pub archetype: String,
    pub slug: String,
    pub category_id: i32,
    pub word_count: Value,
    pub deleted_at: Option<Value>,
    pub user_id: i32,
    pub draft: Option<Value>,
    pub draft_key: String,
    pub draft_sequence: Option<Value>,
    pub unpinned: Option<Value>,
    pub pinned_globally: bool,
    pub pinned: bool,
    pub pinned_at: Option<String>,
    pub pinned_until: Option<Value>,
    pub details: Value,
    pub highest_post_number: i32,
    pub deleted_by: Option<Value>,
    pub actions_summary: Value,
    pub chunk_size: i32,
    pub bookmarked: Option<Value>,
}

#[derive(Serialize, Deserialize)]
pub struct LatestTopic {
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
    pub title: Value,
    pub uploaded_avatar_id: Value,
    pub badge_count: i32,
    pub custom_fields: Value,
    pub pending_count: i32,
    pub profile_view_count: i32,
    pub primary_group_name: Value,
    pub primary_group_flair_url: Value,
    pub primary_group_flair_bg_color: Value,
    pub primary_group_flair_color: Value,
    pub invited_by: Box<User>,
    pub groups: Value,
    pub featured_user_badge_ids: Value,
    pub card_badge: Value,
}

#[derive(Serialize, Deserialize)]
pub struct Group {
   pub id: i32,
   pub automatic: bool,
   pub name: String,
   pub user_count: i32,
   pub alias_level: i32,
   pub visible: bool,
   pub automatic_membership_email_domains: Value,
   pub automatic_membership_retroactive: bool,
   pub primary_group: bool,
   pub title: Value,
   pub grant_trust_level: Value,
   pub incoming_email: Value,
   pub notification_level: i32,
   pub has_messages: bool,
   pub is_member: bool,
   pub mentionable: bool,
   pub flair_url: Value,
   pub flair_bg_color: Value,
   pub flair_color: Value,
}

#[derive(Serialize, Deserialize)]
pub struct Category {
   pub id: i32,
   pub name: String,
   pub color: String,
   pub text_color: String,
   pub slug: String,
   pub topic_count: i32,
   pub post_count: i32,
   pub position: i32,
   pub description: Option<String>,
   pub description_text: Option<String>,
   pub topic_url: Option<String>,
   pub logo_url: Option<String>,
   pub background_url: Option<String>,
   pub read_restricted: bool,
   pub permission: Option<i32>,
   pub notification_level: Option<i32>,
   pub can_edit: Option<bool>,
   pub topic_template: Option<String>,
   pub has_children: bool,
   pub topics_day: i32,
   pub topics_week: i32,
   pub topics_month: i32,
   pub topics_year: i32,
   pub topics_all_time: i32,
   pub description_excerpt: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub name: Option<String>,
    pub username: String,
    pub avatar_template: String,
    pub created_at: String,
    pub cooked: String,
    pub post_number: i32,
    pub post_type: i32,
    pub updated_at: String,
    pub reply_count: i32,
    pub reply_to_post_number: Value,
    pub quote_count: i32,
    pub avg_time: Value,
    pub incoming_link_count: i32,
    pub reads: i32,
    pub score: f64,
    pub yours: bool,
    pub topic_id: i32,
    pub topic_slug: String,
    pub display_username: Option<String>,
    pub primary_group_name: Value,
    pub primary_group_flair_url: Value,
    pub primary_group_flair_bg_color: Value,
    pub primary_group_flair_color: Value,
    pub version: i32,
    pub can_edit: bool,
    pub can_delete: bool,
    pub can_recover: bool,
    pub user_title: Value,
    pub raw: Option<String>,
    pub actions_summary: Vec<Value>,
    pub moderator: bool,
    pub admin: bool,
    pub staff: bool,
    pub user_id: i32,
    pub hidden: bool,
    pub hidden_reason_id: Value,
    pub trust_level: i32,
    pub deleted_at: Value,
    pub user_deleted: bool,
    pub edit_reason: Value,
    pub can_view_edit_history: bool,
    pub wiki: bool,
}

impl Api {
    pub fn new_unauthenticated(base_url: &str) -> Api {
        Api {
            base_url: base_url.to_string(),
            client_id: None,
            api_key: None,
        }
    }

    pub fn get_latest_topics(&self) -> Result<Vec<LatestTopic>, reqwest::Error> {
        match reqwest::get(&(self.base_url.clone() + "/latest.json")) {
            Ok(mut response) => {
                let mut json = response.text().unwrap();
                let v: Value = serde_json::from_str(&json).unwrap();
                json = v["topic_list"]["topics"].to_string();
                let topics: Vec<LatestTopic> = serde_json::from_str(&json).unwrap();
                Ok(topics)
            },
            Err(err) => return Err(err),
        }
    }

    pub fn get_topic_by_id(&self, id: i32) -> Result<Topic, reqwest::Error> {
        match reqwest::get(&(self.base_url.clone() + &format!("/t/{}.json", id))) {
            Ok(mut response) => {
                let mut json = response.text().unwrap();
                let topic: Topic = serde_json::from_str(&json).unwrap();
                Ok(topic)
            },
            Err(err) => return Err(err),
        }
    }

    pub fn get_categories(&self) -> Result<Vec<Category>, reqwest::Error> {
        match reqwest::get(&(self.base_url.clone() + "/categories.json")) {
            Ok(mut response) => {
                let mut json = response.text().unwrap();
                let v: Value = serde_json::from_str(&json).unwrap();
                json = v["category_list"]["categories"].to_string();
                let categories: Vec<Category> = serde_json::from_str(&json).unwrap();
                Ok(categories)
            },
            Err(err) => return Err(err),
        }
    }

    pub fn get_post_by_id(&self, id: i32) -> Result<Post, reqwest::Error> {
        match reqwest::get(&(self.base_url.clone() + &format!("/posts/{}.json", id))) {
            Ok(mut response) => {
                let mut json = response.text().unwrap();
                let post: Post = serde_json::from_str(&json).unwrap();
                Ok(post)
            },
            Err(err) => return Err(err),
        }
    }

    pub fn get_posts_in_topic(&self, topic: &Topic, index: i32, count: i32) 
        -> Result<Vec<Post>, reqwest::Error> {
        let post_index_list: &Vec<Value> = topic.post_stream["stream"].as_array().unwrap();
        let mut posts: Vec<Post> = Vec::new();
        for i in index..index+count {
            let post_id = post_index_list.get(i as usize).unwrap().as_i64().unwrap();
            match self.get_post_by_id(post_id as i32) {
                Ok(post) => posts.push(post),
                Err(err) => return Err(err),
            }
        }
        Ok(posts)
    }

    pub fn make_post_in_topic(&self, topic_id: i32, raw: String) 
        -> Result<Post, String> {
        match self.api_key {
            None => Err("Cannot post without an API key".to_string()),
            Some(ref api_key) => {
                let client = reqwest::Client::new();
                let form_data = [("topic_id", format!("{}", topic_id)), ("raw", raw)];
                header!{(UserApiKey, "User-Api-Key") => [String]}
                match client.post(&(self.base_url.clone() + "/posts"))
                    .header(UserApiKey(api_key.to_string()))
                    .form(&form_data)
                    .send() {
                        Ok(mut response) => {
                            let json = response.text().unwrap();
                            let post: Post = serde_json::from_str(&json).unwrap();
                            Ok(post)
                        },
                        Err(e) => Err(format!("{}", e))
                }
            }
        }
    }

    pub fn get_forum_name(&self) -> Result<String, reqwest::Error> {
        match reqwest::get(&(self.base_url.clone() + "/about.json")) {
            Ok(mut response) => {
                let mut json = response.text().unwrap();
                let v: Value = serde_json::from_str(&json).unwrap();
                json = v["about"]["title"].to_string();
                let name: String = serde_json::from_str(&json).unwrap();
                Ok(name)
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
            client_id: Some(self.client_id.clone()),
            api_key: Some(api_key), 
            base_url: self.base_url,
        };
        Ok(api)
    }
}
