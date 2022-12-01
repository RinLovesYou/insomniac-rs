use std::{path::{PathBuf, Path}, io::{Read, BufReader, Cursor, Write}, error::Error};

use insomniac_toc::toc::TocFile;



fn main() {
    let default_folder = Path::new("C:\\Program Files (x86)\\Steam\\steamapps\\common\\Marvel's Spider-Man Miles Morales\\asset_archive").to_path_buf();

    let toc = default_folder.join("toc");

    if !toc.exists() {
        println!("toc file not found");
        return;
    }

    let toc = TocFile::new(toc).unwrap();

    println!("toc: {toc:?}");
}