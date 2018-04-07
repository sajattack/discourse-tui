use chrono::prelude::*;
use chrono_humanize;
use chrono;

pub fn datestring_to_humanstring(ds: &str) -> String {
    let dt = ds.parse::<DateTime<Utc>>().unwrap();
    let dur = dt.signed_duration_since(chrono::Utc::now());
    let ht = chrono_humanize::HumanTime::from(dur); 
    format!("{}", ht)
}

pub fn format_int(int: i32) -> String {
    let num_k:  f32;
    let dec_prec:  usize;
    if int > 1000 {
        num_k = int as f32/1000.0;
    } else {
        return int.to_string();
    }
    if int < 100_000 {
        dec_prec = 1;
    } else {
        dec_prec = 0;
    }
    format!("{0:.1$}k", num_k, dec_prec) 
}

#[test]
fn test_100100() {
    assert_eq!(format_int(100100), "100k")
}

pub fn pad_or_trunc(mut s: String, width: usize) -> String {
    if width > s.len() {
        format!("{:w$}", s, w=width)
    } else {
        s.truncate(width);
        s
    }
}
