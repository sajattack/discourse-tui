use chrono::prelude::*;
use chrono_humanize;
use chrono;

pub fn datestring_to_humanstring(ds: &str) -> String {
    let dt = ds.parse::<DateTime<Utc>>().unwrap();
    let dur = dt.signed_duration_since(chrono::Utc::now());
    let ht = chrono_humanize::HumanTime::from(dur); 
    format!("{}", ht)
}
