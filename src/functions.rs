use opml::Outline;

use log::info;
use log::{debug, error, log_enabled, Level};

use std::io::stdout;
use std::io::Write;

use std::process::{exit, Command};

use std::fs;

use std::env;
use std::path::Path;

use feed_rs::parser;

use chrono::{DateTime, Utc};

use std::process::Stdio;

use crossterm::style::Stylize;

use regex::Regex;

// validates link in a file and prints out vectors with links_checked, links_broken and links_error if an error accured
pub fn validate_links(
    links: Vec<String>,
    mut links_checked: Vec<String>,
    mut links_broken: Vec<String>,
    mut links_error: bool,
) -> (Vec<String>, Vec<String>, bool) {
    let out_string: String = "Validating Links".to_string();
    output(0, &out_string, false, false, false);
    for link in links {
        if link.contains("https://www.youtube.com/feeds/videos.xml?channel_id=") == true {
            links_checked.push(link);
        } else {
            links_broken.push(link);
            links_error = true;
        }
    }
    if links_error == true {
        output(2, &out_string, true, true, false);
        println!("Links that are broken:");
        for link_brk in links_broken {
            println!("{}", link_brk);
        }
        println!("exiting");
        exit(9);
    } else {
        output(1, &out_string, true, true, false);
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
    command_exists("wget".to_string(), true);

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
            output(1, &download_information, true, true, false);
        }
    }
    env::set_current_dir(return_path);
}

pub fn command_exists(command: String, stop: bool) -> bool {
    let path_1: String = format!("/bin/{}", command);
    let path_2: String = format!("/usr/bin/{}", command);
    if Path::new(&path_1).exists() == true {
        return true;
    } else if Path::new(&path_2).exists() == true {
        return true;
    } else {
        output(
            2,
            &format!("Command {} not found", &command),
            true,
            true,
            false,
        );
        if stop == true {
            exit(9);
        } else {
            return false;
        }
    }
}

fn progress_bar(string: &String, progres: i32) {
    output(0, string, false, true, false);
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

pub fn download_videos(
    path_links: String,
    path_download: String,
    mut yt_dlp_sett: Vec<String>,
    time: DateTime<Utc>,
) {
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
    command_exists("yt-dlp".to_string(), true);

    // Reading xml files

    let dir_content = fs::read_dir(path_links).unwrap();
    for files in dir_content {
        let file_name = files.unwrap().path();
        debug!("xml files: {:?}", file_name);
        let xml_file = fs::read_to_string(file_name).unwrap();
        let feed = parser::parse(xml_file.as_bytes()).unwrap();
        let title = feed.title.unwrap().content;
        let status_channel: String =
            String::from(format!("Downloading videos for channel: \"{}\":", title));

        let mut showed_beginning_status: bool = false;
        let return_path_download = env::current_dir().unwrap(); // To use later to return to download directory
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
                if showed_beginning_status == false {
                    // shows once the beginning ONLY if there are videos THAT are posted on specified time and need to be downloaded
                    output(3, &status_channel, true, true, true);
                    fs::create_dir_all(&title);
                    env::set_current_dir(&title).is_ok(); // also create the directory once
                    showed_beginning_status = true;
                }
                for video_title in entry.title {
                    let mut video_tittle: String = String::new();
                    let mut countit: usize = 0;
                    let max_chars = 50; // There is a better way to do it counting how many characters a line in terminal fits
                    if &video_title.content.chars().count() > &max_chars {
                        for chars in video_title.content.chars() {
                            if countit < max_chars {
                                countit = countit + 1;
                                video_tittle.push(chars);
                            }
                        }
                        video_tittle.push_str("...");
                    } else {
                        video_tittle = video_title.content.clone();
                    }
                    output(
                        0,
                        &format!("Downloading video: \"{}\"", video_tittle),
                        false,
                        false,
                        false,
                    );
                    for link in &entry.links {
                        // Idk why its a vector but ok

                        let process = Command::new("yt-dlp")
                            // Tell the OS to record the command's output
                            .stdout(Stdio::piped())
                            .args(["--get-duration", &link.href])
                            // execute the command, wait for it to complete, then capture the output
                            .output()
                            // Blow up if the OS was unable to start the program
                            .unwrap();
                        // extract the raw bytes that we captured and interpret them as a string
                        let stdout = String::from_utf8(process.stdout).unwrap();
                        println!("{}", stdout);

                        // Make the repository cleaner! and update toml

                        download_yt(&link.href, yt_dlp_sett.clone());
                    }

                    output(
                        1,
                        &format!("Downloaded video: \"{}\"", video_tittle),
                        true,
                        true,
                        false,
                    );
                }
            }
        }
        env::set_current_dir(return_path_download).is_ok(); // No idea why ../ does not work
    }
    env::set_current_dir(return_path);
}

