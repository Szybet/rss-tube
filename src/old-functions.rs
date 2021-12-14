








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

// Loads the .opml file and reads it
pub fn get_links_file(file_name: String) -> Vec<String> {
    let mut file = File::open(file_name.clone()).unwrap();
    let document = OPML::from_reader(&mut file).unwrap();

    let mut links: Vec<String> = Vec::new(); // variable to get all xml links from all sub directories etc.
    let out_string: String = "Getting Links from".to_string();
    output(0, &format!("{} {}", out_string, &file_name), false, false);

    for outline in &document.body.outlines {
        find_links_loop(outline, &mut links);
    }
    output(1, &format!("{} {}", out_string, &file_name), true, true);
    debug!("links xml: {:?}", links);
    links
}