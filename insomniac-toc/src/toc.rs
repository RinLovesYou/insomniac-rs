use std::{
    fs::File,
    io::{Cursor, Read},
    path::PathBuf,
};

use byteorder::{LittleEndian, ReadBytesExt};
use libflate::zlib::Decoder;

pub struct TocFile {}

impl TocFile {
    pub fn new(path: impl Into<PathBuf>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(path.into())?;

        //get the bytes of the file as an array
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;

        let mut cursor = Cursor::new(data);

        let _ = cursor.read_u32::<LittleEndian>()?; //unknown, possibly an identifier

        let length = cursor.read_i32::<LittleEndian>()?; //length of the compressed data

        let mut buf = vec![0u8; length as usize];
        //write the data to the buffer
        cursor.read(&mut buf)?;

        let mut decoder = Decoder::new(buf.as_slice())?; //zlib decoder

        let mut decompressed_toc = Vec::new();
        decoder.read_to_end(&mut decompressed_toc)?;

        Ok(Self {})
    }
}
