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

// How to get rss from a channel https://pl.unedose.fr/article/how-to-create-an-rss-feed-for-any-youtube-channel
// https://www.youtube.com/feeds/videos.xml?channel_id=
// Get ID https://stackoverflow.com/questions/14366648/how-can-i-get-a-channel-id-from-youtube

fn main() {
    env_logger::init();
    // Getting links from a file
    let file_name: String = String::from("yt-rss.opml");
    let mut file = File::open(file_name.clone()).unwrap();
    let document = OPML::from_reader(&mut file).unwrap();

    let mut links: Vec<String> = Vec::new(); // variable to get all xml links from all sub directories etc.
    print!("[ ] Getting Links from {}", &file_name);
    stdout().flush();

    for outline in &document.body.outlines {
        find_links(outline, &mut links);
    }
    println!("\r[✓] Getting Links from {}", &file_name);
    debug!("links xml: {:?}", links);
    stdout().flush();
    //

    // Checking if links are good
    print!("[ ] Validating Links");
    stdout().flush();
    let mut links_checked: Vec<String> = Vec::new();
    let mut links_broken: Vec<String> = Vec::new();
    let mut links_error: bool = false;
    for link in links.clone() {
        if link.contains("https://www.youtube.com/feeds/videos.xml?channel_id=") == true {
            links_checked.push(link);
        } else {
            links_broken.push(link);
            links_error = true;
        }
    }
    if links_error == true {
        println!("\r[𐄂] Validating Links");
        println!("Links that are broken:");
        for link_brk in links_broken {
            println!("{}", link_brk);
        }
        println!("exiting");
        exit(9);
    } else {
        println!("\r[✓] Validating Links");
    }
    //

    // Downloading links from file
    let download_information: String = String::from("XML Download progress");

    let path_links: String = String::from("./links");
    if Path::new(&path_links).exists() == true {
        fs::remove_dir_all(&path_links);
        fs::create_dir_all(&path_links);
    } else {
        fs::create_dir_all(&path_links);
    }
    env::set_current_dir(&path_links).is_ok();

    let mut count_downloaded: usize = 0;
    let count_links: usize = links.iter().count() - 1;
    let progress_10 = count_links * 10 / 100;
    let progress_20 = count_links * 20 / 100;
    let progress_30 = count_links * 30 / 100;
    let progress_40 = count_links * 40 / 100;
    let progress_50 = count_links * 50 / 100;
    let progress_60 = count_links * 60 / 100;
    let progress_70 = count_links * 70 / 100;
    let progress_80 = count_links * 80 / 100;
    let progress_90 = count_links * 90 / 100;

    // Check if wget exists
    if command_exists("wget".to_string()) == false {
        println!("\r[𐄂] command \"wget\" not found, exiting");
        exit(9);
    }
    //

    for link in links {
        let process = Command::new("wget")
            .args(&["-q", &link])
            .output()
            .expect("wget command failed to start");
        count_downloaded = count_downloaded + 1;
        if count_downloaded == progress_10 {
            progress_bar(&download_information, 1)
        } else if count_downloaded == progress_20 {
            progress_bar(&download_information, 2)
        } else if count_downloaded == progress_30 {
            progress_bar(&download_information, 3)
        } else if count_downloaded == progress_40 {
            progress_bar(&download_information, 4)
        } else if count_downloaded == progress_50 {
            progress_bar(&download_information, 5)
        } else if count_downloaded == progress_60 {
            progress_bar(&download_information, 6)
        } else if count_downloaded == progress_70 {
            progress_bar(&download_information, 7)
        } else if count_downloaded == progress_80 {
            progress_bar(&download_information, 8)
        } else if count_downloaded == progress_90 {
            progress_bar(&download_information, 9)
        } else if count_downloaded == count_links {
            print!("\r");
            for x in 0..50 {
                print!(" ");
            }
            println!("\r[✓] {}", download_information);
        }
    }
    //

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

fn command_exists(command: String) -> bool {
    let path_1: String = format!("/bin/{}", command);
    let path_2: String = format!("/usr/bin/{}", command);
    if Path::new(&path_1).exists() == true {
        return true;
    } else if Path::new(&path_2).exists() == true {
        return true;
    }
    false
}

fn progress_bar(string: &String, progres: i32) {
    print!("\r{}", string);
    print!(" [");
    for x in 1..progres + 1 {
        if x == progres {
            print!(">");
        } else {
            print!("-");
        }
    }
    let missing_progres = 11 - progres;
    for x in 1..missing_progres {
        print!("#");
    }
    print!("]");
    stdout().flush();
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
