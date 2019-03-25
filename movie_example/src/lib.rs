#![doc(html_root_url = "https://movie.pzmarzly.pl")]

//! # Example crate using `movie` library
//!
//! Code of this crate:
//!
//! ```rust,ignore
//! use movie::actor;
//!
//! actor! {
//!     SomeActor
//!         public_visibility: true,
//!         docs: /// This is an example actor.
//!         input: Ping,
//!         on_message:
//!             Ping => (),
//! }
//! ```
//!
//! Expanded (see [`cargo-expand`]):
//!
//! ```rust,ignore
//! /// This is an example actor.
//! pub mod SomeActor {
//!     use super::*;
//!     pub struct Actor {}
//!     pub enum Input {
//!         Ping,
//!     }
//!     pub type Handle = movie::Handle<std::thread::JoinHandle<()>, Input>;
//!     impl Actor {
//!         pub fn start(mut self) -> Handle {
//!             let (tx_ota, rx_ota) = std::sync::mpsc::channel();
//!             let (tx_kill, rx_kill) = std::sync::mpsc::channel();
//!             let handle = std::thread::spawn(move || {
//!                 {}; // on_init
//!                 let mut running = true;
//!                 while running {
//!                     while let Ok(message) = rx_ota.try_recv() {
//!                         use Input::*;
//!                         match message {
//!                             Ping => (), //on_message
//!                         };
//!                     }
//!                     if let Ok(_) = rx_kill.try_recv() {
//!                         running = false;
//!                         {}; // on_stop
//!                     }
//!                     {}; // on_tick
//!                     use std::thread::sleep;
//!                     use std::time::Duration;
//!                     sleep(Duration::from_millis(100));
//!                 }
//!             });
//!             movie::Handle {
//!                 join_handle: handle,
//!                 tx: tx_ota,
//!                 kill: tx_kill,
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! [`cargo-expand`]: https://github.com/dtolnay/cargo-expand

use movie::actor;

actor! {
    SomeActor
        public_visibility: true,
        docs: /// This is an example actor.
        input: Ping,
        on_message:
            Ping => (),
}
