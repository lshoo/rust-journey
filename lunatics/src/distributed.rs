use std::time::Duration;

use lunatic::{host::node_id, sleep, AbstractProcess, Mailbox, Process, ProcessConfig};

use crate::model::Adder;

pub fn run(node_id: u64) {
    let nodes = lunatic::distributed::nodes();

    println!("Nodes {nodes:?}");

    let entry = |i: i32| {
        println!("{i}");
    };

    let nodes = if nodes.is_empty() {
        let node_id = lunatic::distributed::spawn(node_id, node_id as i64, entry, 0).unwrap();
        println!("there is no nodes, spawn node {node_id}");
        vec![node_id]
    } else {
        nodes
    };

    let mut config = ProcessConfig::new().unwrap();
    config.set_max_memory(1_500_000);
    config.set_max_fuel(1);

    let svc = if !nodes.is_empty() {
        Adder::on_node(node_id)
            .configure(&config)
            .start(())
            .unwrap()
    } else {
        Adder::link().start(()).unwrap()
    };

    assert_eq!(svc.request((1, 1)), 2);

    let msgs = [10, 582, 172, 45];
    let procs = nodes
        .into_iter()
        .map(|node| Process::spawn_node_config(node, &config, 101, hello));

    for (i, proc) in procs.enumerate() {
        proc.send(msgs[i % msgs.len()]);
    }
}

fn hello(start: u32, mailbox: Mailbox<u32>) {
    println!("Hi from {}", node_id());
    let m = mailbox.receive();
    println!("{start} + {m} = {}", start + m);
    sleep(Duration::from_millis(2000));
}
