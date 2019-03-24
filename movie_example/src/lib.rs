#![doc(html_root_url = "https://movie.pzmarzly.pl")]

//! # Example crate using `movie` library
//!
//! See ['movie'].
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

use movie::actor;

actor! {
    SomeActor
        public_visibility: true,
        docs: /// This is an example actor.
        input: Ping,
        on_message:
            Ping => (),
}
