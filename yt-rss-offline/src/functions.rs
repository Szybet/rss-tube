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

// Loads the .opml file and reads it
pub fn get_links_file(file_name: String) -> Vec<String> {
    let mut file = File::open(file_name.clone()).unwrap();
    let document = OPML::from_reader(&mut file).unwrap();

    let mut links: Vec<String> = Vec::new(); // variable to get all xml links from all sub directories etc.
    print!("[ ] Getting Links from {}", &file_name);
    stdout().flush();

    for outline in &document.body.outlines {
        find_links_loop(outline, &mut links);
    }
    println!("\r[✓] Getting Links from {}", &file_name);
    debug!("links xml: {:?}", links);
    stdout().flush();
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
    print!("[ ] Validating Links");
    stdout().flush();
    for link in links {
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

    // maCheck if wget exists
    if command_exists("wget".to_string()) == false {
        println!("\r[𐄂] command \"wget\" not found, exiting");
        exit(9);
    }
    //

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
            println!("\r[✓] {}", download_information);
        }
    }
    env::set_current_dir(return_path);   
}

pub fn command_exists(command: String) -> bool {
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