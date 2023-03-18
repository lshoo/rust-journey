fn sum(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let op: fn(i32, i32) -> i32 = sum; // must transfer to function pointers
                                       // let op = sum;
    let op1 = op;
    let op2 = op;

    assert_eq!(op1, op2);
    println!("{op:p}");

    let amount_to_add = 2;
    let add_2 = |y| y + amount_to_add; // closure is a struct
    assert_eq!(7, add_2(5));
}
