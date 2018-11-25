extern crate futures;

use super::interval::Interval;
use future::futures::prelude::*;

pub struct IntervalFuture {
    interval: Interval,
    last: usize,
}

impl IntervalFuture {
    pub fn new(interval: Interval) -> IntervalFuture {
        let last = interval.get_counter();
        IntervalFuture { interval, last }
    }
}

impl Future for IntervalFuture {
    type Item = usize;
    type Error = ();

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        let curr = self.interval.get_counter();
        if curr == self.last {
            let task = futures::task::current();
            self.interval.set_task(task);
            Ok(Async::NotReady)
        } else {
            self.last = curr;
            Ok(Async::Ready(curr))
        }
    }
}
