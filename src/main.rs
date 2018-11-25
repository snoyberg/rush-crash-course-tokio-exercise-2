extern crate futures;
extern crate tokio;

mod future;
mod interval;

use self::interval::Interval;
use self::future::IntervalFuture;
use tokio::prelude::*;

fn main() {
    let interval = Interval::from_millis(500); // half a second
    let interval_future = IntervalFuture::new(interval);
    let interval_printer = interval_future.map(|curr| {
        println!("Counter is: {}", curr);
    });

    tokio::run(interval_printer)
}
