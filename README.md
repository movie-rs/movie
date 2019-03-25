# movie

## An actor/thread orchestration library

### Overview

- next to no boilerplate - see examples
- works with `stable` compiler, but requires 2018 edition
- no external dependencies except for `std`
- enum-based communication over MPSC channels
- by default, one actor = one thread
- by default, actors only accept messages, they do not send replies
  - solution to sending replies is not the most elegant right now,
    see [Advanced example](#advanced-example) below
- network RPC should be possible but is beyond the scope of this crate.
  If you want to do this, you can use `input_derive` and `custom_code` to
  derive `Serialize` and `Deserialize`.
- two procedural macros - see [`movie_derive`]
- actors need to be defined in module/crate scope
- bad error messages for now, macro + manual string parsing magic
- in case of large breaking changes in (stable) `TokenStream::to_string()`,
  the macros may break

### Examples

The examples below are test-`ignore`d as AFAIK it's impossible to run procedural
macros in doc-tests. They are also in `tests` directory, where they are tested.

#### Installation

```toml
[dependencies]
"movie" = "0.1"
```

#### Simple actor

```rust
use movie::actor;

actor! { SimplestActor } // completely useless

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
```

#### Advanced example

```rust
use movie::actor;

use std::sync::mpsc::Sender;
actor! {
    StreamParsingActor
        public_visibility: true,
        docs: /// Actor that parses video from V4L2 device
              /// It's very consistent - failed every time so far.
        input:
            ChangeSource(String),
            SendState,
        // By default, Input enum does not have any trait auto-implemented.
        input_derive: Debug, PartialEq,
        // Whitespace and comments are irrelevant.
        // It's also optional to end sections (attributes) with a comma, with
        // exception of code attributes (on_stop, on_init etc.), which should
        // not end with comma, but rather either with nothing or with a semicolon.
        data:
            pub device: String,
            pub state_tx: Sender<u64>,
        on_init:
            if self.device == "admin secret device" {
                panic!("No access right for admin secret device");
            }
            let mut lines_parsed = 0; // This variable will be exposed to on_message.
                                      // This is suboptimal, but it is the simplest
                                      // way to allow for thread-local variables (`data`
                                      // is sent between threads, so it couldn't be used
                                      // e.g. for GTK references)
        on_message:
            ChangeSource(name) => {
                self.device = name;
            },
            SendState => {
                self.state_tx.send(lines_parsed).unwrap();
            }
        tick_interval: 5, // Every 5ms, default = 100
        on_tick: // on_message have priority over on_tick
            lines_parsed += 1;
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
        state_tx: tx,
    };
    // Spawn the actor, let on_init run
    let actor = cfg.start(); // returns StreamParsingActor::Handle

    use std::thread::sleep;
    use std::time::Duration;
    sleep(Duration::from_millis(100));

    // We can use auto-derived traits on Input
    actor.send(dbg!(Input::SendState));
    println!("Ticked {} times in 100ms", rx.recv().unwrap()); // 20

    actor.stop();
}
```

### Actor attributes

These words if followed by colon, are restricted keywords.

- `input` - defines `Input` enum
- `input_derive` - `#[derive()]` for `Input` enum
- `data` - actor stateful variables, need to be set when creating actor
- `on_init` - runs just before an actor starts accepting messages
- `on_message` - defines `match message` logic
- `tick_interval` - time in milliseconds between tick. When undefined, set to 100ms.
   Affects message polling, so don't set it too high.
- `on_tick` - runs every tick
- `on_stop` - runs just after an actor stops accepting messages
- `spawner` - name of the function that spawns thread (by default
  `std::thread::spawn`, put a function with similar signature here to have actors be run
  as futures, M:N threads etc.)
- `spawner_return_type` - return type of `spawner` (by default
  `std::thread::JoinHandle<()>`)
- `custom_code` - code to be inserted into generated actor module
- `public_visibility` - if `true`, then the actor module is public
- `docs` - place docs here - e.g. `docs: /// An actor`

Some code can break macro internals (e.g. `break` or `continue` without
defining your own loop can break actor's main loop, putting `on_stop: (),` will result
in an invalid comma). Debugging it can be cryptic, hopefully [`actor_dbg`] (when the code
doesn't compile) and `cargo expand` (when it does) will help you in such situations.

### History

Previously, I've written [`x11-input-supercharger`], an utility for auto-scrolling
and conditional mouse-to-keyboard rebinding (without changing the keymap itself,
which causes Chromium-based applications to freeze for a second). It had many threads
running - one for auto-scrolling, other for rebinding, another for polling X11
events, and yet another for displaying GTK3 window with Windows-like indicator
of auto-scrolling state. The code was not that complex, and the messaging hierarchy
was simple (parent messaged its children, never the other way around), yet the boilerplate
grown to become quite a hindrance when it comes to code readability, especially since
the threads also had to do their own work in addition to reacting to messages
(the work includes interacting with X11, GTK3, and tracking time). Instead of using
[`actix`], I decided to try to implement my own library, inspired by [`actress`], and
spent about 12 hours doing so (I'm far from being a fluent Rust programmer, also, this
was my first time using procedural macros).

This is my first "real" library (my previous crates/Rust programs were CLI/GUI utilities,
sometimes with simple public Rust API). I published some of them to crates.io, but not all
(some are on [Github], some not published yet). I learned about procedural macros (mostly about
their current shortcomings) - sadly, there are not many good resources about them.
I sticked mostly to Rust reference. Surprisingly, I was also unable to quickly Google the
"right" way to do documentation (with links and examples) - so I used the first edition of Rust
book. I should probably finally read the second edition to familiarize myself with what's
inside and where it is (back when I started, there was no second edition yet).

### Usage in the wild

If you use this library in your project, consider putting it here (if possible). It will
help me with testing whether the new change I introduce breaks anything.

- [`x11-input-supercharger`] by me

### Docs

README.md is built using [`cargo-readme`] (`cargo readme > README.md`). The documentation
is built by [`docs.sh`] script.

### License

Copyright 2019 Paweł Zmarzły. Licensed under either of

 * Apache License, Version 2.0
   ([`LICENSE-APACHE.md`] or [https://www.apache.org/licenses/LICENSE-2.0])
 * MIT license
   ([`LICENSE-MIT.md`] or [https://opensource.org/licenses/MIT])

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[`movie_derive`]: ../movie_derive/index.html
[`actor_dbg`]: ../movie_derive/macro.actor_dbg.html
[`x11-input-supercharger`]: https://github.com/pzmarzly/x11-input-supercharger
[`actix`]: https://actix.rs
[`actress`]: https://docs.rs/actress/0.1.0/actress
[Github]: https://github.com/pzmarzly
[`cargo-readme`]: https://github.com/livioribeiro/cargo-readme
[`docs.sh`]: https://github.com/movie-rs/movie/blob/master/docs.sh
[`LICENSE-APACHE.md`]: https://github.com/movie-rs/movie/blob/master/LICENSE-APACHE.md
[https://www.apache.org/licenses/LICENSE-2.0]: https://www.apache.org/licenses/LICENSE-2.0
[`LICENSE-MIT.md`]: https://github.com/movie-rs/movie/blob/master/LICENSE-MIT.md
[https://opensource.org/licenses/MIT]: https://opensource.org/licenses/MIT

License: MIT OR Apache-2.0
