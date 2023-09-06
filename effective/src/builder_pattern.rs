use derive_builder::Builder;

macro_rules! factorial {
    ($num: expr) => {{
        let mut result = 1;
        let mut i = $num;

        while i > 0 {
            result *= i;
            i -= 1;
        }

        result
    }};
}

#[derive(Builder, Debug)]
pub struct Car {
    pub color: String,
    pub transmission: Transmission,
    pub convertible: bool,
    pub mileage: u32,
}

#[derive(Clone, Debug)]
pub enum Transmission {
    Automatic,
    SemiAuto,
    Manual,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let car = CarBuilder::default()
        .color("red".to_string())
        .transmission(Transmission::Manual)
        .convertible(true)
        .mileage(2000)
        .build()?;

    println!("{car:?}");

    let factorial_ten = factorial!(10);
    println!("{factorial_ten}");

    Ok(())
}
