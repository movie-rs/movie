# movie

## A concise, but over-engineered and relatively slow actor library

### Examples

The examples below are `ignore`d as AFAIK it's impossible to run procedural
macros in doc-tests. They are also in `tests` directory, where they are tested.

#### The simplest actor

```rust
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
    let mut actor = Actor {}.start();

    actor.send(Input::Ping);
    actor.stop();
}
```

#### Advanced example

```rust
use std::sync::mpsc::Sender;
actor! {
    StreamParsingActor
        input:
            ChangeSource(String),
            SendState,
        // By default, Input struct does not have any trait auto-implemented.
        input_derive: Debug, PartialEq,
        // Whitespace and comments are irrelevant.
        // If there are no bugs, it's also optional to end sections (attributes) with a comma.
        data:
            pub device: String,
            pub lines_parsed: u64,
            // Actors have their own modules, so in order to reference
            // the `use Sender` statement that is just above `actor!` invocation,
            // we need to use `super::`. Similarly in case of custom types.
            pub state_tx: super::Sender<u64>,
        on_init:
            if self.device == "admin secret device" {
                panic!("No access right for admin secret device");
            }
        on_message:
            ChangeSource(name) => {
                self.device = name;
            },
            SendState => {
                self.state_tx.send(self.lines_parsed).unwrap();
            },
        tick_interval: 5, // Every 5ms, default = 100
        on_tick: // on_message have priority over on_tick
            self.lines_parsed += 1;
        on_stop: ()
}

#[test]
fn test_stream_parsing_actor() {
    use StreamParsingActor::{Actor, Input};

    use std::sync::mpsc::channel;
    let (tx, rx) = channel();
    let cfg = Actor {
        device: "video0".to_string(),
        lines_parsed: 0,
        state_tx: tx,
    };
    // Spawn the actor, let on_init run
    let mut actor = cfg.start();

    use std::thread::sleep;
    use std::time::Duration;
    sleep(Duration::from_millis(100));

    // We can use auto-derived traits on Input
    actor.send(dbg!(Input::SendState));
    println!("Ticked {} times in 100ms", rx.recv().unwrap());

    actor.stop();
}
```
