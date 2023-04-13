use coerce::actor::{
    context::ActorContext,
    message::{Handler, Message},
    Actor, ActorId, IntoActorId,
};
use tokio::sync::oneshot::Sender;

use async_trait::async_trait;

pub struct ParentActor {
    pub child_count: usize,
    pub completed_actors: usize,
    pub on_work_completed: Option<Sender<usize>>,
}

#[async_trait]
impl Actor for ParentActor {
    async fn started(&mut self, ctx: &mut ActorContext) {
        for i in 0..self.child_count {
            ctx.spawn(format!("child-{i}").into_actor_id(), ChildActor)
                .await
                .unwrap();
        }
    }

    async fn on_child_stopped(&mut self, id: &ActorId, ctx: &mut ActorContext) {
        println!("id is {id}, id in ctx is {}", ctx.id());

        println!("ChildActor (id={}) stopped in ParentActor", id);

        self.completed_actors += 1;

        if ctx.supervised_count() == 0 && self.completed_actors == self.child_count {
            println!("All ChildActors finished, stopping ParentActor");

            if let Some(on_worker_completed) = self.on_work_completed.take() {
                let _ = on_worker_completed.send(self.completed_actors);
            }

            ctx.stop(None);
        }
    }
}

pub struct ChildActor;

#[async_trait]
impl Actor for ChildActor {
    async fn started(&mut self, ctx: &mut ActorContext) {
        println!("ChildActor (id={}) started", ctx.id());

        let _ = self
            .actor_ref(ctx)
            .scheduled_notify(Done, std::time::Duration::from_millis(5));
    }

    async fn stopped(&mut self, ctx: &mut ActorContext) {
        println!("ChildActor (id={} stopped in ChildActor", ctx.id());
    }
}

#[derive(Debug)]
struct Done;

impl Message for Done {
    type Result = ();
}

#[async_trait]
impl Handler<Done> for ChildActor {
    async fn handle(&mut self, msg: Done, ctx: &mut ActorContext) {
        println!(
            "ChildActor (id={} stopping after received {msg:?}",
            ctx.id()
        );
        ctx.stop(None);
    }
}
