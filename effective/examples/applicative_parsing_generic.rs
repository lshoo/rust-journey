struct Magic<T>(T);

impl<T> Magic<T> {
    fn run(self) -> T {
        self.0
    }
}

impl<T> Magic<T> {
    fn map<R, F: Fn(T) -> R>(self, f: F) -> Magic<R> {
        Magic(f(self.0))
    }
}

fn main() {
    let bin = Magic("Hello");
    let result = bin.run();
    println!("{result:?}");
    let res2 = Magic(20).map(|s| s * s);
    println!("{}", res2.0);

    assert_eq!(result, "Hello");
}
