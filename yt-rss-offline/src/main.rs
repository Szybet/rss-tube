extern crate log;
use opml::Outline;

use log::info;
use log::{debug, error, log_enabled, Level};
use pretty_env_logger::env_logger;

use opml::OPML;
use std::fs::File;

// How to get rss from a channel https://pl.unedose.fr/article/how-to-create-an-rss-feed-for-any-youtube-channel

fn main() {
    env_logger::init();
    let mut file = File::open("rss.opml").unwrap();
    let document = OPML::from_reader(&mut file).unwrap();

    let mut links: Vec<String> = Vec::new(); // variable to get all xml links from all sub directories etc.

    for outline in &document.body.outlines {
        debug!("");
        find_links(outline, &mut links);
    }
    debug!("FINAL: {:?}", links);
}

fn find_links(outline: &Outline, links: &mut Vec<String>) {
    debug!("{:?}", outline.r#type);
    if outline.r#type == Some("folder".to_string()) {
        debug!("folder detected");
        for nextitem in &outline.outlines {
            debug!("nextitem {:?}", nextitem);
            find_links(nextitem, links);
        }
    } else {
        debug!("feed detected");
        debug!("{:?}", outline);
        links.push(outline.xml_url.clone().unwrap());
    } // dodac ze jak none to ignoruje!
}
