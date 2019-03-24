#![doc(html_root_url = "https://movie.pzmarzly.pl")]

//! `movie_utils` - crate containing `Handle` type and `JoinableHandle` trait.

use std::thread::JoinHandle;

/// Trait for `join()` method that allow to to wait on actor.
/// Implemented for [`std::thread::JoinHandle`].
///
/// [`std::thread::JoinHandle`]: https://doc.rust-lang.org/stable/std/thread/struct.JoinHandle.html
pub trait JoinableHandle {
    fn join(self);
}

impl JoinableHandle for JoinHandle<()> {
    #[allow(unused_must_use)]
    fn join(self) {
        self.join();
    }
}

/// Handle returned by `Actor::start()`. Generic version.
pub struct Handle<T: JoinableHandle, TX> {
    /// The underlying handle to process, thread, task, future, etc.
    pub join_handle: T,
    /// Sender of channel used to send messages to an actor.
    pub tx: std::sync::mpsc::Sender<TX>,
    /// Sender of channel used to ask an actor to stop.
    ///
    /// `kill` is used internally, use [`stop()`] instead.
    ///
    /// [`stop()`]: #method.stop
    pub kill: std::sync::mpsc::Sender<()>,
}

impl<T: JoinableHandle, TX> Handle<T, TX> {
    /// Wrapper on `tx.send(msg).unwrap()`.
    pub fn send(&self, msg: TX) {
        self.tx.send(msg).unwrap();
    }
    #[allow(unused_must_use)]
    /// Asks the actor to stop and waits (blocking) for it to stop.
    pub fn stop(self) {
        self.kill.send(());
        self.join_handle.join();
    }
}
