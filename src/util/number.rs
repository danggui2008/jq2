use rand::Rng;

pub fn gen_random_num(length: i32) -> i32 {
    let mut num = 1;

    let random = rand::thread_rng().gen_range(0.1..1.0);

    for _ in 0..length {
        num *= 10
    }

    (random * num as f64) as i32
}
