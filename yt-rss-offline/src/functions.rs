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

use crossterm::style::Stylize;
use crossterm::style::{
    Attribute, Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor,
};
use crossterm::{execute, Result};

// Loads the .opml file and reads it
pub fn get_links_file(file_name: String) -> Vec<String> {
    let mut file = File::open(file_name.clone()).unwrap();
    let document = OPML::from_reader(&mut file).unwrap();

    let mut links: Vec<String> = Vec::new(); // variable to get all xml links from all sub directories etc.
    let out_string: String = "Getting Links from".to_string();
    output(0, &format!("Getting Links from {}", &file_name), false, false);

    for outline in &document.body.outlines {
        find_links_loop(outline, &mut links);
    }
    output(1, &format!("Getting Links from {}", &file_name), true, true);
    debug!("links xml: {:?}", links);
    links
}

fn find_links_loop(outline: &Outline, links: &mut Vec<String>) {
    match outline.xml_url.clone() {
        Some(x) => links.push(x),
        None => {
            // Here if xml_url doesnt exist and its not a folder then the loop will not work even one becouse &outline.outlines is []
            for subfolder in &outline.outlines {
                find_links_loop(subfolder, links);
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

// validates link in a file and prints out vectors with links_checked, links_broken and links_error if an error accured
pub fn validate_links(
    links: Vec<String>,
    mut links_checked: Vec<String>,
    mut links_broken: Vec<String>,
    mut links_error: bool,
) -> (Vec<String>, Vec<String>, bool) {
    let out_string: String = "Validating Links".to_string();
    output(0, &out_string, false, false);
    for link in links {
        if link.contains("https://www.youtube.com/feeds/videos.xml?channel_id=") == true {
            links_checked.push(link);
        } else {
            links_broken.push(link);
            links_error = true;
        }
    }
    if links_error == true {
        output(2, &out_string, true, true);
        println!("Links that are broken:");
        for link_brk in links_broken {
            println!("{}", link_brk);
        }
        println!("exiting");
        exit(9);
    } else {
        output(1, &out_string, true, true);
    }
    return (links_checked, links_broken, links_error);
}
pub fn download_xml(links_checked: Vec<String>, path_links: String) {
    // Downloading links from file
    let download_information: String = String::from("XML Download progress");
    let return_path = env::current_dir().unwrap(); // To use later to return to current directory

    if Path::new(&path_links).exists() == true {
        fs::remove_dir_all(&path_links);
        fs::create_dir_all(&path_links);
    } else {
        fs::create_dir_all(&path_links);
    }
    env::set_current_dir(&path_links).is_ok();

    let mut count_downloaded: usize = 0;
    let count_links: usize = links_checked.iter().count() - 1;
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
    command_exists("wget".to_string());

    for link in links_checked {
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
            output(1, &download_information, true, true);
        }
    }
    env::set_current_dir(return_path);
}

pub fn command_exists(command: String) {
    let path_1: String = format!("/bin/{}", command);
    let path_2: String = format!("/usr/bin/{}", command);
    if Path::new(&path_1).exists() == true {
    } else if Path::new(&path_2).exists() == true {
    } else {
        output(2, &format!("Command {} not found", &command), true, true);
        exit(9);
    }
}

fn progress_bar(string: &String, progres: i32) {
    output(0, string, false, true);
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

pub fn download_videos(path_links: String, path_download: String) {
    // Creating download directory
    let return_path = env::current_dir().unwrap(); // To use later to return to current directory

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
    command_exists("yt-dlp".to_string());

    // Reading xml files
    let time = Utc.ymd(2021, 12, 1).and_hms(0, 0, 0); // time that after they will be
                                                      //let time = Utc::today().and_hms(0, 0, 0);

    let dir_content = fs::read_dir(path_links).unwrap();
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
                    output(0, &format!("Downloading video: \"{}\"", &video_title.content), false, false);
                    for link in &entry.links {
                        // Idk why its a vector but ok
                        debug!("video link: {:?}", link.href);
                        download_yt(&link.href);
                    }
                    output(1, &format!("Downloaded video: \"{}\"", &video_title.content), true, true);
                }
            }
        }
        env::set_current_dir("../").is_ok();
    }
    env::set_current_dir(return_path);
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

pub fn output(mark: i8, string: &String, new_line: bool, begin_line: bool) {
    // mark = 0 loading, mark = 1 true, mark = 2 false
    // https://docs.rs/crossterm/0.22.1/crossterm/style/index.html
    let mut output_string = string;

    if begin_line == true {
        print!("\r");
    }
    let mut mark_letter: String = String::new();

    if mark == 0 {
        mark_letter = " ".to_string();
    } else if mark == 1 {
        mark_letter = "✓".to_string();
    } else if mark == 2 {
        mark_letter = "𐄂".to_string();
    }

    print!(
        "{}{}{} {}",
        "[".cyan(),
        mark_letter.dark_red(),
        "]".cyan(),
        output_string
    );

    if new_line == true {
        print!("\n");
    }
    stdout().flush();
}
