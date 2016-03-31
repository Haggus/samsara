extern crate hyper;
extern crate rss;
extern crate chrono;

mod collector;

use collector::*;

fn main() {
    let mut collector = Collector::new();
    collector.collect();
}

