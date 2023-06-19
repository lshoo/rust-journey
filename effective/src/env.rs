pub fn verbose() {
    let name = "USER";
    match std::env::var(name) {
        Ok(val) => println!("{}: {}", name, val),
        Err(e) => println!("${} is not set ({})", name, e),
    }
}

pub fn short() {
    let v = std::env::var("USER").expect("$USER is not set");
    println!("$USER is {v}");
}
