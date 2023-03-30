use ractor::{Actor, ActorProcessingErr, ActorRef, Message,};


// Define Actor 
pub struct PingPong;

// Define message(event)
#[derive(Debug, Clone)]
pub enum Event {
    Add(u64), 
    Sub(u64),
}

// #[cfg(feature = "culster")]
impl Message for Event { }

impl Event {
    fn next(&self) -> Self {
        match self {
            Self::Add(v) => Self::Sub( v / 2),
            Self::Sub(v) => Self::Add(*v),
        }
    }

    fn print(&self) {
        match self {
            Self::Add(v) => println!("Add {v}"),
            Self::Sub(v) => println!("Sub {v}"),
        }
    }
}

// Actor logic 
#[async_trait::async_trait]
impl Actor for PingPong {
    type Msg = Event;
    type State = u64;
    type Arguments = ();

    async fn pre_start(&self, actor_ref: ActorRef<Self>, _: ()) -> Result<Self::State, ActorProcessingErr> {
        actor_ref.send_message(Event::Add(20)).unwrap();
        Ok(0)
    }

    async fn handle(
        &self, 
        actor_ref: ActorRef<Self>,
        msg: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        if *state < 10 {
            msg.print();
            actor_ref
                .send_message(msg.next())
                .unwrap();
            *state += 1;
        } else {
            actor_ref.stop(None);
        }

        Ok(())
    }
}

pub async fn run()  {
    let (_, actor_handle) = Actor::spawn(Some("PingPongActor".into()), PingPong, ())
        .await
        .expect("Failed to start actor");
    actor_handle.await.expect("Actor failed to exit cleanly");
}

