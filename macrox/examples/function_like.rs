macro_rules! create_function {
    ($func_name: ident) => {
        fn $func_name() {
            println!("You created function: {:?}", stringify!($func_name));
        }
    };
}

create_function!(nba);
create_function!(cba);

macro_rules! print_result {
    ($expression: expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}

macro_rules! name {
    ($n: ident) => {
        fn $n() -> String {
            stringify!($n).into()
        }
    };
}

name!(gooo);

macro_rules! find_max {
    // Base case
    ($x: expr) => ($x);
    // `$x` followed by at least one `$y`
    ($x: expr, $($y: expr), +) => {
        std::cmp::max($x, find_max!($($y), +))
    };
}

macro_rules! calculate {
    (calc $op: expr) => {
        let value: usize = $op;
        println!("{} = {}", stringify!($op), value);
    };
    // Decompose multiple `eval`s recursively
    (calc $op: expr, $(calc $es: expr), +) => {
        calculate! { calc { $op } }
        calculate! { $(calc $es), + }
    };
}

fn main() {
    nba();
    cba();

    println!("gooo() is {:?}", gooo());
    print_result!(3_i32.pow(5));
    print_result!({
        let nba = "James".to_string();
        let laker = "lakers";
        nba + laker
    });

    println!("{}", find_max!(20, -9, 200, 22));
    println!(
        "{}",
        find_max!(30_i32.pow(3), 200000 % 3, 293020909, -200900)
    );

    calculate! {
        calc 2 + 3
    };

    calculate! {
        calc 30_usize.pow(3),
        calc (20 + 3) * 20 / 4 ,
        calc 3 + (4 * 5)
    }
}
