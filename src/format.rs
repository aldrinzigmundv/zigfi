#![forbid(unsafe_code)]

use chrono::{DateTime, Utc};

//Calculates percentage difference between two f64 values
pub fn prcnt(n1: f64, n2: f64) -> f64 {
    //(n1 - n2 / (n1 + n2/ 2_f64)) * 100
    let sub = n1 - n2;
    let add = n1 + n2;
    let div = add / 2_f64;
    sub / div * 100_f64
}

//Reduces f64 length to 2 decimals
pub fn reduc(num: f64) -> f64 {
    (num * 100.0).round() / 100.0
}

//Returns DateTime Utc value based on given value
pub fn get_time(interval: &str) -> (DateTime<Utc>, DateTime<Utc>) {
    let now = chrono::Utc::now();
    let mut back = chrono::Duration::days(2);
    if interval.eq_ignore_ascii_case("1mo") {
        back = chrono::Duration::weeks(4);
    } else if interval.eq_ignore_ascii_case("1y") {
        back = chrono::Duration::weeks(52);
    }
    (now - back, now)
}
