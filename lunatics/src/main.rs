use lunatic::{AbstractProcess, Mailbox};
use lunatics::model::Adder;

#[lunatic::main]
fn main(_: Mailbox<()>) {
    let svc = Adder::link().start(()).unwrap();
    let node_id = svc.node_id();
    println!("the Adder node id is: {node_id:?}");

    assert_eq!(lunatics::req_resp::run(svc, (1, 2)), 3);

    lunatics::distributed::run(node_id);
}
