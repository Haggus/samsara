extern crate hyper;
extern crate rss;
extern crate chrono;

mod collector;

use std::thread;

use chrono::*;
use collector::*;

fn main() {
    let mut tick_time = Local::now();

    loop {
        let current_time = Local::now();

        if current_time > tick_time {
            thread::spawn(move || {
                match read_rss() {
                    Some(channel) => {
                        println!("Title:       {}", channel.title);
                        println!("Link:        {}", channel.link);
                        println!("Description: {}", channel.description);
                        println!("Items:       {}", channel.items.len());
                    }
                    None => println!("Could not read RSS"),
                }
            });

            tick_time = current_time + Duration::seconds(30);
        }
    }
}

