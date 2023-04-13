use std::time::Duration;

use async_trait::async_trait;
use coerce::actor::message::{Handler, Message};
use coerce::actor::scheduler::timer::{Timer, TimerTick};
use coerce::actor::Actor;
use coerce::actor::{context::ActorContext, new_actor};

use serde::{Deserialize, Serialize};
use tokio::time::sleep;

// Define Actor
pub struct EchoActor {}

#[async_trait]
impl Actor for EchoActor {}

// Define handler(behaviour) for Actor
#[async_trait]
impl Handler<EchoMessage> for EchoActor {
    async fn handle(&mut self, message: EchoMessage, _ctx: &mut ActorContext) -> String {
        message.0
    }
}

#[async_trait]
impl Handler<PrintTimer> for EchoActor {
    async fn handle(&mut self, msg: PrintTimer, _ctx: &mut ActorContext) {
        println!("{}", msg.0)
    }
}

// Define event/message
#[derive(Serialize, Deserialize)]
pub struct EchoMessage(String);

impl Message for EchoMessage {
    type Result = String;
}

// Define timer
#[derive(Clone, Debug)]
pub struct PrintTimer(String);

impl Message for PrintTimer {
    type Result = ();
}

impl TimerTick for PrintTimer {}

pub async fn run() {
    let actor = new_actor(EchoActor {})
        .await
        .expect("Failed to create actor");
    let hello_world = "hello world".to_string();

    let result = actor.send(EchoMessage(hello_world.clone())).await;

    println!("{result:?}");

    assert_eq!(result, Ok(hello_world.clone()));

    let timer = Timer::start(
        actor.clone(),
        Duration::from_secs(5),
        PrintTimer(hello_world),
    );

    sleep(Duration::from_secs(23)).await;

    timer.stop();
}
