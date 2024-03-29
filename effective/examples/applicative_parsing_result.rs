pub struct Magic<T>(Result<T, String>);

impl<T> Magic<T> {
    pub fn run(self) -> Result<T, String> {
        self.0
    }

    pub fn flat_map<R, F: Fn(T) -> Result<R, String>>(self, f: F) -> Magic<R> {
        match self.run() {
            Ok(t) => Magic(f(t)),
            Err(e) => Magic(Err(e)),
        }
    }

    pub fn map<R, F: Fn(T) -> R>(self, f: F) -> Magic<R> {
        self.flat_map(|t| Ok(f(t)))
    }

    pub fn zip<R>(self, other: Magic<R>) -> Magic<(T, R)> {
        match (self.0, other.0) {
            (Ok(t), Ok(r)) => Magic(Ok((t, r))),
            (_, Err(err)) | (Err(err), _) => Magic(Err(err)),
        }
    }

    pub fn alt(self, other: Self) -> Self {
        match &self.0 {
            Ok(_) => self,
            Err(_) => other,
        }
    }

    pub fn pure(val: T) -> Self {
        Self(Ok(val))
    }

    pub fn fail(msg: &str) -> Self {
        Self(Err(msg.into()))
    }
}

fn main() {
    let bin = Magic(Ok("Hello"));
    let result = bin.run();
    println!("{result:?}");
    let res2 = Magic(Ok(20)).map(|s| s * s);
    println!("{:?}", res2.0);

    assert_eq!(result, Ok("Hello"));

    let zip1 = Magic(Ok("zip"));
    let zip2 = Magic(Ok(30));
    let zip = zip1.zip(zip2);
    println!("{:?}", zip.0);

    let nick = Magic::fail("No nick name give");
    let fullname = Magic::pure("Bob the Maginficent");
    let name = nick.alt(fullname);

    println!("{:?}", name.0);
}
