extern crate log;
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

// How to get rss from a channel https://pl.unedose.fr/article/how-to-create-an-rss-feed-for-any-youtube-channel
// https://www.youtube.com/feeds/videos.xml?channel_id=
// Get ID https://stackoverflow.com/questions/14366648/how-can-i-get-a-channel-id-from-youtube

fn main() {
    env_logger::init();

    // Get command line arguments
    let mut cliarg: Vec<String> = env::args().collect();
    cliarg.remove(0);
    let mut cliarg_iter = cliarg.iter();

    // Set main variables
    let mut file_name = String::from("yt-rss.opml"); // Default file name

    // Set command line arguments "catchers"
    let file_name_argument: String = String::from("--file-name"); // argument to choose file

    // run through given arguments
    let mut count_iterator: usize = 0; // This variable check whot position is the iterator. there are better ways to do this
    for cliarg in cliarg_iter.clone() {
        // To many clone()
        count_iterator = count_iterator + 1;
        if cliarg == &file_name_argument {
            file_name = cliarg_iter.clone().nth(count_iterator).unwrap().to_string();
        }
    }

    // Get XML links from file
    let mut links: Vec<String> = functions::get_links_file(file_name);

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
    let path_links: String = String::from("/tmp/links");


    // Creating download directory
    let path_download: String = String::from("./download");
    env::set_current_dir("../").is_ok();

    if Path::new(&path_download).exists() == true {
        // this deletes the download directory !
        fs::remove_dir_all(&path_download);
        fs::create_dir_all(&path_download);
    } else {
        fs::create_dir_all(&path_download);
    }
    env::set_current_dir(path_download).is_ok();
    //

    // Check if yt-dlp exists
    if command_exists("yt-dlp".to_string()) == false {
        println!("\r[𐄂] command \"yt-dlp\" not found, exiting");
        exit(9);
    }
    //

    // Reading xml files
    //let time = Utc.ymd(2021, 10, 28).and_hms(0, 0, 0); // time that after they will be
    let time = Utc::today().and_hms(0, 0, 0);

    let dir_content = fs::read_dir("../links/").unwrap();
    for files in dir_content {
        let file_name = files.unwrap().path();
        debug!("xml files: {:?}", file_name);
        let xml_file = fs::read_to_string(file_name).unwrap();
        let feed = parser::parse(xml_file.as_bytes()).unwrap();

        let title = feed.title.unwrap().content;
        println!("Downloading videos for channel: \"{}\":", title);

        fs::create_dir_all(&title);
        env::set_current_dir(&title).is_ok();

        for entry in feed.entries {
            //debug!("{:?}", entry.links);
            //debug!("{:?}", entry.published);

            //let duration = d2.signed_duration_since(d1);
            // if d2 is in the future it will be +, otherwise -
            // https://play.rust-lang.org/?gist=27b579a9b43ce36a4dd0dd0508d426f5&version=stable

            let duration = entry
                .published
                .unwrap()
                .signed_duration_since(time)
                .num_minutes();
            if duration > 0 {
                for video_title in entry.title {
                    print!("[ ] Downloading video: \"{}\"", &video_title.content);
                    stdout().flush();
                    for link in &entry.links {
                        // Idk why its a vector but ok
                        debug!("video link: {:?}", link.href);
                        download_yt(&link.href);
                    }
                    println!("\r[✓] Downloaded video: \"{}\"", &video_title.content);
                }
            }
        }

        env::set_current_dir("../").is_ok();
    }
    //
}

fn download_yt(link: &String) {
    // yt-dlp -i --no-playlist -q --progress -f 'mp4,res:480' --sponsorblock-mark all --add-chapters
    let mut process = Command::new("yt-dlp")
        .args(&[
            "-i",
            "--no-playlist",
            "-q",
            "--progress",
            "-f",
            "mp4,res:480",
            "--sponsorblock-mark",
            "all",
            "--add-chapters",
            &link,
        ])
        .stdin(Stdio::null())
        .spawn()
        .expect("command failed to start");

    process.wait();
}