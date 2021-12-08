extern crate log;
use functions::download_videos;
use opml::Outline;

use log::info;
use log::{debug, error, log_enabled, Level};
use pretty_env_logger::env_logger;

use opml::OPML;
use std::fs::File;
use std::thread::sleep;

use std::io::stdout;
use std::io::Write;

use std::{thread, time};

use std::process::{exit, Command};

use std::fs;

use std::env;
use std::path::Path;

use feed_rs::parser;

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

use std::process::Stdio;

mod functions;
mod parse_opml;




// How to get rss from a channel https://pl.unedose.fr/article/how-to-create-an-rss-feed-for-any-youtube-channel
// https://www.youtube.com/feeds/videos.xml?channel_id=
// Get ID https://stackoverflow.com/questions/14366648/how-can-i-get-a-channel-id-from-youtube

fn main() {
    env_logger::init();

    // Get command line arguments
    let mut cliarg: Vec<String> = env::args().collect();
    cliarg.remove(0);
    let mut cliarg_iter = cliarg.iter();

    // Set main variables, here are defaults
    let mut file_name = String::from("yt-rss.opml");
    let mut path_links: String = String::from("/tmp/links");
    let mut path_download: String = String::from("./download");
    
    // Set command line arguments "catchers"
    let file_name_argument: String = String::from("--file-name"); // argument to choose file
    let path_links_argument: String = String::from("--links-directory"); // argument to choose where to store links
    let path_download_argument: String = String::from("--download-directory"); // argument to choose where to store videos


    // run through given arguments
    let mut count_iterator: usize = 0; // This variable check whot position is the iterator. there are better ways to do this
    for cliarg in cliarg_iter.clone() {
        // To many clone()
        count_iterator = count_iterator + 1;
        if cliarg == &file_name_argument {
            file_name = cliarg_iter.clone().nth(count_iterator).unwrap().to_string();
        }
        if cliarg == &path_links_argument {
            path_links = cliarg_iter.clone().nth(count_iterator).unwrap().to_string();
        }
        if cliarg == &path_download_argument {
            path_download = cliarg_iter.clone().nth(count_iterator).unwrap().to_string();
        }
    }

    // Get XML links from file
    //let mut links: Vec<String> = functions::get_links_file(file_name);
    let mut nw = parse_opml::get_categories(file_name);
    debug!("{:#?}", &nw);
    let veccc: Vec<String> = vec!["Polska".to_string(), "Tech".to_string()];
    let test = parse_opml::unpack_categories(nw, veccc);
    debug!("hmmmmmmmmmm   {:#?}", &test);
    
    /*

    // Checking if links are good, becouse if not wget will get a loop
    let mut links_checked: Vec<String> = Vec::new();
    let mut links_broken: Vec<String> = Vec::new();
    let mut links_error: bool = false;

    let var_func =
        functions::validate_links(links.clone(), links_checked, links_broken, links_error);
    links_checked = var_func.0;
    links_broken = var_func.1;
    links_error = var_func.2;
    drop(links); // It is not needed more becouse everything is in links_x

    // Download XML iles
    functions::download_xml(links_checked, path_links.clone());

    // Download videos
    download_videos(path_links, path_download);
    
    */
}

