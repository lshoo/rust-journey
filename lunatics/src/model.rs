use lunatic::{
    ap::{handlers::Request, Config, RequestHandler, State},
    serializer::MessagePack,
    AbstractProcess,
};

pub struct Adder;

impl AbstractProcess for Adder {
    type Arg = ();
    type State = Self;
    type Handlers = (Request<(i32, i32)>,);
    type Serializer = MessagePack;
    type StartupError = ();

    fn init(_: Config<Self>, _: ()) -> Result<Self, ()> {
        Ok(Adder)
    }
}

impl RequestHandler<(i32, i32)> for Adder {
    type Response = i32;

    fn handle(_: State<Self>, (a, b): (i32, i32)) -> i32 {
        let result = a + b;
        println!("{a} + {b} = {result}");

        result
    }
}
