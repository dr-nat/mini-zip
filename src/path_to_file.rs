//use crate::inputs;
use std::{
    fs::*,
    io::BufReader,
    //io::Read,
    path::PathBuf,
    error::Error,
};

pub fn read_file(file: PathBuf) -> Result<BufReader<File>, Box<dyn Error>> {
    let file_contents = File::open(&file)?;

    let buffer = BufReader::new(file_contents);

    Ok(buffer)
}
