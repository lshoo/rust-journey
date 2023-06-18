//! cargo run -p effective --example  evn_bin

use effective::env::{short, verbose};

fn main() {
    verbose();
    short();

    let shell = env!("SHELL", "$SHELL is not set");
    println!("Shell is set to {shell}");

    let name = env!("NAME");
    println!("the computer name is {name}");

    // let lorem = env!("LOREM_IPSUM");
    // will compile error

    let lorem = option_env!("LOREM_IPSUM").unwrap_or("not set");
    println!("LOREM_IPSUM is {lorem}");

    println!("all envs:---");
    for (n, v) in std::env::vars() {
        println!("{}: {}", n, v);
    }

    println!("------------------");
    for (n, v) in std::env::vars_os() {
        println!("{}: {}", n.into_string().unwrap(), v.into_string().unwrap());
    }

    // set env
    let key = "GOROOT";
    std::env::set_var(key, "goroot");
    println!("{}", std::env::var(key).unwrap());
    
}