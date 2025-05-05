use std::collections::VecDeque;

use super::{MyReciever, MySender};

#[derive(Clone)]
pub struct CachingTransciever<T>(pub VecDeque<T>, bool);

impl<T> Default for CachingTransciever<T> {
    fn default() -> Self {
        Self(Default::default(), false)
    }
}
impl<T> AsRef<VecDeque<T>> for CachingTransciever<T> {
    fn as_ref(&self) -> &VecDeque<T> {
        &self.0
    }
}
impl<T> AsMut<VecDeque<T>> for CachingTransciever<T> {
    fn as_mut(&mut self) -> &mut VecDeque<T> {
        &mut self.0
    }
}

impl<T> MySender<T> for CachingTransciever<T> {
    fn send(&mut self, value: T) -> bool {
        self.0.push_back(value);
        true
    }

    fn set_done(&mut self) {
        self.1 = true;
    }
}
impl<T> MyReciever<T> for CachingTransciever<T> {
    fn receive(&mut self) -> Option<T> {
        self.0.pop_front()
    }

    fn isdone(&self) -> bool {
        self.0.is_empty() && self.1
    }
}