fn download_yt(link: &String, mut arguments: Vec<String>) {
    arguments.push(link.clone());
    let mut process = Command::new("yt-dlp")
        .args(&*arguments)
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("command failed to start");

    /* old
    let mut process = Command::new("yt-dlp")
        .args(&[
            "-i",
            "--no-playlist",
            "-q",
            "--no-simulate",
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
        */
    process.wait();
}

pub fn output(mark: i8, string: &String, new_line: bool, begin_line: bool, colored: bool) {
    // mark = 0 loading, mark = 1 true, mark = 2 false, mark = 3 nothing,
    // https://docs.rs/crossterm/0.22.1/crossterm/style/index.html
    let mut output_string = string;

    if begin_line == true {
        print!("\r");
    }
    let mut mark_letter: String = String::new();
    let mut letter_1: String = String::from("[");
    let mut letter_2: String = String::from("]");

    if mark == 0 {
        mark_letter = " ".to_string();
    } else if mark == 1 {
        mark_letter = "✓".to_string();
    } else if mark == 2 {
        mark_letter = "𐄂".to_string();
    } else if mark == 3 {
        letter_1 = " ".to_string();
        letter_2 = " ".to_string();
        mark_letter = " ".to_string();
    }

    if colored == true {
        print!(
            "{}{}{} {}",
            letter_1.cyan(),
            mark_letter.dark_red(),
            letter_2.cyan(),
            output_string.clone().dark_green()
        );
    } else {
        print!(
            "{}{}{} {}",
            letter_1.cyan(),
            mark_letter.dark_red(),
            letter_2.cyan(),
            output_string
        );
    }

    if new_line == true {
        print!("\n");
    }
    stdout().flush();
}

pub fn stringto_vector(string: String) -> Vec<String> {
    let mut vector: Vec<String> = Vec::new();
    let mut splitted: Vec<&str> = string.split(",").collect();
    for split in splitted {
        vector.push(split.to_string());
    }
    vector
}

pub fn channel_link(link: String) -> String {
    let mut rss_link: String = String::new();
    let return_path = env::current_dir().unwrap();
    let file_name: String = String::from("yt_link");
    env::set_current_dir("/tmp").is_ok();

    if link.contains("www.youtube.com/channel/") == true
        || link.contains("www.youtube.com/watch") == true
    {
        let process = Command::new("wget")
            .args(&["-q", "-O", &file_name, &link])
            .output()
            .expect("wget command failed to start");

        let contents = fs::read_to_string("yt_link").unwrap();

        let regex = Regex::new(r"UC[-_0-9A-Za-z]{21}[AQgw]").unwrap();

        let captured = regex.captures(&contents).unwrap();

        let id_string = captured.get(0).map_or("", |m| m.as_str());
        rss_link = "https://www.youtube.com/feeds/videos.xml?channel_id=".to_owned() + id_string;

        if command_exists("xclip".to_string(), false) == true {
            let command = format!("echo -n \"{}\" | xclip -selection clipboard", rss_link);
            let mut process = Command::new("bash")
                .args(&["-c", &command])
                .spawn()
                .expect("command failed to start");
            let wait = process.wait();
        }

        fs::remove_file(file_name).unwrap();
        env::set_current_dir(return_path);
        rss_link
    } else {
        println!("Link to a channel needs to be build like: \"https://www.youtube.com/channel/\"");
        String::new()
    }
}
