extern crate log;
use opml::Outline;

use log::info;
use log::{debug, error, log_enabled, Level};
use pretty_env_logger::env_logger;

use opml::OPML;
use std::fs::File;

// How to get rss from a channel https://pl.unedose.fr/article/how-to-create-an-rss-feed-for-any-youtube-channel
// Get ID https://stackoverflow.com/questions/14366648/how-can-i-get-a-channel-id-from-youtube

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
    match outline.xml_url.clone() {
        Some(x) => links.push(x),
        None => {
            // Here if xml_url doesnt exist and its not a folder then the loop will not work even one becouse &outline.outlines is []
            for subfolder in &outline.outlines {
                find_links(subfolder, links);
            }
        }
    }
    // Old  function based ot type. its better to just check if xml_url exists, if not then its a folder
    /*
    if outline.r#type == Some("folder".to_string()) || outline.r#type == None {
        debug!("folder detected");
        for nextitem in &outline.outlines {
            debug!("nextitem {:?}", nextitem);
            find_links(nextitem, links);
        }
    } else {
        debug!("feed detected");
        debug!("{:?}", outline);
        let xml = outline.xml_url.clone();
        debug!("XML IS: {:?}", xml);
        match xml {
            Some(x) => links.push(x),
            None => debug!("xml_url is None"),
        }
    }
    */
}
