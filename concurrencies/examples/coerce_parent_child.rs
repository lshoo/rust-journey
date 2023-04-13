use coerce::actor::{system::ActorSystem, IntoActor};
use concurrencies::coerces::parent_child::ParentActor;
use tokio::sync::oneshot::channel;

#[tokio::main]
async fn main() {
    let system = ActorSystem::new();
    let (tx, rx) = channel();

    const TOTAL_CHILD_COUNT: usize = 10;

    let actor = ParentActor {
        child_count: TOTAL_CHILD_COUNT,
        completed_actors: 0,
        on_work_completed: Some(tx),
    }
    .into_actor(Some("ParentActor"), &system)
    .await
    .unwrap();

    let completed_actors = rx.await.ok();

    assert_eq!(completed_actors, Some(TOTAL_CHILD_COUNT));

    tokio::signal::ctrl_c()
        .await
        .expect("failed to listen for event");
}
