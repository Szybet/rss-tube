use log::info;
use log::{debug, error, log_enabled, Level};
use pretty_env_logger::env_logger;

use std::process::exit;

use std::env;

use chrono::{TimeZone, Utc};

mod functions;
mod parse_opml;

// How to get rss from a channel https://pl.unedose.fr/article/how-to-create-an-rss-feed-for-any-youtube-channel
// https://www.youtube.com/feeds/videos.xml?channel_id=
// Get ID https://stackoverflow.com/questions/14366648/how-can-i-get-a-channel-id-from-youtube
// Get channels links
// https://www.youtube.com/feed/channels

fn main() {
    env_logger::init();

    // Get command line arguments
    let mut cliarg: Vec<String> = env::args().collect();
    cliarg.remove(0);
    let mut cliarg_iter = cliarg.iter();

    // Set main variables, here are defaults
    let mut file_name = String::from("yt-rss.opml");
    let mut path_links: String = String::from("/tmp/links");
    let mut path_download: String = String::from("download");
    let mut requested_categories: Vec<String> = vec!["Main".to_string()];
    // yt-dlp default arguments
    let mut yt_dlp_sett: Vec<String> = vec![
        "-i".to_string(),
        "--no-playlist".to_string(),
        "-q".to_string(),
        "--progress".to_string(),
        "-f".to_string(),
        "mp4,res:480".to_string(),
        "--embed-thumbnail".to_string(),
        "--sponsorblock-mark".to_string(),
        "all".to_string(),
        "--add-chapters".to_string(),
        // It doesnt work with big files?
        //"--max-filesize".to_string(),
        // "1K".to_string(),
    ];

    // "-i,--no-playlist,-q,--progress,-f,mp4,res:480,--embed-thumbnail,--sponsorblock-mark,all,--add-chapters,--buffer-size,16k,--http-chunk-size,100M,"


    let mut time = Utc::today().and_hms(0, 0, 0); // Date that videos older than it will be not downloaded
    let help_information: String = String::from("
    arguments:    
    --file-name [Path] - Specifies the path and file name to the OPML file
    --links-directory [Path] - Specifies the path and folder name to where save XML files (rss information of specified channels)
    --download-directory [Path] - Specifies the path and folder name to where save videos
    --set-categories \"category1,category2,category3\" - Chooses from whot categories from OPML file download
    --yt-dlp-arguments \"argument1,argument2,argument3\" - Specifies custom arguments for yt-dlp
    --time \"YYYY,MM,DD,HH\" - Specifies time that older than it, videos will be ignored and not downloaded. HH means Hours of the day
    --channel-link [url] - Turns a yt channel link to a rss link to that channel. if xclip is installed, it puts it to clipboard
    --help - shows this message
    --max-video-time - Specifies the maximum time of a video, in minutes
    --csv-to-opml - Converts the CSV file from youtube export subscription.csv file to a opml file, ready to use. The syntax is: --csv-to-opml csv_file new_opml_file");
    let mut max_video_time: usize = 30 * 60; // Max video time in minutes

    // Set command line arguments "catchers"
    let file_name_argument: String = String::from("--file-name"); // argument to choose file
    let path_links_argument: String = String::from("--links-directory"); // argument to choose where to store links
    let path_download_argument: String = String::from("--download-directory"); // argument to choose where to store videos
    let requested_categories_argument: String = String::from("--set-categories"); // argument to choose categories for links from the file
                                                                                  // after this argument there should be: "category1,category2,category3"
    let yt_dlp_sett_arguments: String = String::from("--yt-dlp-arguments"); // This argument also usues "thing,thing1"
    let time_arguments: String = String::from("--time"); // This argument also usues "thing,thing1" also the syntax is YYYY-MM-DD
    let channel_link_arguments: String = String::from("--channel-link");
    let help_arguments: String = String::from("--help");
    let max_video_time_arguments: String = String::from("--max-video-time");
    let convert_csv: String = String::from("--csv-to-opml"); // syntax: --csv-to-opml csv_file opml_file

    // run through given arguments
    let mut count_iterator: usize = 0; // This variable check whot position is the iterator. there are better ways to do this
    let mut none_arguments = true;
    for cliarg in cliarg_iter.clone() {
        // To many clone() :/


        count_iterator = count_iterator + 1;
        if cliarg == &file_name_argument {
            none_arguments = false;
            file_name = cliarg_iter.clone().nth(count_iterator).unwrap().to_string();
        }
        if cliarg == &path_links_argument {
            none_arguments = false;
            path_links = cliarg_iter.clone().nth(count_iterator).unwrap().to_string();
        }
        if cliarg == &path_download_argument {
            none_arguments = false;
            path_download = cliarg_iter.clone().nth(count_iterator).unwrap().to_string();
        }
        if cliarg == &requested_categories_argument {
            none_arguments = false;
            requested_categories = functions::stringto_vector(
                cliarg_iter.clone().nth(count_iterator).unwrap().to_string(),
            ); // Parse "thing,thing1" to vector
        }
        if cliarg == &yt_dlp_sett_arguments {
            none_arguments = false;
            yt_dlp_sett = functions::stringto_vector(
                cliarg_iter.clone().nth(count_iterator).unwrap().to_string(),
            ); // Parse "thing,thing1" to vector
        }
        if cliarg == &time_arguments {
            none_arguments = false;
            let vector = functions::stringto_vector(
                cliarg_iter.clone().nth(count_iterator).unwrap().to_string(),
            );
            time = Utc
                .ymd(
                    vector[0].parse::<i32>().unwrap(),
                    vector[1].parse::<u32>().unwrap(),
                    vector[2].parse::<u32>().unwrap(),
                )
                .and_hms(vector[3].parse::<u32>().unwrap(), 0, 0); // time that after they will be
        }
        if cliarg == &channel_link_arguments {
            none_arguments = false;
            let link: String = cliarg_iter.clone().nth(count_iterator).unwrap().to_string();
            println!("The RSS link:");
            println!("{}", functions::channel_link(link));
            exit(9);
        }
        if cliarg == &help_arguments {
            none_arguments = false;
            println!("{}", help_information);
            exit(9);
        }
        if cliarg == &max_video_time_arguments {
            none_arguments = false;
            let max_video_time_parse = cliarg_iter.clone().nth(count_iterator).unwrap().to_string().parse::<usize>();
            match max_video_time_parse {
                Ok(v) => max_video_time = v * 60, // turn it here into seconds
                Err(e) => {
                    println!("--max-video-time argument number couldnt be paused, exiting");
                    exit(9);
                },
            }
        }
        if cliarg == &convert_csv {
            none_arguments = false;
            let csv_path: String = cliarg_iter.clone().nth(count_iterator).unwrap().to_string();
            let opml_path: String = cliarg_iter.clone().nth(count_iterator + 1).unwrap().to_string();
            functions::csv_to_opml(csv_path, opml_path);
            exit(9);
        }
    }

    // If 0 arguments supplied, or they are wrong show the help message
    if none_arguments == true {
        println!("  0 arguments supplied, or they are valid. showing help message:");
        println!("{}", help_information);
        exit(9);
    }

    // Get XML links from file
    let mut file_to_struct = parse_opml::get_categories(file_name);
    //println!("{:#?}", &file_to_struct);
    let links = parse_opml::unpack_categories(file_to_struct, requested_categories);

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
    functions::download_videos(path_links, path_download, yt_dlp_sett, time, max_video_time);
}
