macro_rules! vec_str {
    ($($e:expr), *) => {
        {
            let mut v = Vec::new();
            $(
                v.push($e.to_string());
            )*

            v
        }
    };
 }

fn main() {
    let v = vec_str![1, "a", true, 3.14159f32];

    for i in v {
        println!("{i}");
    }
}
