#![feature(test)]

extern crate futures;
extern crate test;

use futures::{Poll, Future, Async};
use futures::executor;
use test::Bencher;

struct NeverBlocks;

impl Future for NeverBlocks {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), ()> {
        Ok(Async::Ready(()))
    }
}

#[bench]
fn wait_new(b: &mut Bencher) {
    let mut task = executor::spawn(NeverBlocks);
    b.iter(|| { task.wait_future() });
}

#[bench]
fn wait_old(b: &mut Bencher) {
    let mut task = executor::spawn(NeverBlocks);
    b.iter(|| { task.wait_future_old() });
}
