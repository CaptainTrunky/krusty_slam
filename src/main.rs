mod data;

use std::path::PathBuf;
use data::stream::build_disk_stream;

fn main() {
    println!("Hello, world!");

    let src = PathBuf::new();

    let stream = build_disk_stream(src);

    for idx in 0..stream.images.len() {
      println!("{}", stream.images[idx].display());
      println!("{}", stream.depths[idx].display());
    }
}
