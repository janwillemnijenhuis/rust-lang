pub fn return_mean(mut v: Vec<i32>) -> f64 {
    let vec_len: f64 = v.len() as f64;
    let mut sum: f64 = 0 as f64;
    for i in v {
        sum += i as f64;
    }
    return sum / vec_len;
}