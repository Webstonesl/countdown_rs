use super::*;
use std::{
    sync::mpsc::{Receiver, RecvTimeoutError, SyncSender},
    time::Duration,
};
pub struct ThreadSender<T>(Option<SyncSender<T>>);
pub struct ThreadReceiver<T>(Receiver<T>, bool);
unsafe impl<T> Sync for ThreadReceiver<T> {}
unsafe impl<T> Send for ThreadReceiver<T> {}
impl<T> MySender<T> for ThreadSender<T> {
    fn send(&mut self, value: T) -> bool {
        match &mut self.0 {
            | Some(e) => {
                if let Err(e) = e.send(value) {
                    eprintln!("{e}");
                    panic!();
                }
                true
            }
            | None => false,
        }
    }
    fn set_done(&mut self) {
        self.0 = None;
    }
}
impl<T> Drop for ThreadSender<T> {
    fn drop(&mut self) {
        self.set_done();
    }
}
impl<T> MyReciever<T> for ThreadReceiver<T> {
    fn receive(&mut self) -> Option<T> {
        match self.0.recv_timeout(Duration::from_millis(10)) {
            | Ok(a) => Some(a),
            | Err(RecvTimeoutError::Disconnected) => {
                eprintln!("Disconected");
                self.1 = true;
                None
            }
            | Err(RecvTimeoutError::Timeout) => None,
        }
    }
    fn isdone(&self) -> bool {
        self.1
    }
}
pub fn channel<T>() -> (ThreadSender<T>, ThreadReceiver<T>) {
    let (sender, receiver) = std::sync::mpsc::sync_channel(10);
    (ThreadSender(Some(sender)), ThreadReceiver(receiver, false))
}
