use std::marker::PhantomData;

use super::{MyReciever, MySender};

pub struct SendOnIterator<T, S: MySender<T>> {
    sender: Box<S>,
    p: PhantomData<T>,
}
impl<T, S: MySender<T>> SendOnIterator<T, S> {
    pub fn new(sender: S) -> Self {
        Self {
            sender: Box::new(sender),
            p: PhantomData,
        }
    }
    pub fn consume<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for value in iter {
            self.sender.send(value);
        }
        self.sender.set_done();
    }
    pub fn new_and_consume<I: IntoIterator<Item = T>>(mut sender: S, iter: I) {
        for value in iter {
            sender.send(value);
        }
        sender.set_done();
    }
}
pub struct ReceiverToIterator<T, R: MyReciever<T>> {
    receiver: Box<R>,
    p: PhantomData<T>,
}
impl<T, R: MyReciever<T>> ReceiverToIterator<T, R> {
    pub fn new(receiver: R) -> Self {
        Self {
            receiver: Box::new(receiver),
            p: PhantomData,
        }
    }
}
impl<T, R: MyReciever<T>> Iterator for ReceiverToIterator<T, R> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.receiver.isdone() {
                break None;
            }
            if let Some(t) = self.receiver.receive() {
                break Some(t);
            }
        }
    }
}
