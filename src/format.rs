#![forbid(unsafe_code)]

pub fn prcnt(n1: f64, n2: f64) -> f64 {
    //(n1 - n2 / (n1 + n2/ 2_f64)) * 100
    let sub = n1 - n2;
    let add = n1 + n2;
    let div = add / 2_f64;
    sub / div * 100_f64
}

pub fn reduc(num: f64) -> f64 {
    (num * 100.0).round() / 100.0
}
