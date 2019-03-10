//! # A neat (but over-engineered) actor library
//!
//! ## Overview
//!
//! - next to no boilerplate - see examples
//! - no external dependencies except for `std`
//! - enum-based communication over MPSC channels
//! - by default, one actor = one thread
//! - by default, actors only accept messages, they do not send replies
//!   - solution to sending replies is not the most elegant right now,
//!     see "Advanced example" below
//! - network RPC should be possible but is beyond the scope of this crate.
//!   If you want to do this, you can use `input_derive` and `custom_code` to
//!   derive `Serialize` and `Deserialize`.
//! - two procedural macros - see [`movie_derive`](../movie_derive/index.html)
//! - actors need to be defined in module/crate scope
//! - bad error messages for now, macro + manual string parsing magic
//!
//! ## Examples
//!
//! The examples below are test-`ignore`d as AFAIK it's impossible to run procedural
//! macros in doc-tests. They are also in `tests` directory, where they are tested.
//!
//! ### Installation
//!
//! ```toml
//! [dependencies]
//! "movie" = "0.1"
//! ```
//!
//! ### The simplest actor
//!
//! ```rust,ignore
//! use movie::actor;
//!
//! actor! {
//!     SimpleActor
//!         input: Ping,
//!         on_message:
//!             Ping => (),
//! }
//!
//! #[test]
//! fn test_simple_actor() {
//!     use SimpleActor::{Actor, Input};
//!     // Create and spawn the actor
//!     let mut actor = Actor {}.start();
//!
//!     actor.send(Input::Ping);
//!     actor.stop(); // Will block, waiting for actor.
//! }
//! ```
//!
//! ### Advanced example
//!
//! ```rust,ignore
//! use movie::actor;
//!
//! use std::sync::mpsc::Sender;
//! actor! {
//!     StreamParsingActor
//!         input:
//!             ChangeSource(String),
//!             SendState,
//!         // By default, Input struct does not have any trait auto-implemented.
//!         input_derive: Debug, PartialEq,
//!         // Whitespace and comments are irrelevant.
//!         // It's also optional to end sections (attributes) with a comma, with
//!         // exception of code attributes (on_stop, on_init etc.), which should
//!         // not end with comma, but either with nothing or with a semicolon.
//!         data:
//!             pub device: String,
//!             pub lines_parsed: u64,
//!             // Actors have their own modules, so in order to reference
//!             // the `use Sender` statement located above this `actor!` invocation,
//!             // we need to use `super::`. Similarly in case of custom types.
//!             pub state_tx: super::Sender<u64>,
//!         on_init:
//!             if self.device == "admin secret device" {
//!                 panic!("No access right for admin secret device");
//!             }
//!         on_message:
//!             ChangeSource(name) => {
//!                 self.device = name;
//!             },
//!             SendState => {
//!                 self.state_tx.send(self.lines_parsed).unwrap();
//!             }
//!         tick_interval: 5, // Every 5ms, default = 100
//!         on_tick: // on_message have priority over on_tick
//!             self.lines_parsed += 1;
//!         on_stop: ()
//!         // custom_code must end with a semicolon
//!         custom_code:
//!             pub const DEFAULT_DEVICE: &'static str = "video0";
//! }
//!
//! #[test]
//! fn test_stream_parsing_actor() {
//!     use StreamParsingActor::{Actor, Input, DEFAULT_DEVICE};
//!
//!     use std::sync::mpsc::channel;
//!     let (tx, rx) = channel();
//!     let cfg = Actor {
//!         device: DEFAULT_DEVICE.to_string(),
//!         lines_parsed: 0,
//!         state_tx: tx,
//!     };
//!     // Spawn the actor, let on_init run
//!     let mut actor = cfg.start();
//!
//!     use std::thread::sleep;
//!     use std::time::Duration;
//!     sleep(Duration::from_millis(100));
//!
//!     // We can use auto-derived traits on Input
//!     actor.send(dbg!(Input::SendState));
//!     println!("Ticked {} times in 100ms", rx.recv().unwrap()); // 20
//!
//!     actor.stop();
//! }
//! ```
//!
//! ## Actor attributes
//!
//! These words if followed by colon, are restricted keywords.
//!
//! - `input` - required, defines `Input` enum
//! - `input_derive` - optional, `#[derive()]` for `Input` enum
//! - `data` - optional, actor stateful variables, need to be set when creating actor
//! - `on_init` - optional, runs just before an actor starts accepting messages
//! - `on_message` - required, defines `match message` logic
//! - `tick_interval` - optional, time in milliseconds between tick. When undefined, set to 100ms.
//!    Affects message polling, so don't set it too high.
//! - `on_tick` - optional, runs every tick
//! - `on_stop` - optional, runs just after an actor stops accepting messages
//! - `custom_code` - optional, code to be inserted into generated actor module
//!
//! Some invalid Rust code can break macro internals (e.g. `break` or `continue` without
//! defining your own loop can break actor's main loop, putting `on_stop: (),` will result
//! in invalid comma). Debugging it can be cryptic, hopefully `actor_dbg` (when the code
//! doesn't compile) and `cargo expand` (when it does) will help you in such situations.
//!

pub use movie_derive::*;
pub use movie_utils::*;
