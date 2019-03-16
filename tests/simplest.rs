use movie::actor;

actor! {
    SimpleActor
        input: Ping,
        on_message:
            Ping => (),
}

#[test]
fn test_simple_actor() {
    use SimpleActor::{Actor, Input};
    // Create and spawn the actor
    let actor = Actor {}.start();

    actor.send(Input::Ping);
    actor.stop(); // Will block, waiting for actor.
}
