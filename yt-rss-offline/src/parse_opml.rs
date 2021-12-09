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

use crate::functions::output;

// Gets categories from a OPML file
#[derive(Debug, Clone)]
pub struct category {
    name: String,
    children: Vec<category>,
    links: Vec<String>,
}

// Main function for parsing the OPML file to a tree structure
pub fn get_categories(file_name: String) -> category {
    let mut file = File::open(file_name.clone()).unwrap();
    let document = OPML::from_reader(&mut file).unwrap();
    let out_string: String = "Getting categories from".to_string();
    output(0, &format!("{} {}", out_string, &file_name), false, false, false);

    let mut categories: category = file_loop(document, "Main".to_string());

    //debug!("{:?}", categories);

    output(1, &format!("{} {}", out_string, &file_name), true, true, false);
    categories
}

// Function that starts the loop, initialises start of the structure
fn file_loop(document: OPML, category_name: String) -> category {
    let mut categories = category {
        name: category_name,
        children: Vec::new(),
        links: outlines_loop_get_links(document.body.outlines.clone()),
    };

    for outline in &document.body.outlines {
        match outline.r#type.clone() {
            Some(out_type) => {
                if out_type == "category" || out_type == "folder" {
                    categories.children.push(outlines_loop_categories(
                        outline.outlines.clone(),
                        outline.text.clone(),
                    ));
                }
            }
            None => {}
        }
    }
    categories
}

// Function that gets all links from an outline
fn outlines_loop_get_links(outlines: Vec<Outline>) -> Vec<String> {
    let mut links: Vec<String> = Vec::new();

    for outline in outlines {
        match outline.xml_url.clone() {
            Some(x) => links.push(x),
            None => {}
        }
    }
    links
}

// main loop to get all categories
fn outlines_loop_categories(outlines: Vec<Outline>, category_name: String) -> category {
    let mut categories = category {
        name: category_name,
        children: Vec::new(),
        links: outlines_loop_get_links(outlines.clone()),
    };
    for outline in outlines {
        match outline.r#type.clone() {
            Some(out_type) => {
                if out_type == "category" || out_type == "folder" {
                    categories.children.push(outlines_loop_categories(
                        outline.outlines.clone(),
                        outline.text.clone(),
                    ));
                }
            }
            None => {}
        }
    }
    categories
}

// Get links from specified categories
// Well it does not work correctly if two cattegories have the same name... ugh
pub fn unpack_categories(
    mut category_main: category,
    categories_names: Vec<String>,
) -> Vec<String> {
    let mut links: Vec<String> = Vec::new();
    
    let searched_categories = search_category(category_main, categories_names);
    for categories in searched_categories {
        links.append(&mut links_subcategories(categories));
    }

    links
}

// Get category with specified names
pub fn search_category(
    mut category_main: category,
    categories_names: Vec<String>,
) -> Vec<category> {
    let mut categories_searched: Vec<category> = Vec::new();
    if categories_names.clone().contains(&category_main.name) {
        categories_searched.push(category_main.clone());
    }

    for categories in category_main.children {
        categories_searched.append(&mut search_category(categories, categories_names.clone()))
    }
    categories_searched
}

// Gets all links from category and children
pub fn links_subcategories(mut category_main: category) -> Vec<String> {
    let mut links: Vec<String> = Vec::new();
    links.append(&mut category_main.links);
    for categories in category_main.children {
        links.append(&mut links_subcategories(categories));
    }
    links
}
