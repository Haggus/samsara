//use std::thread;

use self::rss_channel::RssChannel;
use chrono::*;

mod rss_channel;

pub struct Collector {
    items: Vec<RssChannel>
}

impl Collector {
    pub fn new() -> Collector {
        let mut temp_vec = Vec::new();
        let temp_rss = RssChannel::new("https://www.gamingonlinux.com/article_rss.php".to_owned(), Duration::seconds(3));
        temp_vec.push(temp_rss);
        
        Collector {
            items: temp_vec
        }
    }

    pub fn collect(&mut self) {
        loop {
            let current_time = Local::now();

            for i in &mut self.items {
                if current_time > i.tick {
                    // thread::spawn(move || {
                        match i.read() {
                            Some(channel) => {
                                println!("Title:       {}", channel.title);
                                println!("Link:        {}", channel.link);
                                println!("Description: {}", channel.description);
                                println!("Items:       {}", channel.items.len());
                            }
                            None => println!("Could not read RSS"),
                        }
                    // });
                    i.tick = current_time + i.refresh;
                }
            }
        }
    }
}
