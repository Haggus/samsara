use std::io::*;

use hyper::Client;
use hyper::header::Connection;
use rss::{Rss, Channel};

pub fn read_rss() -> Option<Channel> {
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
