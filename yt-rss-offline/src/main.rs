use std::fs::File;
use std::io::BufReader;
use rss::Channel;

fn main() {
let file = File::open("example.xml").unwrap();
let channel = Channel::read_from(BufReader::new(file)).unwrap();
println!("hell");
}
