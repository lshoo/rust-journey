struct Magic(&'static str);

impl Magic {
    fn run(&self) -> String {
        self.0.into()
    }
}

fn main() {
    let bin = Magic("Hello");
    let result = bin.run();
    println!("{result:?}");

    assert_eq!(result, "Hello");
}
