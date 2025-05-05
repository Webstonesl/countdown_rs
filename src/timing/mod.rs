use std::marker::PhantomData;

use filter::{ReceiverFilter, SenderFilter};
use iterators::ReceiverToIterator;
use map::{ReceiverMap, SenderMap};

pub trait MySender<T>: Sized {
    fn send(&mut self, value: T) -> bool;
    fn set_done(&mut self);
    fn map<'a, S, F: FnMut(T) -> S>(&'a mut self, func: F) -> SenderMap<'a, Self, T, F> {
        SenderMap::new(self, func)
    }
    fn filter<'a, F: FnMut(&T) -> bool>(&'a mut self, func: F) -> SenderFilter<'a, Self, T, F> {
        SenderFilter::new(self, func)
    }
    fn blocked<'a>(&'a mut self) -> BlockedSender<'a, Self, T> {
        BlockedSender::new(self)
    }
}
pub struct BlockedSender<'a, S: MySender<T>, T>(&'a mut S, PhantomData<T>);

impl<'a, S: MySender<T>, T> MySender<T> for BlockedSender<'a, S, T> {
    fn send(&mut self, value: T) -> bool {
        self.0.send(value)
    }

    fn set_done(&mut self) {}
}

impl<'a, S: MySender<T>, T> BlockedSender<'a, S, T> {
    fn new(sender: &'a mut S) -> Self {
        Self(sender, PhantomData)
    }
}

pub trait MyReciever<T>: Sized {
    fn receive(&mut self) -> Option<T>;
    fn isdone(&self) -> bool;
    fn map<'a, S, F: FnMut(T) -> S>(&'a mut self, func: F) -> ReceiverMap<'a, Self, T, F> {
        ReceiverMap::new(self, func)
    }
    fn filter<'a, F: FnMut(&T) -> bool>(&'a mut self, func: F) -> ReceiverFilter<'a, Self, T, F> {
        ReceiverFilter::new(self, func)
    }
    fn into_iterator(self) -> ReceiverToIterator<T, Self> {
        ReceiverToIterator::new(self)
    }
}

pub mod threaded;

pub mod iterators;
pub mod map {
    use std::marker::PhantomData;

    use super::{MyReciever, MySender};

    pub struct SenderMap<'a, S: MySender<T>, T, F> {
        sender: &'a mut S,
        func: F,
        p: PhantomData<T>,
    }

    impl<'a, S: MySender<T>, T, F> SenderMap<'a, S, T, F> {
        pub fn new(sender: &'a mut S, func: F) -> Self {
            Self {
                sender,
                func,
                p: PhantomData,
            }
        }
    }
    impl<'a, S: MySender<T>, T, U, F: FnMut(U) -> T> MySender<U> for SenderMap<'a, S, T, F> {
        fn send(&mut self, value: U) -> bool {
            let value = (self.func)(value);
            self.sender.send(value)
        }

        fn set_done(&mut self) {
            self.sender.set_done();
        }
    }

    pub struct ReceiverMap<'a, R: MyReciever<T>, T, F> {
        receiver: &'a mut R,
        func: F,
        p: PhantomData<T>,
    }

    impl<'a, R: MyReciever<T>, T, U, F: FnMut(T) -> U> ReceiverMap<'a, R, T, F> {
        pub fn new(receiver: &'a mut R, func: F) -> Self {
            Self {
                receiver,
                func,
                p: PhantomData,
            }
        }
    }
    impl<'a, R: MyReciever<T>, T, U, F: FnMut(T) -> U> MyReciever<U> for ReceiverMap<'a, R, T, F> {
        fn receive(&mut self) -> Option<U> {
            self.receiver.receive().map(|a| ((self.func)(a)))
        }

        fn isdone(&self) -> bool {
            self.receiver.isdone()
        }
    }
}
pub mod filter {
    use std::marker::PhantomData;

    use super::{MyReciever, MySender};

    pub struct SenderFilter<'a, S: MySender<T>, T, F> {
        sender: &'a mut S,
        func: F,
        p: PhantomData<T>,
    }

    impl<'a, S: MySender<T>, T, F> SenderFilter<'a, S, T, F> {
        pub fn new(sender: &'a mut S, func: F) -> Self {
            Self {
                sender,
                func,
                p: PhantomData,
            }
        }
    }
    impl<'a, S: MySender<T>, T, F: FnMut(&T) -> bool> MySender<T> for SenderFilter<'a, S, T, F> {
        fn send(&mut self, value: T) -> bool {
            if (self.func)(&value) {
                self.sender.send(value)
            } else {
                true
            }
        }

        fn set_done(&mut self) {
            self.sender.set_done();
        }
    }

    pub struct ReceiverFilter<'a, R: MyReciever<T>, T, F> {
        receiver: &'a mut R,
        func: F,
        p: PhantomData<T>,
    }

    impl<'a, R: MyReciever<T>, T, U, F: FnMut(&T) -> U> ReceiverFilter<'a, R, T, F> {
        pub fn new(receiver: &'a mut R, func: F) -> Self {
            Self {
                receiver,
                func,
                p: PhantomData,
            }
        }
    }
    impl<'a, R: MyReciever<T>, T, F: FnMut(&T) -> bool> MyReciever<T> for ReceiverFilter<'a, R, T, F> {
        fn receive(&mut self) -> Option<T> {
            self.receiver.receive().filter(&mut self.func)
        }

        fn isdone(&self) -> bool {
            self.receiver.isdone()
        }
    }
}
pub mod caching_async {
    use std::{
        collections::VecDeque,
        sync::{Arc, Mutex},
    };

    use super::{MyReciever, MySender};

    #[derive(Clone)]
    pub struct ConcurrentCachingTransiever<T>(pub Arc<Mutex<VecDeque<T>>>, bool);

    impl<T> Default for ConcurrentCachingTransiever<T> {
        fn default() -> Self {
            Self(Default::default(), false)
        }
    }

    impl<T> MySender<T> for ConcurrentCachingTransiever<T> {
        fn send(&mut self, value: T) -> bool {
            match self.0.lock() {
                Ok(mut o) => {
                    o.push_back(value);
                    true
                }
                Err(_) => false,
            }
        }

        fn set_done(&mut self) {
            self.1 = true;
        }
    }
    impl<T> MyReciever<T> for ConcurrentCachingTransiever<T> {
        fn receive(&mut self) -> Option<T> {
            match self.0.lock() {
                Ok(mut o) => o.pop_front(),
                Err(e) => panic!("{:?}", e),
            }
        }

        fn isdone(&self) -> bool {
            (match self.0.lock() {
                Ok(l) => l.is_empty(),
                Err(_) => false,
            } && self.1)
        }
    }
}

pub mod caching;
