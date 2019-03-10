use movie::actor;

use std::sync::mpsc::Sender;
actor! {
    StreamParsingActor
        input:
            ChangeSource(String),
            SendState,
        // By default, Input enum does not have any trait auto-implemented.
        input_derive: Debug, PartialEq,
        // Whitespace and comments are irrelevant.
        // It's also optional to end sections (attributes) with a comma, with
        // exception of code attributes (on_stop, on_init etc.), which should
        // not end with comma, but either with nothing or with a semicolon.
        data:
            pub device: String,
            pub lines_parsed: u64,
            // Actors have their own modules, so in order to reference
            // the `use Sender` statement located above this `actor!` invocation,
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
            }
        tick_interval: 5, // Every 5ms, default = 100
        on_tick: // on_message have priority over on_tick
            self.lines_parsed += 1;
        on_stop: ()
        // custom_code must end with a semicolon
        custom_code:
            pub const DEFAULT_DEVICE: &'static str = "video0";
}

#[test]
fn test_stream_parsing_actor() {
    use StreamParsingActor::{Actor, Input, DEFAULT_DEVICE};

    use std::sync::mpsc::channel;
    let (tx, rx) = channel();
    let cfg = Actor {
        device: DEFAULT_DEVICE.to_string(),
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
    println!("Ticked {} times in 100ms", rx.recv().unwrap()); // 20

    actor.stop();
}
