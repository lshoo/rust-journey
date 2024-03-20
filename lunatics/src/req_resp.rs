use lunatic::ap::ProcessRef;

use crate::model::Adder;

pub fn run(process: ProcessRef<Adder>, arg: (i32, i32)) -> i32 {
    process.request(arg)
}
