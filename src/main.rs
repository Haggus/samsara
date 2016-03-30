extern crate hyper;
extern crate rss;
extern crate chrono;

use std::io::*;
use std::ops::Sub;

use hyper::Client;
use hyper::header::Connection;
use rss::{Rss, Channel};
use chrono::*;

fn main() {
    let start_time = Local::now().time();

    match read_rss() {
        Some(channel) => {
            println!("Title:       {}", channel.title);
            println!("Link:        {}", channel.link);
            println!("Description: {}", channel.description);
            println!("Items:       {}", channel.items.len());
        }
        None => println!("Could not read RSS"),
    }

    let total = Local::now().time().sub(start_time);
    println!("time: {}", total);
}

fn read_rss() -> Option<Channel> {
    let client = Client::new();

    let mut res = client.get("https://www.gamingonlinux.com/article_rss.php")
                        .header(Connection::close())
                        .send()
                        .unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    match body.parse::<Rss>() {
        Ok(p) => Some(p.0),
        Err(e) => {
            println!("Error: {}", e);
            None
        }
    }
}
