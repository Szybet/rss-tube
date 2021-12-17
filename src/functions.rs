use crossterm::execute;
use opml::Outline;

use log::info;
use log::{debug, error, log_enabled, Level};

use std::f32::consts::E;
use std::io::stdout;
use std::io::Write;

use std::process::{exit, id, Command, Output};

use std::fs;

use std::env;
use std::path::Path;

use feed_rs::parser;

use chrono::{DateTime, Utc};

use std::process::Stdio;

use crossterm::style::Stylize;

use regex::Regex;

use std::iter::FromIterator;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use std::time::Duration;

use std::fs::File;

use csv::{ByteRecord, StringRecord};
use std::error::Error;

use opml::OPML;

use std::str;

use std::io::Read;

use std::io::BufRead;
use std::io::BufReader;

use crossterm::cursor::*;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType;
use crossterm::terminal::ClearType::CurrentLine;

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
    let return_path = env::current_dir()
        .expect("current directory does not exist or there are no privileges for it"); // To use later to return to current directory

    if Path::new(&path_links).exists() == true {
        fs::remove_dir_all(&path_links);
        fs::create_dir_all(&path_links);
    } else {
        fs::create_dir_all(&path_links);
    }
    env::set_current_dir(&path_links).is_ok();

    let mut count_downloaded: usize = 0;
    let count_links: usize = links_checked.iter().count() - 1; // It fails if its 0!
    let progress_10 = count_links * 10 / 100;
    let progress_20 = count_links * 20 / 100;
    let progress_30 = count_links * 30 / 100;
    let progress_40 = count_links * 40 / 100;
    let progress_50 = count_links * 50 / 100;
    let progress_60 = count_links * 60 / 100;
    let progress_70 = count_links * 70 / 100;
    let progress_80 = count_links * 80 / 100;
    let progress_90 = count_links * 90 / 100;

    let client = reqwest::blocking::Client::new();
    for link in links_checked {
        let get = client
            .get(&link)
            .send()
            .expect("Failed downloading xml files")
            .bytes()
            .expect("failed converting HTTP get to bytes");

        let file_name = link.clone().replace("https://www.youtube.com/feeds/", "");

        let mut file = File::create(file_name).expect("failed to create XML file");
        file.write_all(&get).expect("failed to write to XML file");

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
        if stop == true {
            output(
                2,
                &format!("Command {} not found", &command),
                true,
                true,
                false,
            );
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
    max_video_time: usize,
) {
    // Creating download directory
    let return_path = env::current_dir().expect("Failed to get current directory"); // To use later to return to current directory

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

    let dir_content = fs::read_dir(path_links).expect("Failed to read directory with xml files");
    for files in dir_content {
        let file_name = files
            .expect("failed to get path to XML files, the directory is propably empty")
            .path();
        debug!("xml files: {:?}", file_name);
        let xml_file = fs::read_to_string(file_name).expect("failed to read file");
        let feed = parser::parse(xml_file.as_bytes()).expect("failed to parse xml file");
        let title = feed
            .title
            .expect("failed to get tittl from xml file")
            .content;
        let status_channel: String =
            String::from(format!("Downloading videos for channel: \"{}\":", title));
        let mut status_channel_bool: bool = false; // Variable to prevent showing status_channel twice

        let mut showed_beginning_status: bool = false;
        let return_path_download = env::current_dir().expect("Failed to get current directory"); // To use later to return to download directory
        for entry in feed.entries {
            //debug!("{:?}", entry.links);
            //debug!("{:?}", entry.published);

            //let duration = d2.signed_duration_since(d1);
            // if d2 is in the future it will be +, otherwise -
            // https://play.rust-lang.org/?gist=27b579a9b43ce36a4dd0dd0508d426f5&version=stable

            let duration = entry
                .published
                .expect("failed to compare dates")
                .signed_duration_since(time)
                .num_minutes();
            if duration > 0 {
                for video_title in entry.title {
                    let mut video_tittle: String = String::new();
                    let mut countit: usize = 0;
                    let max_chars = 40; // There is a better way to do it counting how many characters a line in terminal fits
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
                    for link in &entry.links {
                        // Idk why its a vector but ok
                        let mut stdout_duration =
                            download_yt(&link.href, vec!["--get-duration".to_string()], true)
                                .expect("getting duration of video");
                        let video_duration_sec: usize = string_to_time(stdout_duration);


                        // max_video_time is already turned to seconds
                        let mut video_time_accept: bool = false;
                        let mut is_future: bool = false;
                        if max_video_time > video_duration_sec {
                            video_time_accept = true;
                        }
                        if video_duration_sec == 0 { // This means its in the future ( premieres )
                            video_time_accept = false;
                        }

                        if video_time_accept == true {
                            if showed_beginning_status == false {
                                // Its here becouse it would create directory but then the videos could be too long
                                // shows once the beginning ONLY if there are videos THAT are posted on specified time and need to be downloaded
                                if status_channel_bool == false {
                                    output(3, &status_channel, true, true, true);
                                    status_channel_bool = true;
                                }
                                fs::create_dir_all(&title);
                                env::set_current_dir(&title).is_ok(); // also create the directory once
                                showed_beginning_status = true;
                            }
                            if showed_beginning_status == true {
                                // it will only show if showed_beginning_status is true so it will not say before status_channel
                                output(
                                    0,
                                    &format!("Downloading video: \"{}\"", video_tittle),
                                    true,
                                    true,
                                    false,
                                );
                            }
                            download_yt(&link.href, yt_dlp_sett.clone(), false);

                            execute!(stdout(), MoveUp(1), Clear(ClearType::CurrentLine)); // Goes up and clears up the Downloading video: message to overwrite it
                            
                            output(
                                // It goes only one time anyway
                                1,
                                &format!("Downloaded video: \"{}\"", video_tittle),
                                true,
                                true,
                                false,
                            );
                        } else {
                            if status_channel_bool == false {
                                output(3, &status_channel, true, true, true);
                                status_channel_bool = true;
                            }
                            let mut video_error_status: String = String::new();
                            if is_future == true {
                                video_error_status = "Video is a premiere, in the future".to_string();
                            } else {
                                video_error_status = format!("Video: {} is too long ({} minutes)", video_tittle, (video_duration_sec / 60));
                            }

                            output(
                                // It goes only one time anyway
                                2,
                                &video_error_status,
                                true,
                                true,
                                false,
                            );
                        }
                    }
                }
            }
        }
        env::set_current_dir(return_path_download).is_ok(); // No idea why ../ does not work
    }
    env::set_current_dir(return_path);
}

/// process_output_bool = false -> does not output a "output process", but it does output text to the terminal
///
/// ---
///
/// process_output_bool = true -> returns string with stdout and does output() the stderr
fn download_yt(
    link: &String,
    mut arguments: Vec<String>,
    process_output_bool: bool,
) -> Option<String> {
    arguments.push(link.clone());
    if process_output_bool == false {
        let mut process = Command::new("yt-dlp")
            .args(&*arguments)
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("command failed to start");
        process.wait();
        return None;
    } else {
        let mut process = Command::new("yt-dlp")
            .args(&*arguments)
            .stdin(Stdio::null())
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("command failed to start");

        let process_output = process
            .wait_with_output()
            .expect("failed too wait for process yt-dlp");

        if process_output.stderr.is_empty() == false {
            let error =
                str::from_utf8(&process_output.stderr).expect("failed to convert stderr to string");
            println!("{}", error);
        }

        let string_to_option = str::from_utf8(&process_output.stdout)
            .expect("failed to convert stdout to string")
            .to_string();
        let mut return_option: Option<String> = None;
        Option::insert(&mut return_option, string_to_option);
        return return_option;
    }
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
    let return_path = env::current_dir().expect("Failed to get current directory");
    let file_name: String = String::from("yt_link");
    env::set_current_dir("/tmp").is_ok();

    if link.contains("www.youtube.com/watch") == true {
        let get = reqwest::blocking::get(&link)
            .expect("failed to download site")
            .bytes()
            .expect("failed converting HTTP get to bytes");

        let contents: String = str::from_utf8(&get)
            .expect("failed to convert bytes to string from downloaded yt file")
            .to_string();

        let regex = Regex::new(r"UC[-_0-9A-Za-z]{21}[AQgw]").expect("wrong regex expression");

        let captured = regex
            .captures(&contents)
            .expect("failed to capture with regex");

        let id_string = captured.get(0).map_or("", |m| m.as_str());
        rss_link = "https://www.youtube.com/feeds/videos.xml?channel_id=".to_owned() + id_string;

        fs::remove_file(file_name).expect("failed to remove downloaded file");
        env::set_current_dir(return_path);
    } else if link.contains("www.youtube.com/channel/") == true {
        // There is propably a better way to do this
        let mut id_string: String = String::new();
        if link.contains("https") == true {
            id_string = link.replace("https://www.youtube.com/channel/", "");
        } else {
            id_string = link.replace("http://www.youtube.com/channel/", "");
        }
        rss_link = "https://www.youtube.com/feeds/videos.xml?channel_id=".to_owned() + &id_string;
    } else {
        println!("Link to a channel needs to be build like: \"https://www.youtube.com/channel/\" and its build like : {}", link);
        return String::new();
    }
    if command_exists("xclip".to_string(), false) == true {
        let command = format!("echo -n \"{}\" | xclip -selection clipboard", rss_link);
        let mut process = Command::new("bash")
            .args(&["-c", &command])
            .spawn()
            .expect("command failed to start");
        let wait = process.wait();
    }
    rss_link
}

pub fn string_to_time(string: String) -> usize {
    let mut yt_duration = Vec::from_iter(string.split(":").map(&String::from));

    // Thats becouse the last item has \n
    let mut last_item = yt_duration
        .last()
        .expect("failed to get last item from vector yt_duration")
        .replace("\n", "");
    yt_duration.remove(yt_duration.iter().count() - 1);
    yt_duration.push(last_item);
    //

    let mut seconds: usize = 0;
    let mut minutes: usize = 0;
    let mut hours: usize = 0;

    seconds = vector_parse(&mut yt_duration);
    minutes = vector_parse(&mut yt_duration);
    hours = vector_parse(&mut yt_duration);

    // let mut time = Duration::new(seconds,0); // No becouse it only uses sec and nanosec
    let mut time_sec: usize = (seconds) + (minutes * 60) + (hours * 60 * 60); // simplest but it works
    time_sec
}

pub fn vector_parse(yt_duration: &mut Vec<String>) -> usize {
    let mut output: usize = 0;
    let last_iter = yt_duration.last().cloned();
    let mut count_vector = yt_duration.iter().count();
    if count_vector == 0 {
        // If its 0, there are no items and it cant -1 becouse it will panic
        return 0;
    } else {
        count_vector = count_vector - 1; // Becouse it counts items, and then it access it from 0
        yt_duration.remove(count_vector);

        match last_iter {
            Some(out_type) => {
                let output_result = out_type.parse::<usize>();
                match output_result {
                    Ok(v) => output = v,
                    Err(e) => output = 0,
                }
            }
            None => {}
        }
        output
    }
}

pub fn csv_to_opml(csv_file_path: String, opml_file_path: String) {
    let csv = File::open(csv_file_path);
    match csv {
        Ok(file) => {
            if std::path::Path::new(&opml_file_path).exists() == true {
                println!("file {} exists, exiting", opml_file_path);
                exit(9);
            }
            let mut opml_file =
                File::create(opml_file_path).expect("Failed to create opml file, exiting");

            output(3, &"Parsing files:".to_string(), true, false, true);

            let mut opml = OPML::default();
            let mut rdr = csv::Reader::from_reader(file);
            for result in rdr.records() {
                match result {
                    Ok(stri_rec) => {
                        let byte_record = ByteRecord::from(stri_rec);
                        let str_record = StringRecord::from_byte_record(byte_record)
                            .expect("Failed to parse csv record to UTF");

                        // 0 - ID
                        // 1 - link
                        // 2 - name
                        // let idk = str_record.get(2).expect("CSV file record empty");

                        let name = str_record.get(2).expect("CSV record is empty").to_string();
                        let link = str_record.get(1).expect("CSV record is empty").to_string();
                        let link = channel_link(link);
                        if link.is_empty() == true {
                            println!("exiting");
                            exit(9);
                        }
                        opml.add_feed(&name, &link);
                    }
                    Err(e) => {
                        println!("Error reading csv file: {} \n,exiting", e);
                        exit(9);
                    }
                }
            }
            opml.to_writer(&mut opml_file)
                .expect("failed to write to opml file");
            output(1, &"OPML file created".to_string(), true, false, false)
        }
        Err(e) => {
            println!(".csv file not found, exiting");
            exit(9);
        }
    }
}
