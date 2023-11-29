use macros_builder::Builder;

#[allow(dead_code)]
#[derive(Debug, Builder)]
pub struct Command {
    executable: String,
    #[builder(abc = "xyz")]
    args: Vec<String>,
    env: Option<Vec<String>>,
    current_dir: Option<String>,
}

fn main() {
    let command = Command::builder()
        .executable("find")
        .args(vec!["-c".into(), "-vv".into()])
        .env(vec![])
        // .current_dir(".")
        .finish()
        .unwrap();

    println!("{:?}", command);
}
