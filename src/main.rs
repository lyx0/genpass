use rand::Rng;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // genpass 24
    let length = &args[1];

    let password = generate_pass(length);
    println!("{}", password);
}

fn generate_pass(length: &str) -> String {
    let mut rng = rand::thread_rng();
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()";
    let password: String = (0..length.parse::<usize>().unwrap())
        .map(|_| {
            charset
                .chars()
                .nth(rng.gen_range(0, charset.len()))
                .unwrap()
        })
        .collect();

    password
}
